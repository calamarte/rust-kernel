#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Init...");

    rust_kernel::init();

    use x86_64::registers::control::Cr3;
    let (l4_page_table, _) = Cr3::read();

    println!("Level 4 page table at: {:?}", l4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("Finish!");

    rust_kernel::htl_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_kernel::htl_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info);
}
