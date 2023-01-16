//! Describes the bitmasks used to configure the atmega32.

/// Re-export bitmasks used to configure the atmega32.
use super::{
    super::register::{bitmask, bitmask_list},
    avr_device,
};

bitmask_list!(
    /// Bitfield on register ACSR
    ACBG,
    /// Bitfield on register ACSR
    ACD,
    /// Bitfield on register ACSR
    ACI,
    /// Bitfield on register ACSR
    ACIC,
    /// Bitfield on register ACSR
    ACIE,
    /// Bitfield on register ACSR
    ACIS,
    /// Bitfield on register ACSR
    ACO,
    /// Bitfield on register ADCSRA
    ADATE,
    /// Bitfield on register ADCSRA
    ADEN,
    /// Bitfield on register ADCSRA
    ADIE,
    /// Bitfield on register ADCSRA
    ADIF,
    /// Bitfield on register ADMUX
    ADLAR,
    /// Bitfield on register ADCSRA
    ADPS,
    /// Bitfield on register ADCSRA
    ADSC,
    /// Bitfield on register ASSR
    AS2,
    /// Bitfield on register LOCKBIT
    BLB0,
    /// Bitfield on register LOCKBIT
    BLB1,
    /// Bitfield on register SPMCR
    BLBSET,
    /// Bitfield on register LOW
    BODEN,
    /// Bitfield on register LOW
    BODLEVEL,
    /// Bitfield on register HIGH
    BOOTRST,
    /// Bitfield on register HIGH
    BOOTSZ,
    /// Bitfield on register MCUCSR
    BORF,
    /// Bitfield on register SREG
    C,
    /// Bitfield on register HIGH
    CKOPT,
    /// Bitfield on register TCCR0
    COM0,
    /// Bitfield on register TCCR1A
    COM1A,
    /// Bitfield on register TCCR1A
    COM1B,
    /// Bitfield on register TCCR2
    COM2,
    /// Bitfield on register SPCR
    CPHA,
    /// Bitfield on register SPCR
    CPOL,
    /// Bitfield on register TCCR0
    CS0,
    /// Bitfield on register TCCR1B
    CS1,
    /// Bitfield on register TCCR2
    CS2,
    /// Bitfield on register UCSRA
    DOR,
    /// Bitfield on register SPCR
    DORD,
    /// Bitfield on register EECR
    EEMWE,
    /// Bitfield on register EECR
    EERE,
    /// Bitfield on register EECR
    EERIE,
    /// Bitfield on register HIGH
    EESAVE,
    /// Bitfield on register EECR
    EEWE,
    /// Bitfield on register MCUCSR
    EXTRF,
    /// Bitfield on register UCSRA
    FE,
    /// Bitfield on register TCCR0
    FOC0,
    /// Bitfield on register TCCR1A
    FOC1A,
    /// Bitfield on register TCCR1A
    FOC1B,
    /// Bitfield on register TCCR2
    FOC2,
    /// Bitfield on register SREG
    H,
    /// Bitfield on register SREG
    I,
    /// Bitfield on register TCCR1B
    ICES1,
    /// Bitfield on register TIFR
    ICF1,
    /// Bitfield on register TCCR1B
    ICNC1,
    /// Bitfield on register GICR
    INT0,
    /// Bitfield on register GICR
    INT1,
    /// Bitfield on register GICR
    INT2,
    /// Bitfield on register GIFR
    INTF,
    /// Bitfield on register GIFR
    INTF2,
    /// Bitfield on register MCUCR
    ISC0,
    /// Bitfield on register MCUCR
    ISC1,
    /// Bitfield on register GICR
    IVCE,
    /// Bitfield on register GICR
    IVSEL,
    /// Bitfield on register HIGH
    JTAGEN,
    /// Bitfield on register MCUCSR
    JTD,
    /// Bitfield on register MCUCSR
    JTRF,
    /// Bitfield on register LOCKBIT
    LB,
    /// Bitfield on register UCSRA
    MPCM,
    /// Bitfield on register SPCR
    MSTR,
    /// Bitfield on register ADMUX
    MUX,
    /// Bitfield on register SREG
    N,
    /// Bitfield on register HIGH
    OCDEN,
    /// Bitfield on register TIFR
    OCF1A,
    /// Bitfield on register TIFR
    OCF1B,
    /// Bitfield on register TIMSK
    OCIE1A,
    /// Bitfield on register TIMSK
    OCIE1B,
    /// Bitfield on register ASSR
    OCR2UB,
    /// Bitfield on register SPMCR
    PGERS,
    /// Bitfield on register SPMCR
    PGWRT,
    /// Bitfield on register MCUCSR
    PORF,
    /// Bitfield on register ADMUX
    REFS,
    /// Bitfield on register SPMCR
    RWWSB,
    /// Bitfield on register SPMCR
    RWWSRE,
    /// Bitfield on register UCSRB
    RXB8,
    /// Bitfield on register UCSRA
    RXC,
    /// Bitfield on register UCSRB
    RXCIE,
    /// Bitfield on register UCSRB
    RXEN,
    /// Bitfield on register SREG
    S,
    /// Bitfield on register MCUCR
    SE,
    /// Bitfield on register MCUCR
    SM,
    /// Bitfield on register SPCR
    SPE,
    /// Bitfield on register SPSR
    SPI2X,
    /// Bitfield on register SPCR
    SPIE,
    /// Bitfield on register HIGH
    SPIEN,
    /// Bitfield on register SPSR
    SPIF,
    /// Bitfield on register SPMCR
    SPMEN,
    /// Bitfield on register SPMCR
    SPMIE,
    /// Bitfield on register SPCR
    SPR,
    /// Bitfield on register LOW
    SUT_CKSEL,
    /// Bitfield on register SREG
    T,
    /// Bitfield on register ASSR
    TCN2UB,
    /// Bitfield on register ASSR
    TCR2UB,
    /// Bitfield on register TIMSK
    TICIE1,
    /// Bitfield on register TIMSK
    TOIE1,
    /// Bitfield on register TIFR
    TOV1,
    /// Bitfield on register TWAR
    TWA,
    /// Bitfield on register TWCR
    TWEA,
    /// Bitfield on register TWCR
    TWEN,
    /// Bitfield on register TWAR
    TWGCE,
    /// Bitfield on register TWCR
    TWIE,
    /// Bitfield on register TWCR
    TWINT,
    /// Bitfield on register TWSR
    TWPS,
    /// Bitfield on register TWSR
    TWS,
    /// Bitfield on register TWCR
    TWSTA,
    /// Bitfield on register TWCR
    TWSTO,
    /// Bitfield on register TWCR
    TWWC,
    /// Bitfield on register UCSRB
    TXB8,
    /// Bitfield on register UCSRA
    TXC,
    /// Bitfield on register UCSRB
    TXCIE,
    /// Bitfield on register UCSRB
    TXEN,
    /// Bitfield on register UCSRA
    U2X,
    /// Bitfield on register UCSRC
    UCPOL,
    /// Bitfield on register UCSRC
    UCSZ,
    /// Bitfield on register UCSRB
    UCSZ2,
    /// Bitfield on register UCSRA
    UDRE,
    /// Bitfield on register UCSRB
    UDRIE,
    /// Bitfield on register UCSRC
    UMSEL,
    /// Bitfield on register UCSRA
    UPE,
    /// Bitfield on register UCSRC
    UPM,
    /// Bitfield on register UCSRC
    URSEL,
    /// Bitfield on register UCSRC
    USBS,
    /// Bitfield on register SREG
    V,
    /// Bitfield on register SPSR
    WCOL,
    /// Bitfield on register WDTCR
    WDE,
    /// Bitfield on register WDTCR
    WDP,
    /// Bitfield on register MCUCSR
    WDRF,
    /// Bitfield on register WDTCR
    WDTOE,
    /// Bitfield on register TCCR0
    WGM00,
    /// Bitfield on register TCCR0
    WGM01,
    /// Bitfield on register TCCR2
    WGM20,
    /// Bitfield on register TCCR2
    WGM21,
    /// Bitfield on register SREG
    Z,
);
