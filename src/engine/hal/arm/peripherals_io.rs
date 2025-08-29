use super::super::super::Inputs;
use super::HAL;
use bsp::hal::{
    Clock as _, I2C,
    clocks::init_clocks_and_plls,
    fugit::RateExtU32,
    gpio::{self, FunctionI2C, Pin},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use cortex_m::delay::Delay;
use embedded_hal::digital::InputPin as _;
use iter_variants::IterVariants;
use rp_pico as bsp;

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

        Self {
            delay,
            i2c: Some(i2c),
            timer,
            input_pins,
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
    fn delay_us(self: &mut Self, us: u32) {
        self.delay.delay_us(us);
    }

    /// Returns whether the given input is active
    pub(super) fn input_is_active(&mut self, input: Inputs) -> bool {
        let input_pin = &mut self.input_pins[input as usize];
        input_pin.is_low().unwrap()
    }
}

impl HAL {
    /// Get the current timestamp in microseconds
    pub fn micros(&self) -> u64 {
        self.peripherals.micros()
    }

    /// Delay for a number of microseconds
    pub fn delay_us(self: &mut Self, us: u32) {
        self.peripherals.delay_us(us);
    }

    /// Delay until a given timestamp
    pub fn delay_until_us(&mut self, until: u64) {
        let current_timestamp = self.micros();
        self.delay_us((until - current_timestamp) as u32);
    }

    pub fn update_inputs(&mut self) {
        // Iterate over inputs
        Inputs::iter_variants(|input| {
            // This has got to be dumb in some way, surely
            self.inputs.borrow_mut()[input as usize] = self.peripherals.input_is_active(input);
        });
    }
}
