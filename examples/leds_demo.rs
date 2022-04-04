use std::{thread, time::Duration};

use rasp_ws2812::{colors::*, Error, Leds, Ws2812Rpi};

fn main() -> Result<(), Error> {
    println!("Program start");

    // GPIO Pin 10 is SPI
    // Other modes and PINs are available depending on the Raspberry Pi revision
    // Additional OS configuration might be needed for any mode.
    // Check https://github.com/jgarff/rpi_ws281x for more information.
    const PIN: i32 = 18; // GPIO10 -> SPI0, GPIO18 -> PWM0, GPIO21 -> PCM
    const LED_NUM: usize = 60;
    const DELAY: Duration = Duration::from_millis(1000);

    let ws = Ws2812Rpi::new(LED_NUM as i32, PIN).map_err(|_| Error::Ws2812Error)?;
    let mut leds = Leds::new(ws, LED_NUM);

    let all_colors = [
        ALICE_BLUE,
        ANTINQUE_WHITE,
        AQUA,
        AQUAMARINE,
        AZURE,
        BEIGE,
        BISQUE,
        BLACK,
        BLANCHED_ALMOND,
        BLUE,
        BLUE_VIOLET,
        BROWN,
        BURLYWOOD,
        CADET_BLUE,
        CHARTREUSE,
        CHOCOLATE,
        CORAL,
        CORNFLOWER_BLUE,
        CORNSILK,
        CRIMSON,
        CYAN,
        DARK_BLUE,
        DARK_CYAN,
        DARK_GOLDENROD,
        DARK_GRAY,
        DARK_GREEN,
        DARK_KHAKI,
        DARK_MAGENTA,
        DARK_OLIVE_GREEN,
        DARK_ORANGE,
        DARK_ORCHID,
        DARK_RED,
        DARK_SALMON,
        DARK_SEA_GREEN,
        DARK_SLATE_BLUE,
        DARK_SLATE_GRAY,
        DARK_TURQUOISE,
        DARK_VIOLET,
        DEEP_PINK,
        DEEP_SKY_BLUE,
        DIM_GRAY,
        DODGER_BLUE,
        FIREBRICK,
        FLORAL_WHITE,
        FOREST_GREEN,
        FUCHSIA,
        GAINSBORO,
        GHOST_WHITE,
        GOLD,
        GOLDENROD,
        GRAY,
        GREEN,
        GREEN_YELLOW,
        HONEYDEW,
        HOT_PINK,
        INDIAN_RED,
        INDIGO,
        IVORY,
        KHAKI,
        LAVENDER,
        LAVENDER_BLUSH,
        LAWN_GREEN,
        LEMON_CHIFFON,
        LIGHT_BLUE,
        LIGHT_CORAL,
        LIGHT_CYAN,
        LIGHT_GOLDENROD_YELLOW,
        LIGHT_GRAY,
        LIGHT_GREEN,
        LIGHT_PINK,
        LIGHT_SALMON,
        LIGHT_SEA_GREEN,
        LIGHT_SKY_BLUE,
        LIGHT_SLATE_GRAY,
        LIGHT_STEEL_BLUE,
        LIGHT_YELLOW,
        LIME,
        LIME_GREEN,
        LINEN,
        MAGENTA,
        MAROON,
        MEDIUM_AQUAMARINE,
        MEDIUM_BLUE,
        MEDIUM_ORCHID,
        MEDIUM_PURPLE,
        MEDIUM_SEA_GREEN,
        MEDIUM_SLATE_BLUE,
        MEDIUM_SPRING_GREEN,
        MEDIUM_TURQUOISE,
        MEDIUM_VIOLET_RED,
        MIDNIGHT_BLUE,
        MINT_CREAM,
        MISTY_ROSE,
        MOCCASIN,
        NAVAJO_WHITE,
        NAVY,
        OLD_LACE,
        OLIVE,
        OLIVE_DRAB,
        ORANGE,
        ORANGE_RED,
        ORCHID,
        PALE_GOLDENROD,
        PALE_GREEN,
        PALE_TURQUOISE,
        PALE_VIOLET_RED,
        PAPAYA_WHIP,
        PEACH_PUFF,
        PERU,
        PINK,
        PLUM,
        POWDER_BLUE,
        PURPLE,
        RED,
        ROSY_BROWN,
        ROYAL_BLUE,
        SADDLE_BROWN,
        SALMON,
        SANDY_BROWN,
        SEASHELL,
        SEA_GREEN,
        SIENNA,
        SILVER,
        SKY_BLUE,
        SLATE_BLUE,
        SLATE_GRAY,
        SNOW,
        SPRING_GREEN,
        STEEL_BLUE,
        TAN,
        TEAL,
        THISTLE,
        TOMATO,
        TURQUOISE,
        VIOLET,
        WHEAT,
        WHITE,
        WHITE_SMOKE,
        YELLOW,
        YELLOW_GREEN,
    ];

    loop {
        // Breathing LED demo
        println!("Triangle Wave Breathing ...");
        for _ in 0..3 {
            leds.triangle_wave_breathing(BLUE, Duration::from_millis(10))?;
        }
        println!("Circular Wave Breathing ...");
        for _ in 0..3 {
            leds.circular_wave_breathing(GREEN, Duration::from_millis(10))?;
        }
        for _ in 0..3 {
            leds.gaussian_wave_breathing(ORANGE, Duration::from_millis(10))?;
        }

        // Rainbow LED demo
        println!("Rainbow ...");
        leds.rainbow(Duration::from_millis(10))?;

        // All colors demo
        println!("Show all colors ...");
        for color in all_colors {
            leds.on(color, 150)?;
            thread::sleep(DELAY);
        }
    }
}
