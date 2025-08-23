use crate::engine::{EngineInteractionLayer, GameTrait};

pub struct Game {}

impl GameTrait for Game {
    fn new() -> Self {
        Self {}
    }

    fn init(&mut self, engine: &mut EngineInteractionLayer) {}

    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        engine.set_pixel_state(
            tick_count as usize % 128,
            (tick_count as usize / 128) % 64,
            true,
        );
    }
}
