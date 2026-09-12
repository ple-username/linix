// VGA Display Driver for LINIX

#include "vga.h"

static uint16_t *vga_buffer = (uint16_t *)0xB8000;
static uint8_t vga_row = 0;
static uint8_t vga_col = 0;
static uint8_t vga_color = VGA_COLOR_WHITE | (VGA_COLOR_BLACK << 4);

// Initialize VGA
void init_vga(void) {
    vga_row = 0;
    vga_col = 0;
    vga_color = VGA_COLOR_WHITE | (VGA_COLOR_BLACK << 4);
}

// Set foreground and background color
void vga_set_color(uint8_t fg, uint8_t bg) {
    vga_color = fg | (bg << 4);
}

// Clear VGA screen
void vga_clear(void) {
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        vga_buffer[i] = (vga_color << 8) | ' ';
    }
    vga_row = 0;
    vga_col = 0;
}

// Put a character on screen
void vga_put_char(char c) {
    if (c == '\n') {
        vga_row++;
        vga_col = 0;
        if (vga_row >= VGA_HEIGHT) {
            // Scroll up
            for (int i = 0; i < VGA_WIDTH * (VGA_HEIGHT - 1); i++) {
                vga_buffer[i] = vga_buffer[i + VGA_WIDTH];
            }
            vga_row = VGA_HEIGHT - 1;
            for (int i = VGA_WIDTH * (VGA_HEIGHT - 1); i < VGA_WIDTH * VGA_HEIGHT; i++) {
                vga_buffer[i] = (vga_color << 8) | ' ';
            }
        }
        return;
    }
    
    if (vga_col >= VGA_WIDTH) {
        vga_col = 0;
        vga_row++;
    }
    
    if (vga_row >= VGA_HEIGHT) {
        vga_row = VGA_HEIGHT - 1;
    }
    
    int index = vga_row * VGA_WIDTH + vga_col;
    vga_buffer[index] = (vga_color << 8) | c;
    vga_col++;
}

// Print string
void vga_print_string(const char *str) {
    while (*str) {
        vga_put_char(*str++);
    }
}

// Print integer in hex
void vga_print_hex(uint32_t value) {
    const char *hex = "0123456789ABCDEF";
    vga_print_string("0x");
    for (int i = 28; i >= 0; i -= 4) {
        vga_put_char(hex[(value >> i) & 0xF]);
    }
}
