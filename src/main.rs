#![no_std]
#![no_main]
mod vga_buffer;
use crate::vga_buffer::Color;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println_color!(Color::Red, "Hello World{}", "!");

    loop {}
}
