use super::EngineInteractionLayer;
use iter_variants::IterVariants;
use variant_count::VariantCount;

pub trait GameTrait {
    /// Runs on Engine::new()
    fn new() -> Self;

    /// Runs once when start() is called
    fn init(&mut self, engine: &mut EngineInteractionLayer);

    /// Runs on every tick
    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer);
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
