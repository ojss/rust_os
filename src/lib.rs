#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
extern crate volatile;
extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
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
    vga_buffer::print_something();
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }