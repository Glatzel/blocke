#![no_std]
#![no_main]

use esp_alloc::heap_allocator;
use esp_hal::main;
use esp_println::println;
use {esp_alloc as _, pain as _};
extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();
#[main]
fn main() -> ! {
    heap_allocator!(size:64 * 1024);
    let a = alloc::string::String::from("Hello World!");
    println!("{a}");
    loop {}
}
