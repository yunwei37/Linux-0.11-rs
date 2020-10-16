ARCH  		:= riscv64
TARGET      := $(ARCH)imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/lab
BOOT  		:= boot/$(ARCH)
BIN_FILE    := $(BOOT)/Image

QEMU_BINARY := qemu-system-riscv64
QEMUOPTS	:= -nographic -machine virt -bios none -device loader,file=$(BIN_FILE),addr=0x80000000

kernel:
	@cargo build

build: kernel boot
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_FILE) --strip-all -O binary $(BIN_FILE)
	@cp $(KERNEL_FILE) vmlinux

boot:
	@mkdir boot
	@mkdir $(BOOT)

qemu: build
	@$(QEMU_BINARY) $(QEMUOPTS)

debug: build
	@$(QEMU_BINARY) $(QEMUOPTS) -S -s

run: build qemu

clean:
	@cargo clean
	@rm -rf boot
	@rm vmlinux