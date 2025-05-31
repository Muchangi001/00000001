#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Set up the system clock to 84 MHz
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    // Set up the LED pin (PC13 on STM32F411CEU6 Black Pill)
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let mut sequence_index = 0;

    loop {
        match sequence_index {
            // Fast blink sequence
            0 => {
                for _ in 0..3 {
                    led.set_low(); // LED ON (inverted logic on most boards)
                    delay.delay_ms(100_u32);
                    led.set_high(); // LED OFF
                    delay.delay_ms(100_u32);
                }
                delay.delay_ms(1000_u32); // Pause between sequences
            },
            
            // Slow blink sequence
            1 => {
                for _ in 0..2 {
                    led.set_low(); // LED ON
                    delay.delay_ms(500_u32);
                    led.set_high(); // LED OFF
                    delay.delay_ms(500_u32);
                }
                delay.delay_ms(1000_u32); // Pause between sequences
            },
            
            // SOS pattern
            2 => {
                // S - three short blinks
                for _ in 0..3 {
                    led.set_low();
                    delay.delay_ms(200_u32);
                    led.set_high();
                    delay.delay_ms(200_u32);
                }
                delay.delay_ms(200_u32);
                
                // O - three long blinks
                for _ in 0..3 {
                    led.set_low();
                    delay.delay_ms(600_u32);
                    led.set_high();
                    delay.delay_ms(200_u32);
                }
                delay.delay_ms(200_u32);
                
                // S - three short blinks
                for _ in 0..3 {
                    led.set_low();
                    delay.delay_ms(200_u32);
                    led.set_high();
                    delay.delay_ms(200_u32);
                }
                delay.delay_ms(2000_u32); // Long pause after SOS
            },
            
            // Breathing pattern
            _ => {
                // Fade in effect (simulated with PWM-like blinking)
                for i in 1..=10 {
                    led.set_low();
                    delay.delay_ms((i * 10) as u32);
                    led.set_high();
                    delay.delay_ms((100 - (i * 10)) as u32);
                }
                // Fade out effect
                for i in (1..=10).rev() {
                    led.set_low();
                    delay.delay_ms((i * 10) as u32);
                    led.set_high();
                    delay.delay_ms((100 - (i * 10)) as u32);
                }
                delay.delay_ms(500_u32);
            }
        }
        
        // Move to next sequence
        sequence_index = (sequence_index + 1) % 4;
    }
}