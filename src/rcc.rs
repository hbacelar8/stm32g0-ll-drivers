use crate::pac;
use core::convert::From;
use core::sync::atomic::{AtomicBool, Ordering};

const HSE_FREQ: u32 = 8_000_000;

static TAKEN: AtomicBool = AtomicBool::new(false);

pub struct Rcc {
    rb: *const pac::rcc::RegisterBlock,
}

impl Rcc {
    pub fn take() -> Option<Self> {
        unsafe {
            if TAKEN.load(Ordering::Relaxed) {
                None
            } else {
                TAKEN.store(true, Ordering::Relaxed);

                Some(Self {
                    rb: &*pac::RCC::ptr(),
                })
            }
        }
    }

    pub fn set_sys_clock_source(&mut self, clk_source: SystemClockSource) {
        unsafe {
            (*self.rb)
                .cfgr()
                .modify(|_, w| w.sw().bits(clk_source.into()));
        }
    }

    pub fn get_sys_clock_source(&self) -> SystemClockSource {
        unsafe { SystemClockSource::from_u8((*self.rb).cfgr().read().sws().bits()).unwrap() }
    }

    pub fn get_sys_clock_freq(&self) -> u32 {
        match self.get_sys_clock_source() {
            SystemClockSource::HSE => HSE_FREQ,
            SystemClockSource::PLL => {
                // getfreqdomain
            }
            _ => {
                // todo
            }
        }
    }

    pub fn set_pll_state(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.pllon().bit(state));
        }
    }

    /// Check if the PLL clock is locked (ready)
    pub fn is_pll_locked(&self) -> bool {
        unsafe { (*self.rb).cr().read().pllrdy().is_locked() }
    }

    pub fn set_hsi48_state(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.hsi48on().bit(state));
        }
    }

    pub fn is_hsi48_ready(&self) -> bool {
        unsafe { (*self.rb).cr().read().hsirdy().is_ready() }
    }

    pub fn set_hse_state(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.hseon().bit(state));
        }
    }

    pub fn is_hse_ready(&self) -> bool {
        unsafe { (*self.rb).cr().read().hserdy().is_ready() }
    }

    pub fn set_hsi_state(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.hsion().bit(state));
        }
    }

    pub fn is_hsi_ready(&self) -> bool {
        unsafe { (*self.rb).cr().read().hsirdy().is_ready() }
    }

    pub fn set_clock_security_system(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.csson().bit(state));
        }
    }

    pub fn bypass_hse_crystal_oscillator(&mut self, state: bool) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.hsebyp().bit(state));
        }
    }

    pub fn set_hsi16_division_factor(&mut self, div: HSI16DivisionFactor) {
        unsafe {
            (*self.rb).cr().modify(|_, w| w.hsidiv().bits(div.into()));
        }
    }

    pub fn enable_peripheral_clock(&mut self, p: Peripheral) {
        match p {
            Peripheral::APB1(p) => unsafe {
                (*self.rb)
                    .apbenr1()
                    .modify(|r, w| w.bits(r.bits() | (1u32 << u8::from(p))));
            },
            Peripheral::APB2(p) => unsafe {
                (*self.rb)
                    .apbenr2()
                    .modify(|r, w| w.bits(r.bits() | (1u32 << u8::from(p))));
            },
        }
    }

    pub fn disable_peripheral_clock(&mut self, p: Peripheral) {
        match p {
            Peripheral::APB1(p) => unsafe {
                (*self.rb)
                    .apbenr1()
                    .modify(|r, w| w.bits(r.bits() & !(1u32 << u8::from(p))));
            },
            Peripheral::APB2(p) => unsafe {
                (*self.rb)
                    .apbenr2()
                    .modify(|r, w| w.bits(r.bits() & !(1u32 << u8::from(p))));
            },
        }
    }

    pub fn enable_gpio_port_clock(&mut self, g: GPIOPort) {
        unsafe {
            (*self.rb)
                .iopenr()
                .modify(|r, w| w.bits(r.bits() | (1u32 << u8::from(g))));
        }
    }

    pub fn disable_gpio_port_clock(&mut self, g: GPIOPort) {
        unsafe {
            (*self.rb)
                .iopenr()
                .modify(|r, w| w.bits(r.bits() & !(1u32 << u8::from(g))));
        }
    }
}

/// System clock sources
pub enum SystemClockSource {
    HSI,
    HSE,
    PLL,
    LSI,
    LSE,
}

impl From<SystemClockSource> for u8 {
    fn from(value: SystemClockSource) -> Self {
        use SystemClockSource::*;
        match value {
            HSI => 0,
            HSE => 1,
            PLL => 2,
            LSI => 3,
            LSE => 4,
        }
    }
}

impl SystemClockSource {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::HSI),
            1 => Some(Self::HSE),
            2 => Some(Self::PLL),
            3 => Some(Self::LSI),
            4 => Some(Self::LSE),
            _ => None,
        }
    }
}

/// HSI16 clock division factor
pub enum HSI16DivisionFactor {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
}

impl From<HSI16DivisionFactor> for u8 {
    fn from(value: HSI16DivisionFactor) -> Self {
        use HSI16DivisionFactor::*;
        match value {
            Div1 => 0,
            Div2 => 1,
            Div4 => 2,
            Div8 => 3,
            Div16 => 4,
            Div32 => 5,
            Div64 => 6,
            Div128 => 7,
        }
    }
}

/// RCC's APB1 and APB2 peripherals
pub enum Peripheral {
    APB1(APB1Peripheral),
    APB2(APB2Peripheral),
}

/// RCC APB1 peripherals
pub enum APB1Peripheral {
    /// Timer 2
    TIM2,
    /// Timer 3
    TIM3,
    /// Timer 4
    TIM4,
    /// Timer 6
    TIM6,
    /// Timer 7
    TIM7,
    /// Low Power UART2
    LPUART2,
    /// USART5
    USART5,
    /// USART6
    USART6,
    /// RTC APB
    RTCAPB,
    /// Window Watchdog
    WWDG,
    /// FDCAN
    FDCAN,
    /// USB
    USB,
    /// SPI2
    SPI2,
    /// SPI3
    SPI3,
    /// CRS
    CRS,
    /// USART2
    USART2,
    /// USART3
    USART3,
    /// USART4
    USART4,
    /// Low Power UART1
    LPUART1,
    /// I2C1
    I2C1,
    /// I2C2
    I2C2,
    /// I2C3
    I2C3,
    /// HDMI CEC
    CEC,
    /// USB-C Power Delivery 1
    UCPD1,
    /// USB-C Power Delivery 2
    UCPD2,
    /// Debug Support
    DBG,
    /// Power Interface
    PWR,
    /// DAC1
    DAC1,
    /// Low Power TIM2
    LPTIM2,
    /// Low Power TIM1
    LPTIM1,
}

impl From<APB1Peripheral> for u8 {
    fn from(value: APB1Peripheral) -> Self {
        use APB1Peripheral::*;
        match value {
            TIM2 => 0,
            TIM3 => 1,
            TIM4 => 2,
            TIM6 => 4,
            TIM7 => 5,
            LPUART2 => 7,
            USART5 => 8,
            USART6 => 9,
            RTCAPB => 10,
            WWDG => 11,
            FDCAN => 12,
            USB => 13,
            SPI2 => 14,
            SPI3 => 15,
            CRS => 16,
            USART2 => 17,
            USART3 => 18,
            USART4 => 19,
            LPUART1 => 20,
            I2C1 => 21,
            I2C2 => 22,
            I2C3 => 23,
            CEC => 24,
            UCPD1 => 25,
            UCPD2 => 26,
            DBG => 27,
            PWR => 28,
            DAC1 => 29,
            LPTIM2 => 30,
            LPTIM1 => 31,
        }
    }
}

/// RCC APB2 Peripherals
pub enum APB2Peripheral {
    SYSCFG,
    TIM1,
    SPI1,
    USART1,
    TIM14,
    TIM15,
    TIM16,
    TIM17,
    ADC,
}

impl From<APB2Peripheral> for u8 {
    fn from(value: APB2Peripheral) -> Self {
        use APB2Peripheral::*;
        match value {
            SYSCFG => 0,
            TIM1 => 11,
            SPI1 => 12,
            USART1 => 14,
            TIM14 => 15,
            TIM15 => 16,
            TIM16 => 17,
            TIM17 => 18,
            ADC => 20,
        }
    }
}

pub enum GPIOPort {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
}

impl From<GPIOPort> for u8 {
    fn from(value: GPIOPort) -> Self {
        use GPIOPort::*;
        match value {
            GPIOA => 0,
            GPIOB => 1,
            GPIOC => 2,
            GPIOD => 3,
            GPIOE => 4,
            GPIOF => 5,
        }
    }
}
