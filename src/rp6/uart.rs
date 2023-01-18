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
use core::{fmt::Write, mem::size_of};
use heapless::String;

use avr_config::CPU_FREQUENCY_HZ;

/// Define constants for RP6 baudrates.
pub const BAUD_LOW: u32 = 38400; // Low speed: 38.400 Baud
pub const UBRR_BAUD_LOW: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_LOW)) - 1;
pub const BAUD_HIGH: u32 = 500000; // High speed: 500.000 Baud
pub const UBRR_BAUD_HIGH: u32 = (CPU_FREQUENCY_HZ / (16 * BAUD_HIGH)) - 1;

/// Convenience macro allowing to define multiple writer functions that allow formatting number
/// types and print them to the `Serial` connection of the robot.
///
/// Defines:
/// - `write_$type(value: $type)`
/// - `write_$type_hex(value: $type)`
/// - `write_$type_oct(value: $type)`
/// - `write_$type_bin(value: $type)`
macro_rules! impl_write_num {
    ($(#[$attr: meta])* $type: ty) => {
        $(#[$attr])*
        impl_write_num!(@internal $type, 3, "{}");
        $(#[$attr])*
        impl_write_num!(@internal $type, hex, 2, "{:x}");
        $(#[$attr])*
        impl_write_num!(@internal $type, oct, 4, "{:o}");
        $(#[$attr])*
        impl_write_num!(@internal $type, bin, 8, "{:b}");
    };
    (@internal $(#[$attr: meta])* $type: ty, $($suffix: ident,)? $bits_per_byte: literal, $format: literal) => {
        paste::paste! {
            $(#[$attr])*
            #[doc = "Print an `" $type "` number " $("in " $suffix:upper " format")? " and send it to the robot's `Serial` connection."]
            pub fn [<write_ $type $(_$suffix)?>](value: $type) {
                let mut buffer: String<{ size_of::<$type>() * $bits_per_byte }> = String::new();
                write!(&mut buffer, $format, value).unwrap();
                Self::write_str(buffer.as_str());
            }
        }
    };
}

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

    /// Writes a single raw byte to the `Serial` connection. Blocks until the processor is ready to
    /// send the next byte, i.e., the corresponding bit `UDRE` is set in `UCSRA`.
    #[inline(always)]
    fn write_raw(b: u8) {
        UCSRA::wait_until_mask_set_raw(UDRE);
        UDR::write(b);
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
        Self::write_raw(c as u8);
    }

    /// Write text to the serial connection.
    pub fn write_str(s: &str) {
        for c in s.chars() {
            Self::write_char(c);
        }
    }

    // define write functions for a bunch of types
    impl_write_num!(u8);
    impl_write_num!(i8);
    impl_write_num!(u16);
    impl_write_num!(i16);
    impl_write_num!(u32);
    impl_write_num!(i32);
    impl_write_num!(u64);
    impl_write_num!(i64);
    impl_write_num!(usize);
    impl_write_num!(isize);

    /// Write a `'\n'` (newline character) to the serial connection.
    pub fn new_line() {
        Self::write_char('\n');
    }
}
