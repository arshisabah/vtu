#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.sysclk(72.MHz()).freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    // USART2 on PA2 (TX) and PA3 (RX) - Connected to ST-Link VCP
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    let serial = Serial::new(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    );

    let (mut tx, mut rx) = serial.split();

    use core::fmt::Write;

    // Send startup message
    writeln!(tx, "\r\nSTM32F103 Ready - Send 'hi' to test\r\n").ok();

    // Buffer to store received characters
    let mut buffer: [u8; 20] = [0; 20];
    let mut index = 0;

    loop {
        // Check for received data
        if let Ok(byte) = rx.read() {
            match byte {
                b'\r' | b'\n' => {
                    // End of message - process the buffer
                    if index > 0 {
                        // Check if buffer contains exactly "hi"
                        if index == 2 && buffer[0] == b'h' && buffer[1] == b'i' {
                            writeln!(tx, "hello\r").ok();
                            led.set_low(); // Turn LED ON
                        } else {
                            writeln!(tx, "Invalid message\r").ok();
                            led.set_high(); // Turn LED OFF
                        }
                        
                        // Clear buffer for next message
                        index = 0;
                        for i in 0..buffer.len() {
                            buffer[i] = 0;
                        }
                    }
                }
                _ => {
                    // Store character in buffer
                    if index < buffer.len() {
                        buffer[index] = byte;
                        index += 1;
                    } else {
                        // Buffer overflow - clear and report invalid
                        writeln!(tx, "Invalid message (too long)\r").ok();
                        index = 0;
                        for i in 0..buffer.len() {
                            buffer[i] = 0;
                        }
                    }
                }
            }
        }
    }
}