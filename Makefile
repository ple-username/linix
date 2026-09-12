# LINIX OS Makefile

CC = gcc
AS = nasm
LD = ld
CFLAGS = -ffreestanding -fno-pie -m64
ASFLAGS = -f elf64

BOOTLOADER_ASM = bootloader/legacy/boot.asm
KERNEL_ASM = kernel/arch/x86_64/boot.asm
KERNEL_C = kernel/main.c kernel/vga.c

OBJ_FILES = kernel_boot.o kernel_main.o kernel_vga.o
OUTPUT = linix.elf
ISO_OUTPUT = linix.iso

.PHONY: all clean iso

all: $(OUTPUT)

$(OUTPUT): $(OBJ_FILES)
	$(LD) -T kernel.ld -o $@ $^

kernel_boot.o: $(KERNEL_ASM)
	$(AS) $(ASFLAGS) -o $@ $<

kernel_main.o: kernel/main.c
	$(CC) $(CFLAGS) -c -o $@ $<

kernel_vga.o: kernel/vga.c
	$(CC) $(CFLAGS) -c -o $@ $<

iso: all
	@echo "Building ISO image..."
	@echo "ISO support requires additional tools (grub, xorriso)"

clean:
	@echo "Cleaning build files..."
	rm -f *.o *.elf *.iso
	@echo "Clean complete!"

help:
	@echo "LINIX OS Build Targets:"
	@echo "  make          - Build kernel ELF"
	@echo "  make clean    - Remove build files"
	@echo "  make iso      - Build bootable ISO"
	@echo "  make help     - Show this message"

.PRECIOUS: $(OBJ_FILES)
