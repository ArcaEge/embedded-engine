use super::super::{
    collisions::CollisionTypes,
    world_actor_abstractions::{ActorTrait, GameInteractionLayer, WorldInteractionLayer},
};
use crate::engine::{
    EngineInteractionLayer, PrecisePoint, Sprite, SpriteAnimation, Spritesheet,
    alloc::{Box, Rc, Vec},
};

/// A static actor, it can have an animation but not much else
pub struct StaticActor {
    location: PrecisePoint,
    sprite_animation: SpriteAnimation,
    flipped: bool,
    collision_type: Rc<CollisionTypes>,
}

impl StaticActor {
    pub fn create(
        location: PrecisePoint,
        spritesheet: Rc<Spritesheet>,
        sprite_vec: Vec<(usize, u64)>,
        flipped: bool,
        collision_type: CollisionTypes,
    ) -> Box<dyn ActorTrait> {
        Box::new(Self {
            location,
            sprite_animation: SpriteAnimation::new(sprite_vec, spritesheet),
            flipped,
            collision_type: Rc::new(collision_type),
        })
    }
}

impl ActorTrait for StaticActor {
    fn tick(
        &mut self,
        _tick_count: u64,
        _self_index: Option<u32>,
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
        self.flipped
    }

    fn get_collision_type(&self) -> Rc<CollisionTypes> {
        self.collision_type.clone()
    }
}
