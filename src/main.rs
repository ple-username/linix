// LINIX OS - Rust Kernel Main

#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod arch;
mod display;
mod memory;

use display::vga::VGA_WRITER;

/// Kernel entry point (64-bit)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize display
    display::vga::init_vga();
    
    println!("\n=== LINIX OS (Rust Edition) ===");
    println!("64-bit Long Mode Active\n");
    
    // Initialize memory
    memory::init();
    println!("Memory management initialized\n");
    
    // Print system info
    print_system_info();
    
    // Main kernel loop
    loop {
        asm!("hlt");
    }
}

fn print_system_info() {
    println!("System Information:");
    println!("  Arch: x86-64");
    println!("  CPU: Intel/AMD compatible");
    println!("  Paging: Enabled");
    println!("  Memory: Protected\n");
    println!("Kernel is running...");
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    if let Some(location) = info.location() {
        println!("Location: {}:{}", location.file(), location.line());
    }
    if let Some(message) = info.message() {
        println!("Message: {}", message);
    }
    loop {
        asm!("hlt");
    }
}

/// Print macro
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::vga::_print(format_args!($($arg)*)));
}

/// Println macro
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}", format_args!($($arg)*)));
}
