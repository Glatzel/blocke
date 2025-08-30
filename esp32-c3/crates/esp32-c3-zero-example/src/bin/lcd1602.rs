#![no_std]
#![no_main]
use esp_alloc as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use mischief::IntoMischief;
esp_bootloader_esp_idf::esp_app_desc!();
#[main] // esp-hal handles entry
fn main() -> ! {
    app().unwrap();
    loop {
<<<<<<< HEAD
        riscv::asm::wfi();
=======
        panic!()
>>>>>>> 8cbf0c79eb41a2e39a62f4cbd13b044cbc215fbf
    }
}

fn app() -> mischief::Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let _oe = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);

    // PCF8574T adapter for a single HD44780 controller
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, delay);
    if let Err(e) = lcd.init() {
        panic!("Error initializing LCD: {}", e);
    };
    lcd.backlight(true)
        .unwrap()
        .print("Hello World!")
        .into_mischief()?;
    loop {
        riscv::asm::wfi();
    }
}
