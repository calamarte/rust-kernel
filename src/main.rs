#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use alloc::boxed::Box;
use rust_kernel::{allocator, memory};

mod serial;
mod vga_buffer;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use rust_kernel::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Init...");

    rust_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_alloc)
        .expect("Heap initialization failed!");

    let x = Box::new(22);
    println!("heap value {:p}", x);

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
