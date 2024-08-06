#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_kernel::{
    allocator, memory,
    task::{executor::Executor, keyboard, Task},
};

mod serial;
mod vga_buffer;

async fn ansync_number() -> u32 {
    27
}

async fn example_task() {
    let number = ansync_number().await;
    println!("async number: {number}");
}

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use rust_kernel::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Init...");

    rust_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_alloc).expect("Heap initialization failed!");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
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
