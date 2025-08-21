mod hal;
use defmt::info;
use hal::HAL;

// Game engine
pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        let mut hal = HAL::new();
        info!("{}", hal.micros());
        hal.delay_ms(50);
        info!("{}", hal.micros());
        Self {}
    }
}
