window.SIDEBAR_ITEMS = {"fn":[["wait_until","Waits until some condition is true of the register."]],"macro":[["bitmask","Convenience macro to define a bitmask as a `RegisterValue`- directly from `avrd::<device>::*` identifiers. Requires you to have `RegisterValue` from this module and `avrd::<your-device> as avr_device` in scope. By default, `$reg_value_type` is set to `u8`. Additionally, one can provide documentation for the `RegisterValue` inside the macro’s parenthesis."],["bitmask_list","Convenience macro to define multiple bitmasks as `RegisterValue` at once. Requires you to have `bitmask!` and `RegisterValue` from this module in scope. Additionally, one can provide documentation for each list element as usual."],["reg","Convenience macro to define a register struct directly from `avrd::<device>::*` identifiers. Requires you to have `Register` from this module and `use avrd::<your-device> as avr_device` in scope. By default, `$reg_type` is set to `u8`. Additionally, one can provide documentation for the `Register` struct inside the macro’s parenthesis."],["reg_list","Convenience macro to define multiple registers at once. Requires you to have `reg!` and `register::$reg_name` from this module in scope. Additionally, one can provide documentation for each list element as usual."]],"struct":[["RegisterBits","Represents a set of bits within a specific register."]],"trait":[["Register","A register."],["RegisterValue","A value that a register can store."]]};