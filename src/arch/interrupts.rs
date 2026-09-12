// Interrupt handling for x86_64

pub fn init() {
    // IDT initialization will be implemented here
}

pub fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti");
    }
}

pub fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cli");
    }
}
