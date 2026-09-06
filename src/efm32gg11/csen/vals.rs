#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acu {
    #[doc = "Accumulate 1 sample."]
    Acc1 = 0x0,
    #[doc = "Accumulate 2 sample."]
    Acc2 = 0x01,
    #[doc = "Accumulate 4 sample."]
    Acc4 = 0x02,
    #[doc = "Accumulate 8 sample."]
    Acc8 = 0x03,
    #[doc = "Accumulate 16 sample."]
    Acc16 = 0x04,
    #[doc = "Accumulate 32 sample."]
    Acc32 = 0x05,
    #[doc = "Accumulate 64 sample."]
    Acc64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Acu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acu {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acu {
    #[inline(always)]
    fn from(val: u8) -> Acu {
        Acu::from_bits(val)
    }
}
impl From<Acu> for u8 {
    #[inline(always)]
    fn from(val: Acu) -> u8 {
        Acu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    Sgl = 0x0,
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    Scan = 0x01,
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    Contsgl = 0x02,
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    Contscan = 0x03,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crmode {
    #[doc = "10-bit delta modulator."]
    Dm10 = 0x0,
    #[doc = "12-bit delta modulator."]
    Dm12 = 0x01,
    #[doc = "14-bit delta modulator."]
    Dm14 = 0x02,
    #[doc = "16-bit delta modulator."]
    Dm16 = 0x03,
}
impl Crmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmode {
    #[inline(always)]
    fn from(val: u8) -> Crmode {
        Crmode::from_bits(val)
    }
}
impl From<Crmode> for u8 {
    #[inline(always)]
    fn from(val: Crmode) -> u8 {
        Crmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emasample {
    #[doc = "EMA weight (N) is 1."]
    W1 = 0x0,
    #[doc = "EMA weight (N) is 2."]
    W2 = 0x01,
    #[doc = "EMA weight (N) is 4."]
    W4 = 0x02,
    #[doc = "EMA weight (N) is 8."]
    W8 = 0x03,
    #[doc = "EMA weight (N) is 16."]
    W16 = 0x04,
    #[doc = "EMA weight (N) is 32."]
    W32 = 0x05,
    #[doc = "EMA weight (N) is 64."]
    W64 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Emasample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emasample {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emasample {
    #[inline(always)]
    fn from(val: u8) -> Emasample {
        Emasample::from_bits(val)
    }
}
impl From<Emasample> for u8 {
    #[inline(always)]
    fn from(val: Emasample) -> u8 {
        Emasample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input0to7sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input0to7sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input0to7sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input0to7sel {
    #[inline(always)]
    fn from(val: u8) -> Input0to7sel {
        Input0to7sel::from_bits(val)
    }
}
impl From<Input0to7sel> for u8 {
    #[inline(always)]
    fn from(val: Input0to7sel) -> u8 {
        Input0to7sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input16to23sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input16to23sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input16to23sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input16to23sel {
    #[inline(always)]
    fn from(val: u8) -> Input16to23sel {
        Input16to23sel::from_bits(val)
    }
}
impl From<Input16to23sel> for u8 {
    #[inline(always)]
    fn from(val: Input16to23sel) -> u8 {
        Input16to23sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input24to31sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input24to31sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input24to31sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input24to31sel {
    #[inline(always)]
    fn from(val: u8) -> Input24to31sel {
        Input24to31sel::from_bits(val)
    }
}
impl From<Input24to31sel> for u8 {
    #[inline(always)]
    fn from(val: Input24to31sel) -> u8 {
        Input24to31sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input32to39sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input32to39sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input32to39sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input32to39sel {
    #[inline(always)]
    fn from(val: u8) -> Input32to39sel {
        Input32to39sel::from_bits(val)
    }
}
impl From<Input32to39sel> for u8 {
    #[inline(always)]
    fn from(val: Input32to39sel) -> u8 {
        Input32to39sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input40to47sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input40to47sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input40to47sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input40to47sel {
    #[inline(always)]
    fn from(val: u8) -> Input40to47sel {
        Input40to47sel::from_bits(val)
    }
}
impl From<Input40to47sel> for u8 {
    #[inline(always)]
    fn from(val: Input40to47sel) -> u8 {
        Input40to47sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input48to55sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input48to55sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input48to55sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input48to55sel {
    #[inline(always)]
    fn from(val: u8) -> Input48to55sel {
        Input48to55sel::from_bits(val)
    }
}
impl From<Input48to55sel> for u8 {
    #[inline(always)]
    fn from(val: Input48to55sel) -> u8 {
        Input48to55sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input56to63sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input56to63sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input56to63sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input56to63sel {
    #[inline(always)]
    fn from(val: u8) -> Input56to63sel {
        Input56to63sel::from_bits(val)
    }
}
impl From<Input56to63sel> for u8 {
    #[inline(always)]
    fn from(val: Input56to63sel) -> u8 {
        Input56to63sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input8to15sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
}
impl Input8to15sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input8to15sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input8to15sel {
    #[inline(always)]
    fn from(val: u8) -> Input8to15sel {
        Input8to15sel::from_bits(val)
    }
}
impl From<Input8to15sel> for u8 {
    #[inline(always)]
    fn from(val: Input8to15sel) -> u8 {
        Input8to15sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcpresc {
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1."]
    Div1 = 0x0,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2."]
    Div2 = 0x01,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4."]
    Div4 = 0x02,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8."]
    Div8 = 0x03,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16."]
    Div16 = 0x04,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32."]
    Div32 = 0x05,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64."]
    Div64 = 0x06,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128."]
    Div128 = 0x07,
}
impl Pcpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcpresc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcpresc {
    #[inline(always)]
    fn from(val: u8) -> Pcpresc {
        Pcpresc::from_bits(val)
    }
}
impl From<Pcpresc> for u8 {
    #[inline(always)]
    fn from(val: Pcpresc) -> u8 {
        Pcpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarcr {
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    Clk10 = 0x0,
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    Clk12 = 0x01,
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    Clk14 = 0x02,
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    Clk16 = 0x03,
}
impl Sarcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarcr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarcr {
    #[inline(always)]
    fn from(val: u8) -> Sarcr {
        Sarcr::from_bits(val)
    }
}
impl From<Sarcr> for u8 {
    #[inline(always)]
    fn from(val: Sarcr) -> u8 {
        Sarcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Singlesel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    Aport1xch0 = 0x20,
    Aport1ych1 = 0x21,
    Aport1xch2 = 0x22,
    Aport1ych3 = 0x23,
    Aport1xch4 = 0x24,
    Aport1ych5 = 0x25,
    Aport1xch6 = 0x26,
    Aport1ych7 = 0x27,
    Aport1xch8 = 0x28,
    Aport1ych9 = 0x29,
    Aport1xch10 = 0x2a,
    Aport1ych11 = 0x2b,
    Aport1xch12 = 0x2c,
    Aport1ych13 = 0x2d,
    Aport1xch14 = 0x2e,
    Aport1ych15 = 0x2f,
    Aport1xch16 = 0x30,
    Aport1ych17 = 0x31,
    Aport1xch18 = 0x32,
    Aport1ych19 = 0x33,
    Aport1xch20 = 0x34,
    Aport1ych21 = 0x35,
    Aport1xch22 = 0x36,
    Aport1ych23 = 0x37,
    Aport1xch24 = 0x38,
    Aport1ych25 = 0x39,
    Aport1xch26 = 0x3a,
    Aport1ych27 = 0x3b,
    Aport1xch28 = 0x3c,
    Aport1ych29 = 0x3d,
    Aport1xch30 = 0x3e,
    Aport1ych31 = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    Aport3xch0 = 0x60,
    Aport3ych1 = 0x61,
    Aport3xch2 = 0x62,
    Aport3ych3 = 0x63,
    Aport3xch4 = 0x64,
    Aport3ych5 = 0x65,
    Aport3xch6 = 0x66,
    Aport3ych7 = 0x67,
    Aport3xch8 = 0x68,
    Aport3ych9 = 0x69,
    Aport3xch10 = 0x6a,
    Aport3ych11 = 0x6b,
    Aport3xch12 = 0x6c,
    Aport3ych13 = 0x6d,
    Aport3xch14 = 0x6e,
    Aport3ych15 = 0x6f,
    Aport3xch16 = 0x70,
    Aport3ych17 = 0x71,
    Aport3xch18 = 0x72,
    Aport3ych19 = 0x73,
    Aport3xch20 = 0x74,
    Aport3ych21 = 0x75,
    Aport3xch22 = 0x76,
    Aport3ych23 = 0x77,
    Aport3xch24 = 0x78,
    Aport3ych25 = 0x79,
    Aport3xch26 = 0x7a,
    Aport3ych27 = 0x7b,
    Aport3xch28 = 0x7c,
    Aport3ych29 = 0x7d,
    Aport3xch30 = 0x7e,
    Aport3ych31 = 0x7f,
}
impl Singlesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Singlesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Singlesel {
    #[inline(always)]
    fn from(val: u8) -> Singlesel {
        Singlesel::from_bits(val)
    }
}
impl From<Singlesel> for u8 {
    #[inline(always)]
    fn from(val: Singlesel) -> u8 {
        Singlesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stm {
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    Prs = 0x0,
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    Timer = 0x01,
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    Start = 0x02,
    _RESERVED_3 = 0x03,
}
impl Stm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stm {
    #[inline(always)]
    fn from(val: u8) -> Stm {
        Stm::from_bits(val)
    }
}
impl From<Stm> for u8 {
    #[inline(always)]
    fn from(val: Stm) -> u8 {
        Stm::to_bits(val)
    }
}
