#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;


/// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("=======");
    println!("*SCREEEK* tetanOS has panicked!");
    println!("info below...");
    println!("=======");
    println!();
    println!("{}", _info);
    loop {}
}
static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {





    println!("Hello world!");

    let answer: i32 = 7331 - 5994;
    println!("7331-5994={}", answer);

    panic!("Some panic message");


    loop {}
}
