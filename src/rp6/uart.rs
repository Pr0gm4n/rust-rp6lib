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
pub use core::{
    fmt::{Binary, Display, LowerExp, LowerHex, Octal, Write},
    mem::size_of,
};
pub use heapless::String;

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

/// Trait to allow implementing specific `Serial::write` behavior for types.
pub trait SerialWritable {
    /// Takes a reference to the instance of this type and writes it to the `Serial` connection.
    fn write_to_serial(&self);
}

impl SerialWritable for char {
    /// Writes a single character to the UART connection.
    ///
    /// Example:
    /// ```rust
    ///	Serial::write('R');
    ///	Serial::write('P');
    ///	Serial::write('6');
    ///	Serial::write('\n'); // '\n' is a special code for the "new line" character!
    ///	Serial::write('0'); // the ASCII character '0'
    ///	Serial::write(48 as char); // 48 is numeric ASCII code for '0'
    ///	Serial::write(49 as char); // '1'
    ///	Serial::write(50 as char); // '2'
    ///	Serial::write(51 as char); // '3'
    ///	```
    ///
    /// This example would send:
    /// ```
    /// RP6
    /// 00123
    /// ```
    /// via the robot's serial connection.
    fn write_to_serial(&self) {
        Serial::write_raw(*self as u8);
    }
}

impl SerialWritable for &str {
    /// Write text to the `Serial` connection.
    fn write_to_serial(&self) {
        for c in self.chars() {
            Serial::write(c);
        }
    }
}

/// Trait to allow instantiation and passing as `&str` for a type.
pub trait StringType: Write {
    /// Instantiate the `StringType`.
    fn new() -> Self;
    /// Allow passing the `StringType` as `&str`.
    fn as_str(&self) -> &str;
}

impl<const N: usize> StringType for String<N> {
    fn new() -> Self {
        String::<N>::new()
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }
}

/// Trait to allow implementing specific `Serial::write_dec` behavior for types.
pub trait SerialWritableDecimal: Display {
    type DecimalString: StringType;

    /// Format the given number as decimal and write it to the `Serial` connection.
    fn write_to_serial_as_dec(&self) {
        let mut buffer = Self::DecimalString::new();
        write!(&mut buffer, "{}", self).unwrap();
        Serial::write(StringType::as_str(&buffer));
    }
}

/// Default implementation for numbers is to format them as decimal.
impl<T: SerialWritableDecimal> SerialWritable for T {
    /// Default formatter: format numbers as decimal.
    fn write_to_serial(&self) {
        self.write_to_serial_as_dec();
    }
}

/// Trait to allow implementing specific `Serial::write_bin` behavior for types.
pub trait SerialWritableBinary: Binary {
    type BinaryString: StringType;

    /// Format the given number as binary and write it to the `Serial` connection.
    fn write_to_serial_as_bin(&self) {
        let mut buffer = Self::BinaryString::new();
        write!(&mut buffer, "{:b}", self).unwrap();
        Serial::write(StringType::as_str(&buffer));
    }
}

/// Trait to allow implementing specific `Serial::write_exp` behavior for types.
pub trait SerialWritableExponential: LowerExp {
    type ExponentialString: StringType;

    /// Format the given number as decimal and write it to the `Serial` connection.
    fn write_to_serial_as_exp(&self) {
        let mut buffer = Self::ExponentialString::new();
        write!(&mut buffer, "{:e}", self).unwrap();
        Serial::write(StringType::as_str(&buffer));
    }
}

/// Trait to allow implementing specific `Serial::write_hex` behavior for types.
pub trait SerialWritableHexadecimal: LowerHex {
    type HexadecimalString: StringType;

    /// Format the given number as hexadecimal and write it to the `Serial` connection.
    fn write_to_serial_as_hex(&self) {
        let mut buffer = Self::HexadecimalString::new();
        write!(&mut buffer, "{:x}", self).unwrap();
        Serial::write(StringType::as_str(&buffer));
    }
}

/// Trait to allow implementing specific `Serial::write_oct` behavior for types.
pub trait SerialWritableOctal: Octal {
    type OctalString: StringType;

    /// Format the given number as octal and write it to the `Serial` connection.
    fn write_to_serial_as_oct(&self) {
        let mut buffer = Self::OctalString::new();
        write!(&mut buffer, "{:o}", self).unwrap();
        Serial::write(StringType::as_str(&buffer));
    }
}

/// Implement the traits for `Binary`, `Decimal`, `Hexadecimal` and `Octal` formatting of a number.
macro_rules! impl_serial_writable_num {
    // default: use 4 * bytesize as $size_dec (accounting for signed types)
    ($type: ty $(,)?) => {
        impl_serial_writable_num!($type, 4 * ::core::mem::size_of::<$type>());
    };
    // default: use 3 * bytesize as $size_oct
    ($type: ty, $size_dec: expr $(,)?) => {
        impl_serial_writable_num!($type, $size_dec, 3 * ::core::mem::size_of::<$type>());
    };
    // implement traits for Binary, Decimal, Hexadecimal and Octal
    ($type: ty, $size_dec: expr, $size_oct: expr $(,)?) => {
        impl_serial_writable_num!(@impl $type, Binary, 8 * ::core::mem::size_of::<$type>());
        impl_serial_writable_num!(@impl $type, Decimal, $size_dec);
        impl_serial_writable_num!(@impl $type, Hexadecimal, 2 * ::core::mem::size_of::<$type>());
        impl_serial_writable_num!(@impl $type, Octal, $size_oct);
    };
    // implement the trait `SerialWritable{$base_ident}` for `$type`.
    (@impl $type: ty, $base_name: ident, $size: expr) => {
        paste::paste! {
            impl [<SerialWritable $base_name>] for $type {
                type [<$base_name String>] = String<{ $size }>;
            }
        }
    };
}

impl_serial_writable_num!(u8);
impl_serial_writable_num!(u16);
impl_serial_writable_num!(u32);
impl_serial_writable_num!(u64);
impl_serial_writable_num!(u128);
impl_serial_writable_num!(usize);
impl_serial_writable_num!(i8);
impl_serial_writable_num!(i16);
impl_serial_writable_num!(i32);
impl_serial_writable_num!(i64);
impl_serial_writable_num!(i128);
impl_serial_writable_num!(isize);

/// Implement the trait for `Decimal` formatting of a number.
macro_rules! impl_serial_writable_float {
    ($type: ty, $size_in_chars: expr) => {
        impl_serial_writable_num!(@impl $type, Decimal, $size_in_chars);
        impl_serial_writable_num!(@impl $type, Exponential, $size_in_chars);
    };
}

impl_serial_writable_float!(f32, 100);
impl_serial_writable_float!(f64, 100);
