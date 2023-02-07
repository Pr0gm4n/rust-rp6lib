//! Module to allow safe access of shared data structures within a `CriticalSection`.

use super::CriticalSection;
use core::cell::{Cell, RefCell, UnsafeCell};

/// `Mutex` implements a guard that is safe for "concurrent" data access on single-core devices
/// that can only experience non-atomic data access via interrupts. It restricts data access to
/// within a `CriticalSection`, which can only be obtained within a closure given to
/// `without_interrupts`.
///
/// The data is wrapped in a `core::cell::Cell` for mutability. For more complex data structures,
/// see the `DynamicMutex` type which is based on a `RefCell`.
pub struct Mutex<T: ?Sized> {
    data: UnsafeCell<Cell<T>>,
}

// FIXME: figure out how to loosen the restriction on `Sized` here.
impl<T: Sized> Mutex<T> {
    /// Create a new `Mutex` wrapping data of type `T`.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(Cell::new(data)),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Get uninterrupted access to the encapsulated data. Can only be called inside a
    /// `CriticalSection`, ensured by requiring a reference to one.
    pub fn lock<'access_time>(
        &'access_time self,
        _cs: &'access_time CriticalSection,
    ) -> &'access_time Cell<T> {
        unsafe { &*self.data.get() }
    }
}

// NOTE: A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g., interrupt handlers).
unsafe impl<T> Sync for Mutex<T> where T: Send {}

/// `DynamicMutex` implements a guard that is safe for "concurrent" data access on single-core
/// devices that can only experience non-atomic data access via interrupts. It restricts data
/// access to within a `CriticalSection`, which can only be obtained within a closure given to
/// `without_interrupts`.
///
/// The data is wrapped in a `core::cell::RefCell` for mutability, which is dynamically checked for
/// consistency and can lead to a panic. Whenever possible, it is recommended to use a `Mutex`
/// instead, which is based on a `Cell`.
pub struct DynamicMutex<T: ?Sized> {
    data: UnsafeCell<RefCell<T>>,
}

// FIXME: figure out how to loosen the restriction on `Sized` here.
impl<T: Sized> DynamicMutex<T> {
    /// Create a new `DynamicMutex` wrapping data of type `T`.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(RefCell::new(data)),
        }
    }
}

impl<T: ?Sized> DynamicMutex<T> {
    /// Get uninterrupted access to the encapsulated data. Can only be called inside a
    /// `CriticalSection`, ensured by requiring a reference to one.
    pub fn lock<'access_time>(
        &'access_time self,
        _cs: &'access_time CriticalSection,
    ) -> &'access_time RefCell<T> {
        unsafe { &*self.data.get() }
    }
}

// NOTE: A `DynamicMutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g., interrupt handlers).
unsafe impl<T> Sync for DynamicMutex<T> where T: Send {}
