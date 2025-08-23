use super::{Engine, GameTrait, hal::HAL};

pub const DISPLAY_WIDTH: u8 = 128;
pub const DISPLAY_HEIGHT: u8 = 64; // Must be a multiple of 8
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
        hal.display_buffer(&self.buffer);
    }

    pub(super) fn set_pixel_state(&mut self, x: usize, y: usize, state: bool) {
        let page_no = Self::get_pixel_page_no(y);
        let y_coord_in_page = y % 8;

        let bitmask: u8 = 0x01 << y_coord_in_page;
        if state {
            self.buffer[page_no][x] |= bitmask;
        } else {
            self.buffer[page_no][x] &= !bitmask;
        }
    }

    pub(super) fn get_pixel_state(&self, x: usize, y: usize) -> bool {
        let page_no = Self::get_pixel_page_no(y);
        let y_coord_in_page = y % 8;

        let bitmask: u8 = 0x01 << y_coord_in_page;

        (self.buffer[page_no][x] & bitmask) > 0
    }

    pub(super) fn clear(&mut self) {
        self.buffer = [[0x00; DISPLAY_WIDTH as usize]; DISPLAY_PAGE_COUNT as usize];
    }

    fn get_pixel_page_no(y: usize) -> usize {
        y / 8 // Each page is 8 pixels tall
    }
}

impl<T: GameTrait> Engine<T> {}
