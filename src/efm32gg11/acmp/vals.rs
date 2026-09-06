#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aportsel {
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    Aport0x = 0x0,
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    Aport0y = 0x01,
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1x = 0x02,
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1y = 0x03,
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1xy = 0x04,
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2x = 0x05,
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2y = 0x06,
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2yx = 0x07,
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3x = 0x08,
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3y = 0x09,
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3xy = 0x0a,
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4x = 0x0b,
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4y = 0x0c,
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4yx = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Aportsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aportsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aportsel {
    #[inline(always)]
    fn from(val: u8) -> Aportsel {
        Aportsel::from_bits(val)
    }
}
impl From<Aportsel> for u8 {
    #[inline(always)]
    fn from(val: Aportsel) -> u8 {
        Aportsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csressel {
    #[doc = "Internal capacitive sense resistor value 0."]
    Res0 = 0x0,
    #[doc = "Internal capacitive sense resistor value 1."]
    Res1 = 0x01,
    #[doc = "Internal capacitive sense resistor value 2."]
    Res2 = 0x02,
    #[doc = "Internal capacitive sense resistor value 3."]
    Res3 = 0x03,
    #[doc = "Internal capacitive sense resistor value 4."]
    Res4 = 0x04,
    #[doc = "Internal capacitive sense resistor value 5."]
    Res5 = 0x05,
    #[doc = "Internal capacitive sense resistor value 6."]
    Res6 = 0x06,
    #[doc = "Internal capacitive sense resistor value 7."]
    Res7 = 0x07,
}
impl Csressel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csressel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csressel {
    #[inline(always)]
    fn from(val: u8) -> Csressel {
        Csressel::from_bits(val)
    }
}
impl From<Csressel> for u8 {
    #[inline(always)]
    fn from(val: Csressel) -> u8 {
        Csressel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hysteresis0Hyst {
    #[doc = "No hysteresis."]
    Hyst0 = 0x0,
    #[doc = "14 mV hysteresis."]
    Hyst1 = 0x01,
    #[doc = "25 mV hysteresis."]
    Hyst2 = 0x02,
    #[doc = "30 mV hysteresis."]
    Hyst3 = 0x03,
    #[doc = "35 mV hysteresis."]
    Hyst4 = 0x04,
    #[doc = "39 mV hysteresis."]
    Hyst5 = 0x05,
    #[doc = "42 mV hysteresis."]
    Hyst6 = 0x06,
    #[doc = "45 mV hysteresis."]
    Hyst7 = 0x07,
    #[doc = "No hysteresis."]
    Hyst8 = 0x08,
    #[doc = "-14 mV hysteresis."]
    Hyst9 = 0x09,
    #[doc = "-25 mV hysteresis."]
    Hyst10 = 0x0a,
    #[doc = "-30 mV hysteresis."]
    Hyst11 = 0x0b,
    #[doc = "-35 mV hysteresis."]
    Hyst12 = 0x0c,
    #[doc = "-39 mV hysteresis."]
    Hyst13 = 0x0d,
    #[doc = "-42 mV hysteresis."]
    Hyst14 = 0x0e,
    #[doc = "-45 mV hysteresis."]
    Hyst15 = 0x0f,
}
impl Hysteresis0Hyst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hysteresis0Hyst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hysteresis0Hyst {
    #[inline(always)]
    fn from(val: u8) -> Hysteresis0Hyst {
        Hysteresis0Hyst::from_bits(val)
    }
}
impl From<Hysteresis0Hyst> for u8 {
    #[inline(always)]
    fn from(val: Hysteresis0Hyst) -> u8 {
        Hysteresis0Hyst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hysteresis1Hyst {
    #[doc = "No hysteresis."]
    Hyst0 = 0x0,
    #[doc = "14 mV hysteresis."]
    Hyst1 = 0x01,
    #[doc = "25 mV hysteresis."]
    Hyst2 = 0x02,
    #[doc = "30 mV hysteresis."]
    Hyst3 = 0x03,
    #[doc = "35 mV hysteresis."]
    Hyst4 = 0x04,
    #[doc = "39 mV hysteresis."]
    Hyst5 = 0x05,
    #[doc = "42 mV hysteresis."]
    Hyst6 = 0x06,
    #[doc = "45 mV hysteresis."]
    Hyst7 = 0x07,
    #[doc = "No hysteresis."]
    Hyst8 = 0x08,
    #[doc = "-14 mV hysteresis."]
    Hyst9 = 0x09,
    #[doc = "-25 mV hysteresis."]
    Hyst10 = 0x0a,
    #[doc = "-30 mV hysteresis."]
    Hyst11 = 0x0b,
    #[doc = "-35 mV hysteresis."]
    Hyst12 = 0x0c,
    #[doc = "-39 mV hysteresis."]
    Hyst13 = 0x0d,
    #[doc = "-42 mV hysteresis."]
    Hyst14 = 0x0e,
    #[doc = "-45 mV hysteresis."]
    Hyst15 = 0x0f,
}
impl Hysteresis1Hyst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hysteresis1Hyst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hysteresis1Hyst {
    #[inline(always)]
    fn from(val: u8) -> Hysteresis1Hyst {
        Hysteresis1Hyst::from_bits(val)
    }
}
impl From<Hysteresis1Hyst> for u8 {
    #[inline(always)]
    fn from(val: Hysteresis1Hyst) -> u8 {
        Hysteresis1Hyst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inputrange {
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    Full = 0x0,
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    Gtvdddiv2 = 0x01,
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    Ltvdddiv2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Inputrange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputrange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputrange {
    #[inline(always)]
    fn from(val: u8) -> Inputrange {
        Inputrange::from_bits(val)
    }
}
impl From<Inputrange> for u8 {
    #[inline(always)]
    fn from(val: Inputrange) -> u8 {
        Inputrange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outloc {
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
impl Outloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outloc {
    #[inline(always)]
    fn from(val: u8) -> Outloc {
        Outloc::from_bits(val)
    }
}
impl From<Outloc> for u8 {
    #[inline(always)]
    fn from(val: Outloc) -> u8 {
        Outloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrsel {
    #[doc = "AVDD supply."]
    Avdd = 0x0,
    #[doc = "DVDD supply."]
    Dvdd = 0x01,
    #[doc = "IOVDD/IOVDD0 supply."]
    Iovdd0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "IOVDD1 supply (if part has two I/O voltages)."]
    Iovdd1 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vasel {
    #[doc = "ACMPVDD."]
    Vdd = 0x0,
    #[doc = "APORT2Y Channel 0."]
    Aport2ych0 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "APORT2Y Channel 2."]
    Aport2ych2 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "APORT2Y Channel 4."]
    Aport2ych4 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "APORT2Y Channel 6."]
    Aport2ych6 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "APORT2Y Channel 8."]
    Aport2ych8 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "APORT2Y Channel 10."]
    Aport2ych10 = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "APORT2Y Channel 12."]
    Aport2ych12 = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "APORT2Y Channel 14."]
    Aport2ych14 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "APORT2Y Channel 16."]
    Aport2ych16 = 0x11,
    _RESERVED_12 = 0x12,
    #[doc = "APORT2Y Channel 18."]
    Aport2ych18 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "APORT2Y Channel 20."]
    Aport2ych20 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "APORT2Y Channel 22."]
    Aport2ych22 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "APORT2Y Channel 24."]
    Aport2ych24 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "APORT2Y Channel 26."]
    Aport2ych26 = 0x1b,
    _RESERVED_1c = 0x1c,
    #[doc = "APORT2Y Channel 28."]
    Aport2ych28 = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "APORT2Y Channel 30."]
    Aport2ych30 = 0x1f,
    #[doc = "APORT1X Channel 0."]
    Aport1xch0 = 0x20,
    #[doc = "APORT1Y Channel 1."]
    Aport1ych1 = 0x21,
    #[doc = "APORT1X Channel 2."]
    Aport1xch2 = 0x22,
    #[doc = "APORT1Y Channel 3."]
    Aport1ych3 = 0x23,
    #[doc = "APORT1X Channel 4."]
    Aport1xch4 = 0x24,
    #[doc = "APORT1Y Channel 5."]
    Aport1ych5 = 0x25,
    #[doc = "APORT1X Channel 6."]
    Aport1xch6 = 0x26,
    #[doc = "APORT1Y Channel 7."]
    Aport1ych7 = 0x27,
    #[doc = "APORT1X Channel 8."]
    Aport1xch8 = 0x28,
    #[doc = "APORT1Y Channel 9."]
    Aport1ych9 = 0x29,
    #[doc = "APORT1X Channel 10."]
    Aport1xch10 = 0x2a,
    #[doc = "APORT1Y Channel 11."]
    Aport1ych11 = 0x2b,
    #[doc = "APORT1X Channel 12."]
    Aport1xch12 = 0x2c,
    #[doc = "APORT1Y Channel 13."]
    Aport1ych13 = 0x2d,
    #[doc = "APORT1X Channel 14."]
    Aport1xch14 = 0x2e,
    #[doc = "APORT1Y Channel 15."]
    Aport1ych15 = 0x2f,
    #[doc = "APORT1X Channel 16."]
    Aport1xch16 = 0x30,
    #[doc = "APORT1Y Channel 17."]
    Aport1ych17 = 0x31,
    #[doc = "APORT1X Channel 18."]
    Aport1xch18 = 0x32,
    #[doc = "APORT1Y Channel 19."]
    Aport1ych19 = 0x33,
    #[doc = "APORT1X Channel 20."]
    Aport1xch20 = 0x34,
    #[doc = "APORT1Y Channel 21."]
    Aport1ych21 = 0x35,
    #[doc = "APORT1X Channel 22."]
    Aport1xch22 = 0x36,
    #[doc = "APORT1Y Channel 23."]
    Aport1ych23 = 0x37,
    #[doc = "APORT1X Channel 24."]
    Aport1xch24 = 0x38,
    #[doc = "APORT1Y Channel 25."]
    Aport1ych25 = 0x39,
    #[doc = "APORT1X Channel 26."]
    Aport1xch26 = 0x3a,
    #[doc = "APORT1Y Channel 27."]
    Aport1ych27 = 0x3b,
    #[doc = "APORT1X Channel 28."]
    Aport1xch28 = 0x3c,
    #[doc = "APORT1Y Channel 29."]
    Aport1ych29 = 0x3d,
    #[doc = "APORT1X Channel 30."]
    Aport1xch30 = 0x3e,
    #[doc = "APORT1Y Channel 31."]
    Aport1ych31 = 0x3f,
}
impl Vasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vasel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vasel {
    #[inline(always)]
    fn from(val: u8) -> Vasel {
        Vasel::from_bits(val)
    }
}
impl From<Vasel> for u8 {
    #[inline(always)]
    fn from(val: Vasel) -> u8 {
        Vasel::to_bits(val)
    }
}
