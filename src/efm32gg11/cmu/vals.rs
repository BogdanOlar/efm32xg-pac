#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdiv {
    Nodivision = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Adc0clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdiv {
        Adc0clkdiv::from_bits(val)
    }
}
impl From<Adc0clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdiv) -> u8 {
        Adc0clkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clksel {
    #[doc = "ADC0 is not clocked."]
    Disabled = 0x0,
    #[doc = "AUXHFRCO is clocking ADC0."]
    Auxhfrco = 0x01,
    #[doc = "HFXO is clocking ADC0."]
    Hfxo = 0x02,
    #[doc = "HFSRCCLK is clocking ADC0."]
    Hfsrcclk = 0x03,
}
impl Adc0clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clksel {
    #[inline(always)]
    fn from(val: u8) -> Adc0clksel {
        Adc0clksel::from_bits(val)
    }
}
impl From<Adc0clksel> for u8 {
    #[inline(always)]
    fn from(val: Adc0clksel) -> u8 {
        Adc0clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdiv {
    Nodivision = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Adc1clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdiv {
        Adc1clkdiv::from_bits(val)
    }
}
impl From<Adc1clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdiv) -> u8 {
        Adc1clkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clksel {
    #[doc = "ADC1 is not clocked."]
    Disabled = 0x0,
    #[doc = "AUXHFRCO is clocking ADC1."]
    Auxhfrco = 0x01,
    #[doc = "HFXO is clocking ADC1."]
    Hfxo = 0x02,
    #[doc = "HFSRCCLK is clocking ADC1."]
    Hfsrcclk = 0x03,
}
impl Adc1clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clksel {
    #[inline(always)]
    fn from(val: u8) -> Adc1clksel {
        Adc1clksel::from_bits(val)
    }
}
impl From<Adc1clksel> for u8 {
    #[inline(always)]
    fn from(val: Adc1clksel) -> u8 {
        Adc1clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AuxhfrcoctrlClkdiv {
    #[doc = "Divide by 1."]
    Div1 = 0x0,
    #[doc = "Divide by 2."]
    Div2 = 0x01,
    #[doc = "Divide by 4."]
    Div4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl AuxhfrcoctrlClkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AuxhfrcoctrlClkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AuxhfrcoctrlClkdiv {
    #[inline(always)]
    fn from(val: u8) -> AuxhfrcoctrlClkdiv {
        AuxhfrcoctrlClkdiv::from_bits(val)
    }
}
impl From<AuxhfrcoctrlClkdiv> for u8 {
    #[inline(always)]
    fn from(val: AuxhfrcoctrlClkdiv) -> u8 {
        AuxhfrcoctrlClkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkin0loc {
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
impl Clkin0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkin0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkin0loc {
    #[inline(always)]
    fn from(val: u8) -> Clkin0loc {
        Clkin0loc::from_bits(val)
    }
}
impl From<Clkin0loc> for u8 {
    #[inline(always)]
    fn from(val: Clkin0loc) -> u8 {
        Clkin0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkout0loc {
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
impl Clkout0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkout0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkout0loc {
    #[inline(always)]
    fn from(val: u8) -> Clkout0loc {
        Clkout0loc::from_bits(val)
    }
}
impl From<Clkout0loc> for u8 {
    #[inline(always)]
    fn from(val: Clkout0loc) -> u8 {
        Clkout0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkout1loc {
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
impl Clkout1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkout1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkout1loc {
    #[inline(always)]
    fn from(val: u8) -> Clkout1loc {
        Clkout1loc::from_bits(val)
    }
}
impl From<Clkout1loc> for u8 {
    #[inline(always)]
    fn from(val: Clkout1loc) -> u8 {
        Clkout1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkout2loc {
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
impl Clkout2loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkout2loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkout2loc {
    #[inline(always)]
    fn from(val: u8) -> Clkout2loc {
        Clkout2loc::from_bits(val)
    }
}
impl From<Clkout2loc> for u8 {
    #[inline(always)]
    fn from(val: Clkout2loc) -> u8 {
        Clkout2loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkoutsel0 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "ULFRCO (directly from oscillator)."]
    Ulfrco = 0x01,
    #[doc = "LFRCO (directly from oscillator)."]
    Lfrco = 0x02,
    #[doc = "LFXO (directly from oscillator)."]
    Lfxo = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "HFXO (directly from oscillator)."]
    Hfxo = 0x06,
    #[doc = "HFEXPCLK."]
    Hfexpclk = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ULFRCO (qualified)."]
    Ulfrcoq = 0x09,
    #[doc = "LFRCO (qualified)."]
    Lfrcoq = 0x0a,
    #[doc = "LFXO (qualified)."]
    Lfxoq = 0x0b,
    #[doc = "HFRCO (qualified)."]
    Hfrcoq = 0x0c,
    #[doc = "AUXHFRCO (qualified)."]
    Auxhfrcoq = 0x0d,
    #[doc = "HFXO (qualified)."]
    Hfxoq = 0x0e,
    #[doc = "HFSRCCLK."]
    Hfsrcclk = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "USHFRCO (qualified)."]
    Ushfrcoq = 0x12,
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
}
impl Clkoutsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkoutsel0 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkoutsel0 {
    #[inline(always)]
    fn from(val: u8) -> Clkoutsel0 {
        Clkoutsel0::from_bits(val)
    }
}
impl From<Clkoutsel0> for u8 {
    #[inline(always)]
    fn from(val: Clkoutsel0) -> u8 {
        Clkoutsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkoutsel1 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "ULFRCO (directly from oscillator)."]
    Ulfrco = 0x01,
    #[doc = "LFRCO (directly from oscillator)."]
    Lfrco = 0x02,
    #[doc = "LFXO (directly from oscillator)."]
    Lfxo = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "HFXO (directly from oscillator)."]
    Hfxo = 0x06,
    #[doc = "HFEXPCLK."]
    Hfexpclk = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ULFRCO (qualified)."]
    Ulfrcoq = 0x09,
    #[doc = "LFRCO (qualified)."]
    Lfrcoq = 0x0a,
    #[doc = "LFXO (qualified)."]
    Lfxoq = 0x0b,
    #[doc = "HFRCO (qualified)."]
    Hfrcoq = 0x0c,
    #[doc = "AUXHFRCO (qualified)."]
    Auxhfrcoq = 0x0d,
    #[doc = "HFXO (qualified)."]
    Hfxoq = 0x0e,
    #[doc = "HFSRCCLK."]
    Hfsrcclk = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "USHFRCO (qualified)."]
    Ushfrcoq = 0x12,
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
}
impl Clkoutsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkoutsel1 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkoutsel1 {
    #[inline(always)]
    fn from(val: u8) -> Clkoutsel1 {
        Clkoutsel1::from_bits(val)
    }
}
impl From<Clkoutsel1> for u8 {
    #[inline(always)]
    fn from(val: Clkoutsel1) -> u8 {
        Clkoutsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkoutsel2 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "ULFRCO (directly from oscillator)."]
    Ulfrco = 0x01,
    #[doc = "LFRCO (directly from oscillator)."]
    Lfrco = 0x02,
    #[doc = "LFXO (directly from oscillator)."]
    Lfxo = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "HFXO divided by two (qualified)."]
    Hfxodiv2q = 0x05,
    #[doc = "HFXO (directly from oscillator)."]
    Hfxo = 0x06,
    #[doc = "HFEXPCLK."]
    Hfexpclk = 0x07,
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)."]
    Hfxox2q = 0x08,
    #[doc = "ULFRCO (qualified)."]
    Ulfrcoq = 0x09,
    #[doc = "LFRCO (qualified)."]
    Lfrcoq = 0x0a,
    #[doc = "LFXO (qualified)."]
    Lfxoq = 0x0b,
    #[doc = "HFRCO (qualified)."]
    Hfrcoq = 0x0c,
    #[doc = "AUXHFRCO (qualified)."]
    Auxhfrcoq = 0x0d,
    #[doc = "HFXO (qualified)."]
    Hfxoq = 0x0e,
    #[doc = "HFSRCCLK."]
    Hfsrcclk = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "USHFRCO (qualified)."]
    Ushfrcoq = 0x12,
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
}
impl Clkoutsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkoutsel2 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkoutsel2 {
    #[inline(always)]
    fn from(val: u8) -> Clkoutsel2 {
        Clkoutsel2::from_bits(val)
    }
}
impl From<Clkoutsel2> for u8 {
    #[inline(always)]
    fn from(val: Clkoutsel2) -> u8 {
        Clkoutsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csen {
    #[doc = "LFBCLKCSEN = LFBCLK/16."]
    Div16 = 0x0,
    #[doc = "LFBCLKCSEN = LFBCLK/32."]
    Div32 = 0x01,
    #[doc = "LFBCLKCSEN = LFBCLK/64."]
    Div64 = 0x02,
    #[doc = "LFBCLKCSEN = LFBCLK/128."]
    Div128 = 0x03,
}
impl Csen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csen {
    #[inline(always)]
    fn from(val: u8) -> Csen {
        Csen::from_bits(val)
    }
}
impl From<Csen> for u8 {
    #[inline(always)]
    fn from(val: Csen) -> u8 {
        Csen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbg {
    #[doc = "AUXHFRCO is the debug trace clock."]
    Auxhfrco = 0x0,
    #[doc = "HFCLK is the debug trace clock."]
    Hfclk = 0x01,
    #[doc = "HFRCO divided by 2 is the debug trace clock."]
    Hfrcodiv2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbg {
    #[inline(always)]
    fn from(val: u8) -> Dbg {
        Dbg::from_bits(val)
    }
}
impl From<Dbg> for u8 {
    #[inline(always)]
    fn from(val: Dbg) -> u8 {
        Dbg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Downsel {
    #[doc = "Select HFCLK for down-counter."]
    Hfclk = 0x0,
    #[doc = "Select HFXO for down-counter."]
    Hfxo = 0x01,
    #[doc = "Select LFXO for down-counter."]
    Lfxo = 0x02,
    #[doc = "Select HFRCO for down-counter."]
    Hfrco = 0x03,
    #[doc = "Select LFRCO for down-counter."]
    Lfrco = 0x04,
    #[doc = "Select AUXHFRCO for down-counter."]
    Auxhfrco = 0x05,
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter."]
    Prs = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select USHFRCO for down-counter."]
    Ushfrco = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Downsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Downsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Downsel {
    #[inline(always)]
    fn from(val: u8) -> Downsel {
        Downsel::from_bits(val)
    }
}
impl From<Downsel> for u8 {
    #[inline(always)]
    fn from(val: Downsel) -> u8 {
        Downsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hf {
    _RESERVED_0 = 0x0,
    #[doc = "Select HFRCO as HFCLK."]
    Hfrco = 0x01,
    #[doc = "Select HFXO as HFCLK."]
    Hfxo = 0x02,
    #[doc = "Select LFRCO as HFCLK."]
    Lfrco = 0x03,
    #[doc = "Select LFXO as HFCLK."]
    Lfxo = 0x04,
    #[doc = "Select HFRCO divided by 2 as HFCLK."]
    Hfrcodiv2 = 0x05,
    #[doc = "Select USHFRCO as HFCLK."]
    Ushfrco = 0x06,
    #[doc = "Select CLKIN0 as HFCLK."]
    Clkin0 = 0x07,
}
impl Hf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hf {
    #[inline(always)]
    fn from(val: u8) -> Hf {
        Hf::from_bits(val)
    }
}
impl From<Hf> for u8 {
    #[inline(always)]
    fn from(val: Hf) -> u8 {
        Hf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfbusprescPresc(u16);
impl HfbusprescPresc {
    pub const Nodivision: Self = Self(0x0);
}
impl HfbusprescPresc {
    pub const fn from_bits(val: u16) -> HfbusprescPresc {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HfbusprescPresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HfbusprescPresc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HfbusprescPresc {
    #[inline(always)]
    fn from(val: u16) -> HfbusprescPresc {
        HfbusprescPresc::from_bits(val)
    }
}
impl From<HfbusprescPresc> for u16 {
    #[inline(always)]
    fn from(val: HfbusprescPresc) -> u16 {
        HfbusprescPresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hfclklepresc {
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    Div2 = 0x0,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    Div4 = 0x01,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    Div8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hfclklepresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclklepresc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclklepresc {
    #[inline(always)]
    fn from(val: u8) -> Hfclklepresc {
        Hfclklepresc::from_bits(val)
    }
}
impl From<Hfclklepresc> for u8 {
    #[inline(always)]
    fn from(val: Hfclklepresc) -> u8 {
        Hfclklepresc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfcoreprescPresc(u16);
impl HfcoreprescPresc {
    pub const Nodivision: Self = Self(0x0);
}
impl HfcoreprescPresc {
    pub const fn from_bits(val: u16) -> HfcoreprescPresc {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HfcoreprescPresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HfcoreprescPresc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HfcoreprescPresc {
    #[inline(always)]
    fn from(val: u16) -> HfcoreprescPresc {
        HfcoreprescPresc::from_bits(val)
    }
}
impl From<HfcoreprescPresc> for u16 {
    #[inline(always)]
    fn from(val: HfcoreprescPresc) -> u16 {
        HfcoreprescPresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HfexpprescPresc {
    Nodivision = 0x0,
    _RESERVED_1 = 0x01,
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
}
impl HfexpprescPresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfexpprescPresc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfexpprescPresc {
    #[inline(always)]
    fn from(val: u8) -> HfexpprescPresc {
        HfexpprescPresc::from_bits(val)
    }
}
impl From<HfexpprescPresc> for u8 {
    #[inline(always)]
    fn from(val: HfexpprescPresc) -> u8 {
        HfexpprescPresc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfperprescPresc(u16);
impl HfperprescPresc {
    pub const Nodivision: Self = Self(0x0);
}
impl HfperprescPresc {
    pub const fn from_bits(val: u16) -> HfperprescPresc {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HfperprescPresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HfperprescPresc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HfperprescPresc {
    #[inline(always)]
    fn from(val: u16) -> HfperprescPresc {
        HfperprescPresc::from_bits(val)
    }
}
impl From<HfperprescPresc> for u16 {
    #[inline(always)]
    fn from(val: HfperprescPresc) -> u16 {
        HfperprescPresc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfperprescbPresc(u16);
impl HfperprescbPresc {
    pub const Nodivision: Self = Self(0x0);
}
impl HfperprescbPresc {
    pub const fn from_bits(val: u16) -> HfperprescbPresc {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HfperprescbPresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HfperprescbPresc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HfperprescbPresc {
    #[inline(always)]
    fn from(val: u16) -> HfperprescbPresc {
        HfperprescbPresc::from_bits(val)
    }
}
impl From<HfperprescbPresc> for u16 {
    #[inline(always)]
    fn from(val: HfperprescbPresc) -> u16 {
        HfperprescbPresc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfperpresccPresc(u16);
impl HfperpresccPresc {
    pub const Nodivision: Self = Self(0x0);
}
impl HfperpresccPresc {
    pub const fn from_bits(val: u16) -> HfperpresccPresc {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HfperpresccPresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HfperpresccPresc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HfperpresccPresc {
    #[inline(always)]
    fn from(val: u16) -> HfperpresccPresc {
        HfperpresccPresc::from_bits(val)
    }
}
impl From<HfperpresccPresc> for u16 {
    #[inline(always)]
    fn from(val: HfperpresccPresc) -> u16 {
        HfperpresccPresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HfprescPresc {
    Nodivision = 0x0,
    _RESERVED_1 = 0x01,
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
}
impl HfprescPresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfprescPresc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfprescPresc {
    #[inline(always)]
    fn from(val: u8) -> HfprescPresc {
        HfprescPresc::from_bits(val)
    }
}
impl From<HfprescPresc> for u8 {
    #[inline(always)]
    fn from(val: HfprescPresc) -> u8 {
        HfprescPresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HfrcoctrlClkdiv {
    #[doc = "Divide by 1."]
    Div1 = 0x0,
    #[doc = "Divide by 2."]
    Div2 = 0x01,
    #[doc = "Divide by 4."]
    Div4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl HfrcoctrlClkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfrcoctrlClkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfrcoctrlClkdiv {
    #[inline(always)]
    fn from(val: u8) -> HfrcoctrlClkdiv {
        HfrcoctrlClkdiv::from_bits(val)
    }
}
impl From<HfrcoctrlClkdiv> for u8 {
    #[inline(always)]
    fn from(val: HfrcoctrlClkdiv) -> u8 {
        HfrcoctrlClkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HfxoctrlMode {
    #[doc = "4 MHz - 50 MHz crystal oscillator."]
    Xtal = 0x0,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    Acbufextclk = 0x01,
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    Dcbufextclk = 0x02,
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    Digextclk = 0x03,
}
impl HfxoctrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfxoctrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfxoctrlMode {
    #[inline(always)]
    fn from(val: u8) -> HfxoctrlMode {
        HfxoctrlMode::from_bits(val)
    }
}
impl From<HfxoctrlMode> for u8 {
    #[inline(always)]
    fn from(val: HfxoctrlMode) -> u8 {
        HfxoctrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcd {
    #[doc = "LFACLKLCD = LFACLK."]
    Div1 = 0x0,
    #[doc = "LFACLKLCD = LFACLK/2."]
    Div2 = 0x01,
    #[doc = "LFACLKLCD = LFACLK/4."]
    Div4 = 0x02,
    #[doc = "LFACLKLCD = LFACLK/8."]
    Div8 = 0x03,
    #[doc = "LFACLKLCD = LFACLK/16."]
    Div16 = 0x04,
    #[doc = "LFACLKLCD = LFACLK/32."]
    Div32 = 0x05,
    #[doc = "LFACLKLCD = LFACLK/64."]
    Div64 = 0x06,
    #[doc = "LFACLKLCD = LFACLK/128."]
    Div128 = 0x07,
}
impl Lcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcd {
    #[inline(always)]
    fn from(val: u8) -> Lcd {
        Lcd::from_bits(val)
    }
}
impl From<Lcd> for u8 {
    #[inline(always)]
    fn from(val: Lcd) -> u8 {
        Lcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lesense {
    #[doc = "LFACLKLESENSE = LFACLK."]
    Div1 = 0x0,
    #[doc = "LFACLKLESENSE = LFACLK/2."]
    Div2 = 0x01,
    #[doc = "LFACLKLESENSE = LFACLK/4."]
    Div4 = 0x02,
    #[doc = "LFACLKLESENSE = LFACLK/8."]
    Div8 = 0x03,
}
impl Lesense {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lesense {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lesense {
    #[inline(always)]
    fn from(val: u8) -> Lesense {
        Lesense::from_bits(val)
    }
}
impl From<Lesense> for u8 {
    #[inline(always)]
    fn from(val: Lesense) -> u8 {
        Lesense::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Letimer0 {
    #[doc = "LFACLKLETIMER0 = LFACLK."]
    Div1 = 0x0,
    #[doc = "LFACLKLETIMER0 = LFACLK/2."]
    Div2 = 0x01,
    #[doc = "LFACLKLETIMER0 = LFACLK/4."]
    Div4 = 0x02,
    #[doc = "LFACLKLETIMER0 = LFACLK/8."]
    Div8 = 0x03,
    #[doc = "LFACLKLETIMER0 = LFACLK/16."]
    Div16 = 0x04,
    #[doc = "LFACLKLETIMER0 = LFACLK/32."]
    Div32 = 0x05,
    #[doc = "LFACLKLETIMER0 = LFACLK/64."]
    Div64 = 0x06,
    #[doc = "LFACLKLETIMER0 = LFACLK/128."]
    Div128 = 0x07,
    #[doc = "LFACLKLETIMER0 = LFACLK/256."]
    Div256 = 0x08,
    #[doc = "LFACLKLETIMER0 = LFACLK/512."]
    Div512 = 0x09,
    #[doc = "LFACLKLETIMER0 = LFACLK/1024."]
    Div1024 = 0x0a,
    #[doc = "LFACLKLETIMER0 = LFACLK/2048."]
    Div2048 = 0x0b,
    #[doc = "LFACLKLETIMER0 = LFACLK/4096."]
    Div4096 = 0x0c,
    #[doc = "LFACLKLETIMER0 = LFACLK/8192."]
    Div8192 = 0x0d,
    #[doc = "LFACLKLETIMER0 = LFACLK/16384."]
    Div16384 = 0x0e,
    #[doc = "LFACLKLETIMER0 = LFACLK/32768."]
    Div32768 = 0x0f,
}
impl Letimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Letimer0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Letimer0 {
    #[inline(always)]
    fn from(val: u8) -> Letimer0 {
        Letimer0::from_bits(val)
    }
}
impl From<Letimer0> for u8 {
    #[inline(always)]
    fn from(val: Letimer0) -> u8 {
        Letimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Letimer1 {
    #[doc = "LFACLKLETIMER1 = LFACLK."]
    Div1 = 0x0,
    #[doc = "LFACLKLETIMER1 = LFACLK/2."]
    Div2 = 0x01,
    #[doc = "LFACLKLETIMER1 = LFACLK/4."]
    Div4 = 0x02,
    #[doc = "LFACLKLETIMER1 = LFACLK/8."]
    Div8 = 0x03,
    #[doc = "LFACLKLETIMER1 = LFACLK/16."]
    Div16 = 0x04,
    #[doc = "LFACLKLETIMER1 = LFACLK/32."]
    Div32 = 0x05,
    #[doc = "LFACLKLETIMER1 = LFACLK/64."]
    Div64 = 0x06,
    #[doc = "LFACLKLETIMER1 = LFACLK/128."]
    Div128 = 0x07,
    #[doc = "LFACLKLETIMER1 = LFACLK/256."]
    Div256 = 0x08,
    #[doc = "LFACLKLETIMER1 = LFACLK/512."]
    Div512 = 0x09,
    #[doc = "LFACLKLETIMER1 = LFACLK/1024."]
    Div1024 = 0x0a,
    #[doc = "LFACLKLETIMER1 = LFACLK/2048."]
    Div2048 = 0x0b,
    #[doc = "LFACLKLETIMER1 = LFACLK/4096."]
    Div4096 = 0x0c,
    #[doc = "LFACLKLETIMER1 = LFACLK/8192."]
    Div8192 = 0x0d,
    #[doc = "LFACLKLETIMER1 = LFACLK/16384."]
    Div16384 = 0x0e,
    #[doc = "LFACLKLETIMER1 = LFACLK/32768."]
    Div32768 = 0x0f,
}
impl Letimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Letimer1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Letimer1 {
    #[inline(always)]
    fn from(val: u8) -> Letimer1 {
        Letimer1::from_bits(val)
    }
}
impl From<Letimer1> for u8 {
    #[inline(always)]
    fn from(val: Letimer1) -> u8 {
        Letimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Leuart0 {
    #[doc = "LFBCLKLEUART0 = LFBCLK."]
    Div1 = 0x0,
    #[doc = "LFBCLKLEUART0 = LFBCLK/2."]
    Div2 = 0x01,
    #[doc = "LFBCLKLEUART0 = LFBCLK/4."]
    Div4 = 0x02,
    #[doc = "LFBCLKLEUART0 = LFBCLK/8."]
    Div8 = 0x03,
}
impl Leuart0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Leuart0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Leuart0 {
    #[inline(always)]
    fn from(val: u8) -> Leuart0 {
        Leuart0::from_bits(val)
    }
}
impl From<Leuart0> for u8 {
    #[inline(always)]
    fn from(val: Leuart0) -> u8 {
        Leuart0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Leuart1 {
    #[doc = "LFBCLKLEUART1 = LFBCLK."]
    Div1 = 0x0,
    #[doc = "LFBCLKLEUART1 = LFBCLK/2."]
    Div2 = 0x01,
    #[doc = "LFBCLKLEUART1 = LFBCLK/4."]
    Div4 = 0x02,
    #[doc = "LFBCLKLEUART1 = LFBCLK/8."]
    Div8 = 0x03,
}
impl Leuart1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Leuart1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Leuart1 {
    #[inline(always)]
    fn from(val: u8) -> Leuart1 {
        Leuart1::from_bits(val)
    }
}
impl From<Leuart1> for u8 {
    #[inline(always)]
    fn from(val: Leuart1) -> u8 {
        Leuart1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lfa {
    #[doc = "LFACLK is disabled."]
    Disabled = 0x0,
    #[doc = "LFRCO selected as LFACLK."]
    Lfrco = 0x01,
    #[doc = "LFXO selected as LFACLK."]
    Lfxo = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ULFRCO selected as LFACLK."]
    Ulfrco = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfa {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfa {
    #[inline(always)]
    fn from(val: u8) -> Lfa {
        Lfa::from_bits(val)
    }
}
impl From<Lfa> for u8 {
    #[inline(always)]
    fn from(val: Lfa) -> u8 {
        Lfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lfb {
    #[doc = "LFBCLK is disabled."]
    Disabled = 0x0,
    #[doc = "LFRCO selected as LFBCLK."]
    Lfrco = 0x01,
    #[doc = "LFXO selected as LFBCLK."]
    Lfxo = 0x02,
    #[doc = "HFCLK divided by two/four is selected as LFBCLK."]
    Hfclkle = 0x03,
    #[doc = "ULFRCO selected as LFBCLK."]
    Ulfrco = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lfb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfb {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfb {
    #[inline(always)]
    fn from(val: u8) -> Lfb {
        Lfb::from_bits(val)
    }
}
impl From<Lfb> for u8 {
    #[inline(always)]
    fn from(val: Lfb) -> u8 {
        Lfb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lfc {
    #[doc = "LFCCLK is disabled."]
    Disabled = 0x0,
    #[doc = "LFRCO selected as LFCCLK."]
    Lfrco = 0x01,
    #[doc = "LFXO selected as LFCCLK."]
    Lfxo = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ULFRCO selected as LFCCLK."]
    Ulfrco = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfc {
    #[inline(always)]
    fn from(val: u8) -> Lfc {
        Lfc::from_bits(val)
    }
}
impl From<Lfc> for u8 {
    #[inline(always)]
    fn from(val: Lfc) -> u8 {
        Lfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lfe {
    #[doc = "LFECLK is disabled."]
    Disabled = 0x0,
    #[doc = "LFRCO selected as LFECLK."]
    Lfrco = 0x01,
    #[doc = "LFXO selected as LFECLK."]
    Lfxo = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ULFRCO selected as LFECLK."]
    Ulfrco = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfe {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfe {
    #[inline(always)]
    fn from(val: u8) -> Lfe {
        Lfe::from_bits(val)
    }
}
impl From<Lfe> for u8 {
    #[inline(always)]
    fn from(val: Lfe) -> u8 {
        Lfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LfrcoctrlTimeout {
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x0,
    #[doc = "Timeout period of 16 cycles."]
    _16cycles = 0x01,
    #[doc = "Timeout period of 32 cycles."]
    _32cycles = 0x02,
    _RESERVED_3 = 0x03,
}
impl LfrcoctrlTimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfrcoctrlTimeout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfrcoctrlTimeout {
    #[inline(always)]
    fn from(val: u8) -> LfrcoctrlTimeout {
        LfrcoctrlTimeout::from_bits(val)
    }
}
impl From<LfrcoctrlTimeout> for u8 {
    #[inline(always)]
    fn from(val: LfrcoctrlTimeout) -> u8 {
        LfrcoctrlTimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lftimeout {
    #[doc = "Timeout period of 0 cycles (disabled)."]
    _0cycles = 0x0,
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x01,
    #[doc = "Timeout period of 4 cycles."]
    _4cycles = 0x02,
    #[doc = "Timeout period of 16 cycles."]
    _16cycles = 0x03,
    #[doc = "Timeout period of 32 cycles."]
    _32cycles = 0x04,
    #[doc = "Timeout period of 64 cycles."]
    _64cycles = 0x05,
    #[doc = "Timeout period of 1024 cycles."]
    _1kcycles = 0x06,
    #[doc = "Timeout period of 4096 cycles."]
    _4kcycles = 0x07,
}
impl Lftimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lftimeout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lftimeout {
    #[inline(always)]
    fn from(val: u8) -> Lftimeout {
        Lftimeout::from_bits(val)
    }
}
impl From<Lftimeout> for u8 {
    #[inline(always)]
    fn from(val: Lftimeout) -> u8 {
        Lftimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LfxoctrlMode {
    #[doc = "32768 Hz crystal oscillator."]
    Xtal = 0x0,
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    Bufextclk = 0x01,
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    Digextclk = 0x02,
    _RESERVED_3 = 0x03,
}
impl LfxoctrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfxoctrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfxoctrlMode {
    #[inline(always)]
    fn from(val: u8) -> LfxoctrlMode {
        LfxoctrlMode::from_bits(val)
    }
}
impl From<LfxoctrlMode> for u8 {
    #[inline(always)]
    fn from(val: LfxoctrlMode) -> u8 {
        LfxoctrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LfxoctrlTimeout {
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x0,
    #[doc = "Timeout period of 256 cycles."]
    _256cycles = 0x01,
    #[doc = "Timeout period of 1024 cycles."]
    _1kcycles = 0x02,
    #[doc = "Timeout period of 2048 cycles."]
    _2kcycles = 0x03,
    #[doc = "Timeout period of 4096 cycles."]
    _4kcycles = 0x04,
    #[doc = "Timeout period of 8192 cycles."]
    _8kcycles = 0x05,
    #[doc = "Timeout period of 16384 cycles."]
    _16kcycles = 0x06,
    #[doc = "Timeout period of 32768 cycles."]
    _32kcycles = 0x07,
}
impl LfxoctrlTimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfxoctrlTimeout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfxoctrlTimeout {
    #[inline(always)]
    fn from(val: u8) -> LfxoctrlTimeout {
        LfxoctrlTimeout::from_bits(val)
    }
}
impl From<LfxoctrlTimeout> for u8 {
    #[inline(always)]
    fn from(val: LfxoctrlTimeout) -> u8 {
        LfxoctrlTimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peakdetmode {
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    Oncecmd = 0x0,
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    Autocmd = 0x01,
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    Cmd = 0x02,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    Manual = 0x03,
}
impl Peakdetmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peakdetmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peakdetmode {
    #[inline(always)]
    fn from(val: u8) -> Peakdetmode {
        Peakdetmode::from_bits(val)
    }
}
impl From<Peakdetmode> for u8 {
    #[inline(always)]
    fn from(val: Peakdetmode) -> u8 {
        Peakdetmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peakdetthr {
    #[doc = "50mV amplitude detection level."]
    Thr0 = 0x0,
    #[doc = "75mV amplitude detection level."]
    Thr1 = 0x01,
    #[doc = "115mV amplitude detection level."]
    Thr2 = 0x02,
    #[doc = "160mV amplitude detection level."]
    Thr3 = 0x03,
    #[doc = "220mV amplitude detection level."]
    Thr4 = 0x04,
    #[doc = "260mV amplitude detection level."]
    Thr5 = 0x05,
    #[doc = "320mV amplitude detection level."]
    Thr6 = 0x06,
    #[doc = "Same as THR6."]
    Thr7 = 0x07,
}
impl Peakdetthr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peakdetthr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peakdetthr {
    #[inline(always)]
    fn from(val: u8) -> Peakdetthr {
        Peakdetthr::from_bits(val)
    }
}
impl From<Peakdetthr> for u8 {
    #[inline(always)]
    fn from(val: Peakdetthr) -> u8 {
        Peakdetthr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peakdettimeout {
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x0,
    #[doc = "Timeout period of 4 cycles."]
    _4cycles = 0x01,
    #[doc = "Timeout period of 16 cycles."]
    _16cycles = 0x02,
    #[doc = "Timeout period of 32 cycles."]
    _32cycles = 0x03,
    #[doc = "Timeout period of 64 cycles."]
    _64cycles = 0x04,
    #[doc = "Timeout period of 128 cycles."]
    _128cycles = 0x05,
    #[doc = "Timeout period of 256 cycles."]
    _256cycles = 0x06,
    #[doc = "Timeout period of 1024 cycles."]
    _1kcycles = 0x07,
    #[doc = "Timeout period of 2048 cycles."]
    _2kcycles = 0x08,
    #[doc = "Timeout period of 4096 cycles."]
    _4kcycles = 0x09,
    #[doc = "Timeout period of 8192 cycles."]
    _8kcycles = 0x0a,
    #[doc = "Timeout period of 16384 cycles."]
    _16kcycles = 0x0b,
    #[doc = "Timeout period of 32768 cycles."]
    _32kcycles = 0x0c,
    #[doc = "Timeout period of 65536 cycles."]
    _64kcycles = 0x0d,
    #[doc = "Timeout period of 131072 cycles."]
    _128kcycles = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Peakdettimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peakdettimeout {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peakdettimeout {
    #[inline(always)]
    fn from(val: u8) -> Peakdettimeout {
        Peakdettimeout::from_bits(val)
    }
}
impl From<Peakdettimeout> for u8 {
    #[inline(always)]
    fn from(val: Peakdettimeout) -> u8 {
        Peakdettimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qspi0clksel {
    #[doc = "HFRCO clock is used to clock QSPI0."]
    Hfrco = 0x0,
    #[doc = "HFXO clock is used to clock QSPI0."]
    Hfxo = 0x01,
    #[doc = "AUXHFRCO is used to clock QSPI0."]
    Auxhfrco = 0x02,
    #[doc = "USHFRCO is used to clock QSPI0."]
    Ushfrco = 0x03,
}
impl Qspi0clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qspi0clksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qspi0clksel {
    #[inline(always)]
    fn from(val: u8) -> Qspi0clksel {
        Qspi0clksel::from_bits(val)
    }
}
impl From<Qspi0clksel> for u8 {
    #[inline(always)]
    fn from(val: Qspi0clksel) -> u8 {
        Qspi0clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "HFXO selected."]
    Hfxo = 0x0,
    #[doc = "LFXO selected."]
    Lfxo = 0x01,
    #[doc = "USHFRCO selected."]
    Ushfrco = 0x02,
    #[doc = "CLKIN0 selected."]
    Clkin0 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "LFACLKRTC = LFACLK."]
    Div1 = 0x0,
    #[doc = "LFACLKRTC = LFACLK/2."]
    Div2 = 0x01,
    #[doc = "LFACLKRTC = LFACLK/4."]
    Div4 = 0x02,
    #[doc = "LFACLKRTC = LFACLK/8."]
    Div8 = 0x03,
    #[doc = "LFACLKRTC = LFACLK/16."]
    Div16 = 0x04,
    #[doc = "LFACLKRTC = LFACLK/32."]
    Div32 = 0x05,
    #[doc = "LFACLKRTC = LFACLK/64."]
    Div64 = 0x06,
    #[doc = "LFACLKRTC = LFACLK/128."]
    Div128 = 0x07,
    #[doc = "LFACLKRTC = LFACLK/256."]
    Div256 = 0x08,
    #[doc = "LFACLKRTC = LFACLK/512."]
    Div512 = 0x09,
    #[doc = "LFACLKRTC = LFACLK/1024."]
    Div1024 = 0x0a,
    #[doc = "LFACLKRTC = LFACLK/2048."]
    Div2048 = 0x0b,
    #[doc = "LFACLKRTC = LFACLK/4096."]
    Div4096 = 0x0c,
    #[doc = "LFACLKRTC = LFACLK/8192."]
    Div8192 = 0x0d,
    #[doc = "LFACLKRTC = LFACLK/16384."]
    Div16384 = 0x0e,
    #[doc = "LFACLKRTC = LFACLK/32768."]
    Div32768 = 0x0f,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcc {
    #[doc = "LFECLKRTCC = LFECLK."]
    Div1 = 0x0,
    #[doc = "LFECLKRTCC = LFECLK/2."]
    Div2 = 0x01,
    #[doc = "LFECLKRTCC = LFECLK/4."]
    Div4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rtcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcc {
    #[inline(always)]
    fn from(val: u8) -> Rtcc {
        Rtcc::from_bits(val)
    }
}
impl From<Rtcc> for u8 {
    #[inline(always)]
    fn from(val: Rtcc) -> u8 {
        Rtcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdioclksel {
    #[doc = "HFRCO clock is used to clock SDIO."]
    Hfrco = 0x0,
    #[doc = "HFXO clock is used to clock SDIO."]
    Hfxo = 0x01,
    #[doc = "AUXHFRCO is used to clock SDIO."]
    Auxhfrco = 0x02,
    #[doc = "USHFRCO is used to clock SDIO."]
    Ushfrco = 0x03,
}
impl Sdioclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdioclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdioclksel {
    #[inline(always)]
    fn from(val: u8) -> Sdioclksel {
        Sdioclksel::from_bits(val)
    }
}
impl From<Sdioclksel> for u8 {
    #[inline(always)]
    fn from(val: Sdioclksel) -> u8 {
        Sdioclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selected {
    _RESERVED_0 = 0x0,
    #[doc = "HFRCO is selected as HFCLK clock source."]
    Hfrco = 0x01,
    #[doc = "HFXO is selected as HFCLK clock source."]
    Hfxo = 0x02,
    #[doc = "LFRCO is selected as HFCLK clock source."]
    Lfrco = 0x03,
    #[doc = "LFXO is selected as HFCLK clock source."]
    Lfxo = 0x04,
    #[doc = "HFRCO divided by 2 is selected as HFCLK clock source."]
    Hfrcodiv2 = 0x05,
    #[doc = "USHFRCO is selected as HFCLK clock source."]
    Ushfrco = 0x06,
    #[doc = "CLKIN0 is selected as HFCLK clock source."]
    Clkin0 = 0x07,
}
impl Selected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selected {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selected {
    #[inline(always)]
    fn from(val: u8) -> Selected {
        Selected::from_bits(val)
    }
}
impl From<Selected> for u8 {
    #[inline(always)]
    fn from(val: Selected) -> u8 {
        Selected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Startuptimeout {
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x0,
    #[doc = "Timeout period of 4 cycles."]
    _4cycles = 0x01,
    #[doc = "Timeout period of 16 cycles."]
    _16cycles = 0x02,
    #[doc = "Timeout period of 32 cycles."]
    _32cycles = 0x03,
    #[doc = "Timeout period of 64 cycles."]
    _64cycles = 0x04,
    #[doc = "Timeout period of 128 cycles."]
    _128cycles = 0x05,
    #[doc = "Timeout period of 256 cycles."]
    _256cycles = 0x06,
    #[doc = "Timeout period of 1024 cycles."]
    _1kcycles = 0x07,
    #[doc = "Timeout period of 2048 cycles."]
    _2kcycles = 0x08,
    #[doc = "Timeout period of 4096 cycles."]
    _4kcycles = 0x09,
    #[doc = "Timeout period of 8192 cycles."]
    _8kcycles = 0x0a,
    #[doc = "Timeout period of 16384 cycles."]
    _16kcycles = 0x0b,
    #[doc = "Timeout period of 32768 cycles."]
    _32kcycles = 0x0c,
    #[doc = "Timeout period of 65536 cycles."]
    _64kcycles = 0x0d,
    #[doc = "Timeout period of 131072 cycles."]
    _128kcycles = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Startuptimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Startuptimeout {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Startuptimeout {
    #[inline(always)]
    fn from(val: u8) -> Startuptimeout {
        Startuptimeout::from_bits(val)
    }
}
impl From<Startuptimeout> for u8 {
    #[inline(always)]
    fn from(val: Startuptimeout) -> u8 {
        Startuptimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Steadytimeout {
    #[doc = "Timeout period of 2 cycles."]
    _2cycles = 0x0,
    #[doc = "Timeout period of 4 cycles."]
    _4cycles = 0x01,
    #[doc = "Timeout period of 16 cycles."]
    _16cycles = 0x02,
    #[doc = "Timeout period of 32 cycles."]
    _32cycles = 0x03,
    #[doc = "Timeout period of 64 cycles."]
    _64cycles = 0x04,
    #[doc = "Timeout period of 128 cycles."]
    _128cycles = 0x05,
    #[doc = "Timeout period of 256 cycles."]
    _256cycles = 0x06,
    #[doc = "Timeout period of 1024 cycles."]
    _1kcycles = 0x07,
    #[doc = "Timeout period of 2048 cycles."]
    _2kcycles = 0x08,
    #[doc = "Timeout period of 4096 cycles."]
    _4kcycles = 0x09,
    #[doc = "Timeout period of 8192 cycles."]
    _8kcycles = 0x0a,
    #[doc = "Timeout period of 16384 cycles."]
    _16kcycles = 0x0b,
    #[doc = "Timeout period of 32768 cycles."]
    _32kcycles = 0x0c,
    #[doc = "Timeout period of 65536 cycles."]
    _64kcycles = 0x0d,
    #[doc = "Timeout period of 131072 cycles."]
    _128kcycles = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Steadytimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Steadytimeout {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Steadytimeout {
    #[inline(always)]
    fn from(val: u8) -> Steadytimeout {
        Steadytimeout::from_bits(val)
    }
}
impl From<Steadytimeout> for u8 {
    #[inline(always)]
    fn from(val: Steadytimeout) -> u8 {
        Steadytimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systick {
    #[doc = "LFBCLKSYSTICK = LFBCLK."]
    Div1 = 0x0,
    _RESERVED_1 = 0x01,
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
}
impl Systick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systick {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systick {
    #[inline(always)]
    fn from(val: u8) -> Systick {
        Systick::from_bits(val)
    }
}
impl From<Systick> for u8 {
    #[inline(always)]
    fn from(val: Systick) -> u8 {
        Systick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upsel {
    #[doc = "Select HFXO as up-counter."]
    Hfxo = 0x0,
    #[doc = "Select LFXO as up-counter."]
    Lfxo = 0x01,
    #[doc = "Select HFRCO as up-counter."]
    Hfrco = 0x02,
    #[doc = "Select LFRCO as up-counter."]
    Lfrco = 0x03,
    #[doc = "Select AUXHFRCO as up-counter."]
    Auxhfrco = 0x04,
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter."]
    Prs = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select USHFRCO as up-counter."]
    Ushfrco = 0x07,
}
impl Upsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upsel {
    #[inline(always)]
    fn from(val: u8) -> Upsel {
        Upsel::from_bits(val)
    }
}
impl From<Upsel> for u8 {
    #[inline(always)]
    fn from(val: Upsel) -> u8 {
        Upsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbclksel {
    #[doc = "USHFRCO (clock recovery) is clocking USB."]
    Ushfrco = 0x0,
    #[doc = "HFXO clock is used to clock USB."]
    Hfxo = 0x01,
    #[doc = "HFXO clock doubler is used to clock USB."]
    Hfxox2 = 0x02,
    #[doc = "HFRCO clock is used to clock USB."]
    Hfrco = 0x03,
    #[doc = "LFXO clock is used to clock USB."]
    Lfxo = 0x04,
    #[doc = "LFRCO clock is used to clock USB."]
    Lfrco = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Usbclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbclksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbclksel {
    #[inline(always)]
    fn from(val: u8) -> Usbclksel {
        Usbclksel::from_bits(val)
    }
}
impl From<Usbclksel> for u8 {
    #[inline(always)]
    fn from(val: Usbclksel) -> u8 {
        Usbclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UshfrcoctrlClkdiv {
    #[doc = "Divide by 1."]
    Div1 = 0x0,
    #[doc = "Divide by 2."]
    Div2 = 0x01,
    #[doc = "Divide by 4."]
    Div4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl UshfrcoctrlClkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UshfrcoctrlClkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UshfrcoctrlClkdiv {
    #[inline(always)]
    fn from(val: u8) -> UshfrcoctrlClkdiv {
        UshfrcoctrlClkdiv::from_bits(val)
    }
}
impl From<UshfrcoctrlClkdiv> for u8 {
    #[inline(always)]
    fn from(val: UshfrcoctrlClkdiv) -> u8 {
        UshfrcoctrlClkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrefupdate {
    #[doc = "32 clocks."]
    _32cycles = 0x0,
    #[doc = "64 clocks."]
    _64cycles = 0x01,
    #[doc = "128 clocks."]
    _128cycles = 0x02,
    #[doc = "256 clocks."]
    _256cycles = 0x03,
}
impl Vrefupdate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrefupdate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrefupdate {
    #[inline(always)]
    fn from(val: u8) -> Vrefupdate {
        Vrefupdate::from_bits(val)
    }
}
impl From<Vrefupdate> for u8 {
    #[inline(always)]
    fn from(val: Vrefupdate) -> u8 {
        Vrefupdate::to_bits(val)
    }
}
