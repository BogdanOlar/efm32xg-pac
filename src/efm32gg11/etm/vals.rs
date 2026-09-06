#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extinpnum {
    #[doc = "Zero inputs presents."]
    Zero = 0x0,
    #[doc = "One inputs presents."]
    One = 0x01,
    #[doc = "Two inputs presents."]
    Two = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Extinpnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extinpnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extinpnum {
    #[inline(always)]
    fn from(val: u8) -> Extinpnum {
        Extinpnum::from_bits(val)
    }
}
impl From<Extinpnum> for u8 {
    #[inline(always)]
    fn from(val: Extinpnum) -> u8 {
        Extinpnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nonsecnoninvdbg {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Non-secure non-invasive debug disable."]
    Disable = 0x02,
    #[doc = "Non-secure non-invasive debug enable."]
    Enable = 0x03,
}
impl Nonsecnoninvdbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nonsecnoninvdbg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nonsecnoninvdbg {
    #[inline(always)]
    fn from(val: u8) -> Nonsecnoninvdbg {
        Nonsecnoninvdbg::from_bits(val)
    }
}
impl From<Nonsecnoninvdbg> for u8 {
    #[inline(always)]
    fn from(val: Nonsecnoninvdbg) -> u8 {
        Nonsecnoninvdbg::to_bits(val)
    }
}
