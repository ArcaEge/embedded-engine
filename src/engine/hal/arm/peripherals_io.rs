use super::super::super::Inputs;
use super::HAL;
use bsp::hal::{
    Clock as _, I2C,
    clocks::{ClocksManager, init_clocks_and_plls},
    fugit::RateExtU32,
    gpio::{self, FunctionI2C, Pin},
    pac,
    pwm::{Pwm2, Slice, Slices},
    sio::Sio,
    watchdog::Watchdog,
};
use cortex_m::delay::Delay;
use embedded_hal::{digital::InputPin as _, pwm::SetDutyCycle};
use iter_variants::IterVariants;
use libm::roundf;
use rp_pico::{self as bsp, hal::pwm::FreeRunning};

const EXTERNAL_OSCILLATOR_FREQ_HZ: u32 = 12_000_000u32;

pub(super) type I2CType = I2C<
    pac::I2C0,
    (
        Pin<gpio::bank0::Gpio4, gpio::FunctionI2c, gpio::PullUp>,
        Pin<gpio::bank0::Gpio5, gpio::FunctionI2c, gpio::PullUp>,
    ),
>;

type InputPin = Pin<gpio::DynPinId, gpio::FunctionSio<gpio::SioInput>, gpio::PullUp>;

/// Peripherals, I/O, clock
pub(super) struct PeripheralsIO {
    timer: pac::TIMER,
    pub(super) i2c: Option<I2CType>,
    delay: Delay,
    input_pins: [InputPin; Inputs::VARIANT_COUNT],
    clocks: ClocksManager,
    pwm: Slice<Pwm2, FreeRunning>,
}

impl PeripheralsIO {
    pub fn new() -> Self {
        let mut board_peripherals = pac::Peripherals::take().unwrap();
        let sio = Sio::new(board_peripherals.SIO);

        let pins = bsp::Pins::new(
            board_peripherals.IO_BANK0,
            board_peripherals.PADS_BANK0,
            sio.gpio_bank0,
            &mut board_peripherals.RESETS,
        );

        let mut watchdog = Watchdog::new(board_peripherals.WATCHDOG);
        let core = pac::CorePeripherals::take().unwrap();

        // Initialise clocks and PLLs
        let clocks_manager = init_clocks_and_plls(
            EXTERNAL_OSCILLATOR_FREQ_HZ,
            board_peripherals.XOSC,
            board_peripherals.CLOCKS,
            board_peripherals.PLL_SYS,
            board_peripherals.PLL_USB,
            &mut board_peripherals.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay = Delay::new(core.SYST, clocks_manager.system_clock.freq().to_Hz());
        let timer = board_peripherals.TIMER;

        // Initialise I2C pins
        let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio4.reconfigure();
        let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio5.reconfigure();

        // Initialise I2C itself
        let i2c: I2CType = I2C::i2c0(
            board_peripherals.I2C0,
            sda_pin,
            scl_pin,
            1.MHz(), // Using 1MHz, because why not? Somewhat overkill but seems to handle it just fine up to 3MHz
            &mut board_peripherals.RESETS,
            &clocks_manager.system_clock,
        );

        // Initialise pins
        let input_pins: [InputPin; Inputs::VARIANT_COUNT] = [
            pins.gpio6.into_pull_up_input().into_dyn_pin(),  // Up
            pins.gpio7.into_pull_up_input().into_dyn_pin(),  // Down
            pins.gpio8.into_pull_up_input().into_dyn_pin(),  // Left
            pins.gpio9.into_pull_up_input().into_dyn_pin(),  // Right
            pins.gpio10.into_pull_up_input().into_dyn_pin(), // Jump
        ];

        // PWM for sound
        let pwm_slices = Slices::new(board_peripherals.PWM, &mut board_peripherals.RESETS);
        let mut pwm = pwm_slices.pwm2; // GPIO20 is on slice 2
        pwm.channel_a.output_to(pins.gpio20);
        pwm.enable();

        // Set the PWM off for now
        pwm.channel_a
            .set_duty_cycle_percent(0)
            .expect("PWM error, failed to set the PWM state");

        Self {
            delay,
            i2c: Some(i2c),
            timer,
            input_pins,
            clocks: clocks_manager,
            pwm,
        }
    }

    /// Returns the number of microseconds since boot
    fn micros(&self) -> u64 {
        // Always read timelr before timehr
        let lower = self.timer.timelr().read().bits() as u64;
        let higher = self.timer.timehr().read().bits() as u64;
        (higher << 32) | lower
    }

    /// Delay for a number of microseconds
    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us);
    }

    /// Returns whether the given input is active
    pub(super) fn input_is_active(&mut self, input: Inputs) -> bool {
        let input_pin = &mut self.input_pins[input as usize];
        input_pin.is_low().unwrap()
    }

    /// Sets the frequency of the sound tone
    fn set_sound_freq(&mut self, freq: f32) {
        /*
        Ok so this bit is a little complicated and requires some algebra so here's a shortish explanation:
        PWM has a counter inside, it's on for [duty cycle] ticks, off until the counter reaches top and resets.

        Top = what number the pwm counter counts up to before resetting, inclusive so counter resets at top + 1
        Div = pwm clock divider

        If div = 5, pwm counter increments every 5 system_clock_freqs
        Div is u8 (max 255), top is u16 (max 65535), overflow = bad because then frequency is wrong :(

        To calculate the frequency produced with a certain clock freq, div and top, we can make this equation using those facts:
        freq = (system_clock_freq / div) / (top + 1) = system_clock_freq / (div * (top + 1))
        Using top + 1 instead of top because top is inclusive as I said, so top + 1 is actually what the pwm counter resets at

        To avoid overflowing top, I'll keep its value fixed and calculate a div instead (this is a lot harder to overflow).
        But this is imprecise because div is an integer (well, technically not but let's ignore that for now) and it is a low number.
        This would mean that pitches would become inaccurate.
        So, the best solution is to calculate a div using a desired top, round this div to an int, then calculate the actual top from that.
        This results in top that is somewhere around the desired top whilst keeping the frequency accurate and also being able to
        accommodate a wide range of output frequencies.
        Technically, div could overflow but that won't happen unless your frequency is below 15Hz or something like that.
        Also it (I tested with logic analyser) seems to handle up to 1.6Mhz, very much more than enough for producing sound.

        So here are the two rearranged equations to calculate div and top:
        div = system_clock_freq / (freq * (top + 1))
        top = system_clock_freq / (freq * div) - 1

        I'll use a desired_top of 32768 (2^16 / 2) because it seems like a sensible number, although anything reasonable can be used; it's arbitrary within reason.
        */

        const DESIRED_TOP: u16 = 32768;
        let system_clock_freq = self.clocks.system_clock.freq().to_Hz();

        // max() ensures that div is never below 1
        let div = max(
            roundf(system_clock_freq as f32 / (freq * (DESIRED_TOP as f32 + 1.0))) as u8,
            1,
        );
        let top = roundf(system_clock_freq as f32 / (freq * div as f32)) as u16 - 1;

        self.pwm.set_div_int(div);
        self.pwm.set_top(top);
    }

    /// Sets the state of the sound, i.e. whether a tone is being played or not
    fn set_sound_state(&mut self, state: bool) {
        // No if statement needed :D
        self.pwm
            .channel_a
            .set_duty_cycle_percent(50 * state as u8)
            .expect("PWM error, failed to set the PWM state");
    }
}

impl HAL {
    /// Get the current timestamp in microseconds
    pub fn micros(&self) -> u64 {
        self.peripherals.micros()
    }

    /// Delay for a number of microseconds
    pub fn delay_us(&mut self, us: u32) {
        self.peripherals.delay_us(us);
    }

    /// Delay until a given timestamp
    pub fn delay_until_us(&mut self, until: u64) {
        let current_timestamp = self.micros();

        self.delay_us(until.saturating_sub(current_timestamp) as u32);
    }

    pub fn update_inputs(&mut self) {
        // Iterate over inputs
        Inputs::iter_variants(|input| {
            // This has got to be dumb in some way, surely
            self.inputs.borrow_mut()[input as usize] = self.peripherals.input_is_active(input);
        });
    }

    /// Sets the frequency of the sound tone
    pub fn set_sound_freq(&mut self, freq: f32) {
        self.peripherals.set_sound_freq(freq);
    }

    /// Sets the state of the sound, i.e. whether a tone is being played or not
    pub fn set_sound_state(&mut self, state: bool) {
        self.peripherals.set_sound_state(state);
    }
}

fn max(a: u8, b: u8) -> u8 {
    if a > b { a } else { b }
}
