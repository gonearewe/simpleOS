#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use simple_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    simple_os::init();

    println!("Hello World {}", "!!!");
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
        test_main();

    println!("Everything goes fine ...");
    loop {}
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
