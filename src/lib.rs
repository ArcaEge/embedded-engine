//! A simple sprite-based game engine that can run on both an RP2040 with an SSD1306 display and also on web assembly
//! without any code changes, just a different compile target, thanks to conditional compilation.

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

// Import the entire engine and game for rustdoc purposes
pub mod engine;
pub mod game;

use game::Game;

/// Tick rate of the engine
///
/// 60 ticks/second seems like an OK number
///
/// TODO: Make this a parameter passed from the game
pub const TICK_RATE: f32 = 60.0;

#[cfg(target_arch = "arm")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 65536; // 64 KiB

// Pico main
#[doc(hidden)]
#[cfg(target_arch = "arm")]
pub fn pico_main() -> ! {
    debug!("Program start");

    // Set up heap
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let engine: engine::Engine<Game> = engine::Engine::new();
    engine.start(TICK_RATE);
}

// WASM main
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_main() {
    console_log::init_with_level(log::Level::Info).unwrap();

    let engine: engine::Engine<Game> = engine::Engine::new();
    engine.start(TICK_RATE).await;
}
