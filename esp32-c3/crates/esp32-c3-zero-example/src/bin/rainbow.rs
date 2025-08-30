#![no_std]
#![no_main]
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use esp_println::println;
use smart_leds::hsv::{Hsv, hsv2rgb};
use smart_leds::{SmartLedsWrite, brightness};
use {esp_alloc as _, panic_handler as _};
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
    let level = 255;
    let mut hue: u8 = 0;
    loop {
        let color = hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 255,
        });

        led.write(brightness([color].into_iter(), level)).unwrap();
        delay.delay_millis(10);

        hue = hue.wrapping_add(1);
        if hue == 0 {
            println!("one rainbow cycle finished");
        }
    }
}
