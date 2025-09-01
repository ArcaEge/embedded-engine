#[cfg(target_arch = "arm")]
pub use alloc::{boxed::Box, vec::Vec};

#[cfg(target_arch = "wasm32")]
pub use std::{boxed::Box, vec::Vec};
