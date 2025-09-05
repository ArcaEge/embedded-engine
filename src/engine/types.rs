use super::EngineInteractionLayer;
use super::alloc::{Rc, Vec};
use iter_variants::IterVariants;
use libm::roundf;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

pub trait GameTrait {
    /// Runs on Engine::new()
    fn new() -> Self;

    /// Runs once when start() is called
    fn init(&mut self, engine: &mut EngineInteractionLayer);

    /// Runs on every tick
    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer);

    /// Runs after every tick, used to render graphics onto the framebuffer
    fn render(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer);
}

#[derive(Clone, Copy)]
pub struct Input {
    pub state: bool,
    pub pressed_tick: Option<u64>,
    pub released_tick: Option<u64>,
}

/// A single input
impl Input {
    pub fn new() -> Self {
        Self {
            state: false,
            pressed_tick: None,
            released_tick: None,
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// A less stupid way of doing inputs (I think?), helps avoid code duplication
#[repr(usize)]
#[derive(VariantCount, IterVariants, Clone, Copy)]
pub enum Inputs {
    Up,
    Down,
    Left,
    Right,
    Jump,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct PrecisePoint {
    pub x: f32,
    pub y: f32,
}

impl From<PrecisePoint> for Point {
    fn from(value: PrecisePoint) -> Self {
        Self {
            x: roundf(value.x) as i32,
            y: roundf(value.y) as i32,
        }
    }
}

impl From<Point> for PrecisePoint {
    fn from(value: Point) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl PrecisePoint {
    pub fn apply_offset(&self, offset: PreciseOffset) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn apply_inverted_offset(&self, offset: PreciseOffset) -> Self {
        Self {
            x: self.x - offset.x,
            y: self.y - offset.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PreciseOffset {
    pub x: f32,
    pub y: f32,
}

/// Rectangle from origin (top left point), width and height
#[derive(Clone, Copy)]
pub struct Rect {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

/// Rectangle from corner coordinates, both inclusive
#[derive(Clone, Copy)]
pub struct CornerRect {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rect {
    /// Creates a Rect from a CornerRect
    pub fn from_corner_rect(corner_rect: CornerRect) -> Self {
        Self {
            origin: corner_rect.top_left,
            width: (corner_rect.bottom_right.x - corner_rect.top_left.x + 1) as u32,
            height: (corner_rect.bottom_right.x - corner_rect.top_left.x + 1) as u32,
        }
    }
}

impl CornerRect {
    /// Creates a CornerRect from a Rect
    pub fn from_rect(rect: Rect) -> Self {
        Self {
            top_left: rect.origin,
            bottom_right: Point {
                x: rect.origin.x + rect.width as i32 - 1,
                y: rect.origin.y + rect.height as i32 - 1,
            },
        }
    }
}

/// Initially deserialize to SpritesheetInitial, then convert to Spritesheet
#[derive(Serialize, Deserialize)]
pub struct SpritesheetInitial {
    pub sprites: Vec<Sprite>,
}

/// Uses Rc to prevent unnecessary cloning if more than one Actor uses the same sprite
pub struct Spritesheet {
    pub sprites: Vec<Rc<Sprite>>,
}

impl From<SpritesheetInitial> for Spritesheet {
    fn from(initial: SpritesheetInitial) -> Self {
        Self {
            sprites: initial.sprites.into_iter().map(Rc::new).collect(),
        }
    }
}

/// Sprite
/// TODO: Make this more memory efficient (SpritePixels only need 2 bits of memory but currently use 8)
#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<SpritePixel>,
}

impl Sprite {
    /// Returns the pixel at the given coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> SpritePixel {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Render the sprite
    pub fn render(
        &self,
        location: Point,
        engine: &mut EngineInteractionLayer,
        draw_white: bool,
        draw_black: bool,
        flip_x: bool,
    ) {
        engine.draw_sprite(self, location, draw_white, draw_black, flip_x);
    }
}

/// A single frame of an animation
pub struct SpriteAnimationFrame {
    sprite: Rc<Sprite>,
    length_ticks: u64,
}

/// An entire animation
pub struct SpriteAnimation {
    frames: Vec<SpriteAnimationFrame>,
    ticks_elapsed_in_current_frame: u64,
    current_frame_index: usize,
}

impl SpriteAnimation {
    /// From Vec<(sprite_index, length_ticks)>
    pub fn new(sprite_vec: Vec<(usize, u64)>, spritesheet: &Spritesheet) -> Self {
        let ticks_elapsed_in_current_frame = 0;

        let frames = sprite_vec
            .iter()
            .map(|s| SpriteAnimationFrame {
                sprite: spritesheet.sprites.get(s.0).unwrap().clone(),
                length_ticks: s.1,
            })
            .collect();

        Self {
            frames,
            ticks_elapsed_in_current_frame,
            current_frame_index: 0,
        }
    }

    pub fn from_animation_vec(frames: Vec<SpriteAnimationFrame>) -> Self {
        let ticks_elapsed_in_current_frame = 0;

        Self {
            frames,
            ticks_elapsed_in_current_frame,
            current_frame_index: 0,
        }
    }

    pub fn get_current_sprite(&self) -> Rc<Sprite> {
        self.frames
            .get(self.current_frame_index)
            .unwrap()
            .sprite
            .clone()
    }

    pub fn tick(&mut self) {
        let current_frame = self.frames.get(self.current_frame_index).unwrap();
        self.ticks_elapsed_in_current_frame += 1;

        if self.ticks_elapsed_in_current_frame >= current_frame.length_ticks {
            self.ticks_elapsed_in_current_frame = 0;
            self.current_frame_index += 1;

            if self.current_frame_index >= self.frames.len() {
                self.current_frame_index = 0;
            }
        }
    }
}

/// Pixel of a sprite, Black, White or Transparent
#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SpritePixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

/// Sound sample
pub type Sound = Vec<SoundTone>;

/// Tone + length in us
pub struct SoundTone {
    pub freq: f32,
    pub length_us: u64,
}
