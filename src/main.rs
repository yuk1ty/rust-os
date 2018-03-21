#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

static HELLO: &[u8] = b"Hello, World!";
static PANIC: &[u8] = b"Panic has been occurred!";

// linux
#[no_mangle]
pub extern "C" fn _start() -> ! {
//    use core::fmt::Write;
//    vga_buffer::WRITER.lock().write_str("Hello, again").unwrap();
//    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello, World{}", "!");

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}
