#![no_std]
#![no_main]

use panic_halt as _;

use stm32g0_ll_drivers::{adc, rcc};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut rcc = rcc::Rcc::new().unwrap();
    let mut adc = adc::Adc::new(&mut rcc).unwrap();

    adc.set_clock_mode(adc::ClockMode::Async);

    #[allow(clippy::empty_loop)]
    loop {}
}
