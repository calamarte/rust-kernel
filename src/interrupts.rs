

use crate::{gdt, println, print};

use lazy_static::lazy_static;
use x86_64::{instructions::port::Port, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};
use pic8259::ChainedPics;
use spin::Mutex;


pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Interruptions
        idt[InterruptIndex::Timer as u8]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard as u8]
            .set_handler_fn(keyboard_interrupt_handler);


        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8)
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

    lazy_static!{
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
        );
    }


    let mut kboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let code: u8 = unsafe { port.read() };
    if let Ok(Some(k_event)) = kboard.add_byte(code) {
        if let Some(key) = kboard.process_keyevent(k_event) {
            match key {
                DecodedKey::Unicode(char) => print!("{char}"),
                DecodedKey::RawKey(key) => print!("{:?}", key)
            }
        }
    }



    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8)
    }
}


#[test_case]
fn test_break_exception() {
    x86_64::instructions::interrupts::int3();
}