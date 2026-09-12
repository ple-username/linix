# LINIX Kernel

## Architecture: x86-64 (Long Mode)

### Kernel Boot Process (boot.asm)

**Mode:** 32-bit Protected Mode → 64-bit Long Mode

#### Key Steps:

1. **CPU Validation**
   - Check CPUID support
   - Verify long mode (LM) support

2. **Memory Setup**
   - Setup page tables (PML4, PDP, PD)
   - Identity map first 1GB of RAM
   - 2MB page entries for efficiency

3. **64-bit Transition**
   - Enable PAE (Physical Address Extension)
   - Set CR3 to PML4
   - Enable LME (Long Mode Enable) in EFER MSR
   - Enable paging (PG bit in CR0)
   - Load 64-bit GDT
   - Jump to 64-bit code segment

4. **Call kernel_main()**

### Kernel Main (main.c)

#### Features:
- VGA display initialization
- System information printing
- Kernel main loop

#### Output on Boot:
```
=== LINIX Kernel ===
64-bit Long Mode Active

Kernel initialized successfully!
Starting system services...

System Information:
CPU: x86-64
Architecture: 64-bit
Paging: Enabled

Kernel running. Halting...
```

### VGA Display Driver (vga.c/vga.h)

#### Features:
- VGA text mode (80x25)
- 16 colors support
- String printing
- Screen scrolling
- Hex value printing

#### VGA Memory:
- Base address: 0xB8000
- Each character = 2 bytes (character + attribute)
- Attribute = foreground color (4 bits) + background color (4 bits)

## Compilation

```bash
# Compile kernel boot
nasm -f elf64 kernel/arch/x86_64/boot.asm -o kernel_boot.o

# Compile C files
gcc -ffreestanding -fno-pie -m64 -c kernel/main.c -o kernel_main.o
gcc -ffreestanding -fno-pie -m64 -c kernel/vga.c -o kernel_vga.o

# Link all together
ld -T kernel.ld -o linix.elf kernel_boot.o kernel_main.o kernel_vga.o
```

## Testing with QEMU

```bash
# Run kernel
qemu-system-x86_64 -kernel linix.elf -m 256M
```
