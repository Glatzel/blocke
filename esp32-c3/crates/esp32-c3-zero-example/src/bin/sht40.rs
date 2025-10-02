#![no_std]
#![no_main]

use dux_sht4x::{Precision, Sht4x};
use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_println::println;
use {esp_alloc as _, pain as _};
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
    let mut delay = Delay::new();

    //init i2c
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .map_err(|e| mischief::mischief!("{e}"))?
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);

    //init SHT40
    let mut sht40 = Sht4x::new(i2c);

    // variables in loop
    let mut temperature: f32;
    let mut humidity: f32;

    loop {
        if let Ok(measurement) = sht40.measure(Precision::Low, &mut delay) {
            temperature = measurement.temperature_celsius().to_num();
            humidity = measurement.humidity_percent().to_num();
            println!("Temp: {:.2}% ", temperature);
            println!("Hum:{:.2} %", humidity);
        }
        delay.delay_millis(3000);
    }
}
