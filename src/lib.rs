use smart_leds::{
    brightness, gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};
use std::{thread, time::Duration};

pub use smart_leds::{colors, RGB8};
pub use ws281x_rpi::Ws2812Rpi;

const LOWEST_BRIGHTNESS: u8 = 60;

#[derive(Debug)]
pub enum Error {
    Ws2812Error,
}

pub struct Leds {
    ws: Ws2812Rpi,
    leds_num: usize,
}

impl Leds {
    pub fn new(ws: Ws2812Rpi, leds_num: usize) -> Self {
        Self { ws, leds_num }
    }

    pub fn on(&mut self, color: RGB8, bright: u8) -> Result<(), Error> {
        let data = vec![color; self.leds_num];
        self.ws
            .write(gamma(brightness(data.iter().cloned(), bright)))
            .map_err(|_| Error::Ws2812Error)
    }

    pub fn off(&mut self) -> Result<(), Error> {
        let data = vec![RGB8::default(); self.leds_num];
        self.ws
            .write(gamma(data.iter().cloned()))
            .map_err(|_| Error::Ws2812Error)
    }

    pub fn rainbow(&mut self, delay: Duration) -> Result<(), Error> {
        let mut data = vec![RGB8::default(); self.leds_num];

        for j in 0..256 {
            for i in 0..self.leds_num {
                // rainbow cycle using HSV, where hue goes through all colors in circle
                // value sets the brightness
                let hsv = Hsv {
                    hue: ((i * 3 + j) % 256) as u8,
                    sat: 255,
                    val: 100,
                };

                data[i] = hsv2rgb(hsv);
            }
            // before writing, apply gamma correction for nicer rainbow
            self.ws
                .write(gamma(data.iter().cloned()))
                .map_err(|_| Error::Ws2812Error)?;
            thread::sleep(delay);
        }

        Ok(())
    }

    pub fn triangle_wave_breathing(&mut self, color: RGB8, delay: Duration) -> Result<(), Error> {
        let mut data = vec![RGB8::default(); self.leds_num];

        for j in LOWEST_BRIGHTNESS as u16..255 * 2 - LOWEST_BRIGHTNESS as u16 {
            let bright = if j > 255 { 255 * 2 - j } else { j };
            for i in 0..self.leds_num {
                data[i] = color;
            }

            self.ws
                .write(gamma(brightness(data.iter().cloned(), bright as u8)))
                .map_err(|_| Error::Ws2812Error)?;
            thread::sleep(delay);
        }

        Ok(())
    }

    pub fn circular_wave_breathing(&mut self, color: RGB8, delay: Duration) -> Result<(), Error> {
        let mut data = vec![RGB8::default(); self.leds_num];
        let smoothness_pts = 500;

        for j in LOWEST_BRIGHTNESS as i32..smoothness_pts {
            let bright = 255.0 * (1.0 - (2.0 * j as f32 / smoothness_pts as f32 - 1.0).powi(2));

            for i in 0..self.leds_num {
                data[i] = color;
            }

            self.ws
                .write(gamma(brightness(data.iter().cloned(), bright as u8)))
                .map_err(|_| Error::Ws2812Error)?;
            thread::sleep(delay);
        }

        Ok(())
    }

    pub fn gaussian_wave_breathing(&mut self, color: RGB8, delay: Duration) -> Result<(), Error> {
        let mut data = vec![RGB8::default(); self.leds_num];
        let smoothness_pts = 500;
        let g = 0.14;
        let b = 0.5;

        for j in LOWEST_BRIGHTNESS as i32..smoothness_pts {
            let bright =
                255.0 * (-((j as f32 / smoothness_pts as f32 - b) / g).powi(2) / 2.0).exp();

            for i in 0..self.leds_num {
                data[i] = color;
            }

            self.ws
                .write(gamma(brightness(data.iter().cloned(), bright as u8)))
                .map_err(|_| Error::Ws2812Error)?;
            thread::sleep(delay);
        }

        Ok(())
    }
}
