default: build

.PHONY: clean

build: build/os.iso

build/multiboot_header.o: multiboot_header.asm
	mkdir -p build
	nasm -f elf64 multiboot_header.asm -o build/multiboot_header.o

build/boot.o: boot.asm
	mkdir -p build
	nasm -f elf64 boot.asm -o build/boot.o

build/long_mode_start.o: long_mode_start.asm
	mkdir -p build
	nasm -f elf64 long_mode_start.asm -o build/long_mode_start.o

build/kernel.bin: build/multiboot_header.o build/boot.o build/long_mode_start.o linker.ld
	x86_64-pc-elf-ld -n -o build/kernel.bin -T linker.ld build/multiboot_header.o build/boot.o build/long_mode_start.o

build/os.iso: build/kernel.bin grub.cfg
	mkdir -p build/isofiles/boot/grub
	cp grub.cfg build/isofiles/boot/grub
	cp build/kernel.bin build/isofiles/boot/
	grub-mkrescue -o build/os.iso build/isofiles

run: build/os.iso
	qemu-system-x86_64 -cdrom build/os.iso
clean:
	rm -rf build