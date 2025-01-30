#![no_std]
#![no_main]

use panic_halt as _;

use stm32g0_ll_drivers::{gpio, rcc};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut rcc = rcc::Rcc::take().unwrap();

    rcc.enable_gpio_port_clock(rcc::GPIOPort::GPIOA);

    let pins = gpio::gpioa::Pins::take();
    let mut led = pins.pa5.into_output_push_pull();

    loop {
        led.set_high();
        for _ in 0..10_000 {}
        led.set_low();
        for _ in 0..10_000 {}
    }
}
