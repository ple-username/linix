// x86_64 Boot initialization

use core::arch::asm;

pub unsafe fn init_long_mode() {
    // Check long mode support (already done in bootloader)
    // This is for any additional setup needed
}

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
