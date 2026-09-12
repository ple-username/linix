// LINIX OS - Rust Kernel Main with Full Features

#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod arch;
mod display;
mod memory;
mod drivers;
mod task;
mod filesystem;
mod ui;
mod shell;
mod power;

use display::vga::VGA_WRITER;
use ui::UITheme;
use shell::SHELL;

/// Kernel entry point (64-bit)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize display first
    display::vga::init_vga();
    
    // Draw beautiful startup screen
    draw_startup_screen();
    
    // Initialize all subsystems
    arch::init();
    memory::init();
    drivers::init();
    task::init();
    filesystem::init();
    ui::init();
    shell::init();
    power::init();
    
    // Show welcome message
    println!("\n");
    println!("All systems initialized successfully!");
    println!("Type 'help' for available commands.\n");
    
    // Print shell prompt
    SHELL.lock().print_prompt();
    
    // Main kernel loop - wait for user input
    kernel_main_loop();
}

fn draw_startup_screen() {
    let theme = UITheme::default();
    let mut writer = VGA_WRITER.lock();
    
    writer.clear();
    writer.set_color(theme.primary_color, theme.background_color);
    
    println!("\n");
    println!("  ╔══════════════════════════════════════════════════════════════════╗");
    println!("  ║                                                         ║");
    
    writer.set_color(theme.secondary_color, theme.background_color);
    println!("  ║           ♥♥♥  LINIX OS v0.1.0  ♥♥♥               ║");
    
    writer.set_color(theme.primary_color, theme.background_color);
    println!("  ║                                                         ║");
    println!("  ║  A Modern Operating System Written in Rust             ║");
    println!("  ║  Supporting Legacy BIOS and UEFI 64-bit                ║");
    println!("  ║                                                         ║");
    println!("  ║  Features:                                              ║");
    println!("  ║    ✓ Memory-safe kernel (Rust)                         ║");
    println!("  ║    ✓ Task management & scheduling                     ║");
    println!("  ║    ✓ Simple filesystem                                 ║");
    println!("  ║    ✓ Multiple device drivers                          ║");
    println!("  ║    ✓ Beautiful UI system                               ║");
    println!("  ║    ✓ Interactive REPL shell                           ║");
    println!("  ║    ✓ ACPI power management                            ║");
    println!("  ║                                                         ║");
    println!("  ╚══════════════════════════════════════════════════════════════════╝\n");
    
    writer.set_color(theme.text_color, theme.background_color);
}

fn kernel_main_loop() -> ! {
    loop {
        // In a real implementation, this would handle interrupts
        // For now, we'll just halt
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = VGA_WRITER.lock();
    writer.set_color(display::vga::Color::LightRed, display::vga::Color::Black);
    
    println!("\n\n");
    println!("  ╔══════════════════════════════════════════════════════════════════╗");
    println!("  ║              ⚠  KERNEL PANIC  ⚠                   ║");
    println!("  ╚══════════════════════════════════════════════════════════════════╝\n");
    
    if let Some(location) = info.location() {
        println!("\n  Location: {}:{}", location.file(), location.line());
    }
    
    if let Some(message) = info.message() {
        println!("  Message: {}", message);
    }
    
    println!("\n  The system will now halt. Please restart your computer.\n");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
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
