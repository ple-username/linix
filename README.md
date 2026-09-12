# LINIX OS

Một hệ điều hành đơn giản hỗ trợ **Legacy BIOS** và **UEFI 64-bit** cho kiến trúc x86-64.

## Cấu trúc Project

```
linix/
├── bootloader/
│   ├── legacy/          # BIOS bootloader (16-bit real mode)
│   │   └── boot.asm     # MBR bootloader
│   └── uefi/            # UEFI bootloader (64-bit)
│       └── main.c       # UEFI entry point
├── kernel/
│   ├── arch/x86_64/
│   │   ├── boot.asm     # Kernel entry point (64-bit)
│   │   └── gdt.c        # Global Descriptor Table
│   ├── mm/
│   │   └── paging.c     # Paging setup
│   ├── main.c           # Kernel main
│   └── vga.c            # VGA display driver
├── Makefile             # Build script
├── README.md            # This file
└── .gitignore
```

## Yêu cầu

- **GCC Cross Compiler** (i686-elf-gcc, x86_64-elf-gcc)
- **NASM** (Assembler)
- **QEMU** (để test)
- **Make**

## Build

```bash
make clean
make
```

## Test với QEMU

```bash
# Legacy BIOS
qemu-system-x86_64 -drive format=raw,file=linix.iso -m 256M

# UEFI
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive format=raw,file=linix.iso -m 256M
```

## Tính năng Hiện tại

- [x] Legacy BIOS bootloader
- [x] UEFI bootloader
- [ ] Kernel 64-bit
- [ ] GDT setup
- [ ] Paging
- [ ] Interrupt handling
- [ ] Shell
- [ ] Filesystem

## Hướng dẫn Phát triển

Xem chi tiết trong từng thư mục.
