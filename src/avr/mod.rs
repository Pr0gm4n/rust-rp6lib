//! This module combines some simple device-abstraction for AVR microcontrollers. Currently only
//! supports the atmega32 target, but could be extended to other avr devices.

mod device;
pub use device::*;

pub mod interrupt;

#[allow(unused)]
pub mod legacy;
#[allow(unused)]
pub mod modules;

/// CPU frequency config.
pub use avr_config as config;

/// Convenience module grouping functions that might be worth re-exporting to other crates.
pub mod prelude {
    pub(crate) use super::device::set_pins;
    pub use super::{
        device::{DataDirection, Pin, Register, RegisterBits, RegisterValue},
        interrupt,
    };
}
