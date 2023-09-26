#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{pac::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let periphs = Peripherals::take().unwrap();
    let gpioa = periphs.GPIOA.split();
    let mut pin = gpioa.pa5.into_push_pull_output();

    let rcc = periphs.RCC.constrain();
    // let clocks = rcc.cfgr.use_hse(8_u32.MHz()).freeze();
    let clocks = rcc.cfgr.sysclk(24_u32.MHz()).freeze();
    let mut delay = periphs.TIM1.delay_ms(&clocks);

    // infinite loop
    loop {
        pin.set_high();
        delay.delay_ms(1000_u16);
        pin.set_low();
        delay.delay_ms(1000_u16);
    }
}
