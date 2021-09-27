#![no_std]

use core::panic::PanicInfo;

fn main() {}

/// Loops indefinitely on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
