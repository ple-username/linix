; LINIX Kernel Boot - 64-bit Long Mode
; Entry point from bootloader at 0x1000:0x0000

[ORG 0x10000]
[BITS 32]

; Multiboot header for GRUB
ALIGN 4
MULTIBOOT_HEADER:
    dd 0x1BADB002               ; Magic number
    dd 0x00000000               ; Flags
    dd -(0x1BADB002 + 0x00000000) ; Checksum

start:
    cli                         ; Disable interrupts
    
    ; Setup stack
    mov esp, kernel_stack_end
    
    ; Check if CPU supports 64-bit
    call check_cpuid
    call check_long_mode
    
    ; Setup paging for long mode
    call setup_page_tables
    
    ; Enable PAE (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5              ; Set PAE bit
    mov cr4, eax
    
    ; Set CR3 to point to PML4
    mov eax, pml4
    mov cr3, eax
    
    ; Enable long mode
    mov ecx, 0xC0000080         ; EFER MSR
    rdmsr
    or eax, 1 << 8              ; Set LME bit
    wrmsr
    
    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31             ; Set PG bit
    mov cr0, eax
    
    ; Load 64-bit GDT
    lgdt [gdt64_descriptor]
    
    ; Jump to 64-bit code
    jmp 0x08:start64

; Check CPUID support
check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21            ; Toggle ID bit
    push eax
    popfd
    pushfd
    pop eax
    
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov eax, 'NO_'
    jmp error

; Check long mode support
check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jl .no_long_mode
    
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29           ; Check LM bit
    jz .no_long_mode
    ret
.no_long_mode:
    mov eax, 'LM_'
    jmp error

; Setup page tables (identity mapping)
setup_page_tables:
    ; Clear page tables
    mov edi, pml4
    xor eax, eax
    mov ecx, 0x1000 * 4 / 4    ; Clear 4 pages (16KB)
    rep stosd
    
    ; Setup PML4
    mov eax, pdp
    or eax, 0x03                ; Present + Write
    mov [pml4], eax
    
    ; Setup PDP
    mov eax, pd
    or eax, 0x03
    mov [pdp], eax
    
    ; Setup PD (2MB pages)
    mov ecx, 512                ; 512 entries for 1GB mapping
    mov eax, 0x00000000 | 0x83 ; Page 0, Present + Write + Huge
    mov edi, pd
.loop:
    mov [edi], eax
    add eax, 0x200000           ; Next 2MB page
    add edi, 8
    loop .loop
    
    ret

error:
    jmp halt

halt:
    cli
    hlt
    jmp halt

; 32-bit GDT
align 8
gdt32:
    dq 0                        ; Null
    ; Code 0x08
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0x9A
    db 0xCF
    db 0x00
    ; Data 0x10
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0x92
    db 0xCF
    db 0x00

gdt32_descriptor:
    dw $ - gdt32 - 1
    dd gdt32

[BITS 64]
start64:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    
    ; Call kernel main function
    extern kernel_main
    call kernel_main
    
    jmp halt

; 64-bit GDT
align 8
gdt64:
    dq 0                        ; Null
    ; Code 0x08
    dq 0x00af9a000000ffff       ; 64-bit code segment
    ; Data 0x10
    dq 0x00af92000000ffff       ; 64-bit data segment

gdt64_descriptor:
    dw $ - gdt64 - 1
    dq gdt64

section .bss
align 4096
pml4:
    resq 512
pdp:
    resq 512
pd:
    resq 512
kernel_stack:
    resb 16384
kernel_stack_end:
