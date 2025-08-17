#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[main]
fn main() -> ! {
    let delay = Delay::new();

    loop {
        println!("hello world");
        delay.delay_millis(1000);
    }
}
