#![no_std]

use core::fmt::Write;
extern crate alloc;

#[derive(Debug)]
pub struct Report {
    pub msg: alloc::string::String,
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
                let mut msg: alloc::string::String = alloc::string::String::new();
                write!(msg, "{:?}", e).ok();
                Err(Report { msg })
            }
        }
    }
}
