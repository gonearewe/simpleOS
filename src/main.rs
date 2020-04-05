#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use simple_os::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!!!");
    simple_os::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
        test_main();

    println!("Everything goes fine ...");
    simple_os::hlt_loop()
}

// OS panic will print info to the terminal and trap into a loop.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_os::test_panic_handler(info)
}
