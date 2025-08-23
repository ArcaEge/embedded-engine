use super::super::InputsState;
use alloc::rc::Rc;
use core::cell::RefCell;

mod display;
mod peripherals_io;

pub struct HAL {
    peripherals: peripherals_io::PeripheralsIO,
    display: display::Display,
    pub inputs: Rc<RefCell<InputsState>>,
}

impl HAL {
    pub fn new() -> Self {
        // Peripherals
        let mut peripherals = peripherals_io::PeripheralsIO::new();

        // Display
        let display = display::Display::new(peripherals.i2c.take().unwrap());

        // Inputs
        let inputs = InputsState {
            ..Default::default()
        };

        Self {
            peripherals,
            display,
            inputs: Rc::new(RefCell::new(inputs)),
        }
    }
}
