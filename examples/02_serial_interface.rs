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
        Serial::write("Counter: ");
        Serial::write_bin(counter);
        Serial::write(" (BIN) | ");
        Serial::write_oct(counter);
        Serial::write(" (OCT) | ");
        Serial::write(counter);
        Serial::write(" (DEC) | ");
        Serial::write_hex(counter);
        Serial::write(" (HEX)");

        Serial::new_line();

        // increment counter
        counter += 1;

        // delay 200ms = 0.2s
        delay_ms(200);
    }
}
