use super::EngineInteractionLayer;
use super::alloc::{Rc, Vec};
use iter_variants::IterVariants;
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

/// Initially serialize to SpritesheetInitial, then convert to Spritesheet
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
    ) {
        engine.draw_sprite(self, location, draw_white, draw_black);
    }
}

// /// A single frame of an animation
// /// TODO: implement this
// pub struct SpriteAnimationFrame {
//     sprite: Rc<Sprite>,
//     length_ticks: u64,
// }

// /// An entire animation
// /// TODO: implement this asw
// pub struct SpriteAnimation {
//     sprites: Vec<SpriteAnimationFrame>,
//     length_ticks_total: u64,
// }

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

/// Tone + length in ms
pub struct SoundTone {
    pub freq: f32,
    pub length_us: u64,
}
