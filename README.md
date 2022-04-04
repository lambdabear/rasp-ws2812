# Welcome to the rust smart-leds driver based on the rpi_ws281x driver

For usage with the [smart-leds](https://github.com/smart-leds-rs/smart-leds)
crate.

Inspired on the [rust bindings](https://github.com/rpi-ws281x/rpi-ws281x-rust)
of the [rpi ws281x driver](https://github.com/jgarff/rpi_ws281x).

## Usage Warning

This project is still a work-in-progress.

For more information on how to use it please read the [original documentation](https://github.com/jgarff/rpi_ws281x#gpio-usage) and pay special attention to the
[limitations section](https://github.com/jgarff/rpi_ws281x#limitations).

## Compiling on Raspbian

- Install Rust from <https://rustup.rs/>
- Run `rustup target add arm-unknown-linux-gnueabihf`
- Install `sudo apt install libclang-dev`
