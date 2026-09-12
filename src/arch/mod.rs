// x86_64 Architecture Module

pub mod boot;
pub mod gdt;
pub mod interrupts;

pub fn init() {
    gdt::init();
    interrupts::init();
}
