// LINIX Kernel Main
// 64-bit kernel entry point

#include "vga.h"

// Kernel main function (called from boot.asm)
void kernel_main(void) {
    // Initialize VGA display
    init_vga();
    
    // Clear screen and print welcome message
    vga_clear();
    vga_set_color(VGA_COLOR_GREEN, VGA_COLOR_BLACK);
    vga_print_string("\n=== LINIX Kernel ===\n");
    vga_print_string("64-bit Long Mode Active\n\n");
    
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print_string("Kernel initialized successfully!\n");
    vga_print_string("Starting system services...\n\n");
    
    // Print system info
    print_system_info();
    
    // Main kernel loop
    kernel_loop();
}

// Print system information
void print_system_info(void) {
    vga_set_color(VGA_COLOR_CYAN, VGA_COLOR_BLACK);
    vga_print_string("System Information:\n");
    vga_set_color(VGA_COLOR_LIGHT_GRAY, VGA_COLOR_BLACK);
    vga_print_string("CPU: x86-64\n");
    vga_print_string("Architecture: 64-bit\n");
    vga_print_string("Paging: Enabled\n");
    vga_print_string("\n");
}

// Main kernel loop
void kernel_loop(void) {
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print_string("Kernel running. Halting...");
    
    while (1) {
        // Halt CPU
        asm("hlt");
    }
}
