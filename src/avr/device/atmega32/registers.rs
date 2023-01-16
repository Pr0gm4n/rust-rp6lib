//! Describes the registers available on the atmega32.
#![allow(non_camel_case_types)]

use super::{
    super::{
        register::{reg, reg_list},
        Register,
    },
    avr_device,
};

reg_list!(
    /// Analog Comparator Control And Status Register.
    ACSR,
    /// ADC Data Register Bytes.
    ADC,
    /// ADC Data Register Bytes high byte.
    ADCH,
    /// ADC Data Register Bytes low byte.
    ADCL,
    /// The ADC Control and Status register.
    ADCSRA,
    /// The ADC multiplexer Selection Register.
    ADMUX,
    /// Asynchronous Status Register.
    ASSR,
    /// Port A Data Direction Register.
    DDRA,
    /// Port B Data Direction Register.
    DDRB,
    /// Port C Data Direction Register.
    DDRC,
    /// Port D Data Direction Register.
    DDRD,
    /// EEPROM Read/Write Access Bytes.
    EEAR,
    /// EEPROM Read/Write Access Bytes high byte.
    EEARH,
    /// EEPROM Read/Write Access Bytes low byte.
    EEARL,
    /// EEPROM Control Register.
    EECR,
    /// EEPROM Data Register.
    EEDR,
    /// General Interrupt Control Register.
    GICR,
    /// General Interrupt Flag Register.
    GIFR,
    /// HIGH register
    HIGH,
    /// Timer/Counter1 Input Capture Register Bytes.
    ICR1,
    /// Timer/Counter1 Input Capture Register Bytes high byte.
    ICR1H,
    /// Timer/Counter1 Input Capture Register Bytes low byte.
    ICR1L,
    /// LOCKBIT register
    LOCKBIT,
    /// LOW register
    LOW,
    /// MCU Control Register.
    MCUCR,
    /// MCU Control And Status Register.
    MCUCSR,
    /// On-Chip Debug Related Register in I/O Memory.
    OCDR,
    /// Output Compare Register.
    OCR0,
    /// Timer/Counter1 Output Compare Register Bytes.
    OCR1A,
    /// Timer/Counter1 Output Compare Register Bytes high byte.
    OCR1AH,
    /// Timer/Counter1 Output Compare Register Bytes low byte.
    OCR1AL,
    /// Timer/Counter1 Output Compare Register Bytes.
    OCR1B,
    /// Timer/Counter1 Output Compare Register Bytes high byte.
    OCR1BH,
    /// Timer/Counter1 Output Compare Register Bytes low byte.
    OCR1BL,
    /// Timer/Counter2 Output Compare Register.
    OCR2,
    /// Oscillator Calibration Value.
    OSCCAL,
    /// Port A Input Pins.
    PINA,
    /// Port B Input Pins.
    PINB,
    /// Port C Input Pins.
    PINC,
    /// Port D Input Pins.
    PIND,
    /// Port A Data Register.
    PORTA,
    /// Port B Data Register.
    PORTB,
    /// Port C Data Register.
    PORTC,
    /// Port D Data Register.
    PORTD,
    /// Special Function IO Register.
    SFIOR,
    /// Stack Pointer.
    SP,
    /// SPI Control Register.
    SPCR,
    /// SPI Data Register.
    SPDR,
    /// Stack Pointer high byte.
    SPH,
    /// Stack Pointer low byte.
    SPL,
    /// Store Program Memory Control Register.
    SPMCR,
    /// SPI Status Register.
    SPSR,
    // Do not define Status Register for safe access!
    // Status Register.
    //SREG,
    /// Timer/Counter Control Register.
    TCCR0,
    /// Timer/Counter1 Control Register A.
    TCCR1A,
    /// Timer/Counter1 Control Register B.
    TCCR1B,
    /// Timer/Counter2 Control Register.
    TCCR2,
    /// Timer/Counter Register.
    TCNT0,
    /// Timer/Counter1 Bytes.
    TCNT1,
    /// Timer/Counter1 Bytes high byte.
    TCNT1H,
    /// Timer/Counter1 Bytes low byte.
    TCNT1L,
    /// Timer/Counter2.
    TCNT2,
    /// Timer/Counter Interrupt Flag register.
    TIFR,
    /// Timer/Counter Interrupt Mask Register.
    TIMSK,
    /// TWI (Slave) Address register.
    TWAR,
    /// TWI Bit Rate register.
    TWBR,
    /// TWI Control Register.
    TWCR,
    /// TWI Data register.
    TWDR,
    /// TWI Status Register.
    TWSR,
    /// USART Baud Rate Register Hight Byte.
    UBRRH,
    /// USART Baud Rate Register Low Byte.
    UBRRL,
    /// USART Control and Status Register A.
    UCSRA,
    /// USART Control and Status Register B.
    UCSRB,
    /// USART Control and Status Register C.
    UCSRC,
    /// USART I/O Data Register.
    UDR,
    /// Watchdog Timer Control Register.
    WDTCR,
);
