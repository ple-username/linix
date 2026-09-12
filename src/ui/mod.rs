// Beautiful UI Module

use crate::display::vga::{Color, VGA_WRITER};

pub struct UITheme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub background_color: Color,
    pub text_color: Color,
}

impl UITheme {
    pub fn default() -> Self {
        UITheme {
            primary_color: Color::LightCyan,
            secondary_color: Color::LightGreen,
            background_color: Color::Black,
            text_color: Color::White,
        }
    }
}

pub struct Window {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
}

impl Window {
    pub fn new(x: usize, y: usize, width: usize, height: usize, title: &'static str) -> Self {
        Window {
            x,
            y,
            width,
            height,
            title,
        }
    }

    pub fn draw(&self, theme: &UITheme) {
        let mut writer = VGA_WRITER.lock();
        
        // Draw window border
        writer.set_color(theme.primary_color, theme.background_color);
        
        // Top border
        for i in self.x..self.x + self.width {
            writer.put_char_at(i, self.y, '═');
        }
        
        // Title bar
        writer.put_char_at(self.x, self.y, '╔');
        writer.put_char_at(self.x + self.width - 1, self.y, '╗');
        
        // Sides
        for i in (self.y + 1)..self.y + self.height {
            writer.put_char_at(self.x, i, '║');
            writer.put_char_at(self.x + self.width - 1, i, '║');
        }
        
        // Bottom border
        writer.put_char_at(self.x, self.y + self.height - 1, '╚');
        writer.put_char_at(self.x + self.width - 1, self.y + self.height - 1, '╝');
        
        for i in (self.x + 1)..self.x + self.width - 1 {
            writer.put_char_at(i, self.y + self.height - 1, '═');
        }
        
        // Title
        writer.set_color(theme.secondary_color, theme.background_color);
        let title_x = self.x + (self.width - self.title.len()) / 2;
        for (i, &ch) in self.title.as_bytes().iter().enumerate() {
            writer.put_char_at(title_x + i, self.y, ch as char);
        }
    }
}

pub fn draw_menu_bar(theme: &UITheme) {
    let mut writer = VGA_WRITER.lock();
    writer.set_color(theme.primary_color, theme.background_color);
    
    for i in 0..80 {
        writer.put_char_at(i, 0, '─');
    }
    
    writer.set_color(theme.secondary_color, theme.background_color);
    let menu_items = vec!["[File]", "[Edit]", "[View]", "[Help]"];
    let mut x = 2;
    for item in menu_items {
        for &ch in item.as_bytes() {
            writer.put_char_at(x, 0, ch as char);
            x += 1;
        }
        x += 3;
    }
}

pub fn draw_status_bar(message: &str, theme: &UITheme) {
    let mut writer = VGA_WRITER.lock();
    writer.set_color(theme.primary_color, theme.background_color);
    
    let row = 24;
    for i in 0..80 {
        writer.put_char_at(i, row, '▄');
    }
    
    writer.set_color(theme.text_color, theme.background_color);
    for (i, &ch) in message.as_bytes().iter().enumerate() {
        if i >= 80 { break; }
        writer.put_char_at(i, row, ch as char);
    }
}

pub fn init() {
    println!("UI system initialized");
}
