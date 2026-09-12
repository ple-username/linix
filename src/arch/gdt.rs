// Global Descriptor Table (GDT) for x86_64

use core::mem::size_of;

#[repr(C)]
pub struct SegmentDescriptor {
    limit: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        Self {
            limit: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (granularity & 0xF0) | ((limit >> 16) & 0x0F) as u8,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    fn null() -> Self {
        Self {
            limit: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,
    offset: u64,
}

pub fn init() {
    // GDT setup for long mode
    // In long mode, only code and data segments are used
    // Base and limit are ignored
}
