use super::hal::HAL;
use bsp::hal::{
    Clock as _,
    clocks::{ClocksManager, init_clocks_and_plls},
    pac,
    watchdog::Watchdog,
};
use cortex_m::delay::Delay;
use rp_pico as bsp;

const EXTERNAL_OSCILLATOR_FREQ_HZ: u32 = 12_000_000u32;

pub struct Clock {
    clocks_manager: ClocksManager,
    delay: Delay,
}

impl Clock {
    // Constructor for clock
    pub fn new(
        watchdog: pac::WATCHDOG,
        xosc: pac::XOSC,
        clocks: pac::CLOCKS,
        pll_sys: pac::PLL_SYS,
        pll_usb: pac::PLL_USB,
        resets: &mut pac::RESETS,
    ) -> Self {
        let mut watchdog = Watchdog::new(watchdog);
        let core = pac::CorePeripherals::take().unwrap();

        // Initialise clocks and PLLs
        let clocks_manager = init_clocks_and_plls(
            EXTERNAL_OSCILLATOR_FREQ_HZ,
            xosc,
            clocks,
            pll_sys,
            pll_usb,
            resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay = Delay::new(core.SYST, clocks_manager.system_clock.freq().to_Hz());

        Self {
            clocks_manager,
            delay,
        }
    }

    // Delay for a number of milliseconds
    pub fn delay_ms(self: &mut Self, ms: u32) {
        self.delay.delay_ms(ms);
    }

    // Delay for a number of microseconds
    pub fn delay_us(self: &mut Self, us: u32) {
        self.delay.delay_us(us);
    }
}

impl HAL {}
