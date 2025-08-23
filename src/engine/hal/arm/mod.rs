mod display;
mod peripherals_io;

pub struct HAL {
    peripherals: peripherals_io::PeripheralsIO,
    display: display::Display,
}

impl HAL {
    pub fn new() -> Self {
        // Peripherals
        let mut peripherals = peripherals_io::PeripheralsIO::new();

        // Display
        let display = display::Display::new(peripherals.i2c.take().unwrap());

        Self {
            peripherals,
            display,
        }
    }
}
