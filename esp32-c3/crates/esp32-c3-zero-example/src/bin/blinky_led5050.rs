#![no_std]
#![no_main]
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use mischief as _;
use esp_alloc as _;
esp_bootloader_esp_idf::esp_app_desc!();
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut red = Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());
    let mut green = Output::new(peripherals.GPIO1, Level::Low, OutputConfig::default());
    let mut blue = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    loop {
        red.set_high();
        delay.delay_millis(1000);
        red.set_low();
        delay.delay_millis(1000);

        green.set_high();
        delay.delay_millis(1000);
        green.set_low();
        delay.delay_millis(1000);

        blue.set_high();
        delay.delay_millis(1000);
        blue.set_low();
        delay.delay_millis(1000);
    }
}
