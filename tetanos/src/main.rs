#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // buffer is loccated there
    for (i, &byte) in HELLO.iter().enumerate() { // index, byte
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // set char
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // set color to 0xb (cyan)
        }
    }
    loop {}
}
