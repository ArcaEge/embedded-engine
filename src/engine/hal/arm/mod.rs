use super::super::Inputs;
use alloc::rc::Rc;
use core::cell::RefCell;

mod display;
mod peripherals_io;

#[allow(clippy::upper_case_acronyms)]
pub struct HAL {
    peripherals: peripherals_io::PeripheralsIO,
    display: display::Display,
    pub inputs: Rc<RefCell<[bool; Inputs::VARIANT_COUNT]>>,
}

impl HAL {
    pub fn new() -> Self {
        // Peripherals
        let mut peripherals = peripherals_io::PeripheralsIO::new();

        // Display
        let display = display::Display::new(peripherals.i2c.take().unwrap());

        // Inputs
        let inputs = [false; Inputs::VARIANT_COUNT];

        Self {
            peripherals,
            display,
            inputs: Rc::new(RefCell::new(inputs)),
        }
    }
}
