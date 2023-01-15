use super::Register;

/// Represents whether a pin is an input or an output.
pub enum DataDirection {
    /// The pin is exclusively used for reading signals.
    Input,
    /// The pin is exclusively used for sending signals.
    Output,
}

/// An IO pin.
pub trait Pin {
    /// The associated data direction register.
    type DDR: Register<T = u8>;
    /// The associated port register.
    type PORT: Register<T = u8>;
    /// Reads from the register will read input bits.
    // FIXME: Writes to the register can be used to toggle bits.
    type PIN: Register<T = u8>;
    /// The numeric offset of the `Pin` in the register
    const OFFSET: u8;
    /// The mask of the pin used for accessing registers.
    const MASK: u8;

    /// Sets the data direction of the pin.
    #[inline(always)]
    fn set_direction(direction: DataDirection) {
        match direction {
            DataDirection::Input => Self::set_input(),
            DataDirection::Output => Self::set_output(),
        }
    }

    /// Sets the pin up as an input.
    #[inline(always)]
    fn set_input() {
        Self::DDR::unset_mask_raw(Self::MASK);
    }

    /// Sets the pin up as an output.
    #[inline(always)]
    fn set_output() {
        Self::DDR::set_mask_raw(Self::MASK);
    }

    /// Set the pin to high.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn set_high() {
        Self::PORT::set_mask_raw(Self::MASK);
    }

    /// Set the pin to low.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn set_low() {
        Self::PORT::unset_mask_raw(Self::MASK);
    }

    /// Toggles the pin.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn toggle() {
        // FIXME: We can optimise this on post-2006 AVRs.
        // http://www.avrfreaks.net/forum/toggle-state-output-pin
        // set(Self::PIN, Self::MASK);
        Self::PORT::toggle_raw(Self::MASK);
    }

    /// Check if the pin is currently high.
    ///
    /// The pin must be configured as an input.
    #[inline(always)]
    fn is_high() -> bool {
        Self::PIN::is_mask_set_raw(Self::MASK)
    }

    /// Checks if the pin is currently low.
    ///
    /// The pin must be configured as an input.
    #[inline(always)]
    fn is_low() -> bool {
        Self::PIN::is_clear_raw(Self::MASK)
    }
}

/// Convenience macro to define a pin struct directly from the `DDR`, `PORT` and `PIN` `Register`s.
/// Requires you to `use Pin;` and `use register::*;` from this module.
///
/// Example: To define `pin::a0` from `DDRA`, `PORTA` and `PINA` registers, use `pin!(A, a0, 0);`.
macro_rules! pin {
    ($pin_group: ident, $mask_bit: expr) => {
        paste::paste! {
            // define new `pub struct` with the `Pin`'s name
            pub struct [<$pin_group:lower $mask_bit>];
            // impl `Pin` for the struct
            impl Pin for [<$pin_group:lower $mask_bit>] {
                /// Data Direction Register.
                type DDR = [<DDR $pin_group>];
                /// output PORT register.
                type PORT = [<PORT $pin_group>];
                /// input PIN register.
                type PIN = [<PIN $pin_group>];
                /// offset of the `Pin` in the register
                const OFFSET: u8 = $mask_bit;
                /// bit MASK for the corresponding pin
                const MASK: u8 = 1 << $mask_bit;
            }
        }
    };
}
// export macro to the crate
pub(crate) use pin;

/// Convenience macro to define all 8 pins grouped into a single PORT group.
/// Requires you to `use Pin;` and `use register::*;` from this module.
///
/// Example: To define `port::a0` through `port::a7` from `DDRA`, `PORTA` and `PINA` registers, use
/// `port!(A);`.
macro_rules! port {
    ($pin_group: ident) => {
        pin!($pin_group, 0);
        pin!($pin_group, 1);
        pin!($pin_group, 2);
        pin!($pin_group, 3);
        pin!($pin_group, 4);
        pin!($pin_group, 5);
        pin!($pin_group, 6);
        pin!($pin_group, 7);
    };
}
// export macro to the crate
pub(crate) use port;

/// Convenience setter and getter macros to set multiple pins in the same register at once.
///
/// Example: To set `b0`, `b1` and `b7` to `0b110`, use `set_pins!(b0, b1, b7, 0b110);`.
macro_rules! set_pins {
    ([$base_pin: ident, $($pin: ident),*], $value: expr $(,)?) => {
        // TODO check that users have really used this macro only for pins in the same PORT group
        //$(assert!(<$base_pin as Pin>::DDR == <$pin as Pin>::DDR);)*

        // set pins as outputs
        let pin_mask = $base_pin::MASK $(| $pin::MASK)*;
        <$base_pin as Pin>::DDR::set_mask_raw(pin_mask);

        // set pins' values
        <$base_pin as Pin>::PORT::write(
            (<$base_pin as Pin>::PORT::read() & !pin_mask)
            | set_pins!(@reverse_for_output_mask [$($pin, )*], [$base_pin], $value)
        );
    };

    // base case: pass reversed array of pins to @output_mask
    (@reverse_for_output_mask [], [$($pin_rev: ident),* $(,)?], $value: expr) => {
        set_pins!(@output_mask 0, [$($pin_rev, )*], $value)
    };
    // otherwise: add front element to front of the reversed array of pins
    (@reverse_for_output_mask [$first_pin: ident, $($pin: ident),* $(,)?], [$($pin_rev: ident),* $(,)?], $value: expr) => {
        set_pins!(@reverse_for_output_mask [$($pin, )*], [$first_pin, $($pin_rev, )*], $value)
    };

    // add correct mask for the next pin
    (@output_mask $position: expr, [$last_pin: ident, $($pin_rev: ident),* $(,)?], $value: expr) => {
        (
        // 1. `>>`-shift $value by $position bits to the right and extract the least significant bit
            (($value >> $position) & 1)
        // 2. `<<`-shift this extracted bit to the correct mask offset of the respective pin
            << <$last_pin as Pin>::OFFSET)
        // 3. compute the logical `|` with the remaining pins' bitmask
            | set_pins!(@output_mask $position, [$($pin_rev, )*], $value >> 1)
    };
    // end of recursion: all pins included in the mask
    (@output_mask $position: expr, [], $value: expr) => {
        0
    };
}
// export macro to the crate
pub(crate) use set_pins;
