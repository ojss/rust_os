#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
extern crate volatile;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;
mod memory;

use memory::FrameAllocator;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) -> ! {
    //    let hello= b"Hello Ojas!";
    //    let color_byte = 0x1f;
    //
    //    let mut hello_colored = [color_byte; 22];
    //
    //    for (i, char_byte) in hello.into_iter().enumerate() {
    //        hello_colored[i * 2] = *char_byte;
    //    }
    //    unsafe {
    //        let vga = (0xb8000) as *mut [u8; 22];
    //        *vga = hello_colored;
    //    };

    // to remove the booting from intermezzOS line
    vga_buffer::clear_screen();

    kprintln!("Hello Word! {}", 42);
    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    kprintln!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        kprintln!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf sections required");
    kprintln!("kernel sections");
    for section in elf_sections_tag.sections() {
        kprintln!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    kprintln!("kernel start: 0x{:x}, kernel end: 0x{:x}", kernel_start, kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    kprintln!("multiboot start: 0x{:x}, multiboot end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator =
        memory::AreaFrameAllocator::new(kernel_start as usize,
                                        kernel_end as usize, multiboot_start,
                                        multiboot_end, memory_map_tag.memory_areas());
    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            kprintln!("allocated {} frames", i);
            break;
        }
    }
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    kprintln!("\n\nPANIC in {} at line {}:", file, line);
    kprintln!("    {}", fmt);
    loop {}
}