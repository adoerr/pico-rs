#![no_std]
#![no_main]

use embassy_executor::Executor;
use static_cell::StaticCell;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        zephyr_sys::printk("Hello 🪁 from Rust 🦀!\n".as_ptr() as *const core::ffi::c_char);
    }

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(task()).unwrap();
    });
}

#[embassy_executor::task]
async fn task() {
    unsafe {
        zephyr_sys::printk("Hello from Embassy\n".as_ptr() as *const core::ffi::c_char);
    }
}
