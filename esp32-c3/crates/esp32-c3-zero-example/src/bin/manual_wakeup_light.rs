#![no_std]
#![no_main]

use esp_alloc as _;
use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Event, Input, InputConfig, Pull};
use esp_hal::rmt::Rmt;
use esp_hal::rtc_cntl::sleep::RtcSleepConfig;
use esp_hal::time::Rate;
use esp_hal::{main, rtc_cntl};
use esp_hal_smartled::{RmtSmartLeds, buffer_size, color_order};
use esp_println as _;
use mischief::WrapErr;
use pain as _;
use smart_leds::{RGB8, SmartLedsWrite, colors};

esp_bootloader_esp_idf::esp_app_desc!();
macro_rules! config_other_pin {
    ($pin:expr) => {
        let config = InputConfig::default().with_pull(Pull::Down);
        let _other_pin = Input::new($pin, config);
    };
}
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

    //init led
    let mut led: RmtSmartLeds<
        '_,
        { buffer_size::<RGB8>(1) },
        esp_hal::Blocking,
        RGB8,
        color_order::Rgb,
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

    // config wakeup pin
    let mut wakeup_pin = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Up),
    );
    wakeup_pin.listen(Event::LowLevel);

    config_other_pin!(peripherals.GPIO0);
    config_other_pin!(peripherals.GPIO1);
    config_other_pin!(peripherals.GPIO2);
    config_other_pin!(peripherals.GPIO3);
    config_other_pin!(peripherals.GPIO4);
    config_other_pin!(peripherals.GPIO5);
    config_other_pin!(peripherals.GPIO6);
    config_other_pin!(peripherals.GPIO7);
    config_other_pin!(peripherals.GPIO8);

    //init rtc
    let mut rtc = rtc_cntl::sleep::LowPower::new(peripherals.LPWR);
    let sleep_config = RtcSleepConfig::default();

    loop {
        clerk::info!("Wakeup.");
        delay.delay_micros(80); //for led reset
        led.write([colors::GREEN])
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        delay.delay_millis(5000);
        led.write([colors::RED])
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        clerk::info!("Sleep.");
        rtc.sleep_light(sleep_config);
        while wakeup_pin.is_low() {
            core::hint::spin_loop();
        }
    }
}
