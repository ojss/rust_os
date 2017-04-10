#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
extern crate volatile;
extern crate rlibc;
extern crate spin;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    use core::fmt::Write;
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
    vga_buffer::WRITER.lock().clear_row(0);
    vga_buffer::WRITER.lock().write_str("Hello again World");
    write!(vga_buffer::WRITER.lock(), ", the secret to the universe is: {}", 42);
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }