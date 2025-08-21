#![no_std]
#![no_main]

use core::f32;

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::ledc::channel::config::PinConfig;
use esp_hal::ledc::channel::{ChannelHW, ChannelIFace};
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{Ledc, channel, timer};
use esp_hal::main;
use esp_hal::time::Rate;
use esp_println::println;
use micromath::F32Ext;
esp_bootloader_esp_idf::esp_app_desc!();

const PERIOD: usize = 2000; //milisecond
const POS_COUNT: usize = 200;
const DELAY: u32 = (PERIOD / POS_COUNT) as u32; //milisecond

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
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(5),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, red);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            pin_config: PinConfig::OpenDrain,
        })
        .unwrap();
    let mut channel1 = ledc.channel(channel::Number::Channel1, green);
    channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            pin_config: PinConfig::OpenDrain,
        })
        .unwrap();
    let mut channel2 = ledc.channel(channel::Number::Channel2, blue);
    channel2
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            pin_config: PinConfig::OpenDrain,
        })
        .unwrap();

    let mut Levels = [0; POS_COUNT];
    for i in 0..POS_COUNT {
        let phase = (i as f32) / (POS_COUNT as f32) * 2.0 * core::f32::consts::PI
            - core::f32::consts::FRAC_PI_2;
        Levels[i] = ((phase.sin() + 1.0) / 2.0 * ((1 << 14) - 1) as f32) as u16;
    }
    loop {
        for &level in Levels.iter() {
            channel0.set_duty_hw(level as u32);
            channel1.set_duty_hw(level as u32);
            channel2.set_duty_hw(level as u32);
            println!("level: {}", level as f32 / ((1 << 14) - 1) as f32);
            delay.delay_millis(DELAY);
        }
    }
}
