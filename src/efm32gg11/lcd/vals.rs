#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aregasc {
    #[doc = "No Shift operation on Animation Register A."]
    Noshift = 0x0,
    #[doc = "Animation Register A is shifted left."]
    Shiftleft = 0x01,
    #[doc = "Animation Register A is shifted right."]
    Shiftright = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aregasc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aregasc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aregasc {
    #[inline(always)]
    fn from(val: u8) -> Aregasc {
        Aregasc::from_bits(val)
    }
}
impl From<Aregasc> for u8 {
    #[inline(always)]
    fn from(val: Aregasc) -> u8 {
        Aregasc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aregbsc {
    #[doc = "No Shift operation on Animation Register B."]
    Noshift = 0x0,
    #[doc = "Animation Register B is shifted left."]
    Shiftleft = 0x01,
    #[doc = "Animation Register B is shifted right."]
    Shiftright = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aregbsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aregbsc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aregbsc {
    #[inline(always)]
    fn from(val: u8) -> Aregbsc {
        Aregbsc::from_bits(val)
    }
}
impl From<Aregbsc> for u8 {
    #[inline(always)]
    fn from(val: Aregbsc) -> u8 {
        Aregbsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bias {
    #[doc = "Static."]
    Static = 0x0,
    #[doc = "1/2 Bias."]
    Onehalf = 0x01,
    #[doc = "1/3 Bias."]
    Onethird = 0x02,
    #[doc = "1/4 Bias."]
    Onefourth = 0x03,
}
impl Bias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bias {
    #[inline(always)]
    fn from(val: u8) -> Bias {
        Bias::from_bits(val)
    }
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(val: Bias) -> u8 {
        Bias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chgrdst {
    #[doc = "Disable charge redistribution."]
    Disable = 0x0,
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    One = 0x01,
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    Two = 0x02,
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    Three = 0x03,
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    Four = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Chgrdst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chgrdst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chgrdst {
    #[inline(always)]
    fn from(val: u8) -> Chgrdst {
        Chgrdst::from_bits(val)
    }
}
impl From<Chgrdst> for u8 {
    #[inline(always)]
    fn from(val: Chgrdst) -> u8 {
        Chgrdst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fcpresc {
    #[doc = "CLKFC = CLKFRAME / 1."]
    Div1 = 0x0,
    #[doc = "CLKFC = CLKFRAME / 2."]
    Div2 = 0x01,
    #[doc = "CLKFC = CLKFRAME / 4."]
    Div4 = 0x02,
    #[doc = "CLKFC = CLKFRAME / 8."]
    Div8 = 0x03,
}
impl Fcpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fcpresc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fcpresc {
    #[inline(always)]
    fn from(val: u8) -> Fcpresc {
        Fcpresc::from_bits(val)
    }
}
impl From<Fcpresc> for u8 {
    #[inline(always)]
    fn from(val: Fcpresc) -> u8 {
        Fcpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    Noextcap = 0x0,
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    Stepdown = 0x01,
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    Cpintosc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Mux {
    #[doc = "Static."]
    Static = 0x0,
    #[doc = "Duplex."]
    Duplex = 0x01,
    #[doc = "Triplex."]
    Triplex = 0x02,
    #[doc = "Quadruplex."]
    Quadruplex = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Sextaplex."]
    Sextaplex = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Octaplex."]
    Octaplex = 0x07,
}
impl Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mux {
    #[inline(always)]
    fn from(val: u8) -> Mux {
        Mux::from_bits(val)
    }
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(val: Mux) -> u8 {
        Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Udctrl {
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible."]
    Regular = 0x0,
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter."]
    Fcevent = 0x01,
    #[doc = "The data transfer is done continuously at every LCD frame start."]
    Framestart = 0x02,
    _RESERVED_3 = 0x03,
}
impl Udctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Udctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Udctrl {
    #[inline(always)]
    fn from(val: u8) -> Udctrl {
        Udctrl::from_bits(val)
    }
}
impl From<Udctrl> for u8 {
    #[inline(always)]
    fn from(val: Udctrl) -> u8 {
        Udctrl::to_bits(val)
    }
}
