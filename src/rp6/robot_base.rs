use crate::{avr::registers, interrupt, set_pins, Pin, Register};

// re-export pins with device-specific function names
mod port {
    pub use crate::avr::port::{
        a0 as ADC0, a1 as ADC1, a2 as LS_R, a3 as LS_L, a4 as ExternalInterrupt,
        a5 as Motor_Current_R, a6 as Motor_Current_L, a7 as UBAT, b0 as led6, b1 as led5,
        b2 as ACS, b3 as ACS_PwrH, b4 as PowerOn, b5 as ResetButton, b6 as ACS_L, b7 as led4,
        c0 as SCL, c1 as SDA, c2 as Dir_L, c3 as Dir_R, c4 as led1, c5 as led2, c6 as led3,
        c7 as ACS_R, d0 as RX, d1 as TX, d2 as Enc_L, d3 as Enc_R, d4 as Motor_L, d5 as Motor_R,
        d6 as ACS_Pwr, d7 as IRComm,
    };
}
// import pins with device-specific function names for convenience
use port::*;

/// Struct managing all actions regarding the robot's base
pub struct RobotBase {}

impl RobotBase {
    pub fn init() {
        // Setup port directions and initial values.
        // THIS IS THE MOST IMPORTANT STEP!
        Self::init_ports();

        // Disable global interrupts
        interrupt::without_interrupts(|| {
            // Make sure the Reset Button is enabled!
            // Do not disable it if you want to be able to
            // reset your robot! (Otherwise you can only
            // stop it by switching it off completely,
            // if it gets out of control ;) )
            Self::enable_reset_button();

            // Make sure that IRCOMM and ACS are turned OFF!
            Self::ensure_ircomm_disabled();
            Self::set_acs_pwr_off();

            /*
            // UART:
            UBRRH = UBRR_BAUD_LOW >> 8;	// Setup UART: Baudrate is Low Speed
            UBRRL = (uint8_t) UBRR_BAUD_LOW;
            UCSRA = 0x00;
            UCSRC = (1<<URSEL)|(1<<UCSZ1)|(1<<UCSZ0);
            UCSRB = (1 << TXEN) | (1 << RXEN) | (1 << RXCIE);

            // Initialize ADC:
            ADMUX = 0; //external reference
            ADCSRA = (0<<ADIE) | (0<<ADEN) | (1<<ADPS2) | (1<<ADPS1) | (1<<ADIF);
            SFIOR = 0;

            // Initialize External interrupts:
            MCUCR = (0 << ISC11) | (1 << ISC10) | (0 << ISC01) | (1 << ISC00);
            GICR = (1 << INT2) | (1 << INT1) | (1 << INT0);
            MCUCSR = (0 << ISC2);

            // Initialize Timer 0 -  100µs cycle for Delays/Stopwatches, RC5 reception etc.:
            TCCR0 =   (0 << WGM00) | (1 << WGM01)
                    | (0 << COM00) | (0 << COM01)
                    | (0 << CS02)  | (1 << CS01) | (0 << CS00);
            OCR0  = 99;

            // Initialize Timer1 - PWM:
            // PWM, phase correct with ICR1 as top value.
            TCCR1A = (0 << WGM10) | (1 << WGM11) | (1 << COM1A1) | (1 << COM1B1);
            TCCR1B =  (1 << WGM13) | (0 << WGM12) | (1 << CS10);
            ICR1 = 210; // Phase corret PWM top value - 210 results in
                        // about 19 kHz PWM.
                        // ICR1 is the maximum (=100% duty cycle) PWM value!
                        // This means that the PWM resolution is a bit lower, but
                        // if the frequency is lower than 19 kHz you may hear very
                        // annoying high pitch noises from the motors!
                        // 19 kHz is a bit over the maximum frequency most people can
                        // hear!
                        //
                        // ATTENTION: Max PWM value is 210 and NOT 255 !!!
            OCR1AL = 0;
            OCR1BL = 0;
            setMotorDir(FWD,FWD); 	// Direction Forwards

            // Initialize Timer2 - ACS:
            TCCR2 = (1 << WGM21) | (0 << COM20) | (1 << CS20);
            OCR2  = 0x6E; // 0x6E = 72kHz @8MHz

            // Initialize Timer Interrupts:
            TIMSK = (1 << OCIE0); //| (1 << OCIE2); // Fixed: Timer2 Interrupt is turned
                                  // off by default now! It is only active
                                  // when ACS/IRCOMM are transmitting.

            // Initialize ACS:
            sysStatACS.channel = ACS_CHANNEL_RIGHT;
            acs_state = ACS_STATE_IRCOMM_DELAY;
               */
        }); // Enable Global Interrupts
    }

    /// Initializes the IO ports of the robot.
    pub fn init_ports() {
        // init all ports to 0 (except `pd0` = RXD)
        registers::PORTA::write(0b00000000);
        registers::PORTB::write(0b00000000);
        registers::PORTC::write(0b00000000);
        registers::PORTD::write(0b00000001);
        // init input/output directions
        registers::DDRA::write(0b00000000);
        registers::DDRB::write(0b01011000);
        registers::DDRC::write(0b10001100);
        registers::DDRD::write(0b11110010);
    }

    /// Enable power on the `RobotBase`.
    pub fn power_on() {
        PowerOn::set_high();
    }

    /// Disable power on the `RobotBase`.
    pub fn power_off() {
        PowerOn::set_low();
    }

    /// Enable the hardware reset button on the robot.
    pub fn enable_reset_button() {
        ResetButton::set_low();
        ResetButton::set_input();
    }

    /// Disable the hardware reset button on the robot.
    pub fn disable_reset_button() {
        ResetButton::set_low();
        ResetButton::set_output();
    }

    /// Disable the IRCOMM of the robot.
    pub fn ensure_ircomm_disabled() {
        IRComm::set_low();
    }

    /// Disable the ACS of the robot.
    pub fn set_acs_pwr_off() {
        ACS_Pwr::set_input();
        ACS_Pwr::set_low();
        ACS_PwrH::set_input();
        ACS_PwrH::set_low();
        ACS_L::set_low();
        ACS_R::set_low();
    }

    /// Set the ACS of the robot to low power.
    pub fn set_acs_pwr_low() {
        ACS_Pwr::set_output();
        ACS_Pwr::set_high();
        ACS_PwrH::set_input();
        ACS_PwrH::set_low();
    }

    /// Set the ACS of the robot to medium power.
    pub fn set_acs_pwr_medium() {
        ACS_Pwr::set_input();
        ACS_Pwr::set_low();
        ACS_PwrH::set_output();
        ACS_PwrH::set_high();
    }

    /// Set the ACS of the robot to high power.
    pub fn set_acs_pwr_high() {
        ACS_Pwr::set_output();
        ACS_Pwr::set_high();
        ACS_PwrH::set_output();
        ACS_PwrH::set_high();
    }

    /// Set the LEDs on the `RobotBase` to the least significant 6 bits of the provided value
    pub fn set_leds(value: u8) {
        // set LEDs SL1-SL3
        set_pins!([led3, led2, led1], value);
        // set LEDs SL4-SL6
        set_pins!([led6, led5, led4], value >> 3);
    }
}
