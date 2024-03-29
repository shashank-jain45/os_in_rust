#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_in_rust::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_println(tests: &[&dyn Fn()]) {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}