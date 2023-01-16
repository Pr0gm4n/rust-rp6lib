//! Module allowing for simple use of the robot's serial UART connection.
use super::port::{RX, TX};
use crate::{
    avr::{
        bitmasks::{RXCIE, RXEN, TXEN, UCSZ, URSEL},
        registers::{UBRRH, UBRRL, UCSRA, UCSRB, UCSRC},
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
    /// Write text to the serial connection.
    pub fn write_str(_s: &'static str) {}

    /// Initialize the serial connection on pins `rx` and `tx`.
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
}
