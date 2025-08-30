#![no_std]

/// Conditional panic handling for embedded development
///
/// On embedded targets, the panic handler determines what happens
/// when a panic occurs (like `panic!()` in Rust). This crate uses
/// different panic backends depending on whether we are in debug
/// mode or release mode:
///
/// - **Debug mode (`debug_assertions`)**: Uses [esp_backtrace](https://docs.rs/esp-backtrace)
///   to provide detailed backtraces and debug information. This is useful
///   during development and debugging on ESP32/ESP32-C3 targets because it can
///   print panic information over UART or JTAG, helping track down issues.
///
/// - **Release mode / non-debug (`not(debug_assertions)`)**: Uses [panic_halt](https://docs.rs/panic-halt)
///   which simply halts the CPU on panic. This keeps the firmware minimal and
///   deterministic in production, avoiding the overhead of printing debug
///   information.
///
/// This setup ensures that you get rich debugging info during development
/// but a small, stable firmware in production.

#[cfg(debug_assertions)]
use esp_backtrace as _; // Attach esp_backtrace to handle panics in debug mode.
#[cfg(not(debug_assertions))]
use panic_halt as _;
