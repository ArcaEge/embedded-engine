// No-std and no-main only if target is Pico
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

// ARM only
#[cfg(target_arch = "arm")]
use {defmt::*, defmt_rtt as _, embedded_alloc::TlsfHeap as Heap, panic_probe as _};
#[cfg(target_arch = "arm")]
extern crate alloc;

// WASM only
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
pub fn pico_main() -> ! {
    debug!("Program start");

    // Set up heap
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let engine: Engine<Game> = Engine::new();
    engine.start(60.0); // 60 ticks/second seems like an OK number
}

// WASM main
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_main() {
    console_log::init_with_level(log::Level::Info).unwrap();

    let engine: Engine<Game> = Engine::new();
    engine.start(60.0).await; // 60 ticks/second seems like an OK number
}
