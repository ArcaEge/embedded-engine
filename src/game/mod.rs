use crate::engine::{DISPLAY_HEIGHT, DISPLAY_WIDTH, EngineInteractionLayer, GameTrait};

const INPUTS_ACCELERATION: f32 = 0.05;
const MAX_VELOCITY: f32 = 1.0;

// Game stuff goes here
pub struct Game {
    velocity_x: f32,
    velocity_y: f32,
    position_x: f32,
    position_y: f32,
}

impl GameTrait for Game {
    fn new() -> Self {
        Self {
            velocity_x: 0.9,
            velocity_y: -0.5,
            position_x: 60.0,
            position_y: 30.0,
        }
    }

    fn init(&mut self, engine: &mut EngineInteractionLayer) {}

    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer) {
        // Check if at edge, if so bounce
        if self.position_x <= 0.0 || self.position_x >= DISPLAY_WIDTH as f32 - 2.0 {
            self.velocity_x = -self.velocity_x;
        }
        if self.position_y <= 0.0 || self.position_y >= DISPLAY_HEIGHT as f32 - 2.0 {
            self.velocity_y = -self.velocity_y;
        }

        if engine.inputs.up.state {
            self.velocity_y -= INPUTS_ACCELERATION;
        }
        if engine.inputs.down.state {
            self.velocity_y += INPUTS_ACCELERATION;
        }
        if engine.inputs.left.state {
            self.velocity_x -= INPUTS_ACCELERATION;
        }
        if engine.inputs.right.state {
            self.velocity_x += INPUTS_ACCELERATION;
        }

        self.position_x += self.velocity_x;
        self.position_y += self.velocity_y;

        self.position_x = self.position_x.clamp(0.0, DISPLAY_WIDTH as f32 - 2.0);
        self.position_y = self.position_y.clamp(0.0, DISPLAY_HEIGHT as f32 - 2.0);

        self.velocity_x = self.velocity_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.velocity_y = self.velocity_y.clamp(-MAX_VELOCITY, MAX_VELOCITY);

        let _ = engine.set_pixel_state_check_bounds(
            self.position_x as i32,
            self.position_y as i32,
            true,
        );

        let _ = engine.set_pixel_state_check_bounds(
            self.position_x as i32 + 1,
            self.position_y as i32,
            true,
        );

        let _ = engine.set_pixel_state_check_bounds(
            self.position_x as i32,
            self.position_y as i32 + 1,
            true,
        );

        let _ = engine.set_pixel_state_check_bounds(
            self.position_x as i32 + 1,
            self.position_y as i32 + 1,
            true,
        );
    }
}
