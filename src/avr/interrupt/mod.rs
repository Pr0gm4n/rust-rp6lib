//! Routines for managing interrupts.
//! Based on:
//! - <https://github.com/avr-rust/ruduino/blob/master/src/interrupt.rs>
//! - <https://docs.rs/bare-metal/0.2.5/src/bare_metal/lib.rs.html>

use core::{
    arch::asm,
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

pub mod mutex;

/// Atomic counter of critical sections to avoid problems when `without_interrupts` is used in
/// nested function calls.
#[cfg(feature = "critical-section-count")]
static CRITICAL_SECTION_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Helper struct that automatically restores interrupts on drop. The wrapped `PhantomData` creates
/// a private field to ensure that this struct cannot safely be initialized from outside of this
/// module. Please use `without_interrupts` or `unsafe { ... }` (only if you know what you are
/// doing!) to enter a `CriticalSection`.
pub struct CriticalSection(PhantomData<()>);

impl CriticalSection {
    /// Upon entering any `CriticalSection`, disable global device interrupts.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        // deactivate interrupts
        asm!("CLI");

        // now, in guaranteed single-threaded mode, increase number of `CriticalSection`s
        #[cfg(feature = "critical-section-count")]
        CRITICAL_SECTION_COUNTER.fetch_add(1, Ordering::SeqCst);

        // finally, enter the new `CriticalSection`
        CriticalSection(PhantomData)
    }
}

impl Drop for CriticalSection {
    /// Upon dropping the last `CriticalSection`, enable global device interrupts.
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(feature = "critical-section-count")]
        if CRITICAL_SECTION_COUNTER.fetch_sub(1, Ordering::SeqCst) <= 0 {
            unsafe { asm!("SEI") }
        }

        #[cfg(not(feature = "critical-section-count"))]
        unsafe {
            asm!("SEI")
        }
    }
}

/// Executes a closure, disabling interrupts until its completion. Introduces a `CriticalSection`
/// that allows to access shared data structures via the guards provided in the `mutex` module.
///
/// Restores interrupts after the closure has completed execution.
#[inline(always)]
pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce(&mut CriticalSection) -> T,
{
    // entering a `CriticalSection` is unsafe
    let mut critical_section = unsafe { CriticalSection::new() };

    // run the given closure with a unique reference to the `CriticalSection` to allow
    // accessing a `Mutex`
    let result = f(&mut critical_section);

    // explicitly ensure that the `CriticalSection` is left after the closure has been processed
    drop(critical_section);

    // return whatever the closure yielded
    result
}
