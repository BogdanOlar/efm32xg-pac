#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adcbiasprog {
    #[doc = "Normal power (use for 1Msps operation)."]
    Normal = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Scaling bias to 1/2."]
    Scale2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Scaling bias to 1/4."]
    Scale4 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Scaling bias to 1/8."]
    Scale8 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Scaling bias to 1/16."]
    Scale16 = 0x0e,
    #[doc = "Scaling bias to 1/32."]
    Scale32 = 0x0f,
}
impl Adcbiasprog {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcbiasprog {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcbiasprog {
    #[inline(always)]
    fn from(val: u8) -> Adcbiasprog {
        Adcbiasprog::from_bits(val)
    }
}
impl From<Adcbiasprog> for u8 {
    #[inline(always)]
    fn from(val: Adcbiasprog) -> u8 {
        Adcbiasprog::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input0negsel {
    #[doc = "Selects ADCn_INPUT1 as negative channel input."]
    Input1 = 0x0,
    #[doc = "Selects ADCn_INPUT3 as negative channel input."]
    Input3 = 0x01,
    #[doc = "Selects ADCn_INPUT5 as negative channel input."]
    Input5 = 0x02,
    #[doc = "Selects ADCn_INPUT7 as negative channel input."]
    Input7 = 0x03,
}
impl Input0negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input0negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input0negsel {
    #[inline(always)]
    fn from(val: u8) -> Input0negsel {
        Input0negsel::from_bits(val)
    }
}
impl From<Input0negsel> for u8 {
    #[inline(always)]
    fn from(val: Input0negsel) -> u8 {
        Input0negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input0to7sel {
    Aport0ch0to7 = 0x0,
    Aport0ch8to15 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    Aport2ch0to7 = 0x08,
    Aport2ch8to15 = 0x09,
    Aport2ch16to23 = 0x0a,
    Aport2ch24to31 = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
    Aport4ch0to7 = 0x10,
    Aport4ch8to15 = 0x11,
    Aport4ch16to23 = 0x12,
    Aport4ch24to31 = 0x13,
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
}
impl Input0to7sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input0to7sel {
        unsafe { core::mem::transmute(val & 0x1f) }
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
pub enum Input11negsel {
    #[doc = "Selects ADCn_INPUT8 as negative channel input."]
    Input8 = 0x0,
    #[doc = "Selects ADCn_INPUT10 as negative channel input."]
    Input10 = 0x01,
    #[doc = "Selects ADCn_INPUT12 as negative channel input."]
    Input12 = 0x02,
    #[doc = "Selects ADCn_INPUT14 as negative channel input."]
    Input14 = 0x03,
}
impl Input11negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input11negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input11negsel {
    #[inline(always)]
    fn from(val: u8) -> Input11negsel {
        Input11negsel::from_bits(val)
    }
}
impl From<Input11negsel> for u8 {
    #[inline(always)]
    fn from(val: Input11negsel) -> u8 {
        Input11negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input13negsel {
    #[doc = "Selects ADCn_INPUT8 as negative channel input."]
    Input8 = 0x0,
    #[doc = "Selects ADCn_INPUT10 as negative channel input."]
    Input10 = 0x01,
    #[doc = "Selects ADCn_INPUT12 as negative channel input."]
    Input12 = 0x02,
    #[doc = "Selects ADCn_INPUT14 as negative channel input."]
    Input14 = 0x03,
}
impl Input13negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input13negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input13negsel {
    #[inline(always)]
    fn from(val: u8) -> Input13negsel {
        Input13negsel::from_bits(val)
    }
}
impl From<Input13negsel> for u8 {
    #[inline(always)]
    fn from(val: Input13negsel) -> u8 {
        Input13negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input15negsel {
    #[doc = "Selects ADCn_INPUT8 as negative channel input."]
    Input8 = 0x0,
    #[doc = "Selects ADCn_INPUT10 as negative channel input."]
    Input10 = 0x01,
    #[doc = "Selects ADCn_INPUT12 as negative channel input."]
    Input12 = 0x02,
    #[doc = "Selects ADCn_INPUT14 as negative channel input."]
    Input14 = 0x03,
}
impl Input15negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input15negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input15negsel {
    #[inline(always)]
    fn from(val: u8) -> Input15negsel {
        Input15negsel::from_bits(val)
    }
}
impl From<Input15negsel> for u8 {
    #[inline(always)]
    fn from(val: Input15negsel) -> u8 {
        Input15negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input16to23sel {
    Aport0ch0to7 = 0x0,
    Aport0ch8to15 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    Aport2ch0to7 = 0x08,
    Aport2ch8to15 = 0x09,
    Aport2ch16to23 = 0x0a,
    Aport2ch24to31 = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
    Aport4ch0to7 = 0x10,
    Aport4ch8to15 = 0x11,
    Aport4ch16to23 = 0x12,
    Aport4ch24to31 = 0x13,
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
}
impl Input16to23sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input16to23sel {
        unsafe { core::mem::transmute(val & 0x1f) }
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
    Aport0ch0to7 = 0x0,
    Aport0ch8to15 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    Aport2ch0to7 = 0x08,
    Aport2ch8to15 = 0x09,
    Aport2ch16to23 = 0x0a,
    Aport2ch24to31 = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
    Aport4ch0to7 = 0x10,
    Aport4ch8to15 = 0x11,
    Aport4ch16to23 = 0x12,
    Aport4ch24to31 = 0x13,
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
}
impl Input24to31sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input24to31sel {
        unsafe { core::mem::transmute(val & 0x1f) }
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
pub enum Input2negsel {
    #[doc = "Selects ADCn_INPUT1 as negative channel input."]
    Input1 = 0x0,
    #[doc = "Selects ADCn_INPUT3 as negative channel input."]
    Input3 = 0x01,
    #[doc = "Selects ADCn_INPUT5 as negative channel input."]
    Input5 = 0x02,
    #[doc = "Selects ADCn_INPUT7 as negative channel input."]
    Input7 = 0x03,
}
impl Input2negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input2negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input2negsel {
    #[inline(always)]
    fn from(val: u8) -> Input2negsel {
        Input2negsel::from_bits(val)
    }
}
impl From<Input2negsel> for u8 {
    #[inline(always)]
    fn from(val: Input2negsel) -> u8 {
        Input2negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input4negsel {
    #[doc = "Selects ADCn_INPUT1 as negative channel input."]
    Input1 = 0x0,
    #[doc = "Selects ADCn_INPUT3 as negative channel input."]
    Input3 = 0x01,
    #[doc = "Selects ADCn_INPUT5 as negative channel input."]
    Input5 = 0x02,
    #[doc = "Selects ADCn_INPUT7 as negative channel input."]
    Input7 = 0x03,
}
impl Input4negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input4negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input4negsel {
    #[inline(always)]
    fn from(val: u8) -> Input4negsel {
        Input4negsel::from_bits(val)
    }
}
impl From<Input4negsel> for u8 {
    #[inline(always)]
    fn from(val: Input4negsel) -> u8 {
        Input4negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input6negsel {
    #[doc = "Selects ADCn_INPUT1 as negative channel input."]
    Input1 = 0x0,
    #[doc = "Selects ADCn_INPUT3 as negative channel input."]
    Input3 = 0x01,
    #[doc = "Selects ADCn_INPUT5 as negative channel input."]
    Input5 = 0x02,
    #[doc = "Selects ADCn_INPUT7 as negative channel input."]
    Input7 = 0x03,
}
impl Input6negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input6negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input6negsel {
    #[inline(always)]
    fn from(val: u8) -> Input6negsel {
        Input6negsel::from_bits(val)
    }
}
impl From<Input6negsel> for u8 {
    #[inline(always)]
    fn from(val: Input6negsel) -> u8 {
        Input6negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input8to15sel {
    Aport0ch0to7 = 0x0,
    Aport0ch8to15 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    Aport1ch0to7 = 0x04,
    Aport1ch8to15 = 0x05,
    Aport1ch16to23 = 0x06,
    Aport1ch24to31 = 0x07,
    Aport2ch0to7 = 0x08,
    Aport2ch8to15 = 0x09,
    Aport2ch16to23 = 0x0a,
    Aport2ch24to31 = 0x0b,
    Aport3ch0to7 = 0x0c,
    Aport3ch8to15 = 0x0d,
    Aport3ch16to23 = 0x0e,
    Aport3ch24to31 = 0x0f,
    Aport4ch0to7 = 0x10,
    Aport4ch8to15 = 0x11,
    Aport4ch16to23 = 0x12,
    Aport4ch24to31 = 0x13,
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
}
impl Input8to15sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input8to15sel {
        unsafe { core::mem::transmute(val & 0x1f) }
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
pub enum Input9negsel {
    #[doc = "Selects ADCn_INPUT8 as negative channel input."]
    Input8 = 0x0,
    #[doc = "Selects ADCn_INPUT10 as negative channel input."]
    Input10 = 0x01,
    #[doc = "Selects ADCn_INPUT12 as negative channel input."]
    Input12 = 0x02,
    #[doc = "Selects ADCn_INPUT14 as negative channel input."]
    Input14 = 0x03,
}
impl Input9negsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input9negsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input9negsel {
    #[inline(always)]
    fn from(val: u8) -> Input9negsel {
        Input9negsel::from_bits(val)
    }
}
impl From<Input9negsel> for u8 {
    #[inline(always)]
    fn from(val: Input9negsel) -> u8 {
        Input9negsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovsrsel {
    #[doc = "2 samples for each conversion result."]
    X2 = 0x0,
    #[doc = "4 samples for each conversion result."]
    X4 = 0x01,
    #[doc = "8 samples for each conversion result."]
    X8 = 0x02,
    #[doc = "16 samples for each conversion result."]
    X16 = 0x03,
    #[doc = "32 samples for each conversion result."]
    X32 = 0x04,
    #[doc = "64 samples for each conversion result."]
    X64 = 0x05,
    #[doc = "128 samples for each conversion result."]
    X128 = 0x06,
    #[doc = "256 samples for each conversion result."]
    X256 = 0x07,
    #[doc = "512 samples for each conversion result."]
    X512 = 0x08,
    #[doc = "1024 samples for each conversion result."]
    X1024 = 0x09,
    #[doc = "2048 samples for each conversion result."]
    X2048 = 0x0a,
    #[doc = "4096 samples for each conversion result."]
    X4096 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ovsrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovsrsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovsrsel {
    #[inline(always)]
    fn from(val: u8) -> Ovsrsel {
        Ovsrsel::from_bits(val)
    }
}
impl From<Ovsrsel> for u8 {
    #[inline(always)]
    fn from(val: Ovsrsel) -> u8 {
        Ovsrsel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Presc(u8);
impl Presc {
    pub const Nodivision: Self = Self(0x0);
}
impl Presc {
    pub const fn from_bits(val: u8) -> Presc {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Presc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Presc {
    #[inline(always)]
    fn from(val: u8) -> Presc {
        Presc::from_bits(val)
    }
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(val: Presc) -> u8 {
        Presc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Progerr {
    _RESERVED_0 = 0x0,
    Busconf = 0x01,
    Negselconf = 0x02,
    _RESERVED_3 = 0x03,
}
impl Progerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Progerr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Progerr {
    #[inline(always)]
    fn from(val: u8) -> Progerr {
        Progerr::from_bits(val)
    }
}
impl From<Progerr> for u8 {
    #[inline(always)]
    fn from(val: Progerr) -> u8 {
        Progerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanctrlAt {
    #[doc = "1 conversion clock cycle acquisition time for scan."]
    _1cycle = 0x0,
    #[doc = "2 conversion clock cycles acquisition time for scan."]
    _2cycles = 0x01,
    #[doc = "3 conversion clock cycles acquisition time for scan."]
    _3cycles = 0x02,
    #[doc = "4 conversion clock cycles acquisition time for scan."]
    _4cycles = 0x03,
    #[doc = "8 conversion clock cycles acquisition time for scan."]
    _8cycles = 0x04,
    #[doc = "16 conversion clock cycles acquisition time for scan."]
    _16cycles = 0x05,
    #[doc = "32 conversion clock cycles acquisition time for scan."]
    _32cycles = 0x06,
    #[doc = "64 conversion clock cycles acquisition time for scan."]
    _64cycles = 0x07,
    #[doc = "128 conversion clock cycles acquisition time for scan."]
    _128cycles = 0x08,
    #[doc = "256 conversion clock cycles acquisition time for scan."]
    _256cycles = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl ScanctrlAt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanctrlAt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanctrlAt {
    #[inline(always)]
    fn from(val: u8) -> ScanctrlAt {
        ScanctrlAt::from_bits(val)
    }
}
impl From<ScanctrlAt> for u8 {
    #[inline(always)]
    fn from(val: ScanctrlAt) -> u8 {
        ScanctrlAt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanctrlRef {
    #[doc = "VFS = 1.25V with internal VBGR reference."]
    _1v25 = 0x0,
    #[doc = "VFS = 2.5V with internal VBGR reference."]
    _2v5 = 0x01,
    #[doc = "VFS = AVDD with AVDD as reference source."]
    Vdd = 0x02,
    #[doc = "VFS = 5V with internal VBGR reference."]
    _5v = 0x03,
    #[doc = "Single ended external reference."]
    Extsingle = 0x04,
    #[doc = "Differential external reference, 2x."]
    _2xextdiff = 0x05,
    #[doc = "VFS=2xAVDD with AVDD as the reference source."]
    _2xvdd = 0x06,
    #[doc = "Use SCANCTRLX to configure reference."]
    Conf = 0x07,
}
impl ScanctrlRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanctrlRef {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanctrlRef {
    #[inline(always)]
    fn from(val: u8) -> ScanctrlRef {
        ScanctrlRef::from_bits(val)
    }
}
impl From<ScanctrlRef> for u8 {
    #[inline(always)]
    fn from(val: ScanctrlRef) -> u8 {
        ScanctrlRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanctrlRes {
    #[doc = "12-bit resolution."]
    _12bit = 0x0,
    #[doc = "8-bit resolution."]
    _8bit = 0x01,
    #[doc = "6-bit resolution."]
    _6bit = 0x02,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    Ovs = 0x03,
}
impl ScanctrlRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanctrlRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanctrlRes {
    #[inline(always)]
    fn from(val: u8) -> ScanctrlRes {
        ScanctrlRes::from_bits(val)
    }
}
impl From<ScanctrlRes> for u8 {
    #[inline(always)]
    fn from(val: ScanctrlRes) -> u8 {
        ScanctrlRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanctrlxVrefsel {
    #[doc = "Internal 0.83V Bandgap reference."]
    Vbgr = 0x0,
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)."]
    Vddxwatt = 0x01,
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)."]
    Vrefpwatt = 0x02,
    #[doc = "Raw single ended external Vref: ADCn_EXTP."]
    Vrefp = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)."]
    Vrefpnwatt = 0x05,
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)."]
    Vrefpn = 0x06,
    #[doc = "Internal Bandgap reference at low setting 0.78V."]
    Vbgrlow = 0x07,
}
impl ScanctrlxVrefsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanctrlxVrefsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanctrlxVrefsel {
    #[inline(always)]
    fn from(val: u8) -> ScanctrlxVrefsel {
        ScanctrlxVrefsel::from_bits(val)
    }
}
impl From<ScanctrlxVrefsel> for u8 {
    #[inline(always)]
    fn from(val: ScanctrlxVrefsel) -> u8 {
        ScanctrlxVrefsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SinglectrlAt {
    #[doc = "1 conversion clock cycle acquisition time for single channel."]
    _1cycle = 0x0,
    #[doc = "2 conversion clock cycles acquisition time for single channel."]
    _2cycles = 0x01,
    #[doc = "3 conversion clock cycles acquisition time for single channel."]
    _3cycles = 0x02,
    #[doc = "4 conversion clock cycles acquisition time for single channel."]
    _4cycles = 0x03,
    #[doc = "8 conversion clock cycles acquisition time for single channel."]
    _8cycles = 0x04,
    #[doc = "16 conversion clock cycles acquisition time for single channel."]
    _16cycles = 0x05,
    #[doc = "32 conversion clock cycles acquisition time for single channel."]
    _32cycles = 0x06,
    #[doc = "64 conversion clock cycles acquisition time for single channel."]
    _64cycles = 0x07,
    #[doc = "128 conversion clock cycles acquisition time for single channel."]
    _128cycles = 0x08,
    #[doc = "256 conversion clock cycles acquisition time for single channel."]
    _256cycles = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SinglectrlAt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SinglectrlAt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SinglectrlAt {
    #[inline(always)]
    fn from(val: u8) -> SinglectrlAt {
        SinglectrlAt::from_bits(val)
    }
}
impl From<SinglectrlAt> for u8 {
    #[inline(always)]
    fn from(val: SinglectrlAt) -> u8 {
        SinglectrlAt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SinglectrlRef {
    #[doc = "VFS = 1.25V with internal VBGR reference."]
    _1v25 = 0x0,
    #[doc = "VFS = 2.5V with internal VBGR reference."]
    _2v5 = 0x01,
    #[doc = "VFS = AVDD with AVDD as reference source."]
    Vdd = 0x02,
    #[doc = "VFS = 5V with internal VBGR reference."]
    _5v = 0x03,
    #[doc = "Single ended external reference."]
    Extsingle = 0x04,
    #[doc = "Differential external reference, 2x."]
    _2xextdiff = 0x05,
    #[doc = "VFS = 2xAVDD with AVDD as the reference source."]
    _2xvdd = 0x06,
    #[doc = "Use SINGLECTRLX to configure reference."]
    Conf = 0x07,
}
impl SinglectrlRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SinglectrlRef {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SinglectrlRef {
    #[inline(always)]
    fn from(val: u8) -> SinglectrlRef {
        SinglectrlRef::from_bits(val)
    }
}
impl From<SinglectrlRef> for u8 {
    #[inline(always)]
    fn from(val: SinglectrlRef) -> u8 {
        SinglectrlRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SinglectrlRes {
    #[doc = "12-bit resolution."]
    _12bit = 0x0,
    #[doc = "8-bit resolution."]
    _8bit = 0x01,
    #[doc = "6-bit resolution."]
    _6bit = 0x02,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    Ovs = 0x03,
}
impl SinglectrlRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SinglectrlRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SinglectrlRes {
    #[inline(always)]
    fn from(val: u8) -> SinglectrlRes {
        SinglectrlRes::from_bits(val)
    }
}
impl From<SinglectrlRes> for u8 {
    #[inline(always)]
    fn from(val: SinglectrlRes) -> u8 {
        SinglectrlRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SinglectrlxVrefsel {
    #[doc = "Internal 0.83V Bandgap reference."]
    Vbgr = 0x0,
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)."]
    Vddxwatt = 0x01,
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)."]
    Vrefpwatt = 0x02,
    #[doc = "Raw single ended external Vref: ADCn_EXTP."]
    Vrefp = 0x03,
    #[doc = "Special mode used to generate ENTROPY."]
    Ventropy = 0x04,
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)."]
    Vrefpnwatt = 0x05,
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)."]
    Vrefpn = 0x06,
    #[doc = "Internal Bandgap reference at low setting 0.78V."]
    Vbgrlow = 0x07,
}
impl SinglectrlxVrefsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SinglectrlxVrefsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SinglectrlxVrefsel {
    #[inline(always)]
    fn from(val: u8) -> SinglectrlxVrefsel {
        SinglectrlxVrefsel::from_bits(val)
    }
}
impl From<SinglectrlxVrefsel> for u8 {
    #[inline(always)]
    fn from(val: SinglectrlxVrefsel) -> u8 {
        SinglectrlxVrefsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Warmupmode {
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    Normal = 0x0,
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    Keepinstandby = 0x01,
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    Keepinslowacc = 0x02,
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    Keepadcwarm = 0x03,
}
impl Warmupmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Warmupmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Warmupmode {
    #[inline(always)]
    fn from(val: u8) -> Warmupmode {
        Warmupmode::from_bits(val)
    }
}
impl From<Warmupmode> for u8 {
    #[inline(always)]
    fn from(val: Warmupmode) -> u8 {
        Warmupmode::to_bits(val)
    }
}
