
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec::Vec};
use rust_kernel::allocator::HEAP_SIZE;


entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use rust_kernel::allocator;
    use rust_kernel::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    rust_kernel::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_alloc = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_alloc)
        .expect("heap initialization failed");

    test_main();

    loop {}
}

#[test_case]
fn simple_alloc() {
    let h_value_1 = Box::new(1);
    let h_value_2 = Box::new(2);

    assert_eq!(*h_value_1, 1);
    assert_eq!(*h_value_2, 2);
}

#[test_case]
fn vec_test() {
    let n = 1000;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info);
}
