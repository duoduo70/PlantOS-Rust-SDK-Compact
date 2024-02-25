#![no_std]
#![no_main]

extern crate libc;
extern crate nalloc;

use core::panic::PanicInfo;

use libc::printf;

struct MyAllocator;

unsafe impl core::alloc::GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        libc::free(ptr as *mut libc::c_void);
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe { printf("hello, rust!".as_ptr()) };
    0
}