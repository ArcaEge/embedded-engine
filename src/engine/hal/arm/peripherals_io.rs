use super::HAL;
use bsp::hal::{
    Clock as _, I2C, Timer,
    clocks::init_clocks_and_plls,
    fugit::RateExtU32,
    gpio::{self, FunctionI2C, Pin},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use cortex_m::delay::Delay;
use rp_pico as bsp;

const EXTERNAL_OSCILLATOR_FREQ_HZ: u32 = 12_000_000u32;

type I2CType = I2C<
    pac::I2C0,
    (
        Pin<gpio::bank0::Gpio4, gpio::FunctionI2c, gpio::PullUp>,
        Pin<gpio::bank0::Gpio5, gpio::FunctionI2c, gpio::PullUp>,
    ),
>;

// Peripherals, I/O, clock
pub(super) struct PeripheralsIO {
    timer: pac::TIMER,
    i2c: I2CType,
    delay: Delay,
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

        let i2c: I2CType = I2C::i2c0(
            board_peripherals.I2C0,
            sda_pin,
            scl_pin,
            400.kHz(),
            &mut board_peripherals.RESETS,
            &clocks_manager.system_clock,
        );

        Self { delay, i2c, timer }
    }

    // Returns the number of microseconds since boot
    fn micros(&self) -> u64 {
        // Always read timelr before timehr
        let lower = self.timer.timelr().read().bits() as u64;
        let higher = self.timer.timehr().read().bits() as u64;
        (higher << 32) | lower
    }

    // Delay for a number of milliseconds
    fn delay_ms(self: &mut Self, ms: u32) {
        self.delay.delay_ms(ms);
    }

    // Delay for a number of microseconds
    fn delay_us(self: &mut Self, us: u32) {
        self.delay.delay_us(us);
    }
}

impl HAL {
    pub fn micros(&self) -> u64 {
        self.peripherals.micros()
    }

    // Delay for a number of milliseconds
    pub fn delay_ms(self: &mut Self, ms: u32) {
        self.peripherals.delay_ms(ms);
    }

    // Delay for a number of microseconds
    pub fn delay_us(self: &mut Self, us: u32) {
        self.peripherals.delay_us(us);
    }
}
