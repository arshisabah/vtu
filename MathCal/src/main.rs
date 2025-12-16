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

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
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
    let mut buffer: String<32> = String::new();
    writeln!(tx, "STM32F103 Math Serial\r").ok();
    writeln!(tx, "Send: <op> <num1> <num2>\r").ok();
    writeln!(tx, "Ops: add, sub, mul, div\r").ok();
    loop {
        if let Ok(byte) = rx.read() {
            if byte == b'\n' || byte == b'\r' {
                if buffer.len() > 0 {
                    let response = process_message(&buffer);
                    for b in response.as_bytes() {
                        block!(tx.write(*b)).ok();
                    }
                    block!(tx.write(b'\n')).ok();
                    buffer.clear();
                }
            } else {
                if buffer.push(byte as char).is_err() {
                    buffer.clear();
                }
            }
        }
    }
}

fn process_message(msg: &str) -> heapless::String<32> {
    let mut out = heapless::String::<32>::new();
    let mut parts = msg.trim().split_whitespace();
    let op = parts.next();
    let n1 = parts.next().and_then(|s| s.parse::<i32>().ok());
    let n2 = parts.next().and_then(|s| s.parse::<i32>().ok());
    match (op, n1, n2) {
        (Some("add"), Some(a), Some(b)) => { write!(out, "{}", a + b).ok(); }
        (Some("sub"), Some(a), Some(b)) => { write!(out, "{}", a - b).ok(); }
        (Some("mul"), Some(a), Some(b)) => { write!(out, "{}", a * b).ok(); }
        (Some("div"), Some(a), Some(b)) => {
            if b == 0 {
                write!(out, "Error: Div by 0").ok();
            } else {
                write!(out, "{}", a / b).ok();
            }
        }
        _ => { write!(out, "Invalid input").ok(); }
    }
    out
}
