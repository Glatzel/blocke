#![no_std]

use core::fmt::Write;
extern crate alloc;

#[derive(Debug)]
pub struct Report {
    pub msg: alloc::string::String,
}
impl From<alloc::string::String> for Report {
    fn from(msg: alloc::string::String) -> Self { Report { msg } }
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
pub trait WrapErr<T> {
    fn wrap_err(self, msg: &'static str) -> Result<T, Report>;
}
impl<T> WrapErr<T> for Result<T, Report> {
    /// Wraps the error with a custom message and returns a `Result`.
    fn wrap_err(self, msg: &'static str) -> Result<T, Report> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut final_msg = alloc::string::String::new();
                // Add custom message + formatted error
                write!(final_msg, "{}: {:?}", msg, e).ok();
                Err(Report::from(final_msg)) // Convert String to E
            }
        }
    }
}
