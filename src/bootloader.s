# LINIX OS Bootloader (Assembly)
# x86_64 Real Mode Entry

[ORG 0x7C00]
[BITS 16]

start:
    cli                         ; Disable interrupts
    mov ax, 0                   ; Setup segments
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    
    ; Enable A20 line
    in al, 0x92
    or al, 2
    out 0x92, al
    
    ; Load GDT
    lgdt [gdt_descriptor]
    
    ; Enable protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    ; Jump to 32-bit code
    jmp 0x08:protected_mode

align 16
gdt:
    dq 0                        ; Null descriptor
    dq 0x00af9a000000ffff      ; Code descriptor
    dq 0x00af92000000ffff      ; Data descriptor

gdt_descriptor:
    dw $ - gdt - 1
    dd gdt

[BITS 32]
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    
    ; Load kernel at 1MB
    mov esi, 0x10000            ; Kernel source (load by bootloader)
    mov edi, 0x100000           ; Kernel destination (1MB)
    mov ecx, 0x200000           ; Size (2MB)
    rep movsb
    
    ; Enter long mode
    jmp 0x08:setup_long_mode

align 16
setup_long_mode:
    ; Setup paging for long mode
    mov eax, page_table_l4
    mov cr3, eax
    
    ; Enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    
    ; Enable long mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    
    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    
    ; Load 64-bit GDT
    lgdt [gdt64_descriptor]
    
    ; Jump to 64-bit kernel
    jmp 0x08:0x100000

align 4096
page_table_l4:
    dq page_table_l3 + 0x03
    times 510 dq 0
    
align 4096
page_table_l3:
    dq page_table_l2 + 0x03
    times 510 dq 0
    
align 4096
page_table_l2:
    dq 0x00000000 | 0x83        ; 2MB pages, identity mapped
    dq 0x00200000 | 0x83
    times 510 dq 0

align 16
gdt64:
    dq 0
    dq 0x00af9a000000ffff      ; Code
    dq 0x00af92000000ffff      ; Data

gdt64_descriptor:
    dw $ - gdt64 - 1
    dq gdt64

; Bootloader signature
times 510 - ($ - $$) db 0
dw 0xAA55
