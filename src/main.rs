#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    loop {}
}

/// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
