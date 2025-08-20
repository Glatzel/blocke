#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::channel::config::PinConfig;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{Ledc, channel, timer};
use esp_hal::main;
use esp_hal::time::Rate;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(esp_hal::ledc::LSGlobalClkSource::APBClk);

    let red = Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());
    let green = Output::new(peripherals.GPIO1, Level::Low, OutputConfig::default());
    let blue = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    let mut lstimer0 = ledc.timer::<esp_hal::ledc::LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(1),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, red);
    let mut channel1 = ledc.channel(channel::Number::Channel1, green);
    let mut channel2 = ledc.channel(channel::Number::Channel2, blue);

    let mut pos = 0u8;
    loop {
        pos = (pos + 1) % 200;

        let level = if pos % 200 < 100 {
            pos % 200
        } else {
            200 - pos
        };
        channel0
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: level,
                pin_config: PinConfig::OpenDrain,
            })
            .unwrap(); // channel0
        channel1
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: level,
                pin_config: PinConfig::OpenDrain,
            })
            .unwrap(); // channel0
        channel2
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: level,
                pin_config: PinConfig::OpenDrain,
            })
            .unwrap();
        delay.delay_millis(100);
        println!("level: {level}");
    }
}
