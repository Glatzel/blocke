#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;

use embedded_hal_bus::i2c as i2c_bus;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use heapless::String;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use mischief::{self as _, IntoMischief};
use sht4x::{Precision, Sht4x};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    app().unwrap();
    loop {}
}
fn app() -> mischief::Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // enable TXS0108
    let _oe = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    //init i2c
    let i2c0 = I2c::new(peripherals.I2C0, Config::default())
        .into_mischief()?
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);
    let i2c_ref_cell = RefCell::new(i2c0);

    // init lcd1602
    let mut lcd = CharacterDisplayPCF8574T::new(
        i2c_bus::RefCellDevice::new(&i2c_ref_cell),
        LcdDisplayType::Lcd16x2,
        delay,
    );
    lcd.init().ok();

    //init SHT40
    let mut sht40 = Sht4x::new(i2c_bus::RefCellDevice::new(&i2c_ref_cell));
    let _serial = sht40.serial_number(&mut delay);

    lcd.backlight(true)
        .map_err(|_| "Error: Failed to initialize LCD")
        .into_mischief()?;
    lcd.print("Hello !")
        .map_err(|_| "Failed to print initial message")
        .into_mischief()?;
    delay.delay_millis(2000);

    loop {
        lcd.clear()
            .map_err(|_| "")
            .into_mischief()?
            .home()
            .map_err(|_| "")
            .into_mischief()?;
        let measurement = sht40.measure(Precision::Low, &mut delay);
        if let Ok(measurement) = measurement {
            let temperature: f32 = measurement.temperature_celsius().to_num();
            let humidity: f32 = measurement.humidity_percent().to_num();

            let mut buf_temp: String<16> = String::new();
            write!(buf_temp, "Temp: {:.2}C", temperature).into_mischief()?;
            let mut buf_humid: String<16> = String::new();
            write!(buf_humid, "Hum:  {:.2}%", humidity).into_mischief()?;
            lcd.write_str(&buf_temp).into_mischief()?;

            lcd.set_cursor(0, 1).map_err(|_| "").into_mischief()?;
            lcd.write_str(&buf_humid).into_mischief()?;
        }
        delay.delay_millis(5000);
    }
}
