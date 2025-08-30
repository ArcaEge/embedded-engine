use super::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FrameBuffer, HAL, Input, Inputs};

/// Engine interaction layer (the functions the game can call and the objects it can access to interact with the engine)
#[allow(dead_code)]
pub struct EngineInteractionLayer<'a> {
    pub(super) hal: &'a HAL,
    pub framebuffer: &'a mut FrameBuffer,
    pub inputs: &'a [Input; Inputs::VARIANT_COUNT],
}

impl<'a> EngineInteractionLayer<'a> {
    pub fn set_pixel_state(&mut self, x: u32, y: u32, state: bool) {
        self.framebuffer.set_pixel_state(x, y, state);
    }

    pub fn set_pixel_state_check_bounds(&mut self, x: i32, y: i32, state: bool) -> Result<(), ()> {
        if x >= DISPLAY_WIDTH as i32 || x < 0 || y >= DISPLAY_HEIGHT as i32 || y < 0 {
            return Err(());
        }

        self.framebuffer.set_pixel_state(x as u32, y as u32, state);
        Ok(())
    }

    pub fn get_pixel_state(&self, x: u32, y: u32) {
        self.framebuffer.get_pixel_state(x, y);
    }
}
