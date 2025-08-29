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
                // ANSI escape code for red text: \x1b[31m
                // \x1b[0m resets the color
                println!("\x1b[31mAn error occurred.\x1b[0m");
                println!("\x1b[31m{}\x1b[0m", e);
                loop {}
            }
        }
    };
    ($expr:expr, $msg:expr) => {
        // Rule for when a message is provided
        match $expr {
            Ok(val) => val,
            Err(e) => {
                // ANSI escape code for red text: \x1b[31m
                // \x1b[0m resets the color
                println!("\x1b[31m{}\x1b[0m", $msg);
                println!("\x1b[31m{}\x1b[0m", e);
                loop {}
            }
        }
    };
}
