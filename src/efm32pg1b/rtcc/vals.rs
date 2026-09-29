#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc0CtrlCmoa {
    #[doc = "A single clock cycle pulse is generated on output."]
    Pulse = 0x0,
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
pub enum Cc0CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Cc0CtrlPrssel {
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
impl Cc0CtrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc0CtrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc0CtrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> Cc0CtrlPrssel {
        Cc0CtrlPrssel::from_bits(val)
    }
}
impl From<Cc0CtrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: Cc0CtrlPrssel) -> u8 {
        Cc0CtrlPrssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc1CtrlCmoa {
    #[doc = "A single clock cycle pulse is generated on output."]
    Pulse = 0x0,
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
pub enum Cc1CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Cc1CtrlPrssel {
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
impl Cc1CtrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc1CtrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc1CtrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> Cc1CtrlPrssel {
        Cc1CtrlPrssel::from_bits(val)
    }
}
impl From<Cc1CtrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: Cc1CtrlPrssel) -> u8 {
        Cc1CtrlPrssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc2CtrlCmoa {
    #[doc = "A single clock cycle pulse is generated on output."]
    Pulse = 0x0,
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
pub enum Cc2CtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Cc2CtrlPrssel {
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
impl Cc2CtrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc2CtrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc2CtrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> Cc2CtrlPrssel {
        Cc2CtrlPrssel::from_bits(val)
    }
}
impl From<Cc2CtrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: Cc2CtrlPrssel) -> u8 {
        Cc2CtrlPrssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntpresc {
    #[doc = "CLKCNT = LFECLKRTCC/1."]
    Div1 = 0x0,
    #[doc = "CLKCNT = LFECLKRTCC/2."]
    Div2 = 0x01,
    #[doc = "CLKCNT = LFECLKRTCC/4."]
    Div4 = 0x02,
    #[doc = "CLKCNT = LFECLKRTCC/8."]
    Div8 = 0x03,
    #[doc = "CLKCNT = LFECLKRTCC/16."]
    Div16 = 0x04,
    #[doc = "CLKCNT = LFECLKRTCC/32."]
    Div32 = 0x05,
    #[doc = "CLKCNT = LFECLKRTCC/64."]
    Div64 = 0x06,
    #[doc = "CLKCNT = LFECLKRTCC/128."]
    Div128 = 0x07,
    #[doc = "CLKCNT = LFECLKRTCC/256."]
    Div256 = 0x08,
    #[doc = "CLKCNT = LFECLKRTCC/512."]
    Div512 = 0x09,
    #[doc = "CLKCNT = LFECLKRTCC/1024."]
    Div1024 = 0x0a,
    #[doc = "CLKCNT = LFECLKRTCC/2048."]
    Div2048 = 0x0b,
    #[doc = "CLKCNT = LFECLKRTCC/4096."]
    Div4096 = 0x0c,
    #[doc = "CLKCNT = LFECLKRTCC/8192."]
    Div8192 = 0x0d,
    #[doc = "CLKCNT = LFECLKRTCC/16384."]
    Div16384 = 0x0e,
    #[doc = "CLKCNT = LFECLKRTCC/32768."]
    Div32768 = 0x0f,
}
impl Cntpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntpresc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntpresc {
    #[inline(always)]
    fn from(val: u8) -> Cntpresc {
        Cntpresc::from_bits(val)
    }
}
impl From<Cntpresc> for u8 {
    #[inline(always)]
    fn from(val: Cntpresc) -> u8 {
        Cntpresc::to_bits(val)
    }
}
