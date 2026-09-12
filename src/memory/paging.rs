// Paging module - Virtual memory management

use core::arch::asm;

const PAGE_SIZE: usize = 4096;

#[repr(transparent)]
pub struct PageTable([PageEntry; 512]);

#[repr(transparent)]
pub struct PageEntry(u64);

impl PageEntry {
    pub fn new(addr: u64, flags: u64) -> Self {
        PageEntry((addr & 0x000FFFFFFFFFF000) | flags)
    }

    pub fn is_present(&self) -> bool {
        self.0 & 1 != 0
    }

    pub fn addr(&self) -> u64 {
        self.0 & 0x000FFFFFFFFFF000
    }
}

pub fn init() {
    // Paging is already set up by bootloader
    // This function is for any additional setup
}

pub fn get_cr3() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) value);
    }
    value
}

pub fn set_cr3(value: u64) {
    unsafe {
        asm!("mov cr3, {}", in(reg) value);
    }
}
