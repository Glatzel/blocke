#![no_std]
#![no_main]

use defmt::println;
use esp_hal::delay::Delay;
use esp_hal::main;
use pain as _;
esp_bootloader_esp_idf::esp_app_desc!();

use esp_println as _;
#[main]
fn main() -> ! {
    let delay = Delay::new();
    loop {
        println!("hello world");
        clerk::info!("hello world");
        delay.delay_millis(1000);
    }
}
