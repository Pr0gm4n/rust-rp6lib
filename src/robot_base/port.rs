// Re-export pins with bindings to their device-specific function names.
pub use crate::avr::port::{
    // PORTA
    a0 as ADC0,
    a1 as ADC1,
    a2 as LS_R,
    a3 as LS_L,
    a4 as ExternalInterrupt,
    a5 as Motor_Current_R,
    a6 as Motor_Current_L,
    a7 as UBAT,
    // PORTB
    b0 as Led6,
    b1 as Led5,
    b2 as ACS,
    b3 as ACS_PwrH,
    b4 as PowerOn,
    b5 as ResetButton,
    b6 as ACS_L,
    b7 as Led4,
    // PORTC
    c0 as SCL,
    c1 as SDA,
    c2 as Dir_L,
    c3 as Dir_R,
    c4 as Led1,
    c5 as Led2,
    c6 as Led3,
    c7 as ACS_R,
    // PORTD
    d0 as RX,
    d1 as TX,
    d2 as Enc_L,
    d3 as Enc_R,
    d4 as Motor_L,
    d5 as Motor_R,
    d6 as ACS_Pwr,
    d7 as IRComm,
};
