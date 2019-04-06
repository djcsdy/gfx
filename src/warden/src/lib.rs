//! Data-driven reference test framework for warding
//! against breaking changes.

extern crate gfx_hal as hal;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate failure;
#[cfg(feature = "shaderc")]
extern crate shaderc;

pub mod gpu;
pub mod raw;
