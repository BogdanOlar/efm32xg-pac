#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autocmden {
    #[doc = "Auto CMD Disabled."]
    Acmddisabled = 0x0,
    #[doc = "Auto CMD12 Enable."]
    Acmd12en = 0x01,
    #[doc = "Auto CMD23 Enable."]
    Acmd23en = 0x02,
    _RESERVED_3 = 0x03,
}
impl Autocmden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autocmden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autocmden {
    #[inline(always)]
    fn from(val: u8) -> Autocmden {
        Autocmden::from_bits(val)
    }
}
impl From<Autocmden> for u8 {
    #[inline(always)]
    fn from(val: Autocmden) -> u8 {
        Autocmden::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blkscntforcurrtfr(u16);
impl Blkscntforcurrtfr {
    pub const Stopcnt: Self = Self(0x0);
}
impl Blkscntforcurrtfr {
    pub const fn from_bits(val: u16) -> Blkscntforcurrtfr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Blkscntforcurrtfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Stopcnt"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blkscntforcurrtfr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Stopcnt"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Blkscntforcurrtfr {
    #[inline(always)]
    fn from(val: u16) -> Blkscntforcurrtfr {
        Blkscntforcurrtfr::from_bits(val)
    }
}
impl From<Blkscntforcurrtfr> for u16 {
    #[inline(always)]
    fn from(val: Blkscntforcurrtfr) -> u16 {
        Blkscntforcurrtfr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
    #[doc = "Location 3."]
    Loc3 = 0x03,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Cdloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdloc {
    #[inline(always)]
    fn from(val: u8) -> Cdloc {
        Cdloc::from_bits(val)
    }
}
impl From<Cdloc> for u8 {
    #[inline(always)]
    fn from(val: Cdloc) -> u8 {
        Cdloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Clkloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkloc {
    #[inline(always)]
    fn from(val: u8) -> Clkloc {
        Clkloc::from_bits(val)
    }
}
impl From<Clkloc> for u8 {
    #[inline(always)]
    fn from(val: Clkloc) -> u8 {
        Clkloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Cmdloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdloc {
    #[inline(always)]
    fn from(val: u8) -> Cmdloc {
        Cmdloc::from_bits(val)
    }
}
impl From<Cmdloc> for u8 {
    #[inline(always)]
    fn from(val: Cmdloc) -> u8 {
        Cmdloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdtype {
    #[doc = "Normal Command."]
    Normal = 0x0,
    #[doc = "Suspend command."]
    Suspend = 0x01,
    #[doc = "Resume command."]
    Resume = 0x02,
    #[doc = "Abort command."]
    Abort = 0x03,
}
impl Cmdtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdtype {
    #[inline(always)]
    fn from(val: u8) -> Cmdtype {
        Cmdtype::from_bits(val)
    }
}
impl From<Cmdtype> for u8 {
    #[inline(always)]
    fn from(val: Cmdtype) -> u8 {
        Cmdtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Datloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datloc {
    #[inline(always)]
    fn from(val: u8) -> Datloc {
        Datloc::from_bits(val)
    }
}
impl From<Datloc> for u8 {
    #[inline(always)]
    fn from(val: Datloc) -> u8 {
        Datloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ddr50drvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Ddr50drvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddr50drvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddr50drvstval {
    #[inline(always)]
    fn from(val: u8) -> Ddr50drvstval {
        Ddr50drvstval::from_bits(val)
    }
}
impl From<Ddr50drvstval> for u8 {
    #[inline(always)]
    fn from(val: Ddr50drvstval) -> u8 {
        Ddr50drvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmasel {
    #[doc = "SDMA selected."]
    Sdma = 0x0,
    #[doc = "32-bit ADMA1 selected."]
    Adma1 = 0x01,
    #[doc = "32-bit ADMA2 selected."]
    Adma2 = 0x02,
    #[doc = "64-bit ADMA2 selected."]
    _64bitadma2 = 0x03,
}
impl Dmasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmasel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmasel {
    #[inline(always)]
    fn from(val: u8) -> Dmasel {
        Dmasel::from_bits(val)
    }
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(val: Dmasel) -> u8 {
        Dmasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drvstnsel {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Drvstnsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drvstnsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drvstnsel {
    #[inline(always)]
    fn from(val: u8) -> Drvstnsel {
        Drvstnsel::from_bits(val)
    }
}
impl From<Drvstnsel> for u8 {
    #[inline(always)]
    fn from(val: Drvstnsel) -> u8 {
        Drvstnsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dspdrvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Dspdrvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dspdrvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dspdrvstval {
    #[inline(always)]
    fn from(val: u8) -> Dspdrvstval {
        Dspdrvstval::from_bits(val)
    }
}
impl From<Dspdrvstval> for u8 {
    #[inline(always)]
    fn from(val: Dspdrvstval) -> u8 {
        Dspdrvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hspdrvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Hspdrvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hspdrvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hspdrvstval {
    #[inline(always)]
    fn from(val: u8) -> Hspdrvstval {
        Hspdrvstval::from_bits(val)
    }
}
impl From<Hspdrvstval> for u8 {
    #[inline(always)]
    fn from(val: Hspdrvstval) -> u8 {
        Hspdrvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hstsdmabufsize {
    #[doc = "4KB(Detects A11 Carry out)."]
    Size4 = 0x0,
    #[doc = "8KB(Detects A12 Carry out)."]
    Size8 = 0x01,
    #[doc = "16KB(Detects A13 Carry out)."]
    Size16 = 0x02,
    #[doc = "32KB(Detects A14 Carry out)."]
    Size32 = 0x03,
    #[doc = "64KB(Detects A15 Carry out)."]
    Size64 = 0x04,
    #[doc = "128KB(Detects A16 Carry out)."]
    Size128 = 0x05,
    #[doc = "256KB(Detects A17 Carry out)."]
    Size256 = 0x06,
    #[doc = "512KB(Detects A18 Carry out)."]
    Size512 = 0x07,
}
impl Hstsdmabufsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hstsdmabufsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hstsdmabufsize {
    #[inline(always)]
    fn from(val: u8) -> Hstsdmabufsize {
        Hstsdmabufsize::from_bits(val)
    }
}
impl From<Hstsdmabufsize> for u8 {
    #[inline(always)]
    fn from(val: Hstsdmabufsize) -> u8 {
        Hstsdmabufsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ifslottype {
    #[doc = "Removable Card Slot."]
    Removable = 0x0,
    #[doc = "Only one non-removable device is conected to a SD bus slot."]
    Embedded = 0x01,
    #[doc = "Can be set if Host controller supports Shared Bus CTRL register."]
    Shared = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ifslottype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ifslottype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ifslottype {
    #[inline(always)]
    fn from(val: u8) -> Ifslottype {
        Ifslottype::from_bits(val)
    }
}
impl From<Ifslottype> for u8 {
    #[inline(always)]
    fn from(val: Ifslottype) -> u8 {
        Ifslottype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Initdrvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Initdrvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Initdrvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Initdrvstval {
    #[inline(always)]
    fn from(val: u8) -> Initdrvstval {
        Initdrvstval::from_bits(val)
    }
}
impl From<Initdrvstval> for u8 {
    #[inline(always)]
    fn from(val: Initdrvstval) -> u8 {
        Initdrvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maxblklen {
    #[doc = "512 Bytes are Selected."]
    _512b = 0x0,
    #[doc = "1024 Bytes are Selected."]
    _1024b = 0x01,
    #[doc = "2048 Bytes are Selected."]
    _2048b = 0x02,
    _RESERVED_3 = 0x03,
}
impl Maxblklen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maxblklen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maxblklen {
    #[inline(always)]
    fn from(val: u8) -> Maxblklen {
        Maxblklen::from_bits(val)
    }
}
impl From<Maxblklen> for u8 {
    #[inline(always)]
    fn from(val: Maxblklen) -> u8 {
        Maxblklen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resptypesel {
    #[doc = "No RESP."]
    Noresp = 0x0,
    #[doc = "RESP Length 136."]
    Resp136 = 0x01,
    #[doc = "RESP Length 48."]
    Resp48 = 0x02,
    #[doc = "RESP Length 48 check busy after RESP."]
    Busyaftresp = 0x03,
}
impl Resptypesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resptypesel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resptypesel {
    #[inline(always)]
    fn from(val: u8) -> Resptypesel {
        Resptypesel::from_bits(val)
    }
}
impl From<Resptypesel> for u8 {
    #[inline(always)]
    fn from(val: Resptypesel) -> u8 {
        Resptypesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdbusvoltsel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select 1.8V."]
    _1p8v = 0x05,
    #[doc = "Select 3.0V."]
    _3p0v = 0x06,
    #[doc = "Select 3.3V."]
    _3p3v = 0x07,
}
impl Sdbusvoltsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdbusvoltsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdbusvoltsel {
    #[inline(always)]
    fn from(val: u8) -> Sdbusvoltsel {
        Sdbusvoltsel::from_bits(val)
    }
}
impl From<Sdbusvoltsel> for u8 {
    #[inline(always)]
    fn from(val: Sdbusvoltsel) -> u8 {
        Sdbusvoltsel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sdclkfreqsel(u8);
impl Sdclkfreqsel {
    pub const Nodivision: Self = Self(0x0);
}
impl Sdclkfreqsel {
    pub const fn from_bits(val: u8) -> Sdclkfreqsel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Sdclkfreqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdclkfreqsel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Sdclkfreqsel {
    #[inline(always)]
    fn from(val: u8) -> Sdclkfreqsel {
        Sdclkfreqsel::from_bits(val)
    }
}
impl From<Sdclkfreqsel> for u8 {
    #[inline(always)]
    fn from(val: Sdclkfreqsel) -> u8 {
        Sdclkfreqsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdr104drvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Sdr104drvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdr104drvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdr104drvstval {
    #[inline(always)]
    fn from(val: u8) -> Sdr104drvstval {
        Sdr104drvstval::from_bits(val)
    }
}
impl From<Sdr104drvstval> for u8 {
    #[inline(always)]
    fn from(val: Sdr104drvstval) -> u8 {
        Sdr104drvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdr12drvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Sdr12drvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdr12drvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdr12drvstval {
    #[inline(always)]
    fn from(val: u8) -> Sdr12drvstval {
        Sdr12drvstval::from_bits(val)
    }
}
impl From<Sdr12drvstval> for u8 {
    #[inline(always)]
    fn from(val: Sdr12drvstval) -> u8 {
        Sdr12drvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdr25drvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Sdr25drvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdr25drvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdr25drvstval {
    #[inline(always)]
    fn from(val: u8) -> Sdr25drvstval {
        Sdr25drvstval::from_bits(val)
    }
}
impl From<Sdr25drvstval> for u8 {
    #[inline(always)]
    fn from(val: Sdr25drvstval) -> u8 {
        Sdr25drvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdr50drvstval {
    #[doc = "Driver Type B is selected (Default)."]
    Typeb = 0x0,
    #[doc = "Driver Type A is selected."]
    Typea = 0x01,
    #[doc = "Driver Type C is selected."]
    Typec = 0x02,
    #[doc = "Driver Type D is selected."]
    Typed = 0x03,
}
impl Sdr50drvstval {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdr50drvstval {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdr50drvstval {
    #[inline(always)]
    fn from(val: u8) -> Sdr50drvstval {
        Sdr50drvstval::from_bits(val)
    }
}
impl From<Sdr50drvstval> for u8 {
    #[inline(always)]
    fn from(val: Sdr50drvstval) -> u8 {
        Sdr50drvstval::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slottype {
    #[doc = "Removable SD Card Slot."]
    Rmsdslot = 0x0,
    #[doc = "Embedded SD Card Slot."]
    Emsdslot = 0x01,
    #[doc = "Shared SD Card Slot."]
    Shbusslot = 0x02,
    _RESERVED_3 = 0x03,
}
impl Slottype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slottype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slottype {
    #[inline(always)]
    fn from(val: u8) -> Slottype {
        Slottype::from_bits(val)
    }
}
impl From<Slottype> for u8 {
    #[inline(always)]
    fn from(val: Slottype) -> u8 {
        Slottype::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tfrblksize(u16);
impl Tfrblksize {
    pub const Noxfer: Self = Self(0x0);
}
impl Tfrblksize {
    pub const fn from_bits(val: u16) -> Tfrblksize {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tfrblksize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Noxfer"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfrblksize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Noxfer"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tfrblksize {
    #[inline(always)]
    fn from(val: u16) -> Tfrblksize {
        Tfrblksize::from_bits(val)
    }
}
impl From<Tfrblksize> for u16 {
    #[inline(always)]
    fn from(val: Tfrblksize) -> u16 {
        Tfrblksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uhsmodesel {
    #[doc = "SDR12."]
    Sdr12 = 0x0,
    #[doc = "SDR25."]
    Sdr25 = 0x01,
    #[doc = "SDR50."]
    Sdr50 = 0x02,
    #[doc = "SDR104."]
    Sdr104 = 0x03,
    #[doc = "DDR50."]
    Ddr50 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Uhsmodesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uhsmodesel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uhsmodesel {
    #[inline(always)]
    fn from(val: u8) -> Uhsmodesel {
        Uhsmodesel::from_bits(val)
    }
}
impl From<Uhsmodesel> for u8 {
    #[inline(always)]
    fn from(val: Uhsmodesel) -> u8 {
        Uhsmodesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wploc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
    #[doc = "Location 3."]
    Loc3 = 0x03,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Wploc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wploc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wploc {
    #[inline(always)]
    fn from(val: u8) -> Wploc {
        Wploc::from_bits(val)
    }
}
impl From<Wploc> for u8 {
    #[inline(always)]
    fn from(val: Wploc) -> u8 {
        Wploc::to_bits(val)
    }
}
