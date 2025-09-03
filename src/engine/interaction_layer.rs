use super::*;

/// Engine interaction layer (i.e. the functions the game can call and the objects it can access to interact with the engine)
#[allow(dead_code)]
pub struct EngineInteractionLayer<'a> {
    pub(super) hal: &'a mut HAL,
    pub framebuffer: &'a mut FrameBuffer,
    pub inputs: &'a [Input; Inputs::VARIANT_COUNT],
}

impl<'a> EngineInteractionLayer<'a> {
    /// Sets the state of a pixel
    pub fn set_pixel_state(&mut self, x: u32, y: u32, state: bool) {
        self.framebuffer.set_pixel_state(x, y, state);
    }

    /// Sets the state of a pixel with bounds checking to avoid crashing on an invalid coordinate
    pub fn set_pixel_state_check_bounds(&mut self, x: i32, y: i32, state: bool) -> Result<(), ()> {
        if x >= DISPLAY_WIDTH as i32 || x < 0 || y >= DISPLAY_HEIGHT as i32 || y < 0 {
            return Err(());
        }

        self.framebuffer.set_pixel_state(x as u32, y as u32, state);
        Ok(())
    }

    /// Returns the state of a pixel at a given coordinate
    pub fn get_pixel_state(&self, x: u32, y: u32) {
        self.framebuffer.get_pixel_state(x, y);
    }

    /// Draw a CornerRect
    /// * `rect` - The Rect to draw
    /// * `state` - Sets pixels white if true, black if false
    pub fn draw_rect(&mut self, rect: Rect, state: bool) {
        self.draw_corner_rect(CornerRect::from_rect(rect), state);
    }

    /// Draw a CornerRect
    /// * `rect` - The CornerRect to draw
    /// * `state` - Sets pixels white if true, black if false
    pub fn draw_corner_rect(&mut self, rect: CornerRect, state: bool) {
        for y in rect.top_left.y..=rect.bottom_right.y {
            // More efficient bounds checking
            if y < 0 {
                continue;
            } else if y >= DISPLAY_HEIGHT as i32 {
                break;
            }

            for x in rect.top_left.x..=rect.bottom_right.x {
                // More efficient bounds checking
                if x < 0 {
                    continue;
                } else if x >= DISPLAY_WIDTH as i32 {
                    break;
                }

                self.set_pixel_state(x as u32, y as u32, state);
            }
        }
    }

    /// Draw a sprite
    /// * `sprite` - The Sprite to draw
    /// * `location` - Where to draw it
    /// * `draw_white` - Draw white pixels
    /// * `draw_black` - Draw black pixels
    pub fn draw_sprite(
        &mut self,
        sprite: &Sprite,
        location: Point,
        draw_white: bool,
        draw_black: bool,
    ) {
        let bottom_right_exclusive = Point {
            x: location.x + sprite.width as i32,
            y: location.y + sprite.height as i32,
        };

        for y in location.y..bottom_right_exclusive.y {
            // More efficient bounds checking
            if y < 0 {
                continue;
            } else if y >= DISPLAY_HEIGHT as i32 {
                break;
            }

            for x in location.x..bottom_right_exclusive.x {
                // More efficient bounds checking
                if x < 0 {
                    continue;
                } else if x >= DISPLAY_WIDTH as i32 {
                    break;
                }

                match sprite.get_pixel((x - location.x) as u32, (y - location.y) as u32) {
                    SpritePixel::Black => {
                        if draw_black {
                            self.set_pixel_state(x as u32, y as u32, false);
                        }
                    }
                    SpritePixel::White => {
                        if draw_white {
                            self.set_pixel_state(x as u32, y as u32, true);
                        }
                    }
                    SpritePixel::Transparent => {} // Do nothing
                }
            }
        }
    }

    /// Sets the raw frequency of the produced sound. Also calls self.set_sound_state(true)
    pub fn set_sound_freq(&mut self, freq: f32) {
        if freq == 0.0 {
            self.set_sound_state(false);
            return;
        }
        self.set_sound_state(true);
        self.hal.set_sound_freq(freq);
    }

    /// Sets the tone of the produced sound
    pub fn set_sound_freq_from_tone(&mut self, tone: tones::Tone) {
        self.set_sound_freq(tone.to_freq());
    }

    /// Sets the state of the sound, i.e. whether a tone is being played or not
    pub fn set_sound_state(&mut self, state: bool) {
        self.hal.set_sound_state(state);
    }

    /// Get the current timestamp in microseconds
    pub fn micros(&self) -> u64 {
        self.hal.micros()
    }
}
