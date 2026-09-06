#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockuprmode {
    #[doc = "Reset request is blocked."]
    Disabled = 0x0,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 0x01,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    Full = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lockuprmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockuprmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockuprmode {
    #[inline(always)]
    fn from(val: u8) -> Lockuprmode {
        Lockuprmode::from_bits(val)
    }
}
impl From<Lockuprmode> for u8 {
    #[inline(always)]
    fn from(val: Lockuprmode) -> u8 {
        Lockuprmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pinrmode {
    #[doc = "Reset request is blocked."]
    Disabled = 0x0,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 0x01,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    Full = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pinrmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinrmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinrmode {
    #[inline(always)]
    fn from(val: u8) -> Pinrmode {
        Pinrmode::from_bits(val)
    }
}
impl From<Pinrmode> for u8 {
    #[inline(always)]
    fn from(val: Pinrmode) -> u8 {
        Pinrmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysrmode {
    #[doc = "Reset request is blocked."]
    Disabled = 0x0,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 0x01,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    Full = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sysrmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysrmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysrmode {
    #[inline(always)]
    fn from(val: u8) -> Sysrmode {
        Sysrmode::from_bits(val)
    }
}
impl From<Sysrmode> for u8 {
    #[inline(always)]
    fn from(val: Sysrmode) -> u8 {
        Sysrmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdogrmode {
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG."]
    Disabled = 0x0,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 0x01,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    Full = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Wdogrmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdogrmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdogrmode {
    #[inline(always)]
    fn from(val: u8) -> Wdogrmode {
        Wdogrmode::from_bits(val)
    }
}
impl From<Wdogrmode> for u8 {
    #[inline(always)]
    fn from(val: Wdogrmode) -> u8 {
        Wdogrmode::to_bits(val)
    }
}
