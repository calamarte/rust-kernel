
use core::u8;

use linked_list_allocator::LockedHeap;
use x86_64::{structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB}, VirtAddr};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAllocator<Size4KiB>
) ->  Result<(), MapToError<Size4KiB>> {
    let p_range = {
        let start = VirtAddr::new(HEAP_START as u64);
        let end = start + HEAP_SIZE as u64 - 1;
        let start_page = Page::containing_address(start);
        let end_page = Page::containing_address(end);
        Page::range_inclusive(start_page, end_page)
    };

    for page in p_range {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_alloc)?.flush()
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    Ok(())
}

