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

/// Module that implements `Serial::write` and formatting behavior for types.
mod serial_writable;
pub use serial_writable::*;

use avr_config::CPU_FREQUENCY_HZ;

/// Define constants for RP6 baudrates.
pub const BAUD_LOW: u32 = 38400; // Low speed: 38.400 Baud
pub const UBRR_BAUD_LOW: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_LOW)) - 1;
pub const BAUD_HIGH: u32 = 500000; // High speed: 500.000 Baud
pub const UBRR_BAUD_HIGH: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_HIGH)) - 1;

/*
/// Macro that allows fixed-size formatting of any values that implement either `Display` or
/// `Debug`. For more details, refer to `core::fmt::Write`.
#[macro_export]
macro_rules! format {
    ($format_str: literal, $($arg: tt: $type: ty),* $(,)?) => {
        format!(@build $format_str, format!(@size $format_str, $($type, )*) $(, $arg)*)
    };

    (@size $format_str: literal, $($type: ty)* $(,)?) => {
        $format_str.len() + 8 * (0 $(+ size_of::<$type>())*)
    };

    (@build $format_str: literal, $size: expr, $($value: tt),*) => {
        {
            let mut buffer: String<{ $size }> = String::new();
            write!(&mut buffer, $format_str $(, $value)*).unwrap();
            buffer
        }
    };
}
//pub(crate) use format;
*/

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

    /// Writes a single raw byte to the `Serial` connection. Blocks until the processor is ready to
    /// send the next byte, i.e., the corresponding bit `UDRE` is set in `UCSRA`.
    #[inline(always)]
    fn write_raw(b: u8) {
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

    /// Write a number formatted as binary to the `Serial` connection.
    pub fn write_bin<T: SerialWritableBinary>(value: T) {
        value.write_to_serial_as_bin();
    }

    /// Write a number formatted as decimal to the `Serial` connection.
    pub fn write_dec<T: SerialWritableDecimal>(value: T) {
        value.write_to_serial_as_dec();
    }

    /// Write a number formatted as exponential to the `Serial` connection.
    pub fn write_exp<T: SerialWritableExponential>(value: T) {
        value.write_to_serial_as_exp();
    }

    /// Write a number formatted as hexadecimal to the `Serial` connection.
    pub fn write_hex<T: SerialWritableHexadecimal>(value: T) {
        value.write_to_serial_as_hex();
    }

    /// Write a number formatted as octal to the `Serial` connection.
    pub fn write_oct<T: SerialWritableOctal>(value: T) {
        value.write_to_serial_as_oct();
    }

    /// Write a `'\n'` (newline character) to the serial connection.
    pub fn new_line() {
        Self::write('\n');
    }
}

/// Convenience macro that allows to write multiple (formatted) `Serial::write` statements as a
/// single call. Currently supported formatters are `bin`, `dec`, `exp`, `hex`, and `oct` for
/// numbers.
#[macro_export]
macro_rules! print {
    ($($writable: expr $(=> $format: tt)?),* $(,)?) => {
        $(print!(@write $writable $(=> $format)?);)*
    };
    (@write $writable: expr => bin) => {
        Serial::write_bin($writable);
    };
    (@write $writable: expr => dec) => {
        Serial::write_dec($writable);
    };
    (@write $writable: expr => exp) => {
        Serial::write_exp($writable);
    };
    (@write $writable: expr => hex) => {
        Serial::write_hex($writable);
    };
    (@write $writable: expr => oct) => {
        Serial::write_oct($writable);
    };
    (@write $writable: expr) => {
        Serial::write($writable);
    };
}

/// Convenience macro that allows to use the `print!` macro and append a newline character.
#[macro_export]
macro_rules! println {
    ($($writable: expr $(=> $format: tt)?),* $(,)?) => {
        print!($($writable $(=> $format)?, )*);
        Serial::new_line();
    };
}
