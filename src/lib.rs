#![no_std]

#[cfg(feature = "stm32g0b1")]
pub use stm32g0::stm32g0b1 as pac;

#[cfg(feature = "stm32g071")]
pub use stm32g0::stm32g071 as pac;

pub trait Taker<T> {
    fn take(self) -> T;
}

//pub mod adc;
pub mod gpio;
pub mod rcc;
