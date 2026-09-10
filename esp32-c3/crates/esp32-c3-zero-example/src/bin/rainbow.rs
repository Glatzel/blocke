#![no_std]
#![no_main]
use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{RmtSmartLeds, buffer_size, color_order};
use esp_println::println;
use pain as _;
use smart_leds::hsv::{Hsv, hsv2rgb};
use smart_leds::{RGB8, SmartLedsWrite, brightness};
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
    let delay = Delay::new();

    let mut led: RmtSmartLeds<
        '_,
        { buffer_size::<RGB8>(1) },
        esp_hal::Blocking,
        RGB8,
        color_order::Rgb,
    > = {
        let frequency = Rate::from_mhz(80);
        let rmt = Rmt::new(peripherals.RMT, frequency).expect("Failed to initialize RMT0");
        RmtSmartLeds::new(
            esp_hal_smartled::WS2812_TIMING,
            rmt.channel0,
            peripherals.GPIO10,
            frequency,
        )?
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
