#![no_std]
#![no_main]
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};
#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Take ownership of the RCC (Reset and Clock Control) registers
    let rcc = dp.RCC.constrain();
    
    // Configure the clock
    let clocks = rcc.cfgr.freeze();
    
    // Get access to GPIOC
    let gpioc = dp.GPIOC.split();
    
    // Configure PC13 as output (this is the LED pin on many STM32 boards)
    let mut led = gpioc.pc13.into_push_pull_output();
    
    // Create a delay abstraction based on SysTick
    let mut delay = dp.TIM2.delay_ms(&clocks);
    
    loop {
        // Turn the LED on
        led.set_low();
        // Wait for 500 ms
        delay.delay_ms(500_u32);
        
        // Turn the LED off
        led.set_high();
        // Wait for 500 ms
        delay.delay_ms(500_u32);
    }
}
 
