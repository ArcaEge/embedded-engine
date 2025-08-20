use crate::engine::peripherals::PeripheralBundle;
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
    pub fn new(peripheral_bundle: &mut PeripheralBundle) -> Self {
        let mut watchdog = Watchdog::new(
            peripheral_bundle
                .watchdog
                .take()
                .expect("watchdog already taken"),
        );
        let core = pac::CorePeripherals::take().unwrap();

        // Initialise clocks and PLLs
        let clocks_manager = init_clocks_and_plls(
            EXTERNAL_OSCILLATOR_FREQ_HZ,
            peripheral_bundle.xosc.take().expect("xosc already taken"),
            peripheral_bundle
                .clocks
                .take()
                .expect("clocks already taken"),
            peripheral_bundle
                .pll_sys
                .take()
                .expect("pll_sys already taken"),
            peripheral_bundle
                .pll_usb
                .take()
                .expect("pll_usb already taken"),
            &mut peripheral_bundle
                .resets
                .as_mut()
                .expect("resets already taken"),
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
