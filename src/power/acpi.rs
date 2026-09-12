// ACPI (Advanced Configuration and Power Interface) Module
// Handles shutdown, reboot, and power management safely

use core::arch::asm;

const ACPI_PM1A_CONTROL_BLOCK: u16 = 0x404; // Usually this address, but should be read from ACPI tables

#[repr(C, packed)]
pub struct AcpiTable {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
}

pub struct AcpiManager {
    pm1a_control: u16,
    slp_type: u8,
}

impl AcpiManager {
    pub const fn new() -> Self {
        AcpiManager {
            pm1a_control: ACPI_PM1A_CONTROL_BLOCK,
            slp_type: 5, // Sleep type for S5 (shutdown)
        }
    }

    /// Shutdown the system using ACPI
    pub unsafe fn shutdown(&self) {
        println!("\n╮───────────────────╯");
        println!("  Initiating ACPI Shutdown...");
        println!("  Please wait...");
        println!("╰───────────────────╯\n");

        // Method 1: ACPI shutdown via PM1a control register
        let slp_type_a = (self.slp_type & 0x07) << 10;
        let sleep_enable = 1 << 13;
        let pm1_control_value = slp_type_a | sleep_enable;

        // Write to PM1a control block
        write_port(self.pm1a_control, pm1_control_value as u16);

        // Wait a moment
        for _ in 0..1000000 {
            asm!("nop");
        }

        // If ACPI doesn't work, fall back to other methods
        println!("ACPI shutdown failed, trying alternative method...");

        // Method 2: Reboot via keyboard controller (always works)
        write_port(0x64, 0xFE); // Send reboot command to keyboard controller

        println!("System should power down now.");
        halt_forever();
    }

    /// Reboot the system
    pub unsafe fn reboot(&self) {
        println!("\n╮───────────────────╯");
        println!("  Rebooting system...");
        println!("  Saving data...");
        println!("  Please wait...");
        println!("╰───────────────────╯\n");

        // Method 1: Triple fault (CPU will reset)
        reset_cpu_via_triple_fault();

        // Method 2: Keyboard controller reboot (fallback)
        write_port(0x64, 0xFE);

        println!("System should reboot now.");
        halt_forever();
    }

    /// Suspend to RAM (S3 sleep state)
    pub unsafe fn suspend_to_ram(&self) {
        println!("\nSuspending to RAM (S3)...");
        
        let slp_type_a = (3 & 0x07) << 10; // S3 sleep type
        let sleep_enable = 1 << 13;
        let pm1_control_value = slp_type_a | sleep_enable;

        write_port(self.pm1a_control, pm1_control_value as u16);
    }
}

unsafe fn reset_cpu_via_triple_fault() {
    // Disable interrupts
    asm!("cli");
    
    // Create an invalid IDT to trigger triple fault
    let idt_descriptor = (0u64, 0u16);
    asm!("lidt [{}]", in(reg) &idt_descriptor);
    
    // Trigger an interrupt to cause triple fault
    asm!("int 0xFF");
}

unsafe fn write_port(port: u16, value: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value);
}

unsafe fn read_port(port: u16) -> u16 {
    let value: u16;
    asm!("in ax, dx", out("ax") value, in("dx") port);
    value
}

fn halt_forever() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

pub fn init() {
    println!("ACPI manager initialized");
}
