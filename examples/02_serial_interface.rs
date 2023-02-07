#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

use rp6::{interrupt::mutex::Mutex, *};

// Shared data: While constants can be accessed safely (since they are never modified, it is
// recommended to wrap your mutable data in an `rp6::interrupt::Mutex`. Note that a `Mutex` can
// only be used inside a `CriticalSection`, e.g., by calling `interrupt::without_interrupts`.
const USART_BUFFER_SIZE: usize = 32;
static USART_BUFFER: Mutex<[u8; USART_BUFFER_SIZE]> = Mutex::new([' ' as u8; USART_BUFFER_SIZE]);
static USART_WRITE_PTR: Mutex<usize> = Mutex::new(0);

#[interrupt]
fn USART_RXC() {
    interrupt::without_interrupts(|cs| {
        let buffer = USART_BUFFER.lock(cs);
        let write_ptr = USART_WRITE_PTR.lock(cs);

        // save newly received byte to the ringbuffer
        buffer.update(|mut b| {
            b[write_ptr.get()] = Serial::read_raw();
            b
        });

        // increment USART write pointer and wrap around if necessary
        write_ptr.update(|x| if x + 1 < USART_BUFFER_SIZE { x + 1 } else { 0 });
    });
}

/// entry point for the embedded rust program
#[entry]
fn main() -> ! {
    RobotBase::init();
    Serial::enable_USART_RXC_interrupt();

    RobotBase::set_leds(0b111111); // turn all LEDs on
    delay_ms(500); // delay 500ms
    RobotBase::set_leds(0b000000); // turn all LEDs off

    // write a text message to the UART:
    Serial::write("\nJust a simple counter program\n\n");

    // define a counting variable:
    let mut counter: u16 = 0;

    // main loop:
    loop {
        println!(
            "Counter:",
            //counter => bin,
            //"(BIN) |",
            //counter => oct,
            //"(OCT) |",
            counter => dec,
            "(DEC) |",
            counter => hex,
            "(HEX)"
        );

        // increment counter
        counter += 1;

        // Note: `USART_BUFFER` can only be accessed from within a `CriticalSection`
        interrupt::without_interrupts(|cs| {
            let buffer = USART_BUFFER.lock(cs);

            print!("Ringbuffer: ");
            for i in 0..USART_BUFFER_SIZE {
                // send each byte stored in the buffer as its raw value
                Serial::write_raw(buffer.get()[i]);
            }
            Serial::new_line();
        });

        // delay 500ms = 0.5s
        delay_ms(500);
    }
}
