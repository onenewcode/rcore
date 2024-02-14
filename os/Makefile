TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
KERNEL_ENTRY_PA := 0x80200000

# BOARD
BOARD = qemu
SBI = rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin



# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64


build: env $(KERNEL_BIN)

switch-check:
# (which last-qemu): 这部分使用 which 命令查找名为 last-qemu 的可执行文件在系统中的路径。如果找到了该文件，则返回其路径；否则，返回非零退出状态（表示未找到）。
# ||：这是逻辑或运算符，在 shell 中用于连接两个命令。如果左侧的命令执行失败（即返回非零退出状态），则执行右侧的命令。
	(which last-qemu) || (touch last-qemu && make clean)
env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@cd ../user && make build 
	@echo Platform: $(BOARD)
	@cargo build --release
clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner

QEMU_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

run-inner: build
	@qemu-system-riscv64 $(QEMU_ARGS)


debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

.PHONY: build env kernel clean disasm disasm-vim run-inner switch-check