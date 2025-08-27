#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_println::println;
use i2c_character_display::{
    CharacterDisplayAIP31068, CharacterDisplayDualHD44780, CharacterDisplayPCF8574T, LcdDisplayType,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);

    // PCF8574T adapter for a single HD44780 controller
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, delay);
    if let Err(e) = lcd.init() {
        panic!("Error initializing LCD: {}", e);
    };
    println!("123");
    loop {
        lcd.backlight(true)
            .unwrap()
            .home()
            .unwrap()
            .show_cursor(true)
            .unwrap();
    }
}
