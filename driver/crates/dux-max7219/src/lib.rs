mod error;
mod max7219;
mod register;
#[cfg(feature = "segment")]
pub mod segment;
pub use max7219::*;
#[cfg(feature = "matrix")]
mod matrix;
#[cfg(feature = "segment")]
use segment::*;
