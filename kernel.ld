/* LINIX Kernel Linker Script */

ENTRY(start64)

SECTIONS {
    . = 0x100000;  /* Kernel loaded at 1MB mark */
    
    .text : {
        *(.text)
    }
    
    .data : {
        *(.data)
    }
    
    .rodata : {
        *(.rodata)
    }
    
    .bss : {
        *(.bss)
    }
}
