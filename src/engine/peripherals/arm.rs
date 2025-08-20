use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use rp_pico as bsp;

pub struct PeripheralBundle {
    pub xosc: Option<pac::XOSC>,
    pub clocks: Option<pac::CLOCKS>,
    pub pll_sys: Option<pac::PLL_SYS>,
    pub pll_usb: Option<pac::PLL_USB>,
    pub resets: Option<pac::RESETS>,
    pub pins: bsp::Pins,
    pub watchdog: Option<pac::WATCHDOG>,
}

impl PeripheralBundle {
    pub fn new() -> Self {
        let mut board_peripherals = pac::Peripherals::take().unwrap();
        let sio = Sio::new(board_peripherals.SIO);

        let pins = bsp::Pins::new(
            board_peripherals.IO_BANK0,
            board_peripherals.PADS_BANK0,
            sio.gpio_bank0,
            &mut board_peripherals.RESETS,
        );

        Self {
            xosc: Some(board_peripherals.XOSC),
            clocks: Some(board_peripherals.CLOCKS),
            pll_sys: Some(board_peripherals.PLL_SYS),
            pll_usb: Some(board_peripherals.PLL_USB),
            resets: Some(board_peripherals.RESETS),
            pins,
            watchdog: Some(board_peripherals.WATCHDOG),
        }
    }
}
