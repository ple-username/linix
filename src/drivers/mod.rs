// Drivers module

pub mod keyboard;
pub mod storage;
pub mod mouse;

pub fn init() {
    keyboard::init();
    storage::init();
    mouse::init();
}
