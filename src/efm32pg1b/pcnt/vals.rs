#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auxcntev {
    #[doc = "Never counts."]
    None = 0x0,
    #[doc = "Counts up on up-count events."]
    Up = 0x01,
    #[doc = "Counts up on down-count events."]
    Down = 0x02,
    #[doc = "Counts up on both up-count and down-count events."]
    Both = 0x03,
}
impl Auxcntev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auxcntev {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auxcntev {
    #[inline(always)]
    fn from(val: u8) -> Auxcntev {
        Auxcntev::from_bits(val)
    }
}
impl From<Auxcntev> for u8 {
    #[inline(always)]
    fn from(val: Auxcntev) -> u8 {
        Auxcntev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntev {
    #[doc = "Counts up on up-count and down on down-count events."]
    Both = 0x0,
    #[doc = "Only counts up on up-count events."]
    Up = 0x01,
    #[doc = "Only counts down on down-count events."]
    Down = 0x02,
    #[doc = "Never counts."]
    None = 0x03,
}
impl Cntev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntev {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntev {
    #[inline(always)]
    fn from(val: u8) -> Cntev {
        Cntev::from_bits(val)
    }
}
impl From<Cntev> for u8 {
    #[inline(always)]
    fn from(val: Cntev) -> u8 {
        Cntev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "The module is disabled."]
    Disable = 0x0,
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    Ovssingle = 0x01,
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    Extclksingle = 0x02,
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    Extclkquad = 0x03,
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    Ovsquad1x = 0x04,
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    Ovsquad2x = 0x05,
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    Ovsquad4x = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0inloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
    #[doc = "Location 3."]
    Loc3 = 0x03,
    #[doc = "Location 4."]
    Loc4 = 0x04,
    #[doc = "Location 5."]
    Loc5 = 0x05,
    #[doc = "Location 6."]
    Loc6 = 0x06,
    #[doc = "Location 7."]
    Loc7 = 0x07,
    #[doc = "Location 8."]
    Loc8 = 0x08,
    #[doc = "Location 9."]
    Loc9 = 0x09,
    #[doc = "Location 10."]
    Loc10 = 0x0a,
    #[doc = "Location 11."]
    Loc11 = 0x0b,
    #[doc = "Location 12."]
    Loc12 = 0x0c,
    #[doc = "Location 13."]
    Loc13 = 0x0d,
    #[doc = "Location 14."]
    Loc14 = 0x0e,
    #[doc = "Location 15."]
    Loc15 = 0x0f,
    #[doc = "Location 16."]
    Loc16 = 0x10,
    #[doc = "Location 17."]
    Loc17 = 0x11,
    #[doc = "Location 18."]
    Loc18 = 0x12,
    #[doc = "Location 19."]
    Loc19 = 0x13,
    #[doc = "Location 20."]
    Loc20 = 0x14,
    #[doc = "Location 21."]
    Loc21 = 0x15,
    #[doc = "Location 22."]
    Loc22 = 0x16,
    #[doc = "Location 23."]
    Loc23 = 0x17,
    #[doc = "Location 24."]
    Loc24 = 0x18,
    #[doc = "Location 25."]
    Loc25 = 0x19,
    #[doc = "Location 26."]
    Loc26 = 0x1a,
    #[doc = "Location 27."]
    Loc27 = 0x1b,
    #[doc = "Location 28."]
    Loc28 = 0x1c,
    #[doc = "Location 29."]
    Loc29 = 0x1d,
    #[doc = "Location 30."]
    Loc30 = 0x1e,
    #[doc = "Location 31."]
    Loc31 = 0x1f,
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
impl S0inloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0inloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0inloc {
    #[inline(always)]
    fn from(val: u8) -> S0inloc {
        S0inloc::from_bits(val)
    }
}
impl From<S0inloc> for u8 {
    #[inline(always)]
    fn from(val: S0inloc) -> u8 {
        S0inloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0prssel {
    #[doc = "PRS Channel 0 selected."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl S0prssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0prssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0prssel {
    #[inline(always)]
    fn from(val: u8) -> S0prssel {
        S0prssel::from_bits(val)
    }
}
impl From<S0prssel> for u8 {
    #[inline(always)]
    fn from(val: S0prssel) -> u8 {
        S0prssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1inloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
    #[doc = "Location 3."]
    Loc3 = 0x03,
    #[doc = "Location 4."]
    Loc4 = 0x04,
    #[doc = "Location 5."]
    Loc5 = 0x05,
    #[doc = "Location 6."]
    Loc6 = 0x06,
    #[doc = "Location 7."]
    Loc7 = 0x07,
    #[doc = "Location 8."]
    Loc8 = 0x08,
    #[doc = "Location 9."]
    Loc9 = 0x09,
    #[doc = "Location 10."]
    Loc10 = 0x0a,
    #[doc = "Location 11."]
    Loc11 = 0x0b,
    #[doc = "Location 12."]
    Loc12 = 0x0c,
    #[doc = "Location 13."]
    Loc13 = 0x0d,
    #[doc = "Location 14."]
    Loc14 = 0x0e,
    #[doc = "Location 15."]
    Loc15 = 0x0f,
    #[doc = "Location 16."]
    Loc16 = 0x10,
    #[doc = "Location 17."]
    Loc17 = 0x11,
    #[doc = "Location 18."]
    Loc18 = 0x12,
    #[doc = "Location 19."]
    Loc19 = 0x13,
    #[doc = "Location 20."]
    Loc20 = 0x14,
    #[doc = "Location 21."]
    Loc21 = 0x15,
    #[doc = "Location 22."]
    Loc22 = 0x16,
    #[doc = "Location 23."]
    Loc23 = 0x17,
    #[doc = "Location 24."]
    Loc24 = 0x18,
    #[doc = "Location 25."]
    Loc25 = 0x19,
    #[doc = "Location 26."]
    Loc26 = 0x1a,
    #[doc = "Location 27."]
    Loc27 = 0x1b,
    #[doc = "Location 28."]
    Loc28 = 0x1c,
    #[doc = "Location 29."]
    Loc29 = 0x1d,
    #[doc = "Location 30."]
    Loc30 = 0x1e,
    #[doc = "Location 31."]
    Loc31 = 0x1f,
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
impl S1inloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1inloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1inloc {
    #[inline(always)]
    fn from(val: u8) -> S1inloc {
        S1inloc::from_bits(val)
    }
}
impl From<S1inloc> for u8 {
    #[inline(always)]
    fn from(val: S1inloc) -> u8 {
        S1inloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1prssel {
    #[doc = "PRS Channel 0 selected."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl S1prssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1prssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1prssel {
    #[inline(always)]
    fn from(val: u8) -> S1prssel {
        S1prssel::from_bits(val)
    }
}
impl From<S1prssel> for u8 {
    #[inline(always)]
    fn from(val: S1prssel) -> u8 {
        S1prssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcccomp {
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    Ltoe = 0x0,
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    Gtoe = 0x01,
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    Range = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tcccomp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcccomp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcccomp {
    #[inline(always)]
    fn from(val: u8) -> Tcccomp {
        Tcccomp::from_bits(val)
    }
}
impl From<Tcccomp> for u8 {
    #[inline(always)]
    fn from(val: Tcccomp) -> u8 {
        Tcccomp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tccmode {
    #[doc = "Triggered compare and clear not enabled."]
    Disabled = 0x0,
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    Lfa = 0x01,
    #[doc = "Compare and clear performed on positive PRS edges."]
    Prs = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tccmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tccmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tccmode {
    #[inline(always)]
    fn from(val: u8) -> Tccmode {
        Tccmode::from_bits(val)
    }
}
impl From<Tccmode> for u8 {
    #[inline(always)]
    fn from(val: Tccmode) -> u8 {
        Tccmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tccpresc {
    #[doc = "Compare and clear event each LFA cycle."]
    Div1 = 0x0,
    #[doc = "Compare and clear performed on every other LFA cycle."]
    Div2 = 0x01,
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    Div4 = 0x02,
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    Div8 = 0x03,
}
impl Tccpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tccpresc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tccpresc {
    #[inline(always)]
    fn from(val: u8) -> Tccpresc {
        Tccpresc::from_bits(val)
    }
}
impl From<Tccpresc> for u8 {
    #[inline(always)]
    fn from(val: Tccpresc) -> u8 {
        Tccpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tccprssel {
    #[doc = "PRS Channel 0 selected."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tccprssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tccprssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tccprssel {
    #[inline(always)]
    fn from(val: u8) -> Tccprssel {
        Tccprssel::from_bits(val)
    }
}
impl From<Tccprssel> for u8 {
    #[inline(always)]
    fn from(val: Tccprssel) -> u8 {
        Tccprssel::to_bits(val)
    }
}
