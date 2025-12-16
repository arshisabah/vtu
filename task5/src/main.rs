#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Take ownership of RCC and FLASH
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    
    // Setup clocks - 72MHz for STM32F103
    let clocks = rcc.cfgr.sysclk(72.MHz()).freeze(&mut flash.acr);
    
    // Setup GPIO ports
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    
    // Configure onboard LED (LD2) as output on PA5
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    
    // Configure onboard button (B1) as input on PC13
    // Button is pulled up, pressed = LOW
    let button = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);
    
    // Start with LED off
    led.set_low();
    
    loop {
        // Read button state (B1 on PC13)
        if button.is_low() {
            // Button is pressed
            led.set_high();  // Turn LED ON
        } else {
            // Button is released
            led.set_low(); // Turn LED OFF
        }
        
        // Small delay to debounce switch (optional)
        cortex_m::asm::delay(10_000);
    }
}
