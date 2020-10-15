TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/lab
BIN_FILE    := Image

kernel:
	@cargo build

build: kernel
	@rust-objcopy --binary-architecture=riscv64 $(KERNEL_FILE) --strip-all -O binary $(BIN_FILE)

qemu: build
	@qemu-system-riscv64 -nographic -machine virt -device loader,file=$(BIN_FILE),addr=0x80000000 

debug: build
	@qemu-system-riscv64 -nographic -machine virt -device loader,file=$(BIN_FILE),addr=0x80000000 -S -s

run: build qemu

clean:
	@cargo clean
	@rm Image