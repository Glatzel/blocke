#![no_std]
#![allow(unused_imports)]

#[cfg(debug_assertions)]
pub use esp_backtrace as _;
#[cfg(not(debug_assertions))]
pub use panic_halt as _;
