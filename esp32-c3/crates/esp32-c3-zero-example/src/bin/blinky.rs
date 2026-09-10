#![no_std]
#![no_main]
use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{RmtSmartLeds, buffer_size, color_order};
use esp_println as _;
use mischief::WrapErr;
use smart_leds::{RGB8, SmartLedsWrite, brightness, colors};
esp_bootloader_esp_idf::esp_app_desc!();
use esp_alloc as _;
use pain as _;
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
        color_order::Grb,
    > = {
        let frequency = Rate::from_mhz(80);
        let rmt = Rmt::new(peripherals.RMT, frequency)
            .map_err(|e| mischief::mischief!("{e:?}"))
            .wrap_err("Failed to initialize RMT0")?;
        RmtSmartLeds::new(
            esp_hal_smartled::WS2812_TIMING,
            rmt.channel0,
            peripherals.GPIO10,
            frequency,
        )?
    };
    let level = 10;
    let color = RGB8::new(255, 0, 0);
    loop {
        led.write(brightness([color].into_iter(), level))
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        delay.delay_millis(500);
        led.write([colors::BLACK])
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        delay.delay_millis(500);
    }
}
