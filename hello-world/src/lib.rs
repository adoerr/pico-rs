#![no_std]

use panic_halt as _;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        zephyr_sys::printk("Hello 🪁 from Rust 🦀!\n".as_ptr() as *const core::ffi::c_char);
    }
}
