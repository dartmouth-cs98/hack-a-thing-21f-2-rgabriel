#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");
    panic!("panicked at 'Some panic message', src/main.rs:28:5");

    loop {}
}

/// Loops indefinitely on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}
