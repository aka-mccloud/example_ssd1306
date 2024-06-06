#![allow(dead_code)]

use rusty_peripheral::i2c::{ Error, I2C };

pub const SSD1306_LCDWIDTH: usize = 128;
pub const SSD1306_LCDHEIGHT: usize = 64;

const SSD1306_SETCONTRAST: u8 = 0x81;
const SSD1306_DISPLAYALLON_RESUME: u8 = 0xa4;
const SSD1306_DISPLAYALLON: u8 = 0xa5;
const SSD1306_NORMALDISPLAY: u8 = 0xa6;
const SSD1306_INVERTDISPLAY: u8 = 0xa7;
const SSD1306_DISPLAYOFF: u8 = 0xae;
const SSD1306_DISPLAYON: u8 = 0xaf;

const SSD1306_SETDISPLAYOFFSET: u8 = 0xd3;
const SSD1306_SETCOMPINS: u8 = 0xda;

const SSD1306_SETVCOMDETECT: u8 = 0xdb;

const SSD1306_SETDISPLAYCLOCKDIV: u8 = 0xd5;
const SSD1306_SETPRECHARGE: u8 = 0xd9;

const SSD1306_SETMULTIPLEX: u8 = 0xa8;

const SSD1306_SETLOWCOLUMN: u8 = 0x00;
const SSD1306_SETHIGHCOLUMN: u8 = 0x10;

const SSD1306_SETSTARTLINE: u8 = 0x40;

const SSD1306_MEMORYMODE: u8 = 0x20;
const SSD1306_COLUMNADDR: u8 = 0x21;
const SSD1306_PAGEADDR: u8 = 0x22;

const SSD1306_COMSCANINC: u8 = 0xc0;
const SSD1306_COMSCANDEC: u8 = 0xc8;

const SSD1306_SEGREMAP: u8 = 0xa0;

const SSD1306_CHARGEPUMP: u8 = 0x8d;

const SSD1306_EXTERNALVCC: u8 = 0x1;
const SSD1306_SWITCHCAPVCC: u8 = 0x2;

// Scrolling consts
const SSD1306_ACTIVATE_SCROLL: u8 = 0x2f;
const SSD1306_DEACTIVATE_SCROLL: u8 = 0x2e;
const SSD1306_SET_VERTICAL_SCROLL_AREA: u8 = 0xa3;
const SSD1306_RIGHT_HORIZONTAL_SCROLL: u8 = 0x26;
const SSD1306_LEFT_HORIZONTAL_SCROLL: u8 = 0x27;
const SSD1306_VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL: u8 = 0x29;
const SSD1306_VERTICAL_AND_LEFT_HORIZONTAL_SCROLL: u8 = 0x2a;

pub struct SSD1306Display {
    i2c_handle: &'static mut I2C,
    addr: u8,
}

impl SSD1306Display {
    pub fn new(i2c_handle: &'static mut I2C, addr: u8) -> Self {
        Self { i2c_handle, addr }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.write_command0(SSD1306_DISPLAYOFF)?;

        self.write_command1(SSD1306_SETDISPLAYCLOCKDIV, 0x80)?;

        self.write_command1(SSD1306_SETMULTIPLEX, (SSD1306_LCDHEIGHT - 1) as u8)?;

        self.write_command1(SSD1306_SETDISPLAYOFFSET, 0x00)?;
        self.write_command0(SSD1306_SETSTARTLINE | 0x00)?; //  line #0
        self.write_command1(SSD1306_CHARGEPUMP, 0x14)?;
        self.write_command1(SSD1306_MEMORYMODE, 0x00)?;
        self.write_command0(SSD1306_SEGREMAP | 0x1)?;
        self.write_command0(SSD1306_COMSCANDEC)?;

        self.write_command1(SSD1306_SETCOMPINS, 0x12)?;
        self.write_command1(SSD1306_SETCONTRAST, 0xcf)?;

        self.write_command1(SSD1306_SETPRECHARGE, 0xf1)?;
        self.write_command1(SSD1306_SETVCOMDETECT, 0x40)?;
        self.write_command0(SSD1306_DISPLAYALLON_RESUME)?;
        self.write_command0(SSD1306_NORMALDISPLAY)?;

        self.write_command0(SSD1306_DEACTIVATE_SCROLL)?;

        self.write_command0(SSD1306_DISPLAYON) // --turn on oled panel
    }

    pub fn draw(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.write_command2(SSD1306_COLUMNADDR, 0, (SSD1306_LCDWIDTH - 1) as u8)?;
        self.write_command2(SSD1306_PAGEADDR, 0, 7)?;
        self.write_data(buf)
    }

    fn write_command0(&mut self, cmd: u8) -> Result<(), Error> {
        self.i2c_handle.master_start()?;
        self.i2c_handle.master_write_address(self.addr, false)?;
        self.i2c_handle.master_write_byte(0x00u8)?;
        self.i2c_handle.master_write_byte(cmd)?;
        self.i2c_handle.master_stop()
    }

    fn write_command1(&mut self, cmd: u8, arg1: u8) -> Result<(), Error> {
        self.i2c_handle.master_start()?;
        self.i2c_handle.master_write_address(self.addr, false)?;
        self.i2c_handle.master_write_byte(0x00u8)?;
        self.i2c_handle.master_write_byte(cmd)?;
        self.i2c_handle.master_write_byte(arg1)?;
        self.i2c_handle.master_stop()
    }

    fn write_command2(&mut self, cmd: u8, arg1: u8, arg2: u8) -> Result<(), Error> {
        self.i2c_handle.master_start()?;
        self.i2c_handle.master_write_address(self.addr, false)?;
        self.i2c_handle.master_write_byte(0x00u8)?;
        self.i2c_handle.master_write_byte(cmd)?;
        self.i2c_handle.master_write_byte(arg1)?;
        self.i2c_handle.master_write_byte(arg2)?;
        self.i2c_handle.master_stop()
    }

    fn write_data(&mut self, data: &[u8]) -> Result<(), Error> {
        self.i2c_handle.master_start()?;
        self.i2c_handle.master_write_address(self.addr, false)?;
        self.i2c_handle.master_write_byte(0x40u8)?;
        self.i2c_handle.master_write_bytes(data)?;
        self.i2c_handle.master_stop()
    }
}
