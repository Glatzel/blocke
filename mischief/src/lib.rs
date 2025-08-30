#![no_std]
use core::fmt::Write;
extern crate alloc;
use alloc::string::String;

#[cfg(all(feature = "esp32", debug_assertions))]
use esp_backtrace as _;
#[cfg(any(all(not(feature = "esp32")), not(debug_assertions)))]
use panic_halt as _;

#[derive(Debug)]
pub struct Report {
    pub msg: String,
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
                let mut msg: String = String::new();
                write!(msg, "{:?}", e).ok();
                Err(Report { msg })
            }
        }
    }
}
