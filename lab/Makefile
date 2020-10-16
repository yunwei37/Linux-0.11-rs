TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/lab
BIN_FILE    := Image

QEMU_BINARY := qemu-system-riscv64
QEMUOPTS	:= -nographic -machine virt -bios none -device loader,file=$(BIN_FILE),addr=0x80000000

kernel:
	@cargo build

build: kernel
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_FILE) --strip-all -O binary $(BIN_FILE)
	cp $(KERNEL_FILE) vmlinux

qemu: build
	@$(QEMU_BINARY) $(QEMUOPTS)

debug: build
	@$(QEMU_BINARY) $(QEMUOPTS) -S -s

run: build qemu

clean:
	@cargo clean
	@rm Image
	@rm vmlinux