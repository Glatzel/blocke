#![no_std]

#[cfg(debug_assertions)]
use esp_backtrace as _; // Attach esp_backtrace to handle panics in debug mode.
#[cfg(not(debug_assertions))]
use panic_halt as _;
