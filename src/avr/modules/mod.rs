//! Modules that can be implemented for specific cores.

pub use self::spi::HardwareSpi;
pub use self::timer::{
    ClockSource16, ClockSource8, Timer16, Timer16Setup, Timer8, Timer8Setup,
    WaveformGenerationMode16, WaveformGenerationMode8,
};
pub use self::usart::HardwareUsart;

mod spi;
mod timer;
mod usart;
