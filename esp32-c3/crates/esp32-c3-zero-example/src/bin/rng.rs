#![no_std]
#![no_main]

use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_println::println;
use panic_handler as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(Default::default());
    let mut rng = Rng::new(peripherals.RNG);
    let delay = Delay::new();
    loop {
        let num: u32 = rng.random();
        delay.delay_millis(1000);
        println!("RNG: {num}");
    }
}
