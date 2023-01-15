//! Describes the pins available on the atmega32.
#![allow(non_camel_case_types)]

use super::{
    super::{
        pin::{pin, port},
        Pin,
    },
    registers::*,
};

port!(A);
port!(B);
port!(C);
port!(D);
