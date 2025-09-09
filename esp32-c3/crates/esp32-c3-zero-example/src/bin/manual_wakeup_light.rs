#![no_std]
#![no_main]

use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Pull, WakeEvent};
use esp_hal::rmt::Rmt;
use esp_hal::rtc_cntl::sleep::WakeSource;
use esp_hal::time::Rate;
use esp_hal::{main, rtc_cntl};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use esp_println::println;
use mischief::WrapErr;
use smart_leds::{RGB8, SmartLedsWrite, brightness, colors};
use {esp_alloc as _, pain as _};

esp_bootloader_esp_idf::esp_app_desc!();
macro_rules! config_other_pin {
    ($pin:expr) => {
        let config = InputConfig::default().with_pull(Pull::Down);
        let mut wakeup_pin = Input::new($pin, config);
        wakeup_pin.wakeup_enable(false, WakeEvent::LowLevel)?;
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
    let mut led = {
        let frequency = Rate::from_mhz(80);
        let rmt = Rmt::new(peripherals.RMT, frequency)
            .map_err(|e| mischief::mischief!("{e:?}"))
            .wrap_err("Failed to initialize RMT0")?;
        SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO10, smart_led_buffer!(1))
    };
    let level = 10;
    let color = RGB8::new(50, 0, 50);

    // config wakeup pin
    let config = InputConfig::default().with_pull(Pull::Up);
    let mut wakeup_pin = Input::new(peripherals.GPIO9, config);
    wakeup_pin.wakeup_enable(true, WakeEvent::LowLevel)?;
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
    let mut rtc = rtc_cntl::Rtc::new(peripherals.LPWR);
    let wakeup_source = esp_hal::rtc_cntl::sleep::GpioWakeupSource::new();
    let mut sleep_config = rtc_cntl::sleep::RtcSleepConfig::default();
    let mut trigger = rtc_cntl::sleep::WakeTriggers::default();
    trigger.set_gpio(true);
    wakeup_source.apply(&rtc, &mut trigger, &mut sleep_config);

    loop {
        println!("Wakeup.");
        delay.delay_micros(80); //for led reset
        led.write(brightness([color].into_iter(), level))
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        delay.delay_millis(5000);
        led.write([colors::BLACK])
            .map_err(|e| mischief::mischief!("{e:?}"))?;
        println!("Sleep.");
        rtc.sleep_light(&[&wakeup_source]);
    }
}
