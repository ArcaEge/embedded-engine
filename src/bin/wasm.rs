// No-std and no-main only if target is Pico (shouldn't be needed under normal circumstances, this is just to make rust-analyzer shut up)
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]
#![no_main]

// WASM only
#[cfg(target_arch = "wasm32")]
use {embedded_engine::wasm_main, wasm_bindgen::prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
fn main() {
    wasm_main();
}
