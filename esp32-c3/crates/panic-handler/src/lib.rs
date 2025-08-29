#![no_std]
#![allow(unused_imports)]

#[cfg(debug_assertions)]
pub use esp_backtrace as _;
#[cfg(not(debug_assertions))]
pub use panic_halt as _;

#[macro_export]
macro_rules! check_result {
    ($expr:expr) => {
        // Rule for when no message is provided
        match $expr {
            Ok(val) => val,
            Err(e) => {
                panic!("\x1b[31m{}\x1b[0m", e);
            }
        }
    };
    ($expr:expr, $msg:expr) => {
        // Rule for when a message is provided
        match $expr {
            Ok(val) => val,
            Err(e) => {
                panic!("{}: {}", $msg, e);
            }
        }
    };
}
