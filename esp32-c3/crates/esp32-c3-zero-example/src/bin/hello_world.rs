#![no_std]
#![no_main]

use esp_hal::delay::Delay;
use esp_hal::main;
use esp_println::println;
use mischief as _;

esp_bootloader_esp_idf::esp_app_desc!();
#[main]
fn main() -> ! {
    let delay = Delay::new();
    loop {
        println!("hello world");
        delay.delay_millis(1000);
    }
}
