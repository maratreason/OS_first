#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", 1);
    panic!("Some panic message");

    loop {}
}

// Урок 3. 46:00.
// https://www.youtube.com/watch?v=DA2zIAkE26Q&list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY&index=3