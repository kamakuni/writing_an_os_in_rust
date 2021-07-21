use x86_64::structures::idt::{InterruptDescriptorTable, IntertuptStackFrame};
use crate::println;

static mut IDT = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.load();    
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: IntertuptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}