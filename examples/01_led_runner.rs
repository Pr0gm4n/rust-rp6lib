#![no_std]
#![no_main]

use rp6::*;

/// entry point for the embedded rust program
#[entry]
fn main() -> ! {
    RobotBase::init();
    Serial::write("Hello world!\n");

    let init_leds: u8 = 0b001001;
    let mut running_light: u8 = init_leds;

    // main loop:
    loop {
        // set LEDs according to the binary number `running_light`
        RobotBase::set_leds(running_light);

        // sleep for 250ms (= a quarter of one second)
        delay_ms(250);

        // shift to the left for the 'running' effect
        running_light <<= 1;

        // reset to the initial LED pattern after the last LED was lit
        if running_light > 0b111111 {
            running_light = init_leds;
        }
    }
}
