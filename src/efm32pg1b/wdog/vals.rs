#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "ULFRCO."]
    Ulfrco = 0x0,
    #[doc = "LFRCO."]
    Lfrco = 0x01,
    #[doc = "LFXO."]
    Lfxo = 0x02,
    _RESERVED_3 = 0x03,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksel {
    #[inline(always)]
    fn from(val: u8) -> Clksel {
        Clksel::from_bits(val)
    }
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(val: Clksel) -> u8 {
        Clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pch0PrsctrlPrssel {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pch0PrsctrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pch0PrsctrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pch0PrsctrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> Pch0PrsctrlPrssel {
        Pch0PrsctrlPrssel::from_bits(val)
    }
}
impl From<Pch0PrsctrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: Pch0PrsctrlPrssel) -> u8 {
        Pch0PrsctrlPrssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pch1PrsctrlPrssel {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pch1PrsctrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pch1PrsctrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pch1PrsctrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> Pch1PrsctrlPrssel {
        Pch1PrsctrlPrssel::from_bits(val)
    }
}
impl From<Pch1PrsctrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: Pch1PrsctrlPrssel) -> u8 {
        Pch1PrsctrlPrssel::to_bits(val)
    }
}
