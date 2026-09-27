#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlCmoa {
    #[doc = "No action on compare match."]
    None = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl CcCtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlCmoa {
        CcCtrlCmoa::from_bits(val)
    }
}
impl From<CcCtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlCmoa) -> u8 {
        CcCtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlCofoa {
    #[doc = "No action on counter overflow."]
    None = 0x0,
    #[doc = "Toggle output on counter overflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter overflow."]
    Clear = 0x02,
    #[doc = "Set output on counter overflow."]
    Set = 0x03,
}
impl CcCtrlCofoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlCofoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlCofoa {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlCofoa {
        CcCtrlCofoa::from_bits(val)
    }
}
impl From<CcCtrlCofoa> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlCofoa) -> u8 {
        CcCtrlCofoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlCufoa {
    #[doc = "No action on counter underflow."]
    None = 0x0,
    #[doc = "Toggle output on counter underflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter underflow."]
    Clear = 0x02,
    #[doc = "Set output on counter underflow."]
    Set = 0x03,
}
impl CcCtrlCufoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlCufoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlCufoa {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlCufoa {
        CcCtrlCufoa::from_bits(val)
    }
}
impl From<CcCtrlCufoa> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlCufoa) -> u8 {
        CcCtrlCufoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl CcCtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlIcedge {
        CcCtrlIcedge::from_bits(val)
    }
}
impl From<CcCtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlIcedge) -> u8 {
        CcCtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlIcevctrl {
    #[doc = "PRS output pulse and interrupt flag set on every capture."]
    Everyedge = 0x0,
    #[doc = "PRS output pulse and interrupt flag set on every second capture."]
    Everysecondedge = 0x01,
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)."]
    Rising = 0x02,
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)."]
    Falling = 0x03,
}
impl CcCtrlIcevctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlIcevctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlIcevctrl {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlIcevctrl {
        CcCtrlIcevctrl::from_bits(val)
    }
}
impl From<CcCtrlIcevctrl> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlIcevctrl) -> u8 {
        CcCtrlIcevctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    #[doc = "Pulse-Width Modulation."]
    Pwm = 0x03,
}
impl CcCtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlMode {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlMode {
        CcCtrlMode::from_bits(val)
    }
}
impl From<CcCtrlMode> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlMode) -> u8 {
        CcCtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcLoc {
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
impl CcLoc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcLoc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcLoc {
    #[inline(always)]
    fn from(val: u8) -> CcLoc {
        CcLoc::from_bits(val)
    }
}
impl From<CcLoc> for u8 {
    #[inline(always)]
    fn from(val: CcLoc) -> u8 {
        CcLoc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdti0loc {
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
impl Cdti0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdti0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdti0loc {
    #[inline(always)]
    fn from(val: u8) -> Cdti0loc {
        Cdti0loc::from_bits(val)
    }
}
impl From<Cdti0loc> for u8 {
    #[inline(always)]
    fn from(val: Cdti0loc) -> u8 {
        Cdti0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdti1loc {
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
impl Cdti1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdti1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdti1loc {
    #[inline(always)]
    fn from(val: u8) -> Cdti1loc {
        Cdti1loc::from_bits(val)
    }
}
impl From<Cdti1loc> for u8 {
    #[inline(always)]
    fn from(val: Cdti1loc) -> u8 {
        Cdti1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdti2loc {
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
impl Cdti2loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdti2loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdti2loc {
    #[inline(always)]
    fn from(val: u8) -> Cdti2loc {
        Cdti2loc::from_bits(val)
    }
}
impl From<Cdti2loc> for u8 {
    #[inline(always)]
    fn from(val: Cdti2loc) -> u8 {
        Cdti2loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "Prescaled HFPERCLK."]
    Preschfperclk = 0x0,
    #[doc = "Compare/Capture Channel 1 Input."]
    Cc1 = 0x01,
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer."]
    Timerouf = 0x02,
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
pub enum CtrlMode {
    #[doc = "Up-count mode."]
    Up = 0x0,
    #[doc = "Down-count mode."]
    Down = 0x01,
    #[doc = "Up/down-count mode."]
    Updown = 0x02,
    #[doc = "Quadrature decoder mode."]
    Qdec = 0x03,
}
impl CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> CtrlMode {
        CtrlMode::from_bits(val)
    }
}
impl From<CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: CtrlMode) -> u8 {
        CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtfa {
    #[doc = "No action on fault."]
    None = 0x0,
    #[doc = "Set outputs inactive."]
    Inactive = 0x01,
    #[doc = "Clear outputs."]
    Clear = 0x02,
    #[doc = "Tristate outputs."]
    Tristate = 0x03,
}
impl Dtfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtfa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtfa {
    #[inline(always)]
    fn from(val: u8) -> Dtfa {
        Dtfa::from_bits(val)
    }
}
impl From<Dtfa> for u8 {
    #[inline(always)]
    fn from(val: Dtfa) -> u8 {
        Dtfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtpresc {
    #[doc = "The HFPERCLK is undivided."]
    Div1 = 0x0,
    #[doc = "The HFPERCLK is divided by 2."]
    Div2 = 0x01,
    #[doc = "The HFPERCLK is divided by 4."]
    Div4 = 0x02,
    #[doc = "The HFPERCLK is divided by 8."]
    Div8 = 0x03,
    #[doc = "The HFPERCLK is divided by 16."]
    Div16 = 0x04,
    #[doc = "The HFPERCLK is divided by 32."]
    Div32 = 0x05,
    #[doc = "The HFPERCLK is divided by 64."]
    Div64 = 0x06,
    #[doc = "The HFPERCLK is divided by 128."]
    Div128 = 0x07,
    #[doc = "The HFPERCLK is divided by 256."]
    Div256 = 0x08,
    #[doc = "The HFPERCLK is divided by 512."]
    Div512 = 0x09,
    #[doc = "The HFPERCLK is divided by 1024."]
    Div1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dtpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtpresc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtpresc {
    #[inline(always)]
    fn from(val: u8) -> Dtpresc {
        Dtpresc::from_bits(val)
    }
}
impl From<Dtpresc> for u8 {
    #[inline(always)]
    fn from(val: Dtpresc) -> u8 {
        Dtpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Falla {
    #[doc = "No action."]
    None = 0x0,
    #[doc = "Start counter without reload."]
    Start = 0x01,
    #[doc = "Stop counter without reload."]
    Stop = 0x02,
    #[doc = "Reload and start counter."]
    Reloadstart = 0x03,
}
impl Falla {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Falla {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Falla {
    #[inline(always)]
    fn from(val: u8) -> Falla {
        Falla::from_bits(val)
    }
}
impl From<Falla> for u8 {
    #[inline(always)]
    fn from(val: Falla) -> u8 {
        Falla::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Presc {
    #[doc = "The HFPERCLK is undivided."]
    Div1 = 0x0,
    #[doc = "The HFPERCLK is divided by 2."]
    Div2 = 0x01,
    #[doc = "The HFPERCLK is divided by 4."]
    Div4 = 0x02,
    #[doc = "The HFPERCLK is divided by 8."]
    Div8 = 0x03,
    #[doc = "The HFPERCLK is divided by 16."]
    Div16 = 0x04,
    #[doc = "The HFPERCLK is divided by 32."]
    Div32 = 0x05,
    #[doc = "The HFPERCLK is divided by 64."]
    Div64 = 0x06,
    #[doc = "The HFPERCLK is divided by 128."]
    Div128 = 0x07,
    #[doc = "The HFPERCLK is divided by 256."]
    Div256 = 0x08,
    #[doc = "The HFPERCLK is divided by 512."]
    Div512 = 0x09,
    #[doc = "The HFPERCLK is divided by 1024."]
    Div1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Presc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Presc {
        unsafe { core::mem::transmute(val & 0x0f) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Risea {
    #[doc = "No action."]
    None = 0x0,
    #[doc = "Start counter without reload."]
    Start = 0x01,
    #[doc = "Stop counter without reload."]
    Stop = 0x02,
    #[doc = "Reload and start counter."]
    Reloadstart = 0x03,
}
impl Risea {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Risea {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Risea {
    #[inline(always)]
    fn from(val: u8) -> Risea {
        Risea::from_bits(val)
    }
}
impl From<Risea> for u8 {
    #[inline(always)]
    fn from(val: Risea) -> u8 {
        Risea::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timerlockkey(u16);
impl Timerlockkey {
    pub const Unlocked: Self = Self(0x0);
    pub const Locked: Self = Self(0x01);
}
impl Timerlockkey {
    pub const fn from_bits(val: u16) -> Timerlockkey {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Timerlockkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Unlocked"),
            0x01 => f.write_str("Locked"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timerlockkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Unlocked"),
            0x01 => defmt::write!(f, "Locked"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Timerlockkey {
    #[inline(always)]
    fn from(val: u16) -> Timerlockkey {
        Timerlockkey::from_bits(val)
    }
}
impl From<Timerlockkey> for u16 {
    #[inline(always)]
    fn from(val: Timerlockkey) -> u16 {
        Timerlockkey::to_bits(val)
    }
}
