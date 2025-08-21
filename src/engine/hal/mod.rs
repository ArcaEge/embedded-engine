#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
#[cfg(target_arch = "wasm32")]
pub use wasm32::*;
