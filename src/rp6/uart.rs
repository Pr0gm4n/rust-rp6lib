//! Module allowing for simple use of the robot's serial UART connection.
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
    pub fn init() {}
}
