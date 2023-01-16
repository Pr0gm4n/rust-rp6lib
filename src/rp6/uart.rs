//! UART = "Universal Aynchronous Receiver Transceiver"
//!
//! This module contains data transfer functions that allow easy access to the robot's serial UART
//! connection. Receiving messages is asynchronous (using interrupts) and uses ringbuffers for
//! intermediate storage of the received messages.
use super::port::{RX, TX};
use crate::{
    avr::{
        bitmasks::{RXCIE, RXEN, TXEN, UCSZ, UDRE, URSEL},
        registers::{UBRRH, UBRRL, UCSRA, UCSRB, UCSRC, UDR},
    },
    Pin, Register,
};

use avr_config::CPU_FREQUENCY_HZ;

/// Define constants for RP6 baudrates.
pub const BAUD_LOW: u32 = 38400; // Low speed: 38.400 Baud
pub const UBRR_BAUD_LOW: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_LOW)) - 1;
pub const BAUD_HIGH: u32 = 500000; // High speed: 500.000 Baud
pub const UBRR_BAUD_HIGH: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_HIGH)) - 1;

/// Struct managing all access to the robot's serial port connection
pub struct Serial;

impl Serial {
    /// Initialize the serial connection on pins `RX` and `TX`.
    pub fn init() {
        RX::set_input();
        TX::set_low();
        TX::set_output();
        // UART:
        UBRRH::write((UBRR_BAUD_LOW >> 8) as u8); // Setup UART: Baudrate is Low Speed
        UBRRL::write(UBRR_BAUD_LOW as u8);
        UCSRA::write(0x00);
        UCSRC::write(URSEL | UCSZ);
        UCSRB::write(TXEN | RXEN | RXCIE);
    }

    /// Writes a single character to the UART connection.
    ///
    /// Example:
    /// ```rust
    ///	Serial::write_char('R');
    ///	Serial::write_char('P');
    ///	Serial::write_char('6');
    ///	Serial::write_char('\n'); // '\n' is a special code for the "new line" character!
    ///	Serial::write_char('0'); // the ASCII character '0'
    ///	Serial::write_char(48); // 48 is numeric ASCII code for '0'
    ///	Serial::write_char(49); // '1'
    ///	Serial::write_char(50); // '2'
    ///	Serial::write_char(51); // '3'
    ///	```
    ///
    /// This example would send:
    /// ```
    /// RP6
    /// 00123
    /// ```
    /// via the robot's serial connection.
    pub fn write_char(c: char) {
        UCSRA::wait_until_mask_set_raw(UDRE);
        UDR::write(c as u8);
    }

    /// Write text to the serial connection.
    pub fn write_str(s: &str) {
        for c in s.chars() {
            Self::write_char(c);
        }
    }

    /// Write a `'\n'` (newline character) to the serial connection.
    pub fn new_line() {
        Self::write_char('\n');
    }
}
