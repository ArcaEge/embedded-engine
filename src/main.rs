// No-std and no-main only if target is Pico
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

// ARM only
#[cfg(target_arch = "arm")]
use {
    bsp::entry, defmt::*, defmt_rtt as _, embedded_hal::digital::OutputPin, panic_probe as _,
    rp_pico as bsp,
};

// WASM only
#[cfg(target_arch = "wasm32")]
use {
    log::{debug, info},
    std::{thread, time::Duration},
    wasm_bindgen::prelude::*,
};

mod engine;
mod game;

// Pico main
#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    debug!("Program start");

    engine::Engine::new();

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

// Make cargo shut up about not having a main (workaround because we're using bin instead of lib for package type)
#[cfg(target_arch = "wasm32")]
fn main() {}
