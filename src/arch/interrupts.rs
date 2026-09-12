// Interrupt Descriptor Table (IDT) for x86_64

use core::arch::asm;
use spin::Mutex;

const IDT_SIZE: usize = 256;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    options: u8,
    offset_mid: u8,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    fn new() -> Self {
        IdtEntry {
            offset_low: 0,
            segment_selector: 0,
            options: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64, selector: u16) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.offset_mid = ((handler >> 16) & 0xFF) as u8;
        self.offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
        self.segment_selector = selector;
        self.options = 0x8E; // Present, Ring 0, Type 110 (Interrupt Gate)
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,
    offset: u64,
}

pub struct Idt {
    entries: [IdtEntry; IDT_SIZE],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            entries: [IdtEntry::new(); IDT_SIZE],
        }
    }

    pub fn set_handler(&mut self, index: u8, handler: u64) {
        self.entries[index as usize].set_handler(handler, 0x08); // Code segment
    }

    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            size: (core::mem::size_of::<Idt>() - 1) as u16,
            offset: self as *const _ as u64,
        };

        unsafe {
            asm!("lidt [{}]", in(reg) &descriptor);
        }
    }
}

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

pub fn init() {
    let mut idt = IDT.lock();
    
    // Set up exception handlers
    idt.set_handler(0, exception_handler_divide_by_zero as u64);
    idt.set_handler(1, exception_handler_debug as u64);
    idt.set_handler(3, exception_handler_breakpoint as u64);
    idt.set_handler(6, exception_handler_invalid_opcode as u64);
    idt.set_handler(8, exception_handler_double_fault as u64);
    idt.set_handler(11, exception_handler_segment_not_present as u64);
    idt.set_handler(12, exception_handler_stack_segment_fault as u64);
    idt.set_handler(13, exception_handler_general_protection_fault as u64);
    idt.set_handler(14, exception_handler_page_fault as u64);
    
    drop(idt);
    IDT.lock().load();
}

// Exception handlers
pub extern "C" fn exception_handler_divide_by_zero() {
    println!("\n!!! EXCEPTION: Divide by Zero !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_debug() {
    println!("\n!!! EXCEPTION: Debug !!!");
}

pub extern "C" fn exception_handler_breakpoint() {
    println!("\n!!! EXCEPTION: Breakpoint !!!");
}

pub extern "C" fn exception_handler_invalid_opcode() {
    println!("\n!!! EXCEPTION: Invalid Opcode !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_double_fault() {
    println!("\n!!! EXCEPTION: Double Fault - CRITICAL !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_segment_not_present() {
    println!("\n!!! EXCEPTION: Segment Not Present !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_stack_segment_fault() {
    println!("\n!!! EXCEPTION: Stack Segment Fault !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_general_protection_fault() {
    println!("\n!!! EXCEPTION: General Protection Fault !!!");
    loop { unsafe { asm!("hlt"); } }
}

pub extern "C" fn exception_handler_page_fault() {
    println!("\n!!! EXCEPTION: Page Fault !!!");
    loop { unsafe { asm!("hlt"); } }
}
