mod actors;
mod sounds;
mod sprites;
pub mod world_actor_abstractions;
mod worlds;

use crate::engine::{
    alloc::{Box, Rc},
    *,
};
use postcard::from_bytes;
pub use world_actor_abstractions::*;

// Game stuff goes here
pub struct Game {
    world: Box<dyn WorldTrait>,
    spritesheet: Rc<Spritesheet>,
}

impl GameTrait for Game {
    fn new() -> Self {
        let spritesheet_bytes = include_bytes!("sprites/spritesheet.embsprite");
        let spritesheet_initial: SpritesheetInitial = from_bytes(spritesheet_bytes)
            .expect("Failed to parse spritesheet file, invalid format");
        let spritesheet = Rc::new(Spritesheet::from(spritesheet_initial));
        Self {
            world: worlds::MainWorld::create(spritesheet.clone()),
            spritesheet: spritesheet,
        }
    }

    fn init(&mut self, engine: &mut EngineInteractionLayer) {
        // Currently not utilised but could be used for stuff
        let mut interaction_layer = GameInteractionLayer {
            spritesheet: &self.spritesheet,
        };

        self.world.init(&mut interaction_layer, engine);
    }

    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        let mut interaction_layer = GameInteractionLayer {
            spritesheet: &self.spritesheet,
        };

        self.world
            .as_mut()
            .tick(tick_count, &mut interaction_layer, engine);
    }

    fn render(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        // epilepsy inducer
        // engine.framebuffer.inverted = !engine.framebuffer.inverted;

        let mut interaction_layer = GameInteractionLayer {
            spritesheet: &self.spritesheet,
        };

        self.world
            .as_mut()
            .render(tick_count, &mut interaction_layer, engine);
    }
}
