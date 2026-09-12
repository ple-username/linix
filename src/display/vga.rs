// VGA Text Mode Display Driver

use core::fmt::{self, Write};
use core::ptr;
use spin::Mutex;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BASE: usize = 0xB8000;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VgaChar {
    ascii: u8,
    color: ColorCode,
}

pub struct VgaWriter {
    col: usize,
    row: usize,
    color: ColorCode,
    buffer: &'static mut [[VgaChar; VGA_WIDTH]; VGA_HEIGHT],
}

impl VgaWriter {
    pub fn new() -> Self {
        let buffer = unsafe { &mut *(VGA_BASE as *mut [[VgaChar; VGA_WIDTH]; VGA_HEIGHT]) };
        let mut writer = VgaWriter {
            col: 0,
            row: 0,
            color: ColorCode::new(Color::White, Color::Black),
            buffer,
        };
        writer.clear();
        writer
    }

    pub fn clear(&mut self) {
        let blank = VgaChar {
            ascii: b' ',
            color: self.color,
        };
        for row in &mut self.buffer {
            for cell in row {
                *cell = blank;
            }
        }
        self.col = 0;
        self.row = 0;
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color = ColorCode::new(foreground, background);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            byte => {
                if self.col >= VGA_WIDTH {
                    self.new_line();
                }
                self.buffer[self.row][self.col] = VgaChar {
                    ascii: byte,
                    color: self.color,
                };
                self.col += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;

        if self.row >= VGA_HEIGHT {
            // Scroll up
            for row in 0..VGA_HEIGHT - 1 {
                for col in 0..VGA_WIDTH {
                    self.buffer[row][col] = self.buffer[row + 1][col];
                }
            }

            // Clear last row
            let blank = VgaChar {
                ascii: b' ',
                color: self.color,
            };
            for col in 0..VGA_WIDTH {
                self.buffer[VGA_HEIGHT - 1][col] = blank;
            }

            self.row = VGA_HEIGHT - 1;
        }
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static VGA_WRITER: Mutex<VgaWriter> = Mutex::new(unsafe { VgaWriter {
    col: 0,
    row: 0,
    color: ColorCode::new(Color::White, Color::Black),
    buffer: &mut *(VGA_BASE as *mut _),
}});

pub fn init_vga() {
    VGA_WRITER.lock().clear();
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}
