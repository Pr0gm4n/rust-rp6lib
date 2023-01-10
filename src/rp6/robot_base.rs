/// Struct managing all actions regarding the robot's base
pub struct RobotBase {}

use crate::avr::interrupt::without_interrupts;
use avrd::atmega32::*;

impl RobotBase {
    pub fn init() {
        // Setup port directions and initial values.
        // THIS IS THE MOST IMPORTANT STEP!
        Self::init_ports();

        // Disable global interrupts
        without_interrupts(|| {
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
        unsafe {
            *PORTA = 0b00000000;
            *PORTB = 0b00000000;
            *PORTC = 0b00000000;
            *PORTD = 0b00000001;
            *DDRA = 0b00000000;
            *DDRB = 0b01011000;
            *DDRC = 0b10001100;
            *DDRD = 0b11110010;
        }
    }

    /// Enable the hardware reset button on the robot.
    pub fn enable_reset_button() {
        unsafe {
            *PORTB &= !(1 << 5);
            *DDRB |= 1 << 5;
        }
    }

    /// Disable the hardware reset button on the robot.
    pub fn disable_reset_button() {
        unsafe {
            *PORTB &= !(1 << 5);
            *DDRB &= !(1 << 5);
        }
    }

    /// Disable the IRCOMM of the robot.
    pub fn ensure_ircomm_disabled() {
        unsafe { *PORTD &= !(1 << 7) }
    }

    /// Disable the ACS of the robot.
    pub fn set_acs_pwr_off() {
        unsafe {
            *DDRD &= !(1 << 6);
            *PORTD &= !(1 << 6);
            *DDRB &= !(1 << 3);
            *PORTB &= !(1 << 3);
            *PORTB &= !(1 << 6);
            *PORTC &= !(1 << 7);
        }
    }

    /// Set the LEDs on the `RobotBase` to the least significant 6 bits of the provided value
    pub fn set_leds(value: u8) {
        unsafe {
            // reset LEDs 1-3
            *DDRC &= 0b10001111;
            *PORTC &= 0b10001111;

            // set LEDs 1-3
            *DDRC |= (value << 4) & 0b01110000;
            *PORTC |= (value << 4) & 0b01110000;

            // reset LEDs 4-6
            *DDRB &= 0b01111100;
            *PORTB &= 0b01111100;

            // set LED 4: PB7
            let led4 = (value >> 3) & 1;
            *DDRB |= led4 << 7;
            *PORTB |= led4 << 7;

            // set LED 5: PB1
            let led5 = (value >> 4) & 1;
            *DDRB |= led5 << 1;
            *PORTB |= led5 << 1;

            // set LED 6: PB0
            let led6 = (value >> 5) & 1;
            *DDRB |= led6;
            *PORTB |= led6;
        }
    }
}
