# LINIX OS - Rust Edition

Một hệ điều hành viết bằng **Rust** hỗ trợ **Legacy BIOS** và **UEFI 64-bit** với memory management an toàn.

## Cấu trúc Project

```
linix-rs/
├── src/
│   ├── main.rs              # Kernel entry point
│   ├── bootloader.rs        # Bootloader setup
│   ├── memory/
│   │   ├── mod.rs           # Memory management
│   │   ├── allocator.rs     # Custom allocator
│   │   ├── paging.rs        # Page table management
│   │   └── heap.rs          # Heap initialization
│   ├── display/
│   │   ├── mod.rs           # Display subsystem
│   │   └── vga.rs           # VGA text mode
│   ├── arch/
│   │   └── x86_64/
│   │       ├── mod.rs
│   │       ├── boot.s        # Assembly entry
│   │       ├── gdt.rs        # Global Descriptor Table
│   │       └── interrupts.rs # Interrupt handling
│   └── lib.rs
├── Cargo.toml               # Rust dependencies
├── Cargo.lock
├── x86_64-unknown-none.json  # Rust target spec
├── Makefile                 # Build script
└── README.md
```

## Yêu cầu

- **Rust** (nightly version)
- **Cargo**
- **NASM** (Assembler cho bootloader)
- **QEMU** (để test)
- **Cross compiler** (x86_64-elf-gcc, optional)

## Cài đặt Rust cho x86_64

```bash
rustup toolchain install nightly
rustup target add x86_64-unknown-none
rustup component add rust-src
```

## Build

```bash
cargo build --target x86_64-unknown-none --release
```

## Test với QEMU

```bash
# Legacy BIOS
qemu-system-x86_64 -kernel target/x86_64-unknown-none/release/linix_os -m 256M

# UEFI
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -kernel target/x86_64-unknown-none/release/linix_os -m 256M
```

## Tính năng Hiện tại

- [x] Rust bootloader setup
- [x] VGA display driver
- [x] Memory management (allocator, paging)
- [x] GDT configuration
- [ ] Interrupt Descriptor Table (IDT)
- [ ] Exception handling
- [ ] Task management
- [ ] Filesystem
- [ ] Shell

## Memory Management

### Allocator
Custom allocator không dùng std library:
- Bump allocator (simple, fast)
- Linked list allocator (flexible)

### Paging
- Identity mapping (virtual = physical)
- Page table management
- Safe Rust abstractions

### Heap
- Dynamic memory allocation
- Drop trait support
- Ownership system

## Tham khảo

- [Writing an OS in Rust](https://os.phil-opp.com/)
- [OSDev.org](https://wiki.osdev.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [x86_64 Crate](https://docs.rs/x86_64/)
