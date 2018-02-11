#![feature(lang_items)]
#![no_std]
#![no_main]

static HELLO: &[u8] = b"Hello, World!";

// mac
#[no_mangle]
pub extern "C" fn main() -> ! {
    let vga_buffer = 0xb8000 as *const u8 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// linux
#[no_mangle]
pub extern "C" fn _start() -> ! {
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
