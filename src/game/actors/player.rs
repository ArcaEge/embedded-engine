use super::super::world_actor_abstractions::{
    ActorTrait, GameInteractionLayer, WorldInteractionLayer,
};
use crate::engine::{
    EngineInteractionLayer, PrecisePoint, Sprite, SpriteAnimation, Spritesheet,
    alloc::{Rc, Vec},
};

/// The player
pub struct Player {
    location: PrecisePoint,
    sprite_animation: SpriteAnimation,
}

impl Player {
    pub fn create(location: PrecisePoint, spritesheet: &Spritesheet) -> Self {
        Self {
            location,
            sprite_animation: SpriteAnimation::new(
                Vec::from([(0, 5), (1, 5), (2, 5), (3, 5)]),
                spritesheet,
            ),
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
        self.sprite_animation.tick();
    }

    fn get_precise_location(&self) -> PrecisePoint {
        self.location
    }

    fn get_sprite(&self) -> Rc<Sprite> {
        self.sprite_animation.get_current_sprite()
    }

    fn is_flipped(&self) -> bool {
        false
    }
}
