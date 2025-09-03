#[cfg(target_arch = "arm")]
pub use alloc::{boxed::Box, rc::Rc, vec::Vec};

#[cfg(target_arch = "wasm32")]
pub use std::{boxed::Box, rc::Rc, vec::Vec};
