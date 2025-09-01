use super::super::sprites::PLAYER_SPRITE;
use super::super::world_actor_abstractions::{
    ActorTrait, GameInteractionLayer, WorldInteractionLayer,
};
use crate::engine::{EngineInteractionLayer, Point, Sprite};

/// The player
pub struct Player {
    location: Point,
    sprite: Sprite,
}

impl Player {
    pub fn new(location: Point) -> Self {
        Self {
            location,
            sprite: PLAYER_SPRITE,
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
