//! Describes the registers available on the atmega32.
#![allow(non_camel_case_types)]

use super::super::{register::reg, Register};
use avrd::atmega32 as avr_device;

reg!(DDRA);
reg!(DDRB);
reg!(DDRC);
reg!(DDRD);
reg!(PINA);
reg!(PINB);
reg!(PINC);
reg!(PIND);
reg!(PORTA);
reg!(PORTB);
reg!(PORTC);
reg!(PORTD);
