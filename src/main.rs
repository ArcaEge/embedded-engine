// No-std and no-main only if target is Pico
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

// ARM only
#[cfg(target_arch = "arm")]
use {
    bsp::entry, defmt::*, defmt_rtt as _, embedded_alloc::TlsfHeap as Heap,
    embedded_hal::digital::OutputPin, panic_probe as _, rp_pico as bsp,
};
#[cfg(target_arch = "arm")]
extern crate alloc;

// WASM only
#[cfg(target_arch = "wasm32")]
use {
    log::{debug, info},
    std::{thread, time::Duration},
    wasm_bindgen::prelude::*,
};

mod engine;
mod game;

use engine::Engine;
use game::Game;

#[cfg(target_arch = "arm")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 65536; // 64 KiB

// Pico main
#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    debug!("Program start");

    // Set up heap
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    common_main();
}

// WASM main
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_log::init_with_level(log::Level::Info).unwrap();

    info!("hello world!");
}

fn common_main() -> ! {
    let mut engine: Engine<Game> = Engine::new();
    engine.start(60.0); // 60 ticks/second seems like an OK number
}

// Make cargo shut up about not having a main (workaround because we're using bin instead of lib for package type)
#[cfg(target_arch = "wasm32")]
fn main() {}
