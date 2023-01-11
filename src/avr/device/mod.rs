//! Simple device-abstraction providing device-specific pins and registers. Currently only supports
//! the atmega32 target, but could be extended to other avr devices.

// Do not export this module to avoid conflicts with exported `Pin`s in the device-specific `pin`
// module.
mod pin;
pub use pin::{DataDirection, Pin};

// Do not export this module to avoid conflicts with exported `Register`s in the device-specific
// `register` module.
mod register;
pub use register::{Register, RegisterBits, RegisterValue};

// TODO: Consider selecting device using feature flags.
mod atmega32;
pub use atmega32::port;
pub use atmega32::registers::*;
