#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// Урок 3. 31:00.
// https://www.youtube.com/watch?v=DA2zIAkE26Q&list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY&index=3