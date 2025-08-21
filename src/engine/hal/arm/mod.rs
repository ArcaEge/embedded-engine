pub mod peripherals_io;

pub struct HAL {
    peripherals: peripherals_io::PeripheralsIO,
}

impl HAL {
    pub fn new() -> Self {
        let peripherals = peripherals_io::PeripheralsIO::new();
        Self { peripherals }
    }
}
