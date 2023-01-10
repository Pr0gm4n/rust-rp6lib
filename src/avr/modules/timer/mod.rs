pub use self::timer16::{
    ClockSource as ClockSource16, Timer16, Timer16Setup,
    WaveformGenerationMode as WaveformGenerationMode16,
};
pub use self::timer8::{
    ClockSource as ClockSource8, Timer8, Timer8Setup,
    WaveformGenerationMode as WaveformGenerationMode8,
};

mod timer16;
mod timer8;
