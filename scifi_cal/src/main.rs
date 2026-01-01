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
    writeln!(tx, "Format: expr with (), +, -, *, /, pow\r").ok();
    writeln!(tx, "Functions: sin/cos/tan/sqrt/exp/ln/log/fact\r").ok();
    writeln!(tx, "Precedence: () > pow > */  > +-\r").ok();
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
    let tokens: heapless::Vec<&str, 64> = tokenize(input);
    
    if tokens.is_empty() {
        write!(result, "Error: No input").ok();
        return result;
    }
    
    let mut idx = 0;
    
    match parse_expression(&tokens, &mut idx) {
        Some(value) => {
            if idx < tokens.len() {
                write!(result, "Error: Unexpected token").ok();
            } else {
                write!(result, "{:.4}", value).ok();
            }
        }
        None => {
            write!(result, "Error: Invalid expression").ok();
        }
    }
    
    result
}

// Tokenize input, handling parentheses as separate tokens
fn tokenize(input: &str) -> heapless::Vec<&str, 64> {
    let mut tokens: heapless::Vec<&str, 64> = heapless::Vec::new();
    let mut start = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            if start < i {
                if let Ok(token) = core::str::from_utf8(&bytes[start..i]) {
                    tokens.push(token).ok();
                }
            }
            start = i + 1;
        } else if bytes[i] == b'(' || bytes[i] == b')' {
            if start < i {
                if let Ok(token) = core::str::from_utf8(&bytes[start..i]) {
                    tokens.push(token).ok();
                }
            }
            if let Ok(token) = core::str::from_utf8(&bytes[i..i+1]) {
                tokens.push(token).ok();
            }
            start = i + 1;
        }
        i += 1;
    }
    
    if start < bytes.len() {
        if let Ok(token) = core::str::from_utf8(&bytes[start..]) {
            tokens.push(token).ok();
        }
    }
    
    tokens
}

// Parse expression: handles + and - (lowest precedence)
fn parse_expression(tokens: &heapless::Vec<&str, 64>, idx: &mut usize) -> Option<f32> {
    let mut left = parse_term(tokens, idx)?;
    
    while *idx < tokens.len() {
        let op = tokens[*idx];
        match op {
            "+" | "-" => {
                *idx += 1;
                let right = parse_term(tokens, idx)?;
                left = if op == "+" { left + right } else { left - right };
            }
            _ => break,
        }
    }
    
    Some(left)
}

// Parse term: handles * and / (medium precedence)
fn parse_term(tokens: &heapless::Vec<&str, 64>, idx: &mut usize) -> Option<f32> {
    let mut left = parse_power(tokens, idx)?;
    
    while *idx < tokens.len() {
        let op = tokens[*idx];
        match op {
            "*" | "/" => {
                *idx += 1;
                let right = parse_power(tokens, idx)?;
                if op == "*" {
                    left *= right;
                } else {
                    if right == 0.0 {
                        return None;
                    }
                    left /= right;
                }
            }
            _ => break,
        }
    }
    
    Some(left)
}

// Parse power: handles pow (highest precedence, right-associative)
fn parse_power(tokens: &heapless::Vec<&str, 64>, idx: &mut usize) -> Option<f32> {
    let base = parse_primary(tokens, idx)?;
    
    if *idx < tokens.len() && tokens[*idx] == "pow" {
        *idx += 1;
        let exponent = parse_power(tokens, idx)?; // Right-associative
        Some(powf(base, exponent))
    } else {
        Some(base)
    }
}

// Parse primary: numbers, functions, and parentheses
fn parse_primary(tokens: &heapless::Vec<&str, 64>, idx: &mut usize) -> Option<f32> {
    if *idx >= tokens.len() {
        return None;
    }
    
    let token = tokens[*idx];
    
    // Handle parentheses
    if token == "(" {
        *idx += 1;
        let value = parse_expression(tokens, idx)?;
        if *idx >= tokens.len() || tokens[*idx] != ")" {
            return None; // Missing closing parenthesis
        }
        *idx += 1;
        return Some(value);
    }
    
    *idx += 1;
    
    // Check if it's a scientific function
    match token {
        "sin" | "cos" | "tan" | "sqrt" | "exp" | "ln" | "log" | "fact" => {
            let operand = parse_primary(tokens, idx)?;
            
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
                "fact" => {
                    if operand < 0.0 || operand > 12.0 {
                        None // Factorial only valid for 0-12 (stays within f32 range)
                    } else {
                        Some(factorial(operand as u32))
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

fn factorial(n: u32) -> f32 {
    if n == 0 || n == 1 {
        1.0
    } else {
        let mut result = 1u32;
        for i in 2..=n {
            result = result.saturating_mul(i);
        }
        result as f32
    }
}
