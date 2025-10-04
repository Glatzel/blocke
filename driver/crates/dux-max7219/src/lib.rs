#![no_std]
mod error;
mod max7219;
mod primitives;
#[cfg(feature = "segment")]
pub mod segment;
pub use max7219::*;
#[cfg(feature = "segment")]
pub use segment::*;
#[cfg(feature = "matrix")]
mod matrix;
