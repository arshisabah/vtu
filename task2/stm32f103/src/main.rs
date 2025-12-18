#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};
use heapless::String;
use nb::block;

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Take ownership of RCC and FLASH
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    
    // Setup clocks - 72MHz for STM32F103
    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .freeze(&mut flash.acr);
    
    // Setup GPIO
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();
    
    // USART1 pins
    // PA9 - TX
    // PA10 - RX
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;
    
    // Setup USART1 - 9600 baud, 8N1
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9600.bps())
            .wordlength_8bits()
            .parity_none()
            .stopbits(stm32f1xx_hal::serial::StopBits::STOP1),
        &clocks,
    );
    
    let (mut tx, mut rx) = serial.split();
    
    // Buffer for received data
    let mut buffer: String<32> = String::new();
    
    loop {
        // Read incoming byte
        if let Ok(byte) = rx.read() {
            // Check for newline or carriage return (end of message)
            if byte == b'\n' || byte == b'\r' {
                if buffer.len() > 0 {
                    // Process the received message
                    let response = process_message(&buffer);
                    
                    // Send response
                    for byte in response.as_bytes() {
                        block!(tx.write(*byte)).ok();
                    }
                    // Send newline
                    block!(tx.write(b'\n')).ok();
                    
                    // Clear buffer for next message
                    buffer.clear();
                }
            } else {
                // Add byte to buffer if there's space
                if buffer.push(byte as char).is_err() {
                    // Buffer full, clear it
                    buffer.clear();
                }
            }
        }
    }
}

fn process_message(msg: &str) -> &str {
    // Trim whitespace
    let trimmed = msg.trim();
    
    match trimmed {
        "hi" => "hello",
        "hello" => "hi",
        _ => "invalid msg",
    }
}