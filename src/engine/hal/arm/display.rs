use super::super::super::{DISPLAY_HEIGHT, DISPLAY_PAGE_COUNT, DISPLAY_WIDTH, FrameBuffer};
use super::{HAL, peripherals_io};
use bsp::hal::i2c;
use defmt::debug;
use embedded_hal::i2c::I2c;
use rp_pico as bsp;

const I2C_ADDRESS: u8 = 0x3C; // or 0x3D in some displays, depending on whether the D/C (Data/Command) pin is pulled HIGH or LOW
const DISPLAY_SET_COLUMN_ADDRESS: u8 = 0x21;
const DISPLAY_SET_PAGE_ADDRESS: u8 = 0x22;
const DISPLAY_SET_STATE: u8 = 0xAE; // Send directly to set display off, OR with 1 to set it on
const DISPLAY_SET_MEMORY_ADDRESSING_MODE: u8 = 0x20; // Send 0x01 after this to set to vertical addressing mode, 0x00 for horizontal
const DISPLAY_START_LINE: u8 = 0x40;
const DISPLAY_SEG_REMAP: u8 = 0xA0;
const DISPLAY_SET_MUX_RATIO: u8 = 0xA8;
const DISPLAY_COM_OUT_DIR: u8 = 0xC0;
const DISPLAY_SET_OFFSET: u8 = 0xD3;
const DISPLAY_SET_COM_PIN_CONFIG: u8 = 0xDA; // Send 0x12 after this for a 128x64 display
const DISPLAY_SET_CLOCK_DIV: u8 = 0xD5;
const DISPLAY_SET_PRECHARGE: u8 = 0xD9;
const DISPLAY_SET_VCOM_DESELECT: u8 = 0xDB;
const DISPLAY_SET_CONTRAST: u8 = 0x81;
const DISPLAY_ENTIRE_ON: u8 = 0xA4;
const DISPLAY_INVERTED_STATE: u8 = 0xA6;
const DISPLAY_SET_CHARGE_PUMP: u8 = 0x8D;

pub(super) struct Display {
    i2c: peripherals_io::I2CType,
}

impl Display {
    pub fn new(i2c: peripherals_io::I2CType) -> Self {
        // s == self but we can't use the variable name self
        let mut s = Self { i2c };

        // == Initialise display ==
        // Set display state to off while configuring everything
        s.write_command(DISPLAY_SET_STATE | 0).unwrap();

        // Set to horizontal addressing mode
        s.write_command(DISPLAY_SET_MEMORY_ADDRESSING_MODE).unwrap();
        s.write_command(0x00).unwrap();

        s.write_command(DISPLAY_START_LINE | 0x00).unwrap();
        s.write_command(DISPLAY_SEG_REMAP | 0x01).unwrap();

        s.write_command(DISPLAY_SET_MUX_RATIO).unwrap();
        s.write_command(DISPLAY_HEIGHT - 1).unwrap();

        s.write_command(DISPLAY_COM_OUT_DIR | 0x08).unwrap();

        s.write_command(DISPLAY_SET_OFFSET).unwrap();
        s.write_command(0x00).unwrap();

        s.write_command(DISPLAY_SET_COM_PIN_CONFIG).unwrap();
        s.write_command(0x12).unwrap();

        s.write_command(DISPLAY_SET_CLOCK_DIV).unwrap();
        s.write_command(0x80).unwrap();

        s.write_command(DISPLAY_SET_PRECHARGE).unwrap();
        s.write_command(0x80).unwrap();

        s.write_command(DISPLAY_SET_VCOM_DESELECT).unwrap();
        s.write_command(0x30).unwrap();

        s.write_command(DISPLAY_SET_CONTRAST).unwrap();
        s.write_command(0xFF).unwrap();

        s.write_command(DISPLAY_ENTIRE_ON | 0).unwrap();
        s.write_command(DISPLAY_INVERTED_STATE | 0).unwrap();

        s.write_command(DISPLAY_SET_CHARGE_PUMP).unwrap();
        s.write_command(0x14).unwrap();

        s.write_command(DISPLAY_SET_STATE | 1).unwrap();

        debug!("Display init complete!");
        s
    }

    /// Write a command to the display, commands start with 0x80
    fn write_command(&mut self, command: u8) -> Result<(), i2c::Error> {
        self.i2c.write(I2C_ADDRESS, &[0x80, command])
    }

    /// Write the contents of the given framebuffer
    fn write_buffer(&mut self, framebuffer: &FrameBuffer) -> Result<(), i2c::Error> {
        // Following two sections are to prevent the display getting desynced
        // Set column range to 0-127
        self.write_command(DISPLAY_SET_COLUMN_ADDRESS)?;
        self.write_command(0x00)?;
        self.write_command(DISPLAY_WIDTH - 1)?;

        // Set page range to 0-7
        self.write_command(DISPLAY_SET_PAGE_ADDRESS)?;
        self.write_command(0x00)?;
        self.write_command(DISPLAY_PAGE_COUNT - 1)?;

        // Set inverted state
        self.write_command(DISPLAY_INVERTED_STATE | framebuffer.inverted as u8)
            .unwrap();

        let raw_data = bytemuck::cast_slice(&framebuffer.buffer);
        self.i2c_write_with_prefix(I2C_ADDRESS, 0x40, raw_data)
    }

    fn i2c_write_with_prefix(
        &mut self,
        address: u8,
        prefix: u8,
        data: &[u8],
    ) -> Result<(), i2c::Error> {
        let mut arr = [prefix; DISPLAY_PAGE_COUNT as usize * DISPLAY_WIDTH as usize + 1];
        arr[1..].copy_from_slice(data);
        self.i2c.write(address, &arr)
    }
}

impl HAL {
    // Displays the given framebuffer
    pub fn display_buffer(&mut self, framebuffer: &FrameBuffer) {
        self.display.write_buffer(framebuffer).unwrap();
    }
}
