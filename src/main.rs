#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 13).unwrap();

    loop {}
}

/// Loops indefinitely on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
