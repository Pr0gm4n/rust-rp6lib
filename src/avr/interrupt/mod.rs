//! Routines for managing interrupts.
//! Based on:
//! - https://github.com/avr-rust/ruduino/blob/master/src/interrupt.rs
//! - https://docs.rs/bare-metal/0.2.5/src/bare_metal/lib.rs.html

use core::{arch::asm, marker::PhantomData};

pub mod mutex;

/// Helper struct that automatically restores interrupts on drop. The wrapped `PhantomData` creates
/// a private field to ensure that this struct cannot safely be initialized from outside of this
/// module. Please use `without_interrupts` or `unsafe { ... }` (only if you know what you are
/// doing!) to enter a `CriticalSection`.
pub struct CriticalSection(PhantomData<()>);

impl CriticalSection {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        asm!("CLI");
        CriticalSection(PhantomData)
    }
}

impl Drop for CriticalSection {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { asm!("SEI") }
    }
}

/// Executes a closure, disabling interrupts until its completion. Introduces a `CriticalSection`
/// that allows to access shared data structures via the guards provided in the `mutex` module.
///
/// Restores interrupts after the closure has completed execution.
#[inline(always)]
pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce(&CriticalSection) -> T,
{
    // entering a `CriticalSection` is unsafe
    unsafe {
        // enter a `CriticalSection`
        let critical_section = CriticalSection::new();

        // run the given closure with a reference to the `CriticalSection` to allow accessing a `Mutex`
        let result = f(&critical_section);

        // ensure that the `CriticalSection` is only left after the closure has been processed
        drop(critical_section);

        // return whatever the closure yielded
        result
    }
}
