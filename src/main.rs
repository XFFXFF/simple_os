#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use simple_os::println;
use bootloader::{BootInfo, entry_point};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    simple_os::hlt_loop();
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    simple_os::init();

    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    simple_os::hlt_loop();
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_os::test_panic_handler(info)
}
