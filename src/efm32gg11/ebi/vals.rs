#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Adloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adloc {
    #[inline(always)]
    fn from(val: u8) -> Adloc {
        Adloc::from_bits(val)
    }
}
impl From<Adloc> for u8 {
    #[inline(always)]
    fn from(val: Adloc) -> u8 {
        Adloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alb {
    #[doc = "Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    A0 = 0x0,
    #[doc = "Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    A8 = 0x01,
    #[doc = "Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    A16 = 0x02,
    #[doc = "Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    A24 = 0x03,
}
impl Alb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alb {
    #[inline(always)]
    fn from(val: u8) -> Alb {
        Alb::from_bits(val)
    }
}
impl From<Alb> for u8 {
    #[inline(always)]
    fn from(val: Alb) -> u8 {
        Alb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aliasbank {
    #[doc = "Graphic Bank Select is alias to Bank Select 0."]
    Aliasbank0 = 0x0,
    #[doc = "Graphic Bank Select is alias to Bank Select 1."]
    Aliasbank1 = 0x01,
    #[doc = "Graphic Bank Select is alias to Bank Select 2."]
    Aliasbank2 = 0x02,
    #[doc = "Graphic Bank Select is alias to Bank Select 3."]
    Aliasbank3 = 0x03,
}
impl Aliasbank {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aliasbank {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aliasbank {
    #[inline(always)]
    fn from(val: u8) -> Aliasbank {
        Aliasbank::from_bits(val)
    }
}
impl From<Aliasbank> for u8 {
    #[inline(always)]
    fn from(val: Aliasbank) -> u8 {
        Aliasbank::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aloc {
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
impl Aloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aloc {
    #[inline(always)]
    fn from(val: u8) -> Aloc {
        Aloc::from_bits(val)
    }
}
impl From<Aloc> for u8 {
    #[inline(always)]
    fn from(val: Aloc) -> u8 {
        Aloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apen {
    #[doc = "All EBI_A pins are disabled."]
    A0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "EBI_A\\[4:L\\] pins enabled."]
    A5 = 0x05,
    #[doc = "EBI_A\\[5:L\\] pins enabled."]
    A6 = 0x06,
    #[doc = "EBI_A\\[6:L\\] pins enabled."]
    A7 = 0x07,
    #[doc = "EBI_A\\[7:L\\] pins enabled."]
    A8 = 0x08,
    #[doc = "EBI_A\\[8:L\\] pins enabled."]
    A9 = 0x09,
    #[doc = "EBI_A\\[9:L\\] pins enabled."]
    A10 = 0x0a,
    #[doc = "EBI_A\\[10:L\\] pins enabled."]
    A11 = 0x0b,
    #[doc = "EBI_A\\[11:L\\] pins enabled."]
    A12 = 0x0c,
    #[doc = "EBI_A\\[12:L\\] pins enabled."]
    A13 = 0x0d,
    #[doc = "EBI_A\\[13:L\\] pins enabled."]
    A14 = 0x0e,
    #[doc = "EBI_A\\[14:L\\] pins enabled."]
    A15 = 0x0f,
    #[doc = "EBI_A\\[15:L\\] pins enabled."]
    A16 = 0x10,
    #[doc = "EBI_A\\[16:L\\] pins enabled."]
    A17 = 0x11,
    #[doc = "EBI_A\\[17:L\\] pins enabled."]
    A18 = 0x12,
    #[doc = "EBI_A\\[18:L\\] pins enabled."]
    A19 = 0x13,
    #[doc = "EBI_A\\[19:L\\] pins enabled."]
    A20 = 0x14,
    #[doc = "EBI_A\\[20:L\\] pins enabled."]
    A21 = 0x15,
    #[doc = "EBI_A\\[21:L\\] pins enabled."]
    A22 = 0x16,
    #[doc = "EBI_A\\[22:L\\] pins enabled."]
    A23 = 0x17,
    #[doc = "EBI_A\\[23:L\\] pins enabled."]
    A24 = 0x18,
    #[doc = "EBI_A\\[24:L\\] pins enabled."]
    A25 = 0x19,
    #[doc = "EBI_A\\[25:L\\] pins enabled."]
    A26 = 0x1a,
    #[doc = "EBI_A\\[26:L\\] pins enabled."]
    A27 = 0x1b,
    #[doc = "EBI_A\\[27:L\\] pins enabled."]
    A28 = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Apen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apen {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apen {
    #[inline(always)]
    fn from(val: u8) -> Apen {
        Apen::from_bits(val)
    }
}
impl From<Apen> for u8 {
    #[inline(always)]
    fn from(val: Apen) -> u8 {
        Apen::to_bits(val)
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
pub enum Dd {
    #[doc = "Direct Drive is disabled."]
    Disabled = 0x0,
    #[doc = "Direct Drive from internal memory enabled and started."]
    Internal = 0x01,
    #[doc = "Direct Drive from external memory enabled and started."]
    External = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dd {
    #[inline(always)]
    fn from(val: u8) -> Dd {
        Dd::from_bits(val)
    }
}
impl From<Dd> for u8 {
    #[inline(always)]
    fn from(val: Dd) -> u8 {
        Dd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ebiloc {
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
impl Ebiloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ebiloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ebiloc {
    #[inline(always)]
    fn from(val: u8) -> Ebiloc {
        Ebiloc::from_bits(val)
    }
}
impl From<Ebiloc> for u8 {
    #[inline(always)]
    fn from(val: Ebiloc) -> u8 {
        Ebiloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK."]
    Unlimited = 0x0,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    Oneperdclk = 0x01,
    #[doc = "Only allow EBI accesses during TFT porches."]
    Porch = 0x02,
    _RESERVED_3 = 0x03,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maskblend {
    #[doc = "Masking and Blending are disabled."]
    Disabled = 0x0,
    #[doc = "Internal Masking is enabled."]
    Imask = 0x01,
    #[doc = "Internal Alpha Blending is enabled."]
    Ialpha = 0x02,
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    Imaskalpha = 0x03,
    #[doc = "External Frame Buffer Masking is enabled."]
    Efbmask = 0x04,
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    Efbalpha = 0x05,
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    Efbmaskalpha = 0x06,
    #[doc = "Internal Frame Buffer Masking is enabled."]
    Ifbmask = 0x07,
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    Ifbalpha = 0x08,
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    Ifbmaskalpha = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Maskblend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maskblend {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maskblend {
    #[inline(always)]
    fn from(val: u8) -> Maskblend {
        Maskblend::from_bits(val)
    }
}
impl From<Maskblend> for u8 {
    #[inline(always)]
    fn from(val: Maskblend) -> u8 {
        Maskblend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0x0,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 0x01,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 0x02,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 0x03,
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
pub enum Mode1 {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0x0,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 0x01,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 0x02,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 0x03,
}
impl Mode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode1 {
    #[inline(always)]
    fn from(val: u8) -> Mode1 {
        Mode1::from_bits(val)
    }
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(val: Mode1) -> u8 {
        Mode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode2 {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0x0,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 0x01,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 0x02,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 0x03,
}
impl Mode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode2 {
    #[inline(always)]
    fn from(val: u8) -> Mode2 {
        Mode2::from_bits(val)
    }
}
impl From<Mode2> for u8 {
    #[inline(always)]
    fn from(val: Mode2) -> u8 {
        Mode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode3 {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0x0,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 0x01,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 0x02,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 0x03,
}
impl Mode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode3 {
    #[inline(always)]
    fn from(val: u8) -> Mode3 {
        Mode3::from_bits(val)
    }
}
impl From<Mode3> for u8 {
    #[inline(always)]
    fn from(val: Mode3) -> u8 {
        Mode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NandctrlBanksel {
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    Bank0 = 0x0,
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    Bank1 = 0x01,
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    Bank2 = 0x02,
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    Bank3 = 0x03,
}
impl NandctrlBanksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NandctrlBanksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NandctrlBanksel {
    #[inline(always)]
    fn from(val: u8) -> NandctrlBanksel {
        NandctrlBanksel::from_bits(val)
    }
}
impl From<NandctrlBanksel> for u8 {
    #[inline(always)]
    fn from(val: NandctrlBanksel) -> u8 {
        NandctrlBanksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandloc {
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
impl Nandloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandloc {
    #[inline(always)]
    fn from(val: u8) -> Nandloc {
        Nandloc::from_bits(val)
    }
}
impl From<Nandloc> for u8 {
    #[inline(always)]
    fn from(val: Nandloc) -> u8 {
        Nandloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pagelen {
    #[doc = "4 members in a page."]
    Member4 = 0x0,
    #[doc = "8 members in a page."]
    Member8 = 0x01,
    #[doc = "16 members in a page."]
    Member16 = 0x02,
    #[doc = "32 members in a page."]
    Member32 = 0x03,
}
impl Pagelen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pagelen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pagelen {
    #[inline(always)]
    fn from(val: u8) -> Pagelen {
        Pagelen::from_bits(val)
    }
}
impl From<Pagelen> for u8 {
    #[inline(always)]
    fn from(val: Pagelen) -> u8 {
        Pagelen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pixel0format {
    #[doc = "ARGB data is 0555."]
    Argb0555 = 0x0,
    #[doc = "ARGB data is 0565."]
    Argb0565 = 0x01,
    #[doc = "ARGB data is 0666."]
    Argb0666 = 0x02,
    #[doc = "ARGB data is 0888."]
    Argb0888 = 0x03,
    #[doc = "ARGB data is 5555."]
    Argb5555 = 0x04,
    #[doc = "ARGB data is 6565."]
    Argb6565 = 0x05,
    #[doc = "ARGB data is 6666."]
    Argb6666 = 0x06,
    #[doc = "ARGB data is 8888."]
    Argb8888 = 0x07,
}
impl Pixel0format {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pixel0format {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pixel0format {
    #[inline(always)]
    fn from(val: u8) -> Pixel0format {
        Pixel0format::from_bits(val)
    }
}
impl From<Pixel0format> for u8 {
    #[inline(always)]
    fn from(val: Pixel0format) -> u8 {
        Pixel0format::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pixel1format {
    #[doc = "RGB data is 555."]
    Rgb555 = 0x0,
    #[doc = "RGB data is 565."]
    Rgb565 = 0x01,
    #[doc = "RGB data is 666."]
    Rgb666 = 0x02,
    #[doc = "RGB data is 888."]
    Rgb888 = 0x03,
}
impl Pixel1format {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pixel1format {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pixel1format {
    #[inline(always)]
    fn from(val: u8) -> Pixel1format {
        Pixel1format::from_bits(val)
    }
}
impl From<Pixel1format> for u8 {
    #[inline(always)]
    fn from(val: Pixel1format) -> u8 {
        Pixel1format::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdyloc {
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
impl Rdyloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdyloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdyloc {
    #[inline(always)]
    fn from(val: u8) -> Rdyloc {
        Rdyloc::from_bits(val)
    }
}
impl From<Rdyloc> for u8 {
    #[inline(always)]
    fn from(val: Rdyloc) -> u8 {
        Rdyloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TftctrlBanksel {
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank0 = 0x0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank1 = 0x01,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank2 = 0x02,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank3 = 0x03,
}
impl TftctrlBanksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TftctrlBanksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TftctrlBanksel {
    #[inline(always)]
    fn from(val: u8) -> TftctrlBanksel {
        TftctrlBanksel::from_bits(val)
    }
}
impl From<TftctrlBanksel> for u8 {
    #[inline(always)]
    fn from(val: TftctrlBanksel) -> u8 {
        TftctrlBanksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tftloc {
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
impl Tftloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tftloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tftloc {
    #[inline(always)]
    fn from(val: u8) -> Tftloc {
        Tftloc::from_bits(val)
    }
}
impl From<Tftloc> for u8 {
    #[inline(always)]
    fn from(val: Tftloc) -> u8 {
        Tftloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    #[doc = "TFT Data is 8 bit wide."]
    Byte = 0x0,
    #[doc = "TFT Data is 16 bit wide."]
    Halfword = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
