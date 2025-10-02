#![no_std]
mod error;
mod primitives;
mod responses;

pub use primitives::*;
#[cfg(not(feature = "async"))]
mod sht4x;
#[cfg(not(feature = "async"))]
pub use sht4x::*;
#[cfg(feature = "async")]
mod sht4x_async;

#[cfg(feature = "async")]
pub use sht4x_async::*;
