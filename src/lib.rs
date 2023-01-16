#![feature(associated_type_defaults)]
#![feature(asm_experimental_arch)]
#![feature(concat_idents)]
#![no_std]
//! Rust adaptation of the RP6Lib provided with the original Robby RP6 robot.
//!
//! Example usage (see `examples/` directory for more):
//! ```no_run
#![doc = include_str!("../examples/01_led_runner.rs")]
//! ```

// Contains simple implementations of required language items that libstd normally defines on other
// targets.
extern crate avr_std_stub;

// reexporting delay routines for convenience
extern crate avr_delay;
/// Blocking procedure that allows to
pub use avr_delay::{delay_ms, delay_us};

// Generic AVR API based on the ruduino project.
pub mod avr;
pub use avr::prelude::*;

// RP6-specific API based on the RP6Lib.
mod rp6;
pub use rp6::*;

/// Re-exports commonly-used API that can be imported at once.
pub mod prelude {
    pub use super::{delay_ms, delay_us, interrupt, port, RobotBase};
}
