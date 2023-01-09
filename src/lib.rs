#![no_std]
//! Rust adaptation of the RP6Lib provided with the original Robby RP6 robot.
//!
//! Example usage (see `examples/` directory for more):
//! ```no_run
#![doc = include_str!("../examples/01_led_runner.rs")]
//! ```

// reexporting delay routines for convenience
extern crate avr_delay;
/// Blocking procedure that allows to
pub use avr_delay::{delay_ms, delay_us};

mod rp6;
pub use rp6::*;

/// Registered as a panic handler (required in `#![no_std]` crates)
#[panic_handler]
pub fn panic_loop(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
