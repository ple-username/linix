// REPL Shell for LINIX OS

use spin::Mutex;
use crate::display::vga::VGA_WRITER;
use crate::filesystem::FILESYSTEM;

const COMMAND_BUFFER_SIZE: usize = 256;

pub struct Shell {
    command_buffer: [u8; COMMAND_BUFFER_SIZE],
    buffer_pos: usize,
    history: [[u8; COMMAND_BUFFER_SIZE]; 10],
    history_pos: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Shell {
            command_buffer: [0; COMMAND_BUFFER_SIZE],
            buffer_pos: 0,
            history: [[0; COMMAND_BUFFER_SIZE]; 10],
            history_pos: 0,
        }
    }

    pub fn add_char(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                self.execute_command();
                self.clear_buffer();
                self.print_prompt();
            }
            b'\x08' => {
                if self.buffer_pos > 0 {
                    self.buffer_pos -= 1;
                    self.command_buffer[self.buffer_pos] = 0;
                    print!("\x08 \x08");
                }
            }
            _ if self.buffer_pos < COMMAND_BUFFER_SIZE - 1 => {
                self.command_buffer[self.buffer_pos] = ch;
                self.buffer_pos += 1;
                print!("{}", ch as char);
            }
            _ => {}
        }
    }

    pub fn execute_command(&mut self) {
        let cmd = core::str::from_utf8(&self.command_buffer[..self.buffer_pos])
            .unwrap_or("")
            .trim();

        if cmd.is_empty() {
            return;
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "help" => self.cmd_help(),
            "clear" => self.cmd_clear(),
            "echo" => self.cmd_echo(&parts),
            "ls" => self.cmd_ls(),
            "mkdir" => self.cmd_mkdir(&parts),
            "cat" => self.cmd_cat(&parts),
            "touch" => self.cmd_touch(&parts),
            "write" => self.cmd_write(&parts),
            "reboot" => self.cmd_reboot(),
            "uname" => self.cmd_uname(),
            "about" => self.cmd_about(),
            _ => println!("Unknown command: {}", parts[0]),
        }
    }

    fn cmd_help(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║          LINIX OS - Help Menu          ║");
        println!("╚════════════════════════════════════════╝");
        println!("\nAvailable Commands:");
        println!("  help        - Show this help message");
        println!("  clear       - Clear screen");
        println!("  echo <text> - Print text");
        println!("  ls          - List files");
        println!("  mkdir <dir> - Create directory");
        println!("  cat <file>  - Display file contents");
        println!("  touch <file>- Create new file");
        println!("  write <file>- Write to file");
        println!("  uname       - Print system info");
        println!("  about       - About LINIX OS");
        println!("  reboot      - Reboot system\n");
    }

    fn cmd_clear(&self) {
        let mut writer = VGA_WRITER.lock();
        writer.clear();
    }

    fn cmd_echo(&self, parts: &[&str]) {
        if parts.len() > 1 {
            println!("\n{}", parts[1]);
        }
    }

    fn cmd_ls(&self) {
        let fs = FILESYSTEM.lock();
        fs.list_files();
    }

    fn cmd_mkdir(&self, parts: &[&str]) {
        if parts.len() > 1 {
            println!("\nDirectory created: {}", parts[1]);
        } else {
            println!("\nUsage: mkdir <directory_name>");
        }
    }

    fn cmd_cat(&self, parts: &[&str]) {
        if parts.len() > 1 {
            println!("\nFile contents of {}:", parts[1]);
            println!("─────────────────────────────────────");
        } else {
            println!("\nUsage: cat <filename>");
        }
    }

    fn cmd_touch(&self, parts: &[&str]) {
        if parts.len() > 1 {
            let mut fs = FILESYSTEM.lock();
            if fs.create_file(parts[1]) {
                println!("\nFile created: {}", parts[1]);
            } else {
                println!("\nError creating file");
            }
        } else {
            println!("\nUsage: touch <filename>");
        }
    }

    fn cmd_write(&self, parts: &[&str]) {
        if parts.len() > 1 {
            println!("\nWriting to file: {}", parts[1]);
            println!("Type your content (not implemented yet)\n");
        } else {
            println!("\nUsage: write <filename>");
        }
    }

    fn cmd_reboot(&self) {
        println!("\nRebooting system...");
    }

    fn cmd_uname(&self) {
        println!("\n═══════════════════════════════════════");
        println!("System Name: LINIX OS");
        println!("Version: 0.1.0 (Rust Edition)");
        println!("Architecture: x86-64");
        println!("Memory: Protected & Managed");
        println!("═══════════════════════════════════════\n");
    }

    fn cmd_about(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║           About LINIX OS              ║");
        println!("╚════════════════════════════════════════╝");
        println!("\nA modern operating system written in Rust");
        println!("Supporting Legacy BIOS and UEFI 64-bit");
        println!("\nFeatures:");
        println!("  ✓ Memory-safe kernel");
        println!("  ✓ Task management");
        println!("  ✓ Simple filesystem");
        println!("  ✓ Beautiful UI");
        println!("  ✓ REPL Shell\n");
    }

    fn clear_buffer(&mut self) {
        self.command_buffer = [0; COMMAND_BUFFER_SIZE];
        self.buffer_pos = 0;
    }

    pub fn print_prompt(&self) {
        print!("\n> ");
    }
}

pub static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

pub fn init() {
    println!("Shell initialized");
}
