#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0loc {
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
impl Ch0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0loc {
    #[inline(always)]
    fn from(val: u8) -> Ch0loc {
        Ch0loc::from_bits(val)
    }
}
impl From<Ch0loc> for u8 {
    #[inline(always)]
    fn from(val: Ch0loc) -> u8 {
        Ch0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10loc {
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
impl Ch10loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10loc {
    #[inline(always)]
    fn from(val: u8) -> Ch10loc {
        Ch10loc::from_bits(val)
    }
}
impl From<Ch10loc> for u8 {
    #[inline(always)]
    fn from(val: Ch10loc) -> u8 {
        Ch10loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11loc {
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
impl Ch11loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11loc {
    #[inline(always)]
    fn from(val: u8) -> Ch11loc {
        Ch11loc::from_bits(val)
    }
}
impl From<Ch11loc> for u8 {
    #[inline(always)]
    fn from(val: Ch11loc) -> u8 {
        Ch11loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1loc {
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
impl Ch1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1loc {
    #[inline(always)]
    fn from(val: u8) -> Ch1loc {
        Ch1loc::from_bits(val)
    }
}
impl From<Ch1loc> for u8 {
    #[inline(always)]
    fn from(val: Ch1loc) -> u8 {
        Ch1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2loc {
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
impl Ch2loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2loc {
    #[inline(always)]
    fn from(val: u8) -> Ch2loc {
        Ch2loc::from_bits(val)
    }
}
impl From<Ch2loc> for u8 {
    #[inline(always)]
    fn from(val: Ch2loc) -> u8 {
        Ch2loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3loc {
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
impl Ch3loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3loc {
    #[inline(always)]
    fn from(val: u8) -> Ch3loc {
        Ch3loc::from_bits(val)
    }
}
impl From<Ch3loc> for u8 {
    #[inline(always)]
    fn from(val: Ch3loc) -> u8 {
        Ch3loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4loc {
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
impl Ch4loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4loc {
    #[inline(always)]
    fn from(val: u8) -> Ch4loc {
        Ch4loc::from_bits(val)
    }
}
impl From<Ch4loc> for u8 {
    #[inline(always)]
    fn from(val: Ch4loc) -> u8 {
        Ch4loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5loc {
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
impl Ch5loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5loc {
    #[inline(always)]
    fn from(val: u8) -> Ch5loc {
        Ch5loc::from_bits(val)
    }
}
impl From<Ch5loc> for u8 {
    #[inline(always)]
    fn from(val: Ch5loc) -> u8 {
        Ch5loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6loc {
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
impl Ch6loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6loc {
    #[inline(always)]
    fn from(val: u8) -> Ch6loc {
        Ch6loc::from_bits(val)
    }
}
impl From<Ch6loc> for u8 {
    #[inline(always)]
    fn from(val: Ch6loc) -> u8 {
        Ch6loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7loc {
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
impl Ch7loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7loc {
    #[inline(always)]
    fn from(val: u8) -> Ch7loc {
        Ch7loc::from_bits(val)
    }
}
impl From<Ch7loc> for u8 {
    #[inline(always)]
    fn from(val: Ch7loc) -> u8 {
        Ch7loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8loc {
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
impl Ch8loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8loc {
    #[inline(always)]
    fn from(val: u8) -> Ch8loc {
        Ch8loc::from_bits(val)
    }
}
impl From<Ch8loc> for u8 {
    #[inline(always)]
    fn from(val: Ch8loc) -> u8 {
        Ch8loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9loc {
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
impl Ch9loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9loc {
    #[inline(always)]
    fn from(val: u8) -> Ch9loc {
        Ch9loc::from_bits(val)
    }
}
impl From<Ch9loc> for u8 {
    #[inline(always)]
    fn from(val: Ch9loc) -> u8 {
        Ch9loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChCtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl ChCtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChCtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChCtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> ChCtrlEdsel {
        ChCtrlEdsel::from_bits(val)
    }
}
impl From<ChCtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: ChCtrlEdsel) -> u8 {
        ChCtrlEdsel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChCtrlSourcesel(u8);
impl ChCtrlSourcesel {
    #[doc = "No source selected."]
    pub const None: Self = Self(0x0);
    #[doc = "Peripheral Reflex System."]
    pub const Prsl: Self = Self(0x01);
    #[doc = "Peripheral Reflex System."]
    pub const Prsh: Self = Self(0x02);
    #[doc = "Analog Comparator 0."]
    pub const Acmp0: Self = Self(0x06);
    #[doc = "Analog Comparator 1."]
    pub const Acmp1: Self = Self(0x07);
    #[doc = "Analog to Digital Converter 0."]
    pub const Adc0: Self = Self(0x08);
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    pub const Usart0: Self = Self(0x10);
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    pub const Usart1: Self = Self(0x11);
    #[doc = "Timer 0."]
    pub const Timer0: Self = Self(0x1c);
    #[doc = "Timer 1."]
    pub const Timer1: Self = Self(0x1d);
    #[doc = "Real-Time Counter and Calendar."]
    pub const Rtcc: Self = Self(0x29);
    #[doc = "General purpose Input/Output."]
    pub const Gpiol: Self = Self(0x30);
    #[doc = "General purpose Input/Output."]
    pub const Gpioh: Self = Self(0x31);
    #[doc = "Low Energy Timer 0."]
    pub const Letimer0: Self = Self(0x34);
    #[doc = "Pulse Counter 0."]
    pub const Pcnt0: Self = Self(0x36);
    #[doc = "CRYOTIMER."]
    pub const Cryotimer: Self = Self(0x3c);
    #[doc = "Clock Management Unit."]
    pub const Cmu: Self = Self(0x3d);
    pub const Cm4: Self = Self(0x43);
}
impl ChCtrlSourcesel {
    pub const fn from_bits(val: u8) -> ChCtrlSourcesel {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChCtrlSourcesel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("Prsl"),
            0x02 => f.write_str("Prsh"),
            0x06 => f.write_str("Acmp0"),
            0x07 => f.write_str("Acmp1"),
            0x08 => f.write_str("Adc0"),
            0x10 => f.write_str("Usart0"),
            0x11 => f.write_str("Usart1"),
            0x1c => f.write_str("Timer0"),
            0x1d => f.write_str("Timer1"),
            0x29 => f.write_str("Rtcc"),
            0x30 => f.write_str("Gpiol"),
            0x31 => f.write_str("Gpioh"),
            0x34 => f.write_str("Letimer0"),
            0x36 => f.write_str("Pcnt0"),
            0x3c => f.write_str("Cryotimer"),
            0x3d => f.write_str("Cmu"),
            0x43 => f.write_str("Cm4"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCtrlSourcesel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "Prsl"),
            0x02 => defmt::write!(f, "Prsh"),
            0x06 => defmt::write!(f, "Acmp0"),
            0x07 => defmt::write!(f, "Acmp1"),
            0x08 => defmt::write!(f, "Adc0"),
            0x10 => defmt::write!(f, "Usart0"),
            0x11 => defmt::write!(f, "Usart1"),
            0x1c => defmt::write!(f, "Timer0"),
            0x1d => defmt::write!(f, "Timer1"),
            0x29 => defmt::write!(f, "Rtcc"),
            0x30 => defmt::write!(f, "Gpiol"),
            0x31 => defmt::write!(f, "Gpioh"),
            0x34 => defmt::write!(f, "Letimer0"),
            0x36 => defmt::write!(f, "Pcnt0"),
            0x3c => defmt::write!(f, "Cryotimer"),
            0x3d => defmt::write!(f, "Cmu"),
            0x43 => defmt::write!(f, "Cm4"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChCtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> ChCtrlSourcesel {
        ChCtrlSourcesel::from_bits(val)
    }
}
impl From<ChCtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: ChCtrlSourcesel) -> u8 {
        ChCtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmareq0Prssel {
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
impl Dmareq0Prssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmareq0Prssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmareq0Prssel {
    #[inline(always)]
    fn from(val: u8) -> Dmareq0Prssel {
        Dmareq0Prssel::from_bits(val)
    }
}
impl From<Dmareq0Prssel> for u8 {
    #[inline(always)]
    fn from(val: Dmareq0Prssel) -> u8 {
        Dmareq0Prssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmareq1Prssel {
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
impl Dmareq1Prssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmareq1Prssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmareq1Prssel {
    #[inline(always)]
    fn from(val: u8) -> Dmareq1Prssel {
        Dmareq1Prssel::from_bits(val)
    }
}
impl From<Dmareq1Prssel> for u8 {
    #[inline(always)]
    fn from(val: Dmareq1Prssel) -> u8 {
        Dmareq1Prssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sevonprssel {
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
impl Sevonprssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sevonprssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sevonprssel {
    #[inline(always)]
    fn from(val: u8) -> Sevonprssel {
        Sevonprssel::from_bits(val)
    }
}
impl From<Sevonprssel> for u8 {
    #[inline(always)]
    fn from(val: Sevonprssel) -> u8 {
        Sevonprssel::to_bits(val)
    }
}
