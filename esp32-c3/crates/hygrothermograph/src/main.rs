#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;

use dux_sht4x::{Precision, Sht4x};
use embedded_hal_bus::i2c as i2c_bus;
use esp_alloc as _;
use esp_alloc::heap_allocator;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull, WakeEvent};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::rtc_cntl::sleep::WakeSource;
use esp_hal::{main, rtc_cntl};
use heapless::String;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use pain as _;
esp_bootloader_esp_idf::esp_app_desc!();
use esp_println as _;
macro_rules! config_dangling_pin {
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
    let mut delay = Delay::new();

    // enable TXS0108
    let _oe = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    //init i2c
    let i2c0 = I2c::new(peripherals.I2C0, Config::default())
        .map_err(|e| mischief::mischief!("{e}"))?
        .with_sda(peripherals.GPIO0)
        .with_scl(peripherals.GPIO1);
    let i2c_ref_cell = RefCell::new(i2c0);

    // config wakeup pin
    let config = InputConfig::default().with_pull(Pull::Up);
    let mut wakeup_pin = Input::new(peripherals.GPIO9, config);
    wakeup_pin.wakeup_enable(true, WakeEvent::LowLevel)?;
    config_dangling_pin!(peripherals.GPIO3);
    config_dangling_pin!(peripherals.GPIO4);
    config_dangling_pin!(peripherals.GPIO5);
    config_dangling_pin!(peripherals.GPIO6);
    config_dangling_pin!(peripherals.GPIO7);
    config_dangling_pin!(peripherals.GPIO8);

    //init rtc
    let mut rtc = rtc_cntl::Rtc::new(peripherals.LPWR);
    let wakeup_source = esp_hal::rtc_cntl::sleep::GpioWakeupSource::new();
    let mut sleep_config = rtc_cntl::sleep::RtcSleepConfig::default();
    let mut trigger = rtc_cntl::sleep::WakeTriggers::default();
    trigger.set_gpio(true);
    wakeup_source.apply(&rtc, &mut trigger, &mut sleep_config);

    // init lcd1602
    let mut lcd = CharacterDisplayPCF8574T::new(
        i2c_bus::RefCellDevice::new(&i2c_ref_cell),
        LcdDisplayType::Lcd16x2,
        delay,
    );
    lcd.init().ok();

    //init SHT40
    let mut sht40 = Sht4x::new(i2c_bus::RefCellDevice::new(&i2c_ref_cell));
    let _serial = sht40.serial_number(&mut delay);

    lcd.print("Hello !")
        .map_err(|_| mischief::mischief!("Failed to print initial message"))?;
    delay.delay_millis(2000);
    lcd.clear()
        .map_err(|_| mischief::mischief!("Failed to clear screen"))?
        .home()
        .map_err(|_| mischief::mischief!("Failed to set the cursor to the home position"))?;

    // variables in loop
    let mut temperature: f32;
    let mut humidity: f32;
    let mut buf_temp: String<16> = String::new();
    let mut buf_humid: String<16> = String::new();

    loop {
        delay.delay_millis(500);
        lcd.backlight(true)
            .map_err(|_| mischief::mischief!("Failed to initialize LCD"))?;

        if let Ok(measurement) = sht40.measure(Precision::Low, &mut delay) {
            temperature = measurement.temperature_celsius().to_num();
            humidity = measurement.humidity_percent().to_num();

            buf_temp.clear();
            buf_humid.clear();
            write!(buf_temp, "Temp: {:.2}C", temperature)
                .map_err(|e| mischief::mischief!("{e}"))?;
            write!(buf_humid, "Hum:  {:.2}%", humidity).map_err(|e| mischief::mischief!("{e}"))?;
            lcd.write_str(&buf_temp)
                .map_err(|e| mischief::mischief!("{e}"))?;
            lcd.set_cursor(0, 1)
                .map_err(|_| mischief::mischief!("Failed to set cursor to line 2"))?;
            lcd.write_str(&buf_humid)
                .map_err(|e| mischief::mischief!("{e}"))?;
        }

        // wait 5s and sleep
        delay.delay_millis(5000);
        lcd.clear()
            .map_err(|_| mischief::mischief!("Failed to clear screen"))?
            .home()
            .map_err(|_| mischief::mischief!("Failed to set the cursor to the home position"))?
            .print("Fall Sleep......")
            .map_err(|_| mischief::mischief!("Failed to show text"))?;
        delay.delay_millis(500);
        lcd.backlight(false)
            .map_err(|_| mischief::mischief!("Failed to disable backlight"))?
            .clear()
            .map_err(|_| mischief::mischief!("Failed to clear screen"))?
            .home()
            .map_err(|_| mischief::mischief!("Failed to set the cursor to the home position"))?;
        rtc.sleep_light(&[&wakeup_source]);
    }
}
