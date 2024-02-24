#![no_std]
#![no_main]

extern crate libc;

use core::panic::PanicInfo;

use libc::printf;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe { printf("hello, rust!".as_ptr()) };
    0
}