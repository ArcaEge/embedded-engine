use super::super::{
    collisions::CollisionTypes,
    world_actor_abstractions::{ActorTrait, GameInteractionLayer, WorldInteractionLayer},
};
use crate::engine::{
    EngineInteractionLayer, Inputs, PreciseOffset, PrecisePoint, Sprite, SpriteAnimation,
    Spritesheet, Velocity,
    alloc::{Rc, Vec},
};
use variant_count::VariantCount;

pub const MOVEMENT_SPEED: f32 = 0.8;
const GRAVITY_ACCELERATION: f32 = 0.1;
const JUMP_ACCELERATION: f32 = -1.7;

/// The player
pub struct Player {
    location: PrecisePoint,
    sprite_animations: [SpriteAnimation; PlayerAnimationStates::VARIANT_COUNT],
    sprite_animation_state: PlayerAnimationStates,
    velocity: Velocity,
    collision_type: Rc<CollisionTypes>,
    is_flipped: bool,
}

impl Player {
    pub fn create(location: PrecisePoint, spritesheet: Rc<Spritesheet>) -> Self {
        Self {
            location,
            sprite_animations: [
                // Idle
                SpriteAnimation::new(
                    Vec::from([(0, 5), (1, 5), (2, 5), (3, 5)]),
                    spritesheet.clone(),
                ),
                // Moving
                SpriteAnimation::new(
                    Vec::from([(16, 5), (17, 5), (18, 5), (19, 5)]),
                    spritesheet.clone(),
                ),
                // Jumping
                SpriteAnimation::new(Vec::from([(48, 5), (49, u64::MAX)]), spritesheet.clone()),
                // Falling
                SpriteAnimation::new(Vec::from([(50, 5), (51, u64::MAX)]), spritesheet.clone()),
            ],
            sprite_animation_state: PlayerAnimationStates::Idle,
            velocity: Velocity { x: 0.0, y: 0.0 },
            collision_type: Rc::new(CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 0.0 },
                PreciseOffset { x: 8.0, y: 9.0 },
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

        let mut moving = false;

        if engine.inputs[Inputs::Left as usize].state {
            x_offset -= MOVEMENT_SPEED;
            self.is_flipped = true;
            moving = true;
        }
        if engine.inputs[Inputs::Right as usize].state {
            x_offset += MOVEMENT_SPEED;
            self.is_flipped = false;
            moving = true;
        }

        // Update sprite animation state
        let previous_sprite_animation_state = self.sprite_animation_state;
        if moving {
            self.sprite_animation_state = PlayerAnimationStates::Moving;
        } else {
            self.sprite_animation_state = PlayerAnimationStates::Idle;
        }

        self.velocity.y += GRAVITY_ACCELERATION;

        if engine.inputs[Inputs::Jump as usize].pressed_tick == Some(tick_count) {
            self.velocity.y = JUMP_ACCELERATION;
        }

        // Move in x and y
        let mut offset = PreciseOffset { x: 0.0, y: 0.0 };
        offset.x += self.velocity.x;
        offset.y += self.velocity.y;
        offset.x += x_offset;

        let (x_collided, y_collided) = self.move_with_collisions(offset, self_index, world);

        self.velocity.x = if x_collided { 0.0 } else { self.velocity.x };
        self.velocity.y = if y_collided { 0.0 } else { self.velocity.y };

        if !y_collided {
            // Did not collide in the y axis
            self.sprite_animation_state = if self.velocity.y < 0.0 {
                PlayerAnimationStates::Jumping
            } else if self.velocity.y > 0.0 {
                PlayerAnimationStates::Falling
            } else {
                self.sprite_animation_state
            };
        }

        // Reset animation if it changes
        if previous_sprite_animation_state != self.sprite_animation_state {
            self.sprite_animations[self.sprite_animation_state as usize].reset();
        }

        self.sprite_animations[self.sprite_animation_state as usize].tick();
    }

    fn get_precise_location(&self) -> PrecisePoint {
        self.location
    }

    fn set_precise_location(&mut self, position: PrecisePoint) {
        self.location = position;
    }

    fn get_sprite(&self) -> Rc<Sprite> {
        self.sprite_animations[self.sprite_animation_state as usize].get_current_sprite()
    }

    fn is_flipped(&self) -> bool {
        self.is_flipped
    }

    fn get_collision_type(&self) -> Rc<CollisionTypes> {
        self.collision_type.clone()
    }
}

#[derive(Clone, Copy, VariantCount, PartialEq)]
enum PlayerAnimationStates {
    Idle,
    Moving,
    Jumping,
    Falling,
}
