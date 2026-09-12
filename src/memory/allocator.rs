// Bump Allocator - Simple memory allocator

use core::alloc::Layout;
use core::ptr::NonNull;

const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
const HEAP_START: usize = 0xFFFF800000000000; // Kernel heap start (high memory)

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: HEAP_START,
            heap_end: HEAP_START + HEAP_SIZE,
            next: HEAP_START,
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());

        if alloc_end <= self.heap_end {
            self.next = alloc_end;
            alloc_start as *mut u8
        } else {
            core::ptr::null_mut()
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
