#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkloc {
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
pub enum Clkprssel {
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
impl Clkprssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkprssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkprssel {
    #[inline(always)]
    fn from(val: u8) -> Clkprssel {
        Clkprssel::from_bits(val)
    }
}
impl From<Clkprssel> for u8 {
    #[inline(always)]
    fn from(val: Clkprssel) -> u8 {
        Clkprssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cshold {
    #[doc = "Disable CS being asserted after the end of transmission."]
    Zero = 0x0,
    #[doc = "CS is asserted for 1 baud-times after the end of transmission."]
    One = 0x01,
    #[doc = "CS is asserted for 2 baud-times after the end of transmission."]
    Two = 0x02,
    #[doc = "CS is asserted for 3 baud-times after the end of transmission."]
    Three = 0x03,
    #[doc = "CS is asserted for 7 baud-times after the end of transmission."]
    Seven = 0x04,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times."]
    Tcmp0 = 0x05,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times."]
    Tcmp1 = 0x06,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times."]
    Tcmp2 = 0x07,
}
impl Cshold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cshold {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cshold {
    #[inline(always)]
    fn from(val: u8) -> Cshold {
        Cshold::from_bits(val)
    }
}
impl From<Cshold> for u8 {
    #[inline(always)]
    fn from(val: Cshold) -> u8 {
        Cshold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csloc {
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
impl Csloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csloc {
    #[inline(always)]
    fn from(val: u8) -> Csloc {
        Csloc::from_bits(val)
    }
}
impl From<Csloc> for u8 {
    #[inline(always)]
    fn from(val: Csloc) -> u8 {
        Csloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cssetup {
    #[doc = "CS is not asserted before start of transmission."]
    Zero = 0x0,
    #[doc = "CS is asserted for 1 baud-times before start of transmission."]
    One = 0x01,
    #[doc = "CS is asserted for 2 baud-times before start of transmission."]
    Two = 0x02,
    #[doc = "CS is asserted for 3 baud-times before start of transmission."]
    Three = 0x03,
    #[doc = "CS is asserted for 7 baud-times before start of transmission."]
    Seven = 0x04,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times."]
    Tcmp0 = 0x05,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times."]
    Tcmp1 = 0x06,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times."]
    Tcmp2 = 0x07,
}
impl Cssetup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cssetup {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cssetup {
    #[inline(always)]
    fn from(val: u8) -> Cssetup {
        Cssetup::from_bits(val)
    }
}
impl From<Cssetup> for u8 {
    #[inline(always)]
    fn from(val: Cssetup) -> u8 {
        Cssetup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsloc {
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
impl Ctsloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsloc {
    #[inline(always)]
    fn from(val: u8) -> Ctsloc {
        Ctsloc::from_bits(val)
    }
}
impl From<Ctsloc> for u8 {
    #[inline(always)]
    fn from(val: Ctsloc) -> u8 {
        Ctsloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Databits {
    _RESERVED_0 = 0x0,
    #[doc = "Each frame contains 4 data bits."]
    Four = 0x01,
    #[doc = "Each frame contains 5 data bits."]
    Five = 0x02,
    #[doc = "Each frame contains 6 data bits."]
    Six = 0x03,
    #[doc = "Each frame contains 7 data bits."]
    Seven = 0x04,
    #[doc = "Each frame contains 8 data bits."]
    Eight = 0x05,
    #[doc = "Each frame contains 9 data bits."]
    Nine = 0x06,
    #[doc = "Each frame contains 10 data bits."]
    Ten = 0x07,
    #[doc = "Each frame contains 11 data bits."]
    Eleven = 0x08,
    #[doc = "Each frame contains 12 data bits."]
    Twelve = 0x09,
    #[doc = "Each frame contains 13 data bits."]
    Thirteen = 0x0a,
    #[doc = "Each frame contains 14 data bits."]
    Fourteen = 0x0b,
    #[doc = "Each frame contains 15 data bits."]
    Fifteen = 0x0c,
    #[doc = "Each frame contains 16 data bits."]
    Sixteen = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Databits {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Databits {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Databits {
    #[inline(always)]
    fn from(val: u8) -> Databits {
        Databits::from_bits(val)
    }
}
impl From<Databits> for u8 {
    #[inline(always)]
    fn from(val: Databits) -> u8 {
        Databits::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Format {
    #[doc = "32-bit word, 32-bit data."]
    W32d32 = 0x0,
    #[doc = "32-bit word, 32-bit data with 8 lsb masked."]
    W32d24m = 0x01,
    #[doc = "32-bit word, 24-bit data."]
    W32d24 = 0x02,
    #[doc = "32-bit word, 16-bit data."]
    W32d16 = 0x03,
    #[doc = "32-bit word, 8-bit data."]
    W32d8 = 0x04,
    #[doc = "16-bit word, 16-bit data."]
    W16d16 = 0x05,
    #[doc = "16-bit word, 8-bit data."]
    W16d8 = 0x06,
    #[doc = "8-bit word, 8-bit data."]
    W8d8 = 0x07,
}
impl Format {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Format {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Format {
    #[inline(always)]
    fn from(val: u8) -> Format {
        Format::from_bits(val)
    }
}
impl From<Format> for u8 {
    #[inline(always)]
    fn from(val: Format) -> u8 {
        Format::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ics {
    #[doc = "There is no space between charcters."]
    Zero = 0x0,
    #[doc = "Create a space of 1 baud-times before start of transmission."]
    One = 0x01,
    #[doc = "Create a space of 2 baud-times before start of transmission."]
    Two = 0x02,
    #[doc = "Create a space of 3 baud-times before start of transmission."]
    Three = 0x03,
    #[doc = "Create a space of 7 baud-times before start of transmission."]
    Seven = 0x04,
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times."]
    Tcmp0 = 0x05,
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times."]
    Tcmp1 = 0x06,
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times."]
    Tcmp2 = 0x07,
}
impl Ics {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ics {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ics {
    #[inline(always)]
    fn from(val: u8) -> Ics {
        Ics::from_bits(val)
    }
}
impl From<Ics> for u8 {
    #[inline(always)]
    fn from(val: Ics) -> u8 {
        Ics::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irprssel {
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
impl Irprssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irprssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irprssel {
    #[inline(always)]
    fn from(val: u8) -> Irprssel {
        Irprssel::from_bits(val)
    }
}
impl From<Irprssel> for u8 {
    #[inline(always)]
    fn from(val: Irprssel) -> u8 {
        Irprssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irpw {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1."]
    One = 0x0,
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1."]
    Two = 0x01,
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1."]
    Three = 0x02,
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1."]
    Four = 0x03,
}
impl Irpw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irpw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irpw {
    #[inline(always)]
    fn from(val: u8) -> Irpw {
        Irpw::from_bits(val)
    }
}
impl From<Irpw> for u8 {
    #[inline(always)]
    fn from(val: Irpw) -> u8 {
        Irpw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovs {
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode."]
    X16 = 0x0,
    #[doc = "Double speed with 8X oversampling in asynchronous mode."]
    X8 = 0x01,
    #[doc = "6X oversampling in asynchronous mode."]
    X6 = 0x02,
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode."]
    X4 = 0x03,
}
impl Ovs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovs {
    #[inline(always)]
    fn from(val: u8) -> Ovs {
        Ovs::from_bits(val)
    }
}
impl From<Ovs> for u8 {
    #[inline(always)]
    fn from(val: Ovs) -> u8 {
        Ovs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    #[doc = "Parity bits are not used."]
    None = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    Even = 0x02,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    Odd = 0x03,
}
impl Parity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Parity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Parity {
    #[inline(always)]
    fn from(val: u8) -> Parity {
        Parity::from_bits(val)
    }
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(val: Parity) -> u8 {
        Parity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtsloc {
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
impl Rtsloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtsloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtsloc {
    #[inline(always)]
    fn from(val: u8) -> Rtsloc {
        Rtsloc::from_bits(val)
    }
}
impl From<Rtsloc> for u8 {
    #[inline(always)]
    fn from(val: Rtsloc) -> u8 {
        Rtsloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxloc {
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
impl Rxloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxloc {
    #[inline(always)]
    fn from(val: u8) -> Rxloc {
        Rxloc::from_bits(val)
    }
}
impl From<Rxloc> for u8 {
    #[inline(always)]
    fn from(val: Rxloc) -> u8 {
        Rxloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxprssel {
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
impl Rxprssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxprssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxprssel {
    #[inline(always)]
    fn from(val: u8) -> Rxprssel {
        Rxprssel::from_bits(val)
    }
}
impl From<Rxprssel> for u8 {
    #[inline(always)]
    fn from(val: Rxprssel) -> u8 {
        Rxprssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopbits {
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver."]
    Half = 0x0,
    #[doc = "One stop bit is generated and verified."]
    One = 0x01,
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit."]
    Oneandahalf = 0x02,
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only."]
    Two = 0x03,
}
impl Stopbits {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopbits {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopbits {
    #[inline(always)]
    fn from(val: u8) -> Stopbits {
        Stopbits::from_bits(val)
    }
}
impl From<Stopbits> for u8 {
    #[inline(always)]
    fn from(val: Stopbits) -> u8 {
        Stopbits::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp0Tstart {
    #[doc = "Comparator 0 is disabled."]
    Disable = 0x0,
    #[doc = "Comparator 0 and timer are started at TX end of frame."]
    Txeof = 0x01,
    #[doc = "Comparator 0 and timer are started at TX Complete."]
    Txc = 0x02,
    #[doc = "Comparator 0 and timer are started at RX going Active (default: low)."]
    Rxact = 0x03,
    #[doc = "Comparator 0 and timer are started at RX end of frame."]
    Rxeof = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp0Tstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp0Tstart {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp0Tstart {
    #[inline(always)]
    fn from(val: u8) -> Timecmp0Tstart {
        Timecmp0Tstart::from_bits(val)
    }
}
impl From<Timecmp0Tstart> for u8 {
    #[inline(always)]
    fn from(val: Timecmp0Tstart) -> u8 {
        Timecmp0Tstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp0Tstop {
    #[doc = "Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event."]
    Tcmp0 = 0x0,
    #[doc = "Comparator 0 is disabled at the start of transmission."]
    Txst = 0x01,
    #[doc = "Comparator 0 is disabled on RX going going Active (default: low)."]
    Rxact = 0x02,
    #[doc = "Comparator 0 is disabled on RX going Inactive."]
    Rxactn = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp0Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp0Tstop {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp0Tstop {
    #[inline(always)]
    fn from(val: u8) -> Timecmp0Tstop {
        Timecmp0Tstop::from_bits(val)
    }
}
impl From<Timecmp0Tstop> for u8 {
    #[inline(always)]
    fn from(val: Timecmp0Tstop) -> u8 {
        Timecmp0Tstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp1Tstart {
    #[doc = "Comparator 1 is disabled."]
    Disable = 0x0,
    #[doc = "Comparator 1 and timer are started at TX end of frame."]
    Txeof = 0x01,
    #[doc = "Comparator 1 and timer are started at TX Complete."]
    Txc = 0x02,
    #[doc = "Comparator 1 and timer are started at RX going going Active (default: low)."]
    Rxact = 0x03,
    #[doc = "Comparator 1 and timer are started at RX end of frame."]
    Rxeof = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp1Tstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp1Tstart {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp1Tstart {
    #[inline(always)]
    fn from(val: u8) -> Timecmp1Tstart {
        Timecmp1Tstart::from_bits(val)
    }
}
impl From<Timecmp1Tstart> for u8 {
    #[inline(always)]
    fn from(val: Timecmp1Tstart) -> u8 {
        Timecmp1Tstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp1Tstop {
    #[doc = "Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event."]
    Tcmp1 = 0x0,
    #[doc = "Comparator 1 is disabled at TX start TX Engine."]
    Txst = 0x01,
    #[doc = "Comparator 1 is disabled on RX going going Active (default: low)."]
    Rxact = 0x02,
    #[doc = "Comparator 1 is disabled on RX going Inactive."]
    Rxactn = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp1Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp1Tstop {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp1Tstop {
    #[inline(always)]
    fn from(val: u8) -> Timecmp1Tstop {
        Timecmp1Tstop::from_bits(val)
    }
}
impl From<Timecmp1Tstop> for u8 {
    #[inline(always)]
    fn from(val: Timecmp1Tstop) -> u8 {
        Timecmp1Tstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp2Tstart {
    #[doc = "Comparator 2 is disabled."]
    Disable = 0x0,
    #[doc = "Comparator 2 and timer are started at TX end of frame."]
    Txeof = 0x01,
    #[doc = "Comparator 2 and timer are started at TX Complete."]
    Txc = 0x02,
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)."]
    Rxact = 0x03,
    #[doc = "Comparator 2 and timer are started at RX end of frame."]
    Rxeof = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp2Tstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp2Tstart {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp2Tstart {
    #[inline(always)]
    fn from(val: u8) -> Timecmp2Tstart {
        Timecmp2Tstart::from_bits(val)
    }
}
impl From<Timecmp2Tstart> for u8 {
    #[inline(always)]
    fn from(val: Timecmp2Tstart) -> u8 {
        Timecmp2Tstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecmp2Tstop {
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event."]
    Tcmp2 = 0x0,
    #[doc = "Comparator 2 is disabled at TX start TX Engine."]
    Txst = 0x01,
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)."]
    Rxact = 0x02,
    #[doc = "Comparator 2 is disabled on RX going Inactive."]
    Rxactn = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timecmp2Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecmp2Tstop {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecmp2Tstop {
    #[inline(always)]
    fn from(val: u8) -> Timecmp2Tstop {
        Timecmp2Tstop::from_bits(val)
    }
}
impl From<Timecmp2Tstop> for u8 {
    #[inline(always)]
    fn from(val: Timecmp2Tstop) -> u8 {
        Timecmp2Tstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsel {
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
impl Tsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsel {
    #[inline(always)]
    fn from(val: u8) -> Tsel {
        Tsel::from_bits(val)
    }
}
impl From<Tsel> for u8 {
    #[inline(always)]
    fn from(val: Tsel) -> u8 {
        Tsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdelay {
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy."]
    Disable = 0x0,
    #[doc = "Start of transmission is delayed for 1 baud-times."]
    One = 0x01,
    #[doc = "Start of transmission is delayed for 2 baud-times."]
    Two = 0x02,
    #[doc = "Start of transmission is delayed for 3 baud-times."]
    Three = 0x03,
    #[doc = "Start of transmission is delayed for 7 baud-times."]
    Seven = 0x04,
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times."]
    Tcmp0 = 0x05,
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times."]
    Tcmp1 = 0x06,
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times."]
    Tcmp2 = 0x07,
}
impl Txdelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdelay {
    #[inline(always)]
    fn from(val: u8) -> Txdelay {
        Txdelay::from_bits(val)
    }
}
impl From<Txdelay> for u8 {
    #[inline(always)]
    fn from(val: Txdelay) -> u8 {
        Txdelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txloc {
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
impl Txloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txloc {
    #[inline(always)]
    fn from(val: u8) -> Txloc {
        Txloc::from_bits(val)
    }
}
impl From<Txloc> for u8 {
    #[inline(always)]
    fn from(val: Txloc) -> u8 {
        Txloc::to_bits(val)
    }
}
