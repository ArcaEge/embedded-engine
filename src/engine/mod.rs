pub mod clock;
pub mod display;
pub mod inputs;
mod peripherals;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        let mut peripheral_bundle = peripherals::PeripheralBundle::new();
        let clock = clock::Clock::new(&mut peripheral_bundle);
        Self {}
    }
}
