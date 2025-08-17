#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use esp_println::println;
use smart_leds::{RGB8, SmartLedsWrite, brightness, colors};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut led = {
        let frequency = Rate::from_mhz(80);
        let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");
        SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO10, smart_led_buffer!(1))
    };
    let level = 10;
    let color = RGB8::new(0, 0, 255); // Follow the order of GRB to sent data and the high bit sent at first.
    println("blinky");
    loop {
        led.write(brightness([color].into_iter(), level)).unwrap();
        delay.delay_millis(500);
        led.write([colors::BLACK]).unwrap();
        delay.delay_millis(500);
    }
}
