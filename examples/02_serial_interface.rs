#![no_std]
#![no_main]

use rp6::*;

/// entry point for the embedded rust program
#[no_mangle]
pub extern "C" fn main() {
    RobotBase::init();

    RobotBase::set_leds(0b111111); // turn all LEDs on
    delay_ms(500); // delay 500ms
    RobotBase::set_leds(0b000000); // turn all LEDs off

    // write a text message to the UART:
    Serial::write("\nJust a simple counter program\n\n");

    // define a counting variable:
    let mut counter: u16 = 0;

    // main loop:
    loop {
        println!(
            "Counter:",
            //counter => bin,
            //"(BIN) |",
            //counter => oct,
            //"(OCT) |",
            counter => dec,
            "(DEC) |",
            counter => hex,
            "(HEX)"
        );

        // increment counter
        counter += 1;

        // delay 200ms = 0.2s
        delay_ms(200);
    }
}
