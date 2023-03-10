//! UART = "Universal Aynchronous Receiver Transceiver"
//!
//! This module contains data transfer functions that allow easy access to the robot's serial UART
//! connection. Receiving messages is asynchronous (using interrupts) and uses ringbuffers for
//! intermediate storage of the received messages.
use super::port::{RX, TX};
use crate::{
    avr::{
        bitmasks::{RXC, RXCIE, RXEN, TXCIE, TXEN, UCSZ, UDRE, URSEL},
        registers::{UBRRH, UBRRL, UCSRA, UCSRB, UCSRC, UDR},
    },
    Pin, Register,
};

/// Module that implements `Serial::write` and formatting behavior for types.
mod serial_writable;
pub use serial_writable::*;

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
        Self::set_baudrate_low();
        UCSRA::write(0x00);
        UCSRC::write(URSEL | UCSZ);
        UCSRB::write(TXEN | RXEN | RXCIE);
    }

    /// Enable the USART_RXC interrupt
    #[allow(non_snake_case)]
    pub fn enable_USART_RXC_interrupt() {
        UCSRB::set_mask_raw(RXCIE);
    }

    /// Enable the USART_TXC interrupt
    #[allow(non_snake_case)]
    pub fn enable_USART_TXC_interrupt() {
        UCSRB::set_mask_raw(TXCIE);
    }

    /// Disable the USART_RXC interrupt
    #[allow(non_snake_case)]
    pub fn disable_USART_RXC_interrupt() {
        UCSRB::unset_mask_raw(RXCIE);
    }

    /// Disable the USART_TXC interrupt
    #[allow(non_snake_case)]
    pub fn disable_USART_TXC_interrupt() {
        UCSRB::unset_mask_raw(TXCIE);
    }

    /// Configure serial connection to low baudrate `UBRR_BAUD_LOW`.
    pub fn set_baudrate_low() {
        UBRRH::write((UBRR_BAUD_LOW >> 8) as u8);
        UBRRL::write(UBRR_BAUD_LOW as u8);
    }

    /// Configure serial connection to high baudrate `UBRR_BAUD_HIGH`.
    pub fn set_baudrate_high() {
        UBRRH::write((UBRR_BAUD_HIGH >> 8) as u8);
        UBRRL::write(UBRR_BAUD_HIGH as u8);
    }

    /// Reads a single raw byte from the `Serial` connection. Blocks until the processor is ready
    /// to receive the next byte, i.e., the corresponding bit `UDRE` is set in `UCSRA`.
    #[inline(always)]
    pub fn read_raw() -> u8 {
        UCSRA::wait_until_mask_set_raw(RXC);
        UDR::read()
    }

    /// Writes a single raw byte to the `Serial` connection. Blocks until the processor is ready to
    /// send the next byte, i.e., the corresponding bit `UDRE` is set in `UCSRA`.
    #[inline(always)]
    pub fn write_raw(b: u8) {
        UCSRA::wait_until_mask_set_raw(UDRE);
        UDR::write(b);
    }

    /*
    /// Tries to write a single raw byte to the `Serial` connection. If the processor is not ready
    /// to send, i.e., the corresponding bit `UDRE` is not set in `UCSRA`, returns with an `Error`.
    #[inline(always)]
    fn try_write_raw(b: u8) -> Result<(), Error> {
        if UCSRA::is_mask_set_raw(UDRE) {
            Ok(UDR::write(b))
        } else {
            Error()
        }
    }
    */

    /// Write something to the `Serial` connection. By default, supports `&str`, `char`, and basic
    /// number types (in decimal notation).
    pub fn write<T: SerialWritable>(value: T) {
        value.write_to_serial();
    }

    /*
    /// Write a number formatted as binary to the `Serial` connection.
    pub fn write_bin<T: SerialWritableBinary>(value: T) {
        value.write_to_serial_as_bin();
    }
    */

    /// Write a number formatted as decimal to the `Serial` connection.
    pub fn write_dec<T: SerialWritableDecimal>(value: T) {
        value.write_to_serial_as_dec();
    }

    /*
    /// Write a number formatted as exponential to the `Serial` connection.
    pub fn write_exp<T: SerialWritableExponential>(value: T) {
        value.write_to_serial_as_exp();
    }
    */

    /// Write a number formatted as hexadecimal to the `Serial` connection.
    pub fn write_hex<T: SerialWritableHexadecimal>(value: T) {
        value.write_to_serial_as_hex();
    }

    /*
    /// Write a number formatted as octal to the `Serial` connection.
    pub fn write_oct<T: SerialWritableOctal>(value: T) {
        value.write_to_serial_as_oct();
    }
    */

    /// Write a `'\n'` (newline character) to the serial connection.
    pub fn new_line() {
        Self::write('\n');
    }
}

/// Convenience macro that allows to write multiple (formatted) `Serial::write` statements as a
/// single call. Currently supported formatters are `dec` and `hex` for numbers.
///
/// Example:
/// ```rust
/// let mut counter = 0;
/// loop {
///     println!(
///         "Counter:",
///         counter => dec,
///         "(DEC) | ",
///         counter => hex,
///         "(HEX)"
///     );
///     counter += 1;
/// }
/// ```
#[macro_export]
macro_rules! print {
    ($($writable: expr $(=> $format: tt)?),* $(,)?) => {
        $($crate::print!(@write $writable $(=> $format)?);)*
    };
    (@write $writable: expr => dec) => {
        Serial::write_dec($writable);
    };
    (@write $writable: expr => hex) => {
        Serial::write_hex($writable);
    };
    (@write $writable: expr) => {
        Serial::write($writable);
    };
}

/// Convenience macro that allows to use the `print!` macro and append a newline character.
#[macro_export]
macro_rules! println {
    ($($writable: expr $(=> $format: tt)?),* $(,)?) => {
        $crate::print!($($writable $(=> $format)?, )*);
        Serial::new_line();
    };
}
