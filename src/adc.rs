use crate::{pac, rcc};
use core::convert::From;
use core::sync::atomic::{AtomicBool, Ordering};

static TAKEN: AtomicBool = AtomicBool::new(false);

pub struct Adc<'a> {
    rb: &'a pac::adc::RegisterBlock,
}

impl Adc<'static> {
    pub fn new(rcc: &mut rcc::Rcc<'static>) -> Option<Self> {
        // Enable the ADC peripheral clock
        rcc.enable_peripheral_clock(rcc::Peripheral::APB2(rcc::APB2Peripheral::ADC));

        unsafe {
            if TAKEN.load(Ordering::Relaxed) {
                None
            } else {
                TAKEN.store(true, Ordering::Relaxed);

                Some(Self {
                    rb: &*pac::ADC::ptr(),
                })
            }
        }
    }

    ///  Set ADC clock mode
    pub fn set_clock_mode(&mut self, clock_mode: ClockMode) {
        self.rb
            .cfgr2
            .modify(|_, w| w.ckmode().bits(clock_mode.into()));
    }

    /// Get ADC clock mode
    pub fn get_clock_mode(&mut self) -> Option<ClockMode> {
        ClockMode::from_u8(self.rb.cfgr2.read().ckmode().bits())
    }

    /// Set ADC resolution
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.rb.cfgr1.modify(|_, w| w.res().bits(resolution.into()));
    }

    /// Get ADC resolution
    pub fn get_resolution(&mut self) -> Resolution {
        Resolution::from_u8(self.rb.cfgr1.read().res().bits()).unwrap()
    }

    /// Set ADC data alignment
    pub fn set_data_alignment(&mut self, data_alignment: DataAlignment) {
        self.rb
            .cfgr1
            .modify(|_, w| w.align().bit(data_alignment.into()));
    }

    /// Get ADC data alignment
    pub fn get_data_alignment(&mut self) -> DataAlignment {
        DataAlignment::from_bool(self.rb.cfgr1.read().align().bit())
    }

    /// Set ADC low power mode
    pub fn set_low_power_mode(&mut self, low_power_mode: LowPowerMode) {
        self.rb.cfgr1.modify(|r, w| unsafe {
            w.bits(r.bits() & !((u8::from(low_power_mode) as u32) << 14))
        });
    }

    /// Get ADC low power mode
    pub fn get_low_power_mode(&mut self) -> Option<LowPowerMode> {
        LowPowerMode::from_u8(((self.rb.cfgr1.read().bits() >> 14) & 3u32) as u8)
    }

    /// Set sampling time for a common group
    pub fn set_common_group_sampling_time(
        &mut self,
        common_group: SamplingTimeCommonGroup,
        sampling_time: SamplingTime,
    ) {
        self.rb.smpr.modify(|r, w| unsafe {
            w.bits(r.bits() & !((u8::from(sampling_time) << u8::from(common_group)) as u32))
        });
    }

    /// Get sampling time of a common group
    pub fn get_common_group_sampling_time(
        &mut self,
        common_group: SamplingTimeCommonGroup,
    ) -> SamplingTime {
        SamplingTime::from_u8(((self.rb.smpr.read().bits() >> u8::from(common_group)) & 7u32) as u8)
            .unwrap()
    }

    /// Set sampling time group for a channel
    pub fn set_channel_sampling_time_group(
        &mut self,
        channel: Channel,
        common_group: SamplingTimeCommonGroup,
    ) {
        self.rb.smpr.modify(|r, w| unsafe {
            w.bits(r.bits() & !((bool::from(common_group) as u8) << (u8::from(channel) + 8)) as u32)
        });
    }

    /// Get sampling time group of a channel
    pub fn get_channel_sampling_time_group(&mut self, channel: Channel) -> SamplingTimeCommonGroup {
        SamplingTimeCommonGroup::from_bool(
            ((self.rb.smpr.read().bits() >> (u8::from(channel) + 8)) & 1u32) != 0,
        )
    }
}

/// ADC clock mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockMode {
    /// Synchronous clock mode.
    SyncPclkDiv1,
    /// Synchronous clock mode.
    SyncPclkDiv2,
    /// Synchronous clock mode.
    SyncPclkDiv4,
    /// Asynchronous clock mode.
    Async,
}

impl From<ClockMode> for u8 {
    fn from(value: ClockMode) -> Self {
        use ClockMode::*;
        match value {
            SyncPclkDiv1 => 0,
            SyncPclkDiv2 => 1,
            SyncPclkDiv4 => 2,
            Async => 3,
        }
    }
}

impl ClockMode {
    /// Get clock mode from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        use ClockMode::*;
        match value {
            0 => Some(SyncPclkDiv1),
            1 => Some(SyncPclkDiv2),
            2 => Some(SyncPclkDiv4),
            3 => Some(Async),
            _ => None,
        }
    }
}

/// ADC data resolution
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// 12 bits
    Bits12,
    /// 10 bits
    Bits10,
    /// 8 bits
    Bits8,
    /// 6 bits
    Bits6,
}

impl From<Resolution> for u8 {
    fn from(value: Resolution) -> Self {
        use Resolution::*;
        match value {
            Bits12 => 0,
            Bits10 => 1,
            Bits8 => 2,
            Bits6 => 3,
        }
    }
}

impl Resolution {
    /// Get Resolution from adc::vals::Res
    pub fn from_u8(value: u8) -> Option<Self> {
        use Resolution::*;
        match value {
            0 => Some(Bits12),
            1 => Some(Bits10),
            2 => Some(Bits8),
            3 => Some(Bits6),
            _ => None,
        }
    }
}

/// ADC data alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataAlignment {
    /// Right alignment
    Right,
    /// Left alignment
    Left,
}

impl From<DataAlignment> for bool {
    fn from(value: DataAlignment) -> Self {
        use DataAlignment::*;
        match value {
            Right => false,
            Left => true,
        }
    }
}

impl DataAlignment {
    pub fn from_bool(value: bool) -> DataAlignment {
        use DataAlignment::*;
        match value {
            false => Right,
            true => Left,
        }
    }
}

/// ADC low power mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowPowerMode {
    /// No low power mode activated
    None,
    /// Auto wait mode activated
    AutoWait,
    /// Auto power off mode activated
    AutoPowerOff,
    /// Auto wait and power off modes activated
    AutoWaitAndPowerOff,
}

impl From<LowPowerMode> for u8 {
    fn from(value: LowPowerMode) -> Self {
        use LowPowerMode::*;
        match value {
            None => 0,
            AutoWait => 1,
            AutoPowerOff => 2,
            AutoWaitAndPowerOff => 3,
        }
    }
}

impl LowPowerMode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(LowPowerMode::None),
            1 => Some(LowPowerMode::AutoWait),
            2 => Some(LowPowerMode::AutoPowerOff),
            3 => Some(LowPowerMode::AutoWaitAndPowerOff),
            _ => None,
        }
    }
}

/// ADC sampling time common group
pub enum SamplingTimeCommonGroup {
    /// Sampling time common group 1
    Common1,
    /// Sampling time common group 2
    Common2,
}

impl From<SamplingTimeCommonGroup> for u8 {
    fn from(value: SamplingTimeCommonGroup) -> Self {
        use SamplingTimeCommonGroup::*;
        match value {
            Common1 => 0,
            Common2 => 4,
        }
    }
}

impl From<SamplingTimeCommonGroup> for bool {
    fn from(value: SamplingTimeCommonGroup) -> Self {
        use SamplingTimeCommonGroup::*;
        match value {
            Common1 => false,
            Common2 => true,
        }
    }
}

impl SamplingTimeCommonGroup {
    fn from_bool(value: bool) -> Self {
        use SamplingTimeCommonGroup::*;
        match value {
            false => Common1,
            true => Common2,
        }
    }
}

/// ADC sampling time
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplingTime {
    /// 1.5 cycles sampling time
    T1_5,
    /// 3.5 cycles sampling time
    T3_5,
    /// 7.5 cycles sampling time
    T7_5,
    /// 12.5 cycles sampling time
    T12_5,
    /// 19.5 cycles sampling time
    T19_5,
    /// 39.5 cycles sampling time
    T39_5,
    /// 79.5 cycles sampling time
    T79_5,
    /// 160.5 cycles sampling time
    T160_5,
}

impl From<SamplingTime> for u8 {
    fn from(value: SamplingTime) -> Self {
        use SamplingTime::*;
        match value {
            T1_5 => 0,
            T3_5 => 1,
            T7_5 => 2,
            T12_5 => 3,
            T19_5 => 4,
            T39_5 => 5,
            T79_5 => 6,
            T160_5 => 7,
        }
    }
}

impl SamplingTime {
    pub fn from_u8(value: u8) -> Option<Self> {
        use SamplingTime::*;
        match value {
            0 => Some(T1_5),
            1 => Some(T3_5),
            2 => Some(T7_5),
            3 => Some(T12_5),
            4 => Some(T19_5),
            5 => Some(T39_5),
            6 => Some(T79_5),
            7 => Some(T160_5),
            _ => None,
        }
    }
}

/// ADC channel
pub enum Channel {
    /// ADC channel 0
    C0,
    /// ADC channel 1
    C1,
    /// ADC channel 2
    C2,
    /// ADC channel 3
    C3,
    /// ADC channel 4
    C4,
    /// ADC channel 5
    C5,
    /// ADC channel 6
    C6,
    /// ADC channel 7
    C7,
    /// ADC channel 8
    C8,
    /// ADC channel 9
    C9,
    /// ADC channel 10
    C10,
    /// ADC channel 11
    C11,
    /// ADC channel 12
    C12,
    /// ADC channel 13
    C13,
    /// ADC channel 14
    C14,
    /// ADC channel 15
    C15,
    /// ADC channel 16
    C16,
    /// ADC channel 17
    C17,
    /// ADC channel 18
    C18,
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> u8 {
        use Channel::*;
        match value {
            C0 => 0,
            C1 => 1,
            C2 => 2,
            C3 => 3,
            C4 => 4,
            C5 => 5,
            C6 => 6,
            C7 => 7,
            C8 => 8,
            C9 => 9,
            C10 => 10,
            C11 => 11,
            C12 => 12,
            C13 => 13,
            C14 => 14,
            C15 => 15,
            C16 => 16,
            C17 => 17,
            C18 => 18,
        }
    }
}

impl Channel {
    pub fn from_usize(value: usize) -> Option<Self> {
        use Channel::*;
        match value {
            1 => Some(C0),
            2 => Some(C1),
            3 => Some(C2),
            4 => Some(C3),
            5 => Some(C4),
            6 => Some(C5),
            7 => Some(C6),
            8 => Some(C7),
            9 => Some(C8),
            10 => Some(C9),
            11 => Some(C10),
            12 => Some(C11),
            13 => Some(C12),
            14 => Some(C13),
            15 => Some(C14),
            16 => Some(C15),
            17 => Some(C16),
            18 => Some(C17),
            19 => Some(C18),
            _ => None,
        }
    }
}

/// ADC rank
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegularRank {
    /// ADC regular sequencer rank 1
    R1,
    /// ADC regular sequencer rank 2
    R2,
    /// ADC regular sequencer rank 3
    R3,
    /// ADC regular sequencer rank 4
    R4,
    /// ADC regular sequencer rank 5
    R5,
    /// ADC regular sequencer rank 6
    R6,
    /// ADC regular sequencer rank 7
    R7,
    /// ADC regular sequencer rank 8
    R8,
}

impl From<RegularRank> for u8 {
    fn from(val: RegularRank) -> Self {
        use RegularRank::*;
        match val {
            R1 => 0,
            R2 => 4,
            R3 => 8,
            R4 => 12,
            R5 => 16,
            R6 => 20,
            R7 => 24,
            R8 => 28,
        }
    }
}
