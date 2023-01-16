use super::Serial;
use crate::{avr::registers, interrupt, set_pins, Pin, Register};

/// Module binding pins to their device-specific function names.
pub mod port;
use port::*;

/// Module allowing for simple use of the robot's Anti-Collision System.
mod acs;

/// Struct managing all actions regarding the robot's base.
pub struct RobotBase;

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
            Self::disable_ircomm();
            Self::set_acs_power_off();

            Serial::init();
            /*
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
    pub fn disable_ircomm() {
        IRComm::set_low();
    }

    /// Set the LEDs on the `RobotBase` to the least significant 6 bits of the provided value
    pub fn set_leds(value: u8) {
        // set LEDs SL1-SL3
        set_pins!([Led3, Led2, Led1], value);
        // set LEDs SL4-SL6
        set_pins!([Led6, Led5, Led4], value >> 3);
    }
}
