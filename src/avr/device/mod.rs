//! Simple device-abstraction providing device-specific pins and registers. Currently only supports
//! the atmega32 target, but could be extended to other avr devices.

// Do not export this module, as it should only be used within the device-specific `port` module.
mod pin;
pub(crate) use pin::set_pins;
pub use pin::{DataDirection, Pin};

// Do not export this module, as it should only be used within the device-specific `registers` module.
pub mod register;
pub use register::{Register, RegisterBits, RegisterValue};

// TODO: Consider selecting device using feature flags.
pub mod atmega32;
pub use atmega32 as current_device;

/// Re-export the device's modules for bitmasks, port and register definitions.
pub use current_device::{bitmasks, port, registers};
