//! Simple device-abstraction providing device-specific pins and registers. Currently only supports
//! the atmega32 target, but could be extended to other avr devices.
mod pin;
pub use pin::{DataDirection, Pin};

mod register;
pub use register::{Register, RegisterBits, RegisterValue};

// TODO: Consider selecting device using feature flags.
mod atmega32;
pub use atmega32::pins;
pub use atmega32::registers;
