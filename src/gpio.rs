use core::marker::PhantomData;

/// Default mode (reset state)
type DefaultMode = Analog;

/// Analog mode (type state)
pub struct Analog;

/// Alternate Function mode (type state)
pub struct AlternateFunction;

/// Output Push Pull mode (type state)
pub struct PushPull;

/// Floating mode (type state)
pub struct Floating;

/// Pull Up mode (type state)
pub struct PullUp;

/// Pull Down mode (type state)
pub struct PullDown;

/// Opendrain mode (type state)
pub struct OpenDrain;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

pub struct Pin<PORT, MODE = DefaultMode> {
    pin: u8,
    _port: PhantomData<PORT>,
    _mode: PhantomData<MODE>,
}

/// GPIO alternate functions
pub enum AlternateFunctionList {
    /// Alternate function 0
    AF0,
    /// Alternate function 1
    AF1,
    /// Alternate function 2
    AF2,
    /// Alternate function 3
    AF3,
    /// Alternate function 4
    AF4,
    /// Alternate function 5
    AF5,
    /// Alternate function 6
    AF6,
    /// Alternate function 7
    AF7,
}

impl From<AlternateFunctionList> for u8 {
    fn from(value: AlternateFunctionList) -> Self {
        use AlternateFunctionList::*;
        match value {
            AF0 => 0,
            AF1 => 1,
            AF2 => 2,
            AF3 => 3,
            AF4 => 4,
            AF5 => 5,
            AF6 => 6,
            AF7 => 7,
        }
    }
}

macro_rules! gpio {
    ($gpiox:ident, $GPIOX:ident, [$(($pxi:ident, $i:expr),)+]) => {
        pub mod $gpiox {
            use crate::pac::$GPIOX;
            use super::*;

            pub struct Pins<$GPIOX> {
                $(pub $pxi: Pin<$GPIOX>,)+
            }

            impl Pins<$GPIOX> {
                /// Takes all pins from this port
                ///
                /// # Example
                ///
                /// ```rust
                /// use stm32g0_ll_drivers::{gpio, rcc};
                ///
                /// // Take rcc peripheral and enable GPIOA clock
                /// let mut rcc = rcc::Rcc::take().unwrap();
                /// rcc.enable_gpio_port_clock(rcc::GPIOPort::GPIOA);
                ///
                /// // Take all GPIOA pins
                /// let pins = gpio::gpioa::Pins::take();
                ///
                /// // Take pin 5, convert it into output push-pull and set it set_high
                /// let mut led = pins.pa5.into_output_push_pull();
                /// led.set_high();
                /// ```
                pub fn take() -> Self {
                    Self {
                        $($pxi: Pin {
                            pin: $i,
                            _port: PhantomData,
                            _mode: PhantomData,
                        },)+
                    }
                }
            }

            impl<MODE> Pin<$GPIOX, MODE> {
                /// Configure the pin as analog
                pub fn into_analog(self) -> Pin<$GPIOX, Analog> {
                    unsafe {
                        (*$GPIOX::ptr()).moder().modify(|_, w| w.moder(self.pin).analog());
                    };

                    Pin {
                        pin: self.pin,
                        _port: PhantomData,
                        _mode: PhantomData,
                    }
                }

                /// Configure the pin as output push-pull
                pub fn into_output_push_pull(self) -> Pin<$GPIOX, Output<PushPull>> {
                    unsafe {
                        (*$GPIOX::ptr()).moder().modify(|_, w| w.moder(self.pin).output());
                        (*$GPIOX::ptr())
                            .otyper()
                            .modify(|_, w| w.ot(self.pin).push_pull());
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).floating());
                    };

                    Pin {
                        pin: self.pin,
                        _port: PhantomData,
                        _mode: PhantomData,
                    }
                }

                /// Configure the pin as output opendrain
                pub fn into_output_open_drain(self) -> Pin<$GPIOX, Output<OpenDrain>> {
                    unsafe {
                        (*$GPIOX::ptr()).moder().modify(|_, w| w.moder(self.pin).output());
                        (*$GPIOX::ptr())
                            .otyper()
                            .modify(|_, w| w.ot(self.pin).open_drain());
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).floating());
                    };

                    Pin {
                        pin: self.pin,
                        _port: PhantomData,
                        _mode: PhantomData,
                    }
                }

                /// Configure the pin as input
                pub fn into_input(self) -> Pin<$GPIOX, Input<Floating>> {
                    unsafe {
                        (*$GPIOX::ptr()).moder().modify(|_, w| w.moder(self.pin).input());
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).floating());
                    };

                    Pin {
                        pin: self.pin,
                        _port: PhantomData,
                        _mode: PhantomData,
                    }
                }

                pub fn into_alternate_function(self, function: AlternateFunctionList) -> Pin<$GPIOX, AlternateFunction> {
                    if self.pin < 8 {
                        unsafe {
                            (*$GPIOX::ptr()).afrl().modify(|_, w| w.afr(self.pin).bits(function.into()));
                        };
                    }
                    else {
                        unsafe {
                            (*$GPIOX::ptr()).afrh().modify(|_, w| w.afr(self.pin).bits(function.into()));
                        };
                    }

                    Pin {
                        pin: self.pin,
                        _port: PhantomData,
                        _mode: PhantomData,
                    }
                }
            }

            impl<MODE> Pin<$GPIOX, Output<MODE>> {
                /// Set the output pin
                pub fn set_high(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr()).bsrr().write(|w| w.bs(self.pin).set_bit());
                    }
                }

                /// Clear the output pin
                pub fn set_low(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr()).bsrr().write(|w| w.br(self.pin).set_bit());
                    }
                }

                /// Configure the output pin as pulled up
                pub fn pull_up(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).pull_up());
                    }
                }

                /// Configure the output pin as pulled down
                pub fn pull_down(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).pull_down());
                    }
                }

                /// Configure the output pin as floating
                pub fn floating(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).floating());
                    }
                }
            }

            impl<MODE> Pin<$GPIOX, Input<MODE>> {
                /// Configure the input pin as pulled up
                pub fn pull_up(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).pull_up());
                    }
                }

                /// Configure the input pin as pulled down
                pub fn pull_down(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).pull_down());
                    }
                }

                /// Configure the input pin as floating
                pub fn floating(&mut self) {
                    unsafe {
                        (*$GPIOX::ptr())
                            .pupdr()
                            .modify(|_, w| w.pupdr(self.pin).floating());
                    }
                }
            }
        }
    }
}

gpio!(
    gpioa,
    GPIOA,
    [
        (pa0, 0),
        (pa1, 1),
        (pa2, 2),
        (pa3, 3),
        (pa4, 4),
        (pa5, 5),
        (pa6, 6),
        (pa7, 7),
        (pa8, 8),
        (pa9, 9),
        (pa10, 10),
        (pa11, 11),
        (pa12, 12),
        (pa13, 13),
        (pa14, 14),
        (pa15, 15),
    ]
);

gpio!(
    gpiob,
    GPIOB,
    [
        (pb0, 0),
        (pb1, 1),
        (pb2, 2),
        (pb3, 3),
        (pb4, 4),
        (pb5, 5),
        (pb6, 6),
        (pb7, 7),
        (pb8, 8),
        (pb9, 9),
        (pb10, 10),
        (pb11, 11),
        (pb12, 12),
        (pb13, 13),
        (pb14, 14),
        (pb15, 15),
    ]
);

gpio!(
    gpioc,
    GPIOC,
    [
        (pc0, 0),
        (pc1, 1),
        (pc2, 2),
        (pc3, 3),
        (pc4, 4),
        (pc5, 5),
        (pc6, 6),
        (pc7, 7),
        (pc8, 8),
        (pc9, 9),
        (pc10, 10),
        (pc11, 11),
        (pc12, 12),
        (pc13, 13),
        (pc14, 14),
        (pc15, 15),
    ]
);

gpio!(
    gpiod,
    GPIOD,
    [
        (pd0, 0),
        (pd1, 1),
        (pd2, 2),
        (pd3, 3),
        (pd4, 4),
        (pd5, 5),
        (pd6, 6),
        (pd7, 7),
        (pd8, 8),
        (pd9, 9),
        (pd10, 10),
        (pd11, 11),
        (pd12, 12),
        (pd13, 13),
        (pd14, 14),
        (pd15, 15),
    ]
);

gpio!(
    gpioe,
    GPIOE,
    [
        (pe0, 0),
        (pe1, 1),
        (pe2, 2),
        (pe3, 3),
        (pe4, 4),
        (pe5, 5),
        (pe6, 6),
        (pe7, 7),
        (pe8, 8),
        (pe9, 9),
        (pe10, 10),
        (pe11, 11),
        (pe12, 12),
        (pe13, 13),
        (pe14, 14),
        (pe15, 15),
    ]
);

gpio!(
    gpiof,
    GPIOF,
    [
        (pf0, 0),
        (pf1, 1),
        (pf2, 2),
        (pf3, 3),
        (pf4, 4),
        (pf5, 5),
        (pf6, 6),
        (pf7, 7),
        (pf8, 8),
        (pf9, 9),
        (pf10, 10),
        (pf11, 11),
        (pf12, 12),
        (pf13, 13),
        (pf14, 14),
        (pf15, 15),
    ]
);
