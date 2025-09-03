use super::super::world_actor_abstractions::{
    ActorTrait, GameInteractionLayer, WorldInteractionLayer,
};
use crate::engine::{EngineInteractionLayer, Point, Sprite, Spritesheet, alloc::Rc};

/// The player
pub struct Player {
    location: Point,
    sprite: Rc<Sprite>,
}

impl Player {
    pub fn create(location: Point, spritesheet: &Spritesheet) -> Self {
        Self {
            location,
            sprite: spritesheet
                .sprites
                .get(0)
                .expect("Failed to get sprite 0")
                .clone(),
        }
    }
}

impl ActorTrait for Player {
    fn init(
        &mut self,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        _engine: &mut EngineInteractionLayer,
    ) {
    }

    fn tick(
        &mut self,
        _tick_count: u64,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        _engine: &mut EngineInteractionLayer,
    ) {
    }

    fn get_location(&self) -> Point {
        self.location
    }

    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }
}
