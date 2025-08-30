#![no_std]
use core::fmt::Write;
use core::usize;

#[cfg(all(feature = "esp32", debug_assertions))]
use esp_backtrace as _;
use heapless::string::String;
#[cfg(any(all(not(feature = "esp32")), not(debug_assertions)))]
use panic_halt as _;

const REPORT_SIZE: usize = 255;
#[derive(Debug)]
pub struct Report {
    pub msg: String<REPORT_SIZE>,
}

pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T>;
}
pub type Result<T, E = Report> = core::result::Result<T, E>;

impl<T, E: core::fmt::Debug> IntoMischief<T> for core::result::Result<T, E> {
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut msg: String<REPORT_SIZE> = String::new();
                write!(msg, "{:?}", e).ok();
                Err(Report { msg })
            }
        }
    }
}
