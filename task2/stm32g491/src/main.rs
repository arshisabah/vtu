#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32g4::stm32g491;
use heapless::String;

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = stm32g491::Peripherals::take().unwrap();
    
    // Enable clocks for GPIOA and USART2
    dp.RCC.ahb2enr.modify(|_, w| w.gpioaen().set_bit());
    dp.RCC.apb1enr1.modify(|_, w| w.usart2en().set_bit());
    
    // Configure PA2 as USART2_TX (AF7)
    // Configure PA3 as USART2_RX (AF7)
    dp.GPIOA.moder.modify(|_, w| {
        w.moder2().alternate()
         .moder3().alternate()
    });
    
    dp.GPIOA.otyper.modify(|_, w| {
        w.ot2().push_pull()
    });
    
    dp.GPIOA.ospeedr.modify(|_, w| {
        w.ospeedr2().high_speed()
         .ospeedr3().high_speed()
    });
    
    // Set alternate function 7 (USART2) for PA2 and PA3
    dp.GPIOA.afrl.modify(|_, w| {
        w.afrl2().af7()
         .afrl3().af7()
    });
    
    // Configure USART2
    // Assuming 16 MHz HSI clock
    // Baud rate = 9600
    // BRR = clock / baud = 16000000 / 9600 = 1667 (0x683)
    dp.USART2.brr.write(|w| unsafe { w.bits(1667) });
    
    // Enable USART, transmitter and receiver
    dp.USART2.cr1.modify(|_, w| {
        w.ue().set_bit()    // USART enable
         .te().set_bit()    // Transmitter enable
         .re().set_bit()    // Receiver enable
    });
    
    // Buffer for received data
    let mut buffer: String<32> = String::new();
    
    loop {
        // Check if data is received
        if dp.USART2.isr.read().rxne().bit_is_set() {
            // Read the byte
            let byte = dp.USART2.rdr.read().bits() as u8;
            
            // Check for newline or carriage return (end of message)
            if byte == b'\n' || byte == b'\r' {
                if buffer.len() > 0 {
                    // Process the received message
                    let response = process_message(&buffer);
                    
                    // Send response
                    for &byte in response.as_bytes() {
                        // Wait until transmit data register is empty
                        while dp.USART2.isr.read().txe().bit_is_clear() {}
                        // Write data
                        dp.USART2.tdr.write(|w| unsafe { w.bits(byte as u32) });
                    }
                    
                    // Send newline
                    while dp.USART2.isr.read().txe().bit_is_clear() {}
                    dp.USART2.tdr.write(|w| unsafe { w.bits(b'\n' as u32) });
                    
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