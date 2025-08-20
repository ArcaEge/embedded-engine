#[cfg(target_arch = "arm")]
pub mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
