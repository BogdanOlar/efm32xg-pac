#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(all(feature = "efm32pg1b", feature = "efm32gg11"))]
compile_error!("You must not enable both the `efm32pg1b` and `efm32gg11` Cargo features.");
#[cfg(not(any(feature = "efm32pg1b", feature = "efm32gg11")))]
compile_error!("You must enable either the `efm32pg1b` or `efm32gg11` Cargo feature.");

pub mod common;

#[cfg_attr(feature = "efm32pg1b", path = "./efm32pg1b/mod.rs")]
#[cfg_attr(feature = "efm32gg11", path = "./efm32gg11/mod.rs")]
mod inner;
pub use inner::*;
