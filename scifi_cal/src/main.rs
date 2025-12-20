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
    let mut buffer: String<128> = String::new();
    
    // Print welcome message
    writeln!(tx, "\r\nSci Calc Ready\r").ok();
    writeln!(tx, "Format: <num> <op> <num> [<op> <num>]...\r").ok();
    writeln!(tx, "Ops: +, -, *, /, pow\r").ok();
    writeln!(tx, "Scientific: sin/cos/tan/sqrt/exp/ln/log <num>\r").ok();
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
    let tokens: heapless::Vec<&str, 32> = input.split_whitespace().collect();
    
    if tokens.is_empty() {
        write!(result, "Error: No input").ok();
        return result;
    }
    
    let mut idx = 0;
    
    // Parse first value (number or function)
    let mut acc = match parse_value(&tokens, &mut idx) {
        Some(v) => v,
        None => {
            write!(result, "Error: Invalid input").ok();
            return result;
        }
    };
    
    // Process operation-value pairs
    while idx < tokens.len() {
        let op = tokens[idx];
        idx += 1;
        
        if idx >= tokens.len() {
            write!(result, "Error: Missing operand").ok();
            return result;
        }
        
        let num = match parse_value(&tokens, &mut idx) {
            Some(n) => n,
            None => {
                write!(result, "Error: Invalid operand").ok();
                return result;
            }
        };
        
        match op {
            "+" | "add" => acc += num,
            "-" | "sub" => acc -= num,
            "*" | "mul" => acc *= num,
            "/" | "div" => {
                if num == 0.0 {
                    write!(result, "Error: Division by zero").ok();
                    return result;
                }
                acc /= num;
            }
            "pow" => acc = powf(acc, num),
            _ => {
                write!(result, "Error: Unknown operation").ok();
                return result;
            }
        }
    }
    
    write!(result, "{:.4}", acc).ok();
    result
}

// Parse a value which can be a number or a function call
fn parse_value(tokens: &heapless::Vec<&str, 32>, idx: &mut usize) -> Option<f32> {
    if *idx >= tokens.len() {
        return None;
    }
    
    let token = tokens[*idx];
    *idx += 1;
    
    // Check if it's a scientific function
    match token {
        "sin" | "cos" | "tan" | "sqrt" | "exp" | "ln" | "log" => {
            if *idx >= tokens.len() {
                return None;
            }
            let operand = parse_number(Some(tokens[*idx]))?;
            *idx += 1;
            
            match token {
                "sin" => Some(sinf(operand)),
                "cos" => Some(cosf(operand)),
                "tan" => Some(tanf(operand)),
                "sqrt" => {
                    if operand < 0.0 {
                        None
                    } else {
                        Some(sqrtf(operand))
                    }
                }
                "exp" => Some(expf(operand)),
                "ln" => {
                    if operand <= 0.0 {
                        None
                    } else {
                        Some(logf(operand))
                    }
                }
                "log" => {
                    if operand <= 0.0 {
                        None
                    } else {
                        Some(log10f(operand))
                    }
                }
                _ => None,
            }
        }
        _ => {
            // Try to parse as number
            parse_number(Some(token))
        }
    }
}

fn parse_number(s: Option<&str>) -> Option<f32> {
    s?.parse::<f32>().ok()
}
