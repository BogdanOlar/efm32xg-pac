#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oscsel {
    #[doc = "Output is driven low."]
    Disabled = 0x0,
    #[doc = "Select Low Frequency RC Oscillator."]
    Lfrco = 0x01,
    #[doc = "Select Low Frequency Crystal Oscillator."]
    Lfxo = 0x02,
    #[doc = "Select Ultra Low Frequency RC Oscillator."]
    Ulfrco = 0x03,
}
impl Oscsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oscsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oscsel {
    #[inline(always)]
    fn from(val: u8) -> Oscsel {
        Oscsel::from_bits(val)
    }
}
impl From<Oscsel> for u8 {
    #[inline(always)]
    fn from(val: Oscsel) -> u8 {
        Oscsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Presc {
    #[doc = "LF Oscillator frequency undivided."]
    Div1 = 0x0,
    #[doc = "LF Oscillator frequency divided by 2."]
    Div2 = 0x01,
    #[doc = "LF Oscillator frequency divided by 4."]
    Div4 = 0x02,
    #[doc = "LF Oscillator frequency divided by 8."]
    Div8 = 0x03,
    #[doc = "LF Oscillator frequency divided by 16."]
    Div16 = 0x04,
    #[doc = "LF Oscillator frequency divided by 32."]
    Div32 = 0x05,
    #[doc = "LF Oscillator frequency divided by 64."]
    Div64 = 0x06,
    #[doc = "LF Oscillator frequency divided by 128."]
    Div128 = 0x07,
}
impl Presc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Presc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
