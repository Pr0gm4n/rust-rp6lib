//! Routines for managing interrupts.
//! Based on:
//! - <https://github.com/avr-rust/ruduino/blob/master/src/interrupt.rs>
//! - <https://docs.rs/bare-metal/0.2.5/src/bare_metal/lib.rs.html>

use core::{arch::asm, marker::PhantomData};

pub mod mutex;
use mutex::Mutex;

/// Atomic counter of critical sections to avoid problems when `without_interrupts` is used in
/// nested function calls.
#[cfg(not(feature = "unsafe-no-critical-section-count"))]
static CRITICAL_SECTION_COUNTER: Mutex<usize> = Mutex::new(0);

/// Helper struct that automatically restores interrupts on drop. The wrapped `PhantomData` creates
/// a private field to ensure that this struct cannot be initialized from outside of this module
/// without using its `unsafe` initializer function `new`. The recommended use to enter a
/// `CriticalSection` is to pass a closure to `without_interrupts`.
///
/// When the feature `unsafe-no-critical-section-count` is disabled, this implementation is also
/// safe w.r.t. nested calls of `without_interrupts`. This is achieved by counting how many
/// `CriticalSection`s were entered, and only enabling device interrupts once the last
/// `CriticalSection` is exited. However, as these checks incur a small runtime overhead, they can
/// be disabled with the feature `unsafe-no-critical-section-count`. Note that, for execution
/// consistency, a user must then ensure that `without_interrupts` will never be nested!
pub struct CriticalSection(PhantomData<()>);

impl CriticalSection {
    /// Upon entering any `CriticalSection`, disable global device interrupts.
    ///
    /// # Safety
    /// When the feature `unsafe-no-critical-section-count` is disabled, this implementation is also
    /// safe w.r.t. nested `CriticalSection`s, e.g., by nesting calls to `without_interrupts`. This
    /// is achieved by counting how many `CriticalSection`s were entered, and only enabling device
    /// interrupts once the last `CriticalSection` is exited. However, as these checks incur a small
    /// runtime overhead, they can be disabled with the feature `unsafe-no-critical-section-count`.
    /// Note that, for execution consistency, a user must then ensure that `CriticalSection`s will
    /// never be nested!
    #[inline(always)]
    pub unsafe fn new() -> Self {
        // first, deactivate interrupts
        asm!("CLI");

        // next, create the new `CriticalSection`
        let cs = CriticalSection(PhantomData);

        // now, in guaranteed single-threaded mode, increase number of `CriticalSection`s
        #[cfg(not(feature = "unsafe-no-critical-section-count"))]
        CRITICAL_SECTION_COUNTER.lock(&cs).update(|x| x + 1);

        cs
    }
}

impl Drop for CriticalSection {
    /// Upon dropping the last `CriticalSection`, enable global device interrupts.
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(not(feature = "unsafe-no-critical-section-count"))]
        CRITICAL_SECTION_COUNTER.lock(self).update(|x| {
            if x == 1 {
                unsafe { asm!("SEI") }
            }
            x - 1
        });

        #[cfg(feature = "unsafe-no-critical-section-count")]
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
