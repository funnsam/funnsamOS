use x86_64::{set_general_handler, structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode}};
use lazy_static::lazy_static;

use crate::*;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = unsafe {
        let mut idt = InterruptDescriptorTable::new();

        set_general_handler!(&mut idt, debug);

        idt.page_fault.set_handler_fn(page_fault);
        idt.double_fault.set_handler_fn(double_fault)
                        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        idt.general_protection_fault.set_handler_fn(general_fault);
        idt.breakpoint.set_handler_fn(breakpoint);

        idt
    };
}

pub fn init() {
    IDT.load();
}

fn debug(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("Stub {index} {stack_frame:#?}\n{error_code:?}");
    crate::stacktrace::print_stacktrace();
}

extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    println!("#BP {stack_frame:#?}");
    crate::stacktrace::print_stacktrace();
}

extern "x86-interrupt" fn general_fault(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("#GP {stack_frame:#?}\n{error_code}");
    loop {}
}

extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    println!("#PF {stack_frame:#?}\n{error_code:#?}");
    loop {}
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    println!("#DF {stack_frame:#?}\n{error_code}");
    loop {}
}
