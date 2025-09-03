mod actors;
mod sounds;
mod sprites;
pub mod world_actor_abstractions;
mod worlds;

use crate::engine::{alloc::Box, *};
pub use world_actor_abstractions::*;

// Game stuff goes here
pub struct Game {
    world: Box<dyn WorldTrait>,
}

impl GameTrait for Game {
    fn new() -> Self {
        Self {
            world: worlds::MainWorld::create(),
        }
    }

    fn init(&mut self, engine: &mut EngineInteractionLayer) {
        // Currently not utilised but could be used for stuff
        let mut interaction_layer = GameInteractionLayer {};

        self.world.init(&mut interaction_layer, engine);
    }

    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        let mut interaction_layer = GameInteractionLayer {};

        self.world
            .as_mut()
            .tick(tick_count, &mut interaction_layer, engine);
    }

    fn render(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        // epilepsy inducer
        // engine.framebuffer.inverted = !engine.framebuffer.inverted;

        let mut interaction_layer = GameInteractionLayer {};

        self.world
            .as_mut()
            .render(tick_count, &mut interaction_layer, engine);
    }
}
