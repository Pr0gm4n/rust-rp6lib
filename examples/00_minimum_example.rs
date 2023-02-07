#![no_std]
#![no_main]

use rp6::*;

/// entry point for the embedded rust program
#[entry]
fn main() -> ! {
    RobotBase::init();
    loop {}
}
