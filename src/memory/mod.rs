// Memory Management Module

pub mod allocator;
pub mod paging;
pub mod heap;

use allocator::BumpAllocator;
use core::alloc::GlobalAlloc;
use core::ptr::NonNull;
use core::cell::UnsafeCell;

pub struct KernelAllocator {
    inner: UnsafeCell<BumpAllocator>,
}

impl KernelAllocator {
    pub const fn new() -> Self {
        KernelAllocator {
            inner: UnsafeCell::new(BumpAllocator::new()),
        }
    }
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        (*self.inner.get()).alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Bump allocator doesn't dealloc
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator::new();

pub fn init() {
    paging::init();
    heap::init();
}
