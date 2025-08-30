use super::{Engine, GameTrait, hal::HAL};

/// Width of the display in pixels
pub const DISPLAY_WIDTH: u8 = 128;
/// Height of the display in pixels, must be a multiple of 8
pub const DISPLAY_HEIGHT: u8 = 64;
/// Number of display pages
pub const DISPLAY_PAGE_COUNT: u8 = DISPLAY_HEIGHT / 8;

pub type Buffer = [[u8; DISPLAY_WIDTH as usize]; DISPLAY_PAGE_COUNT as usize];

pub struct FrameBuffer {
    pub buffer: Buffer,
}

impl FrameBuffer {
    pub fn new() -> Self {
        // Initialise empty buffer
        let buffer = [[0x00; DISPLAY_WIDTH as usize]; DISPLAY_PAGE_COUNT as usize];

        Self { buffer }
    }

    pub(super) fn show(&self, hal: &mut HAL) {
        hal.display_buffer(&self);
    }

    pub(super) fn set_pixel_state(&mut self, x: u32, y: u32, state: bool) {
        let page_no = Self::get_pixel_page_no(y);
        let y_coord_in_page = y % 8;

        let bitmask: u8 = 0x01 << y_coord_in_page;
        if state {
            self.buffer[page_no as usize][x as usize] |= bitmask;
        } else {
            self.buffer[page_no as usize][x as usize] &= !bitmask;
        }
    }

    pub(super) fn get_pixel_state(&self, x: u32, y: u32) -> bool {
        let page_no = Self::get_pixel_page_no(y);
        let y_coord_in_page = y % 8;

        let bitmask: u8 = 0x01 << y_coord_in_page;

        (self.buffer[page_no as usize][x as usize] & bitmask) > 0
    }

    /// Clear the framebuffer
    pub(super) fn clear(&mut self) {
        self.buffer = [[0x00; DISPLAY_WIDTH as usize]; DISPLAY_PAGE_COUNT as usize];
    }

    // TODO: invert framebuffer

    fn get_pixel_page_no(y: u32) -> u32 {
        y / 8 // Each page is 8 pixels tall
    }
}

impl<T: GameTrait> Engine<T> {}
