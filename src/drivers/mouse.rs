// Mouse Driver

use spin::Mutex;

const MOUSE_DATA_PORT: u16 = 0x60;
const MOUSE_CONTROL_PORT: u16 = 0x64;

#[derive(Clone, Copy)]
pub struct MouseState {
    pub x: i16,
    pub y: i16,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
}

pub struct MouseDriver {
    state: MouseState,
}

impl MouseDriver {
    pub const fn new() -> Self {
        MouseDriver {
            state: MouseState {
                x: 40,
                y: 12,
                left_button: false,
                right_button: false,
                middle_button: false,
            },
        }
    }

    pub fn update_position(&mut self, dx: i8, dy: i8) {
        self.state.x = (self.state.x + dx as i16).max(0).min(79);
        self.state.y = (self.state.y - dy as i16).max(0).min(24);
    }

    pub fn set_buttons(&mut self, left: bool, right: bool, middle: bool) {
        self.state.left_button = left;
        self.state.right_button = right;
        self.state.middle_button = middle;
    }

    pub fn get_state(&self) -> MouseState {
        self.state
    }
}

pub static MOUSE: Mutex<MouseDriver> = Mutex::new(MouseDriver::new());

pub fn init() {
    println!("Mouse driver initialized");
}
