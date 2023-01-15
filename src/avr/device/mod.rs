//! Simple device-abstraction providing device-specific pins and registers. Currently only supports
//! the atmega32 target, but could be extended to other avr devices.

// Do not export this module, as it should only be used within the device-specific `port` module.
mod pin;
pub(crate) use pin::set_pins;
pub use pin::{DataDirection, Pin};

// Do not export this module, as it should only be used within the device-specific `registers` module.
mod register;
pub use register::{Register, RegisterBits, RegisterValue};

// TODO: Consider selecting device using feature flags.
mod atmega32;
pub use atmega32::port;
pub use atmega32::registers;
