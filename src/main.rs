#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(unos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use unos::{println, println_color, vga_buffer::Color};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println_color!(Color::Cyan, "Welcome to UnOS!");

    unos::init();

    #[cfg(test)]
    test_main();

    println_color!(Color::LightGreen, "\nAll done!");
    unos::hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    unos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unos::test_panic_handler(info)
}