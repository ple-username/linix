; Legacy BIOS Bootloader for LINIX OS
; This bootloader loads the kernel from disk into memory
; Runs in 16-bit real mode

[ORG 0x7C00]                    ; BIOS loads bootloader at 0x7C00
[BITS 16]                       ; 16-bit code

BOOT_DRIVE db 0                 ; Store boot drive number
MSG_REAL_MODE db "LINIX Bootloader - Real Mode", 0
MSG_LOADING db "Loading kernel...", 0
MSG_ERROR db "Error loading kernel!", 0

start:
    ; Setup stack
    mov ax, 0x0000
    mov ss, ax
    mov sp, 0x7C00              ; Stack pointer below bootloader
    
    ; Save boot drive
    mov [BOOT_DRIVE], dl
    
    ; Clear screen
    mov ax, 0x0600              ; AH=6 (scroll), AL=0 (full screen)
    mov bh, 0x07                ; Normal attribute
    xor cx, cx                  ; Top-left corner (0, 0)
    mov dx, 0x184F              ; Bottom-right corner (24, 79)
    int 0x10
    
    ; Print welcome message
    mov si, MSG_REAL_MODE
    call print_string
    
    ; Print loading message
    mov si, MSG_LOADING
    call print_string
    
    ; Load kernel into memory at 0x1000:0000
    mov ax, 0x1000
    mov es, ax
    xor bx, bx                  ; Buffer offset
    
    ; Read disk sectors
    mov ah, 0x02                ; Read sectors function
    mov al, 4                   ; Number of sectors to read
    mov ch, 0                   ; Cylinder
    mov cl, 2                   ; Sector (starts at 2 after bootloader)
    mov dh, 0                   ; Head
    mov dl, [BOOT_DRIVE]
    int 0x13
    
    jc disk_error               ; Jump if error (Carry Flag set)
    
    ; Enter protected mode
    cli                         ; Disable interrupts
    
    ; Load GDT
    lgdt [gdt_descriptor]
    
    ; Enable A20 line
    mov ax, 0x2401
    int 0x15
    
    ; Set CR0 bit 0 to enter protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    ; Jump to protected mode code
    jmp 0x08:protected_mode

disk_error:
    mov si, MSG_ERROR
    call print_string
    jmp halt

; Print string in SI
print_string:
    mov ah, 0x0E                ; Teletype output
.loop:
    lodsb                       ; Load byte from [DS:SI] into AL, increment SI
    test al, al                 ; Check for null terminator
    jz .done
    int 0x10                    ; Print character
    jmp .loop
.done:
    ret

halt:
    cli
    hlt
    jmp halt

; GDT (Global Descriptor Table)
align 8
gdt:
    ; Null descriptor
    dq 0
    
    ; Code descriptor (0x08)
    dw 0xFFFF                   ; Limit
    dw 0x0000                   ; Base (low)
    db 0x00                     ; Base (mid)
    db 10011010b                ; Access byte
    db 11001111b                ; Flags/Limit
    db 0x00                     ; Base (high)
    
    ; Data descriptor (0x10)
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 11001111b
    db 0x00

gdt_descriptor:
    dw $ - gdt - 1              ; Size of GDT - 1
    dd gdt                      ; Address of GDT

[BITS 32]
protected_mode:
    mov ax, 0x10                ; Data segment
    mov ds, ax
    mov es, ax
    mov ss, ax
    
    ; Jump to kernel at 0x1000:0x0000
    jmp 0x1000:0x0000

; Bootloader signature
times 510 - ($ - $$) db 0
dw 0xAA55                       ; Boot signature
