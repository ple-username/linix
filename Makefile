# LINIX OS Makefile (Rust)

.PHONY: all build run run-uefi clean test help setup install-deps

CARGO = cargo
NASM = nasm
QEMU = qemu-system-x86_64
TARGET = x86_64-unknown-none
KERNEL_ELF = target/$(TARGET)/release/linix_os
BOOTLOADER = src/bootloader.s

# Color output
GREEN = \033[0;32m
BLUE = \033[0;34m
YELLOW = \033[0;33m
RED = \033[0;31m
NC = \033[0m

all: build

setup:
	@echo "$(BLUE)[Setting up Rust environment]$(NC)"
	rustup toolchain install nightly
	rustup target add x86_64-unknown-none
	rustup component add rust-src
	@echo "$(GREEN)[Setup complete]$(NC)"

install-deps:
	@echo "$(BLUE)[Checking dependencies]$(NC)"
	@which cargo > /dev/null || (echo "$(RED)Error: Cargo not installed$(NC)" && exit 1)
	@which nasm > /dev/null || (echo "$(RED)Error: NASM not installed$(NC)" && exit 1)
	@which qemu-system-x86_64 > /dev/null || (echo "$(RED)Error: QEMU not installed$(NC)" && exit 1)
	@echo "$(GREEN)[All dependencies found]$(NC)"

build: install-deps
	@echo "$(BLUE)[Building LINIX OS kernel]$(NC)"
	@echo "$(YELLOW)Target: $(TARGET)$(NC)"
	$(CARGO) build --target $(TARGET) --release
	@echo "$(GREEN)[Build successful!]$(NC)"
	@echo "$(GREEN)Kernel ELF: $(KERNEL_ELF)$(NC)"

run: build
	@echo "$(BLUE)[Running LINIX OS - Legacy BIOS Mode]$(NC)"
	@echo "$(YELLOW)Press Ctrl+A then X to exit QEMU$(NC)"
	@echo ""
	$(QEMU) -kernel $(KERNEL_ELF) -m 256M -serial stdio

run-uefi: build
	@echo "$(BLUE)[Running LINIX OS - UEFI 64-bit Mode]$(NC)"
	@echo "$(YELLOW)Press Ctrl+A then X to exit QEMU$(NC)"
	@echo ""
	$(QEMU) -bios /usr/share/ovmf/OVMF.fd -kernel $(KERNEL_ELF) -m 256M -serial stdio

run-debug: build
	@echo "$(BLUE)[Running LINIX OS - Debug Mode]$(NC)"
	$(QEMU) -kernel $(KERNEL_ELF) -m 256M -s -S -serial stdio

run-kvm: build
	@echo "$(BLUE)[Running LINIX OS - KVM Mode]$(NC)"
	$(QEMU) -kernel $(KERNEL_ELF) -m 256M -enable-kvm -cpu host

run-multi: build
	@echo "$(BLUE)[Running LINIX OS - 4 CPUs]$(NC)"
	$(QEMU) -kernel $(KERNEL_ELF) -m 256M -smp 4 -serial stdio

test:
	@echo "$(BLUE)[Running tests]$(NC)"
	$(CARGO) test --target $(TARGET)

clean:
	@echo "$(BLUE)[Cleaning build artifacts]$(NC)"
	$(CARGO) clean
	@rm -f boot.bin linix.iso
	@echo "$(GREEN)[Clean complete]$(NC)"

help:
	@echo ""
	@echo "$(BLUE)╔══════════════════════════════════════╗$(NC)"
	@echo "$(BLUE)║  LINIX OS Build System              ║$(NC)"
	@echo "$(BLUE)╚══════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(GREEN)Setup & Installation:$(NC)"
	@echo "  make setup        - Setup Rust environment"
	@echo "  make install-deps - Check dependencies"
	@echo ""
	@echo "$(GREEN)Build:$(NC)"
	@echo "  make build        - Build kernel (Release)"
	@echo ""
	@echo "$(GREEN)Run & Test:$(NC)"
	@echo "  make run          - Run Legacy BIOS mode"
	@echo "  make run-uefi     - Run UEFI 64-bit mode"
	@echo "  make run-debug    - Run with GDB"
	@echo "  make run-kvm      - Run with KVM (faster)"
	@echo "  make run-multi    - Run 4 CPUs"
	@echo "  make test         - Run tests"
	@echo ""
	@echo "$(GREEN)Maintenance:$(NC)"
	@echo "  make clean        - Clean artifacts"
	@echo "  make help         - Show this help"
	@echo ""
