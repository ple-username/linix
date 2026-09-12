# LINIX Bootloader

## Legacy BIOS Bootloader (boot.asm)

**Mode:** 16-bit Real Mode → 32-bit Protected Mode

### Features:
- Bootloader signature (0xAA55)
- GDT setup
- A20 line enable
- Disk read functionality
- Transition to protected mode
- Jump to kernel at 0x1000:0000

### Process:
1. BIOS loads bootloader at 0x7C00
2. Setup stack and save boot drive
3. Display welcome message
4. Load kernel from disk (4 sectors starting at sector 2)
5. Load GDT
6. Enable A20 line
7. Set PE bit in CR0 (enter protected mode)
8. Jump to kernel

## UEFI Bootloader (main.c)

**Mode:** 64-bit UEFI

### Features:
- UEFI entry point (efi_main)
- Memory map retrieval
- Screen clear and text output
- Boot services management
- Preparation for kernel jump

### Process:
1. Initialize UEFI
2. Display welcome message
3. Get system memory information
4. Load kernel file from EFI filesystem
5. Exit boot services
6. Jump to kernel

## Building

```bash
# Build bootloader
nasm -f bin bootloader/legacy/boot.asm -o boot.bin

# Build UEFI bootloader (requires edk2 or UEFI SDK)
```
