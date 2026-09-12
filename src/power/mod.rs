// Power Management Module

pub mod acpi;

use spin::Mutex;

pub static ACPI_MANAGER: Mutex<acpi::AcpiManager> = Mutex::new(acpi::AcpiManager::new());

pub fn init() {
    acpi::init();
}

pub fn shutdown() -> ! {
    unsafe {
        ACPI_MANAGER.lock().shutdown();
    }
    // This should not return
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

pub fn reboot() -> ! {
    unsafe {
        ACPI_MANAGER.lock().reboot();
    }
    // This should not return
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
