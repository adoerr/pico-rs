#![no_std]
#![no_main]
#![allow(unused_imports)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::uart;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Init HAL and peripherals
    let p = embassy_rp::init(Default::default());

    let cfg = uart::Config::default();
    let mut uart =
        uart::Uart::new_with_rtscts_blocking(p.UART0, p.PIN_0, p.PIN_1, p.PIN_3, p.PIN_2, cfg);
    uart.blocking_write("Hello UART!\r\n".as_bytes()).unwrap();

    loop {
        uart.blocking_write("Ping\r\n".as_bytes()).unwrap();
        cortex_m::asm::delay(50_000_000);
    }
}
