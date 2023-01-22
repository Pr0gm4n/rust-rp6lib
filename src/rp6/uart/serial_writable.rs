use super::Serial;

pub use core::{
    fmt::{Binary, Display, LowerExp, LowerHex, Octal, Write},
    mem::size_of,
};
pub use heapless::String;

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
