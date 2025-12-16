#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
    timer::Timer,
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.sysclk(72.MHz()).freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut delay = Timer::syst(cp.SYST, &clocks).delay();
    
    // USART1 for M66 GSM Module on PA9 (TX) and PA10 (RX)
    let tx_gsm = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_gsm = gpioa.pa10;

    let serial_gsm = Serial::new(
        dp.USART1,
        (tx_gsm, rx_gsm),
        &mut afio.mapr,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    );

    let (mut tx_gsm, mut rx_gsm) = serial_gsm.split();

    // USART2 for Debug on PA2 (TX) and PA3 (RX) - ST-Link VCP
    let tx_debug = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx_debug = gpioa.pa3;

    let serial_debug = Serial::new(
        dp.USART2,
        (tx_debug, rx_debug),
        &mut afio.mapr,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    );

    let (mut tx_debug, _rx_debug) = serial_debug.split();

    // Send debug startup message
    writeln!(tx_debug, "\r\nSTM32F103 M66 GSM Module Ready\r").ok();
    writeln!(tx_debug, "Initializing M66...\r").ok();
    
    // Wait for M66 to boot up
    delay.delay_ms(3000_u16);
    
    // Initialize M66 with AT commands
    writeln!(tx_gsm, "AT\r\n").ok();
    delay.delay_ms(500_u16);
    
    writeln!(tx_gsm, "ATE0\r\n").ok(); // Disable echo
    delay.delay_ms(500_u16);
    
    writeln!(tx_debug, "M66 Initialized. Send 'hii' via SMS or data connection\r").ok();

    // Buffer to store received characters from M66
    let mut buffer: [u8; 128] = [0; 128];
    let mut index = 0;

    loop {
        // Check for received data from M66
        if let Ok(byte) = rx_gsm.read() {
            // Echo to debug port
            nb::block!(tx_debug.write(byte)).ok();
            
            match byte {
                b'\r' | b'\n' => {
                    // End of message - process the buffer
                    if index > 0 {
                        // Check if buffer contains "hii"
                        if contains_substring(&buffer[..index], b"hii") {
                            writeln!(tx_gsm, "hello\r\n").ok();
                            writeln!(tx_debug, "\r\n>> Received 'hii', sent 'hello'\r").ok();
                            led.set_low(); // Turn LED ON
                        } else if index > 2 { // Only respond to actual messages, not empty lines
                            writeln!(tx_gsm, "invalid msg\r\n").ok();
                            writeln!(tx_debug, "\r\n>> Invalid message received\r").ok();
                            led.set_high(); // Turn LED OFF
                        }
                        
                        // Clear buffer for next message
                        buffer.fill(0);
                        index = 0;
                    }
                }
                _ => {
                    // Store character in buffer
                    if index < buffer.len() {
                        buffer[index] = byte;
                        index += 1;
                    } else {
                        // Buffer overflow - clear and report invalid
                        writeln!(tx_gsm, "invalid msg (too long)\r\n").ok();
                        writeln!(tx_debug, "\r\n>> Message too long\r").ok();
                        buffer.fill(0);
                        index = 0;
                    }
                }
            }
        }
    }
}

// Helper function to check if buffer contains a substring
fn contains_substring(buffer: &[u8], target: &[u8]) -> bool {
    if buffer.len() < target.len() {
        return false;
    }
    
    for i in 0..=buffer.len() - target.len() {
        let mut match_found = true;
        for j in 0..target.len() {
            if buffer[i + j] != target[j] {
                match_found = false;
                break;
            }
        }
        if match_found {
            return true;
        }
    }
    false
}
