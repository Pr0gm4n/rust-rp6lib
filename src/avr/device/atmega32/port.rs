//! Describes the pins available on the atmega32.
#![allow(non_camel_case_types)]

use super::{
    super::{pin::pin, Pin},
    registers::*,
};

// define port A
pin!(A, a0, 0);
pin!(A, a1, 1);
pin!(A, a2, 2);
pin!(A, a3, 3);
pin!(A, a4, 4);
pin!(A, a5, 5);
pin!(A, a6, 6);
pin!(A, a7, 7);

// define port B
pin!(B, b0, 0);
pin!(B, b1, 1);
pin!(B, b2, 2);
pin!(B, b3, 3);
pin!(B, b4, 4);
pin!(B, b5, 5);
pin!(B, b6, 6);
pin!(B, b7, 7);

// define port C
pin!(C, c0, 0);
pin!(C, c1, 1);
pin!(C, c2, 2);
pin!(C, c3, 3);
pin!(C, c4, 4);
pin!(C, c5, 5);
pin!(C, c6, 6);
pin!(C, c7, 7);

// define port D
pin!(D, d0, 0);
pin!(D, d1, 1);
pin!(D, d2, 2);
pin!(D, d3, 3);
pin!(D, d4, 4);
pin!(D, d5, 5);
pin!(D, d6, 6);
pin!(D, d7, 7);
