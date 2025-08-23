// No-std and no-main only if target is Pico
#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

// ARM only
#[cfg(target_arch = "arm")]
use embedded_engine::pico_main;
#[cfg(target_arch = "arm")]
use {bsp::entry, defmt_rtt as _, panic_probe as _, rp_pico as bsp};

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    pico_main();
}

// Make rust-analyzer shut up about not having a main
#[cfg(not(target_arch = "arm"))]
fn main() {}
