// Keyboard Driver

use spin::Mutex;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

pub struct KeyboardDriver {
    buffer: [u8; 256],
    buffer_pos: usize,
}

impl KeyboardDriver {
    pub const fn new() -> Self {
        KeyboardDriver {
            buffer: [0; 256],
            buffer_pos: 0,
        }
    }

    pub fn handle_interrupt(&mut self) -> Option<u8> {
        let status = unsafe { read_port(KEYBOARD_STATUS_PORT) };
        if status & 1 == 0 {
            return None;
        }

        let scancode = unsafe { read_port(KEYBOARD_DATA_PORT) };
        Some(scancode)
    }

    pub fn push_char(&mut self, ch: u8) {
        if self.buffer_pos < 256 {
            self.buffer[self.buffer_pos] = ch;
            self.buffer_pos += 1;
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer[..self.buffer_pos]
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = [0; 256];
        self.buffer_pos = 0;
    }
}

pub static KEYBOARD: Mutex<KeyboardDriver> = Mutex::new(KeyboardDriver::new());

unsafe fn read_port(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    value
}

unsafe fn write_port(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

pub fn init() {
    println!("Keyboard driver initialized");
}

// Scancode to ASCII mapping
pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        0x0E => Some(b'\x08'), // Backspace
        0x0F => Some(b'\t'),   // Tab
        0x10 => Some(b'q'),
        0x11 => Some(b'w'),
        0x12 => Some(b'e'),
        0x13 => Some(b'r'),
        0x14 => Some(b't'),
        0x15 => Some(b'y'),
        0x16 => Some(b'u'),
        0x17 => Some(b'i'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x1C => Some(b'\n'), // Enter
        0x1E => Some(b'a'),
        0x1F => Some(b's'),
        0x20 => Some(b'd'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x2C => Some(b'z'),
        0x2D => Some(b'x'),
        0x2E => Some(b'c'),
        0x2F => Some(b'v'),
        0x30 => Some(b'b'),
        0x31 => Some(b'n'),
        0x32 => Some(b'm'),
        0x33 => Some(b'.'),
        0x34 => Some(b'/'),
        0x39 => Some(b' '),  // Space
        _ => None,
    }
}
