#![no_std]
#![no_main]

use rp6::*;

/// entry point for the embedded rust program
#[no_mangle]
pub extern "C" fn main() {
    RobotBase::init();
    UART::write_str("Hello world!\n");

    let init_leds: u8 = 0b001001;
    let mut running_light: u8 = init_leds;
    loop {
        // set LEDs according to the binary number `running_light`
        RobotBase::set_leds(running_light);

        // sleep for one second
        delay_ms(1000);

        // shift to the left for the 'running' effect
        running_light <<= 1;

        // reset to the initial LED pattern after the last LED was lit
        if running_light > 0b111111 {
            running_light = init_leds;
        }
    }
}
