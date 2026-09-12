# LINIX OS Makefile (Rust)

.PHONY: all build run clean test

CARGO = cargo
NASM = nasm
QEMU = qemu-system-x86_64
TARGET = x86_64-unknown-none

all: build

build:
	@echo "[Building LINIX OS with Rust]"
	$(CARGO) build --target $(TARGET) --release

bootloader:
	@echo "[Assembling bootloader]"
	$(NASM) -f bin src/bootloader.s -o boot.bin

run: build
	@echo "[Running in QEMU]"
	$(QEMU) -kernel target/$(TARGET)/release/linix_os -m 256M

run-uefi: build
	@echo "[Running UEFI mode in QEMU]"
	$(QEMU) -bios /usr/share/ovmf/OVMF.fd -kernel target/$(TARGET)/release/linix_os -m 256M

clean:
	@echo "[Cleaning build artifacts]"
	$(CARGO) clean
	@rm -f boot.bin linix.iso

test:
	@echo "[Running tests]"
	$(CARGO) test --target $(TARGET)

help:
	@echo "LINIX OS Build Targets:"
	@echo "  make build     - Build kernel"
	@echo "  make run       - Build and run in QEMU (Legacy BIOS)"
	@echo "  make run-uefi  - Build and run in QEMU (UEFI mode)"
	@echo "  make clean     - Remove build files"
	@echo "  make test      - Run tests"
	@echo "  make help      - Show this message"
