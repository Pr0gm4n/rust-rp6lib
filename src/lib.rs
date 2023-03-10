#![feature(asm_experimental_arch)]
#![feature(associated_type_defaults)]
#![feature(cell_update)]
#![no_std]
//! Rust adaptation of the RP6Lib provided with the original Robby RP6 robot.
//!
//! Example usage (see [`examples/`](https://github.com/Pr0gm4n/rust-rp6lib/tree/main/examples) directory for more):
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

// Proc-macros to define interrupts and the entry point
pub use avr_macros::*;

// RP6-specific API based on the RP6Lib.
pub mod robot_base;
pub use robot_base::{port, RobotBase};
pub mod uart;
pub use uart::*;

/// Re-exports commonly-used API that can be imported at once.
pub mod prelude {
    pub use super::{delay_ms, delay_us, interrupt, port, RobotBase};
}
