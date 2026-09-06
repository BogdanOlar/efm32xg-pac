#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlCmoa {
    #[doc = "No action on compare match."]
    None = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl Cc0CtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlCmoa {
        Cc0CtrlCmoa::from_bits(val)
    }
}
impl From<Cc0CtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlCmoa) -> u8 {
        Cc0CtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlCofoa {
    #[doc = "No action on counter overflow."]
    None = 0x0,
    #[doc = "Toggle output on counter overflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter overflow."]
    Clear = 0x02,
    #[doc = "Set output on counter overflow."]
    Set = 0x03,
}
impl Cc0CtrlCofoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlCofoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlCofoa {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlCofoa {
        Cc0CtrlCofoa::from_bits(val)
    }
}
impl From<Cc0CtrlCofoa> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlCofoa) -> u8 {
        Cc0CtrlCofoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlCufoa {
    #[doc = "No action on counter underflow."]
    None = 0x0,
    #[doc = "Toggle output on counter underflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter underflow."]
    Clear = 0x02,
    #[doc = "Set output on counter underflow."]
    Set = 0x03,
}
impl Cc0CtrlCufoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlCufoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlCufoa {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlCufoa {
        Cc0CtrlCufoa::from_bits(val)
    }
}
impl From<Cc0CtrlCufoa> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlCufoa) -> u8 {
        Cc0CtrlCufoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl Cc0CtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlIcedge {
        Cc0CtrlIcedge::from_bits(val)
    }
}
impl From<Cc0CtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlIcedge) -> u8 {
        Cc0CtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlIcevctrl {
    #[doc = "PRS output pulse and interrupt flag set on every capture."]
    Everyedge = 0x0,
    #[doc = "PRS output pulse and interrupt flag set on every second capture."]
    Everysecondedge = 0x01,
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)."]
    Rising = 0x02,
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)."]
    Falling = 0x03,
}
impl Cc0CtrlIcevctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlIcevctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlIcevctrl {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlIcevctrl {
        Cc0CtrlIcevctrl::from_bits(val)
    }
}
impl From<Cc0CtrlIcevctrl> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlIcevctrl) -> u8 {
        Cc0CtrlIcevctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    #[doc = "Pulse-Width Modulation."]
    Pwm = 0x03,
}
impl Cc0CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlMode {
        Cc0CtrlMode::from_bits(val)
    }
}
impl From<Cc0CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlMode) -> u8 {
        Cc0CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0loc {
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
impl Cc0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0loc {
    #[inline(always)]
    fn from(val: u8) -> Cc0loc {
        Cc0loc::from_bits(val)
    }
}
impl From<Cc0loc> for u8 {
    #[inline(always)]
    fn from(val: Cc0loc) -> u8 {
        Cc0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlCmoa {
    #[doc = "No action on compare match."]
    None = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl Cc1CtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlCmoa {
        Cc1CtrlCmoa::from_bits(val)
    }
}
impl From<Cc1CtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlCmoa) -> u8 {
        Cc1CtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlCofoa {
    #[doc = "No action on counter overflow."]
    None = 0x0,
    #[doc = "Toggle output on counter overflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter overflow."]
    Clear = 0x02,
    #[doc = "Set output on counter overflow."]
    Set = 0x03,
}
impl Cc1CtrlCofoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlCofoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlCofoa {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlCofoa {
        Cc1CtrlCofoa::from_bits(val)
    }
}
impl From<Cc1CtrlCofoa> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlCofoa) -> u8 {
        Cc1CtrlCofoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlCufoa {
    #[doc = "No action on counter underflow."]
    None = 0x0,
    #[doc = "Toggle output on counter underflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter underflow."]
    Clear = 0x02,
    #[doc = "Set output on counter underflow."]
    Set = 0x03,
}
impl Cc1CtrlCufoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlCufoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlCufoa {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlCufoa {
        Cc1CtrlCufoa::from_bits(val)
    }
}
impl From<Cc1CtrlCufoa> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlCufoa) -> u8 {
        Cc1CtrlCufoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl Cc1CtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlIcedge {
        Cc1CtrlIcedge::from_bits(val)
    }
}
impl From<Cc1CtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlIcedge) -> u8 {
        Cc1CtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlIcevctrl {
    #[doc = "PRS output pulse and interrupt flag set on every capture."]
    Everyedge = 0x0,
    #[doc = "PRS output pulse and interrupt flag set on every second capture."]
    Everysecondedge = 0x01,
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)."]
    Rising = 0x02,
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)."]
    Falling = 0x03,
}
impl Cc1CtrlIcevctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlIcevctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlIcevctrl {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlIcevctrl {
        Cc1CtrlIcevctrl::from_bits(val)
    }
}
impl From<Cc1CtrlIcevctrl> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlIcevctrl) -> u8 {
        Cc1CtrlIcevctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    #[doc = "Pulse-Width Modulation."]
    Pwm = 0x03,
}
impl Cc1CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlMode {
        Cc1CtrlMode::from_bits(val)
    }
}
impl From<Cc1CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlMode) -> u8 {
        Cc1CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1loc {
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
impl Cc1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1loc {
    #[inline(always)]
    fn from(val: u8) -> Cc1loc {
        Cc1loc::from_bits(val)
    }
}
impl From<Cc1loc> for u8 {
    #[inline(always)]
    fn from(val: Cc1loc) -> u8 {
        Cc1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlCmoa {
    #[doc = "No action on compare match."]
    None = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl Cc2CtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlCmoa {
        Cc2CtrlCmoa::from_bits(val)
    }
}
impl From<Cc2CtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlCmoa) -> u8 {
        Cc2CtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlCofoa {
    #[doc = "No action on counter overflow."]
    None = 0x0,
    #[doc = "Toggle output on counter overflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter overflow."]
    Clear = 0x02,
    #[doc = "Set output on counter overflow."]
    Set = 0x03,
}
impl Cc2CtrlCofoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlCofoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlCofoa {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlCofoa {
        Cc2CtrlCofoa::from_bits(val)
    }
}
impl From<Cc2CtrlCofoa> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlCofoa) -> u8 {
        Cc2CtrlCofoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlCufoa {
    #[doc = "No action on counter underflow."]
    None = 0x0,
    #[doc = "Toggle output on counter underflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter underflow."]
    Clear = 0x02,
    #[doc = "Set output on counter underflow."]
    Set = 0x03,
}
impl Cc2CtrlCufoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlCufoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlCufoa {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlCufoa {
        Cc2CtrlCufoa::from_bits(val)
    }
}
impl From<Cc2CtrlCufoa> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlCufoa) -> u8 {
        Cc2CtrlCufoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl Cc2CtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlIcedge {
        Cc2CtrlIcedge::from_bits(val)
    }
}
impl From<Cc2CtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlIcedge) -> u8 {
        Cc2CtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlIcevctrl {
    #[doc = "PRS output pulse and interrupt flag set on every capture."]
    Everyedge = 0x0,
    #[doc = "PRS output pulse and interrupt flag set on every second capture."]
    Everysecondedge = 0x01,
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)."]
    Rising = 0x02,
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)."]
    Falling = 0x03,
}
impl Cc2CtrlIcevctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlIcevctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlIcevctrl {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlIcevctrl {
        Cc2CtrlIcevctrl::from_bits(val)
    }
}
impl From<Cc2CtrlIcevctrl> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlIcevctrl) -> u8 {
        Cc2CtrlIcevctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    #[doc = "Pulse-Width Modulation."]
    Pwm = 0x03,
}
impl Cc2CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlMode {
        Cc2CtrlMode::from_bits(val)
    }
}
impl From<Cc2CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlMode) -> u8 {
        Cc2CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2loc {
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
impl Cc2loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2loc {
    #[inline(always)]
    fn from(val: u8) -> Cc2loc {
        Cc2loc::from_bits(val)
    }
}
impl From<Cc2loc> for u8 {
    #[inline(always)]
    fn from(val: Cc2loc) -> u8 {
        Cc2loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlCmoa {
    #[doc = "No action on compare match."]
    None = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl Cc3CtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlCmoa {
        Cc3CtrlCmoa::from_bits(val)
    }
}
impl From<Cc3CtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlCmoa) -> u8 {
        Cc3CtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlCofoa {
    #[doc = "No action on counter overflow."]
    None = 0x0,
    #[doc = "Toggle output on counter overflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter overflow."]
    Clear = 0x02,
    #[doc = "Set output on counter overflow."]
    Set = 0x03,
}
impl Cc3CtrlCofoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlCofoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlCofoa {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlCofoa {
        Cc3CtrlCofoa::from_bits(val)
    }
}
impl From<Cc3CtrlCofoa> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlCofoa) -> u8 {
        Cc3CtrlCofoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlCufoa {
    #[doc = "No action on counter underflow."]
    None = 0x0,
    #[doc = "Toggle output on counter underflow."]
    Toggle = 0x01,
    #[doc = "Clear output on counter underflow."]
    Clear = 0x02,
    #[doc = "Set output on counter underflow."]
    Set = 0x03,
}
impl Cc3CtrlCufoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlCufoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlCufoa {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlCufoa {
        Cc3CtrlCufoa::from_bits(val)
    }
}
impl From<Cc3CtrlCufoa> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlCufoa) -> u8 {
        Cc3CtrlCufoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl Cc3CtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlIcedge {
        Cc3CtrlIcedge::from_bits(val)
    }
}
impl From<Cc3CtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlIcedge) -> u8 {
        Cc3CtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlIcevctrl {
    #[doc = "PRS output pulse and interrupt flag set on every capture."]
    Everyedge = 0x0,
    #[doc = "PRS output pulse and interrupt flag set on every second capture."]
    Everysecondedge = 0x01,
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)."]
    Rising = 0x02,
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)."]
    Falling = 0x03,
}
impl Cc3CtrlIcevctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlIcevctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlIcevctrl {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlIcevctrl {
        Cc3CtrlIcevctrl::from_bits(val)
    }
}
impl From<Cc3CtrlIcevctrl> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlIcevctrl) -> u8 {
        Cc3CtrlIcevctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    #[doc = "Pulse-Width Modulation."]
    Pwm = 0x03,
}
impl Cc3CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> Cc3CtrlMode {
        Cc3CtrlMode::from_bits(val)
    }
}
impl From<Cc3CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: Cc3CtrlMode) -> u8 {
        Cc3CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc3loc {
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
impl Cc3loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc3loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc3loc {
    #[inline(always)]
    fn from(val: u8) -> Cc3loc {
        Cc3loc::from_bits(val)
    }
}
impl From<Cc3loc> for u8 {
    #[inline(always)]
    fn from(val: Cc3loc) -> u8 {
        Cc3loc::to_bits(val)
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
