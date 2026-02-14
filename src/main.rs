#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(unos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use unos::{println, println_color, vga_buffer::Color};
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::PageTable;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use unos::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println_color!(Color::Cyan, "Welcome to UnOS!\n");

    unos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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