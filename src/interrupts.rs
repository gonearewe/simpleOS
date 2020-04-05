use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;

use crate::gdt::DOUBLE_FAULT_IST_INDEX;
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: Breakpoint {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _err_code: u64,
) {
    panic!("EXCEPTION: Double Fault {:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    use crate::{serial_print, serial_println};
    serial_print!("test breakpoint exception ... ");
    x86_64::instructions::interrupts::int3(); // invoke a breakpoint exception
    serial_println!("[OK]")
}