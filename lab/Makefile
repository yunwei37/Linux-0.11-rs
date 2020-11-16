ARCH  		:= riscv64
TARGET      := $(ARCH)imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/lab
BOOT  		:= boot/$(ARCH)
BIN_FILE    := $(BOOT)/Image
ASM_FILE	:= kernel.asm

QEMU_BINARY := qemu-system-riscv64
QEMUOPTS	:= -nographic -machine virt -bios none -device loader,file=$(BIN_FILE),addr=0x80000000

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

SCHED		:= short_job_first

kernel:
	@cargo build --features $(SCHED)

build: kernel boot
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $(BIN_FILE)
	@cp $(KERNEL_FILE) vmlinux
	@$(OBJDUMP) -d $(KERNEL_FILE) > $(ASM_FILE)

boot:
	@mkdir boot
	@mkdir $(BOOT)

qemu: build
	@$(QEMU_BINARY) $(QEMUOPTS)

debug: build
	@$(QEMU_BINARY) $(QEMUOPTS) -S -s

run: build qemu

gdb: build
	riscv64-unknown-elf-gdb vmlinux

clean:
	@cargo clean
	@rm -rf boot
	@rm -f vmlinux
	@rm -f $(ASM_FILE)