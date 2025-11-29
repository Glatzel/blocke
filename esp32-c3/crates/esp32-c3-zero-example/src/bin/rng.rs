#![no_std]
#![no_main]

use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_println::println;
use pain as _;
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let rng = Rng::new();
    let delay = Delay::new();
    loop {
        let num: u32 = rng.random();
        delay.delay_millis(1000);
        println!("RNG: {num}");
    }
}
