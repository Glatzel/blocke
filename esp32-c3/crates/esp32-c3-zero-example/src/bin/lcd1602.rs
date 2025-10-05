#![no_std]
#![no_main]

use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use mischief::WrapErr;
use {esp_println as _, pain as _};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    heap_allocator!(size:64 * 1024);
    app().unwrap();
    loop {
        riscv::asm::wfi();
    }
}

fn app() -> mischief::Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let _oe = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    let i2c = I2c::new(peripherals.I2C0, Config::default())?
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);

    // PCF8574T adapter for a single HD44780 controller
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, delay);
    lcd.init()
        .map_err(|e| mischief::mischief!("{e}"))
        .wrap_err("Error initializing LCD")?;
    lcd.backlight(true)
        .map_err(|e| mischief::mischief!("{e}"))
        .wrap_err("Error enable backlight.")?
        .print("Hello World!")
        .map_err(|e| mischief::mischief!("{e}"))
        .wrap_err("Error show text.")?;
    loop {
        riscv::asm::wfi();
    }
}
