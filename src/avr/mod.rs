mod pin;
pub use self::pin::{DataDirection, Pin};
mod register;
pub use self::register::{Register, RegisterBits, RegisterValue};

pub mod interrupt;
pub mod legacy;
pub mod modules;

/// CPU frequency config.
use avr_config as config;
