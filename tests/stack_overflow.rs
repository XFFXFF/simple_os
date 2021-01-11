#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use simple_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    simple_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations.
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_os::test_panic_handler(info)
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(simple_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_test_idt() {
    TEST_IDT.load();
}
