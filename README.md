# 🦀 pico-rs 🦀

<br>

<p align="center">
    <img src="assets/RP2040.png" width="300" height="200">
</p>

<br>

Writing Rust applications for the `RP2040` microcontroller using the
[Raspberry Pi Pico W](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf) board.

<br>

## Crates

- [async](./async) - Embedded Async Rust using [embassy](https://github.com/embassy-rs/embassy)
- [classic](./classic) - Classic embedded Rust using [rp-hal](https://github.com/rp-rs/rp-hal)
- [board](./board) - Board support crate for [Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/)

<br>

## Hardware

The only supported architecture is [Arm Cortex-M0+](https://developer.arm.com/Processors/Cortex-M0-Plus)
using a [(Armv6-M)](https://developer.arm.com/documentation/ddi0419/latest/) ISA. The only supported board is the
[Raspberry Pi Pico W](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf). For quality of life reasons,
make sure you get the [Debug Probe](https://www.raspberrypi.com/products/debug-probe/) as well.


<p align="center">
    <img src="assets/pinout.svg" width="800" height="600">
</p>

## Console

`screen -L /dev/tty.usbmodemxxx 115200 -L`

Quit with `Ctrl-A + Ctrl-\`


<br>

<p align="center">
<b>🚧 pico-rs is under construction - a hardhat 👷 is recommended beyond this point 🚧</b>
</p>
