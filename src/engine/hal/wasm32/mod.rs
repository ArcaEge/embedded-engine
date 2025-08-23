mod display;

pub struct HAL {}

impl HAL {
    pub fn new() -> Self {
        Self {}
    }

    pub fn micros(&self) -> u64 {
        0
        // self.peripherals.micros()
    }

    // Delay for a number of milliseconds
    pub fn delay_ms(self: &mut Self, ms: u32) {
        // self.peripherals.delay_ms(ms);
    }

    // Delay for a number of microseconds
    pub fn delay_us(self: &mut Self, us: u32) {
        // self.peripherals.delay_us(us);
    }
}
