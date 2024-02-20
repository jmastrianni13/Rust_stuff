#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rOSt::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rOSt::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rOSt::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// This function is called on panic during testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rOSt::test_panic_handler(info)
}
