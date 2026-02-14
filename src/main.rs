#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(unos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use unos::{println, println_color, vga_buffer::Color};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println_color!(Color::Cyan, "Welcome to UnOS!\n");

    unos::init();

    #[cfg(test)]
    test_main();

    println_color!(Color::LightGreen, "All done!");
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