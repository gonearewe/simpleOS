#![no_std]
#![no_main]

use core::panic::PanicInfo;

use simple_os::{exit_qemu, QemuExitCode, serial_println};
use simple_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail ... ");
    assert_eq!(0, 1);
}