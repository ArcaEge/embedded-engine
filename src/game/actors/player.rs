use super::super::{
    collisions::CollisionTypes,
    world_actor_abstractions::{ActorTrait, GameInteractionLayer, WorldInteractionLayer},
};
use crate::engine::{
    EngineInteractionLayer, Inputs, PreciseOffset, PrecisePoint, Sprite, SpriteAnimation,
    Spritesheet, Velocity,
    alloc::{Rc, Vec},
};

pub const MOVEMENT_SPEED: f32 = 0.8;
const GRAVITY_ACCELERATION: f32 = 0.1;
const JUMP_ACCELERATION: f32 = -1.7;

/// The player
pub struct Player {
    location: PrecisePoint,
    sprite_animation: SpriteAnimation,
    velocity: Velocity,
    collision_type: Rc<CollisionTypes>,
    is_flipped: bool,
}

impl Player {
    pub fn create(location: PrecisePoint, spritesheet: Rc<Spritesheet>) -> Self {
        Self {
            location,
            sprite_animation: SpriteAnimation::new(
                Vec::from([(0, 5), (1, 5), (2, 5), (3, 5)]),
                spritesheet,
            ),
            velocity: Velocity { x: 0.0, y: 0.0 },
            collision_type: Rc::new(CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 0.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            )),
            is_flipped: false,
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
        tick_count: u64,
        self_index: Option<u32>,
        world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut x_offset = 0.0f32;
        // let mut y_offset = 0.0f32;

        if engine.inputs[Inputs::Left as usize].state {
            x_offset -= MOVEMENT_SPEED;
            self.is_flipped = true;
        }
        if engine.inputs[Inputs::Right as usize].state {
            x_offset += MOVEMENT_SPEED;
            self.is_flipped = false;
        }

        // TODO: check for collisions first
        self.velocity.y += GRAVITY_ACCELERATION;

        if engine.inputs[Inputs::Jump as usize].pressed_tick == Some(tick_count) {
            self.velocity.y = JUMP_ACCELERATION;
        }

        // Move in x, check collisions
        let old_x = self.location.x;
        self.location.x += self.velocity.x;
        self.location.x += x_offset;
        if !self.get_colliding_objects(self_index, world).is_empty() {
            self.location.x = old_x;
            self.velocity.x = 0.0;
        }

        // Move in y, check collisions
        let old_y = self.location.y;
        self.location.y += self.velocity.y;
        // self.location.y += y_offset;
        if !self.get_colliding_objects(self_index, world).is_empty() {
            self.location.y = old_y;
            self.velocity.y = 0.0;
        }

        self.sprite_animation.tick();
    }

    fn get_precise_location(&self) -> PrecisePoint {
        self.location
    }

    fn get_sprite(&self) -> Rc<Sprite> {
        self.sprite_animation.get_current_sprite()
    }

    fn is_flipped(&self) -> bool {
        self.is_flipped
    }

    fn get_collision_type(&self) -> Rc<CollisionTypes> {
        self.collision_type.clone()
    }
}
