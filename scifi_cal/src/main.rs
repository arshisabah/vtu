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
use core::fmt::Write;
use nb::block;
use libm::{sinf, cosf, tanf, sqrtf, powf, logf, log10f, expf};

#[entry]
fn main() -> ! {
    // Initialize peripherals
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    
    // Configure system clock
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    // Configure USART2 pins (PA2=TX, PA3=RX)
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;
    
    // Initialize serial communication
    let serial = Serial::new(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    );
    
    let (mut tx, mut rx) = serial.split();
    let mut buffer: String<64> = String::new();
    
    // Print welcome message
    writeln!(tx, "\r\nSci Calc Ready\r").ok();
    writeln!(tx, "> ").ok();
    
    loop {
        if let Ok(byte) = rx.read() {
            if byte == b'\n' || byte == b'\r' {
                if buffer.len() > 0 {
                    writeln!(tx, "\r").ok();
                    let response = process_calculation(&buffer);
                    writeln!(tx, "Result: {}\r", response).ok();
                    //writeln!(tx, "Ready> ").ok();
                    buffer.clear();
                }
            } else if byte == 8 || byte == 127 { // Backspace
                if buffer.len() > 0 {
                    buffer.pop();
                    write!(tx, "\x08 \x08").ok(); // Erase character
                }
            } else if byte >= 32 && byte < 127 { // Printable ASCII
                if buffer.push(byte as char).is_ok() {
                    block!(tx.write(byte)).ok();
                } else {
                    writeln!(tx, "\r\nBuffer full!\r").ok();
                    buffer.clear();
                }
            }
        }
    }
}

fn process_calculation(input: &str) -> heapless::String<64> {
    let mut result = heapless::String::<64>::new();
    let mut parts = input.trim().split_whitespace();
    
    let operation = match parts.next() {
        Some(op) => op,
        None => {
            write!(result, "Error: No operation specified").ok();
            return result;
        }
    };
    
    match operation {
        // Basic arithmetic operations (two operands)
        "add" | "sub" | "mul" | "div" | "pow" => {
            let a = match parse_number(parts.next()) {
                Some(n) => n,
                None => {
                    write!(result, "Error: Invalid first operand").ok();
                    return result;
                }
            };
            
            let b = match parse_number(parts.next()) {
                Some(n) => n,
                None => {
                    write!(result, "Error: Invalid second operand").ok();
                    return result;
                }
            };
            
            match operation {
                "add" => { write!(result, "{:.4}", a + b).ok(); }
                "sub" => { write!(result, "{:.4}", a - b).ok(); }
                "mul" => { write!(result, "{:.4}", a * b).ok(); }
                "div" => {
                    if b == 0.0 {
                        write!(result, "Error: Division by zero").ok();
                    } else {
                        write!(result, "{:.4}", a / b).ok();
                    }
                }
                "pow" => { write!(result, "{:.4}", powf(a, b)).ok(); }
                _ => unreachable!(),
            }
        }
        
        // Trigonometric functions (one operand)
        "sin" => {
            if let Some(x) = parse_number(parts.next()) {
                write!(result, "{:.4}", sinf(x)).ok();
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        "cos" => {
            if let Some(x) = parse_number(parts.next()) {
                write!(result, "{:.4}", cosf(x)).ok();
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        "tan" => {
            if let Some(x) = parse_number(parts.next()) {
                write!(result, "{:.4}", tanf(x)).ok();
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        // Mathematical functions
        "sqrt" => {
            if let Some(x) = parse_number(parts.next()) {
                if x < 0.0 {
                    write!(result, "Error: Negative sqrt").ok();
                } else {
                    write!(result, "{:.4}", sqrtf(x)).ok();
                }
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        "exp" => {
            if let Some(x) = parse_number(parts.next()) {
                write!(result, "{:.4}", expf(x)).ok();
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        "ln" => {
            if let Some(x) = parse_number(parts.next()) {
                if x <= 0.0 {
                    write!(result, "Error: Log of non-positive").ok();
                } else {
                    write!(result, "{:.4}", logf(x)).ok();
                }
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        "log" => {
            if let Some(x) = parse_number(parts.next()) {
                if x <= 0.0 {
                    write!(result, "Error: Log of non-positive").ok();
                } else {
                    write!(result, "{:.4}", log10f(x)).ok();
                }
            } else {
                write!(result, "Error: Invalid operand").ok();
            }
        }
        
        _ => {
            write!(result, "Error: Unknown operation '{}'", operation).ok();
        }
    }
    
    result
}

fn parse_number(s: Option<&str>) -> Option<f32> {
    s?.parse::<f32>().ok()
}
