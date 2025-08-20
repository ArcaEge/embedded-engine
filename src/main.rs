// No-std and no-main only if target is Pico
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

// ARM only
#[cfg(target_arch = "arm")]
use {
    bsp::entry,
    bsp::hal::{
        clocks::{Clock, init_clocks_and_plls},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    },
    defmt::*,
    defmt_rtt as _,
    embedded_hal::digital::OutputPin,
    panic_probe as _,
};

// Provide an alias for our BSP so we can switch targets quickly.
#[cfg(target_arch = "arm")]
use rp_pico as bsp;

// WASM only
#[cfg(target_arch = "wasm32")]
use {
    log::{debug, info},
    std::{thread, time::Duration},
    wasm_bindgen::prelude::*,
};

// Pico main
#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let _pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    delay.delay_ms(500);
    info!("hello world!");

    loop {}
}

// WASM main
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_log::init_with_level(log::Level::Info).unwrap();

    info!("hello world!");
}

// Make cargo shut up about not having a main (workaround because we're using bin instead of lib)
#[cfg(target_arch = "wasm32")]
fn main() {}
