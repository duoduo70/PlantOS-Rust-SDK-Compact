#![no_std]
#![no_main]

use core::{ffi::c_int, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn printf(format: *const u8, ...) -> c_int;
  }

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe { printf("hello, rust!".as_ptr()) };
    0
}