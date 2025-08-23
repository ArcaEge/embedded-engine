use gloo_timers::future::sleep;
use std::time::Duration;

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
    pub async fn delay_ms(self: &mut Self, ms: u32) {
        sleep(Duration::from_millis(ms as u64)).await;
    }

    // Delay for a number of microseconds (converts to milliseconds, doesn't actually delay for microseconds)
    pub async fn delay_us(self: &mut Self, us: u32) {
        sleep(Duration::from_millis(
            ((us as f32) / 1000.0f32).round() as u64
        ))
        .await;
    }

    pub async fn delay_until_us(&mut self, until: u64) {
        let current_timestamp = self.micros();
        self.delay_us((until - current_timestamp) as u32).await;
    }
}
