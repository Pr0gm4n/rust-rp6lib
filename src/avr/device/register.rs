use core::{cmp, convert, marker, ops};

/// A value that a register can store.
///
/// All registers are either `u8` or `u16`.
pub trait RegisterValue:
    Copy
    + Clone
    + ops::BitAnd<Output = Self>
    + ops::BitAndAssign
    + ops::BitOr<Output = Self>
    + ops::BitOrAssign
    + ops::BitXor<Output = Self>
    + ops::BitXorAssign
    + ops::Not<Output = Self>
    + cmp::PartialEq
    + cmp::Eq
    + cmp::PartialOrd
    + cmp::Ord
    + convert::From<u8>
{
}

/// A register.
pub trait Register: Default + Sized {
    /// The type that can represent the value of the register.
    type T: RegisterValue;
    /// The type representing a set of bits that may be manipulated
    /// within the register.
    type RegisterBits = RegisterBits<Self>;

    /// The address of the register.
    const ADDRESS: *mut Self::T;

    /// Writes a value to the register.
    #[inline(always)]
    fn write<V>(value: V)
    where
        V: Into<Self::T>,
    {
        unsafe {
            core::ptr::write_volatile(Self::ADDRESS, value.into());
        }
    }

    /// Reads the value of the register.
    #[inline(always)]
    fn read() -> Self::T {
        unsafe { core::ptr::read_volatile(Self::ADDRESS) }
    }

    /// Sets a set of bits to `1` in the register.
    fn set(bits: RegisterBits<Self>) {
        Self::set_mask_raw(bits.mask);
    }

    /// Sets a bitmask in a register.
    ///
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set_mask_raw(mask: Self::T) {
        unsafe {
            core::ptr::write_volatile(
                Self::ADDRESS,
                core::ptr::read_volatile(Self::ADDRESS) | mask,
            );
        }
    }

    /// Unsets a set of bits in the register.
    ///
    /// All of the bits will be set to `0`.
    fn unset(bits: RegisterBits<Self>) {
        Self::unset_mask_raw(bits.mask);
    }

    /// Clears a bitmask from a register.
    ///
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset_mask_raw(mask: Self::T) {
        unsafe {
            core::ptr::write_volatile(
                Self::ADDRESS,
                core::ptr::read_volatile(Self::ADDRESS) & !mask,
            )
        }
    }

    /// Toggles a set of bits within the register.
    ///
    /// All specified bits which were previously `0` will become
    /// `1`, and all specified bits that were previous `1` will
    /// become `0`.
    fn toggle(mask: RegisterBits<Self>) {
        Self::toggle_raw(mask.mask);
    }

    /// Toggles a mask in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    #[inline(always)]
    fn toggle_raw(mask: Self::T) {
        unsafe {
            core::ptr::write_volatile(
                Self::ADDRESS,
                core::ptr::read_volatile(Self::ADDRESS) ^ mask,
            )
        }
    }

    /// Checks if a set of bits are enabled.
    ///
    /// All specifed bits must be set for this function
    /// to return `true`.
    fn is_set(bits: RegisterBits<Self>) -> bool {
        Self::is_mask_set_raw(bits.mask)
    }

    /// Checks if a mask is set in the register.
    ///
    /// This is equivalent to `(r & mask) == mask`.
    #[inline(always)]
    fn is_mask_set_raw(mask: Self::T) -> bool {
        unsafe { (core::ptr::read_volatile(Self::ADDRESS) & mask) == mask }
    }

    /// Checks if a set of bits are not set.
    ///
    /// All specified bits must be `0` for this
    /// function to return `true`.
    fn is_clear(mask: RegisterBits<Self>) -> bool {
        Self::is_clear_raw(mask.mask)
    }

    /// Checks if a mask is clear in the register.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    #[inline(always)]
    fn is_clear_raw(mask: Self::T) -> bool {
        unsafe { (core::ptr::read_volatile(Self::ADDRESS) & mask) == Self::T::from(0) }
    }

    /// Waits until a set of bits are set in the register.
    ///
    /// This function will block until all bits that are set in
    /// the mask are also set in the register.
    fn wait_until_set(bits: RegisterBits<Self>) {
        Self::wait_until_mask_set_raw(bits.mask);
    }

    /// Waits until a bit mask is set in the register.
    ///
    /// This function will block until all bits that are set in
    /// the mask are also set in the register.
    #[inline(always)]
    fn wait_until_mask_set_raw(mask: Self::T) {
        wait_until(|| Self::is_mask_set_raw(mask))
    }
}

/// Represents a set of bits within a specific register.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisterBits<R: Register> {
    /// The raw bitmask.
    mask: R::T,
    _phantom: marker::PhantomData<R>,
}

impl<R> RegisterBits<R>
where
    R: Register,
{
    /// Creates a new register mask.
    pub const fn new(mask: R::T) -> Self {
        RegisterBits {
            mask,
            _phantom: marker::PhantomData,
        }
    }

    pub fn zero() -> Self {
        RegisterBits::new(0u8.into())
    }
}

impl<R> ops::BitOr for RegisterBits<R>
where
    R: Register,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        RegisterBits::new(self.mask | rhs.mask)
    }
}

impl<R> ops::BitOrAssign for RegisterBits<R>
where
    R: Register,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
    }
}

impl<R> ops::BitAnd for RegisterBits<R>
where
    R: Register,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        RegisterBits::new(self.mask & rhs.mask)
    }
}

impl<R> ops::BitAndAssign for RegisterBits<R>
where
    R: Register,
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask;
    }
}

impl<R> ops::Not for RegisterBits<R>
where
    R: Register,
{
    type Output = Self;

    fn not(self) -> Self {
        RegisterBits::new(!self.mask)
    }
}

impl<R> From<RegisterBits<R>> for u8
where
    R: Register<T = u8>,
{
    fn from(other: RegisterBits<R>) -> u8 {
        other.mask
    }
}

impl<R> From<RegisterBits<R>> for u16
where
    R: Register<T = u16>,
{
    fn from(other: RegisterBits<R>) -> u16 {
        other.mask
    }
}

impl RegisterValue for u8 {}
impl RegisterValue for u16 {}

/// Waits until some condition is true of the register.
#[inline(always)]
fn wait_until<F>(mut f: F)
where
    F: FnMut() -> bool,
{
    loop {
        if f() {
            break;
        }
    }
}

/// Convenience macro to define a register struct directly from `avrd::<device>::*` identifiers.
/// Requires you to have `Register` from this module and `use avrd::<your-device> as avr_device` in
/// scope. By default, `$reg_type` is set to `u8`. Additionally, one can provide documentation for
/// the `Register` struct inside the macro's parenthesis.
macro_rules! reg {
    ($(#[$attr: meta])* $reg_name: ident) => {
        reg!($(#[$attr])* $reg_name, u8);
    };
    ($(#[$attr: meta])* $reg_name: ident, $reg_type: ty) => {
        // define new `pub struct` with the `Register`'s name
        $(#[$attr])*
        #[derive(Default)]
        pub struct $reg_name;
        // impl Register for the struct
        impl Register for $reg_name {
            type T = $reg_type;
            const ADDRESS: *mut $reg_type = avr_device::$reg_name as *mut $reg_type;
        }
    };
}
// export macro to the crate
pub(crate) use reg;

/// Convenience macro to define multiple registers at once.
/// Requires you to have `reg!` and `register::$reg_name` from this module in scope. Additionally,
/// one can provide documentation for each list element as usual.
///
/// Example: To define `DDRA`, `PORTA` and `PINA`, use `reg_list!(DDRA, PORTA, PINA);`.
macro_rules! reg_list {
    ($($(#[$attr: meta])* $reg_name: ident),* $(,)?) => {
        $(reg!($(#[$attr])* $reg_name);)*
    };
}
pub(crate) use reg_list;

/// Convenience macro to define a bitmask as a `RegisterValue`- directly from `avrd::<device>::*`
/// identifiers. Requires you to have `RegisterValue` from this module and `avrd::<your-device> as
/// avr_device` in scope. By default, `$reg_value_type` is set to `u8`. Additionally, one can
/// provide documentation for the `RegisterValue` inside the macro's parenthesis.
macro_rules! bitmask {
    ($(#[$attr: meta])* $bitmask_name: ident) => {
        bitmask!($(#[$attr])* $bitmask_name, u8);
    };
    ($(#[$attr: meta])* $bitmask_name: ident, $reg_value_type: ty) => {
        // define new `pub struct` with the `Register`'s name
        $(#[$attr])*
        pub const $bitmask_name: $reg_value_type = unsafe {
            core::intrinsics::transmute::<*mut u8, usize>(avr_device::$bitmask_name) as $reg_value_type
        };
    };
}
// export macro to the crate
pub(crate) use bitmask;

/// Convenience macro to define multiple bitmasks as `RegisterValue` at once.
/// Requires you to have `bitmask!` and `RegisterValue` from this module in scope. Additionally,
/// one can provide documentation for each list element as usual.
///
/// Example: To define `INT0`, `INT1` and `INT2`, use `bitmask_list!(INT0, INT1, INT2);`.
macro_rules! bitmask_list {
    ($($(#[$attr: meta])* $bitmask_name: ident$(: $bitmask_type: ty)?),* $(,)?) => {
        $(bitmask!($(#[$attr])* $bitmask_name $(, $bitmask_type)?);)*
    };
}
pub(crate) use bitmask_list;
