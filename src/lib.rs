#![feature(associated_type_defaults)]
#![feature(asm_experimental_arch)]
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
mod avr;
pub use avr::*;

// RP6-specific API based on the RP6Lib.
mod rp6;
pub use rp6::*;

// helper to import the most common symbols at once
pub mod prelude;
