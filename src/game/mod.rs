use crate::engine::{EngineInteractionLayer, GameTrait};

// Game stuff goes here
pub struct Game {}

impl GameTrait for Game {
    fn new() -> Self {
        Self {}
    }

    fn init(&mut self, engine: &mut EngineInteractionLayer) {}

    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        let _ = engine.set_pixel_state_check_bounds(
            tick_count as i32 % 128,
            (tick_count as i32 / 128) % 64,
            true,
        );
    }
}
