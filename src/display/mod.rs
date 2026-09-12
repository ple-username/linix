// Display module

pub mod vga;

pub fn init() {
    vga::init_vga();
}
