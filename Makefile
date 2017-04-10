multiboot_header_asm=src/asm/multiboot_header.asm
boot_asm=src/asm/boot.asm
long_mode_start_asm=src/asm/long_mode_start.asm
grub_cfg=src/asm/grub.cfg

assembly_src_files=$(multiboot_header) $(boot_asm) $(long_mode_start_asm)
assembly_object_files=target/multiboot_header.o target/boot.o target/long_mode_start.o
static_libs=target/x86_64-unknown-intermezzos-gnu/release/libintermezzos.a
linker_script=src/asm/linker.ld


arch=x86_64
target ?= $(arch)-unknown-linux-gnu
intermezzos := target/$(target)/debug

default: build

build: target/kernel.bin

.PHONY: clean

target/multiboot_header.o: $(multiboot_header_asm)
	mkdir -p target
	nasm -f elf64 $(multiboot_header_asm) -o target/multiboot_header.o

target/boot.o: $(boot_asm)
	mkdir -p target
	nasm -f elf64 $(boot_asm) -o target/boot.o

target/long_mode_start.o: $(long_mode_start_asm)
	mkdir -p target
	nasm -f elf64 $(long_mode_start_asm) -o target/long_mode_start.o

target/kernel.bin: $(assembly_object_files) $(linker_script) cargo
	x86_64-pc-elf-ld -n -o target/kernel.bin -gc-sections -T $(linker_script) $(assembly_object_files) $(static_libs)

target/os.iso: target/kernel.bin $(grub_cfg)
	mkdir -p target/isofiles/boot/grub
	cp $(grub_cfg) target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	grub-mkrescue -o target/os.iso target/isofiles

run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso

clean:
	cargo clean

cargo:
	xargo build --release --target x86_64-unknown-intermezzos-gnu