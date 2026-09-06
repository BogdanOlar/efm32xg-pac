#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etmloc {
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
impl Etmloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etmloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etmloc {
    #[inline(always)]
    fn from(val: u8) -> Etmloc {
        Etmloc::from_bits(val)
    }
}
impl From<Etmloc> for u8 {
    #[inline(always)]
    fn from(val: Etmloc) -> u8 {
        Etmloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel0 {
    #[doc = "Pin 0."]
    Pin0 = 0x0,
    #[doc = "Pin 1."]
    Pin1 = 0x01,
    #[doc = "Pin 2."]
    Pin2 = 0x02,
    #[doc = "Pin 3."]
    Pin3 = 0x03,
}
impl Extipinsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel0 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel0 {
        Extipinsel0::from_bits(val)
    }
}
impl From<Extipinsel0> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel0) -> u8 {
        Extipinsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel1 {
    #[doc = "Pin 0."]
    Pin0 = 0x0,
    #[doc = "Pin 1."]
    Pin1 = 0x01,
    #[doc = "Pin 2."]
    Pin2 = 0x02,
    #[doc = "Pin 3."]
    Pin3 = 0x03,
}
impl Extipinsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel1 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel1 {
        Extipinsel1::from_bits(val)
    }
}
impl From<Extipinsel1> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel1) -> u8 {
        Extipinsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel10 {
    #[doc = "Pin 8."]
    Pin8 = 0x0,
    #[doc = "Pin 9."]
    Pin9 = 0x01,
    #[doc = "Pin 10."]
    Pin10 = 0x02,
    #[doc = "Pin 11."]
    Pin11 = 0x03,
}
impl Extipinsel10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel10 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel10 {
        Extipinsel10::from_bits(val)
    }
}
impl From<Extipinsel10> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel10) -> u8 {
        Extipinsel10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel11 {
    #[doc = "Pin 8."]
    Pin8 = 0x0,
    #[doc = "Pin 9."]
    Pin9 = 0x01,
    #[doc = "Pin 10."]
    Pin10 = 0x02,
    #[doc = "Pin 11."]
    Pin11 = 0x03,
}
impl Extipinsel11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel11 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel11 {
        Extipinsel11::from_bits(val)
    }
}
impl From<Extipinsel11> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel11) -> u8 {
        Extipinsel11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel12 {
    #[doc = "Pin 12."]
    Pin12 = 0x0,
    #[doc = "Pin 13."]
    Pin13 = 0x01,
    #[doc = "Pin 14."]
    Pin14 = 0x02,
    #[doc = "Pin 15."]
    Pin15 = 0x03,
}
impl Extipinsel12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel12 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel12 {
        Extipinsel12::from_bits(val)
    }
}
impl From<Extipinsel12> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel12) -> u8 {
        Extipinsel12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel13 {
    #[doc = "Pin 12."]
    Pin12 = 0x0,
    #[doc = "Pin 13."]
    Pin13 = 0x01,
    #[doc = "Pin 14."]
    Pin14 = 0x02,
    #[doc = "Pin 15."]
    Pin15 = 0x03,
}
impl Extipinsel13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel13 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel13 {
        Extipinsel13::from_bits(val)
    }
}
impl From<Extipinsel13> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel13) -> u8 {
        Extipinsel13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel14 {
    #[doc = "Pin 12."]
    Pin12 = 0x0,
    #[doc = "Pin 13."]
    Pin13 = 0x01,
    #[doc = "Pin 14."]
    Pin14 = 0x02,
    #[doc = "Pin 15."]
    Pin15 = 0x03,
}
impl Extipinsel14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel14 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel14 {
        Extipinsel14::from_bits(val)
    }
}
impl From<Extipinsel14> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel14) -> u8 {
        Extipinsel14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel15 {
    #[doc = "Pin 12."]
    Pin12 = 0x0,
    #[doc = "Pin 13."]
    Pin13 = 0x01,
    #[doc = "Pin 14."]
    Pin14 = 0x02,
    #[doc = "Pin 15."]
    Pin15 = 0x03,
}
impl Extipinsel15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel15 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel15 {
        Extipinsel15::from_bits(val)
    }
}
impl From<Extipinsel15> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel15) -> u8 {
        Extipinsel15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel2 {
    #[doc = "Pin 0."]
    Pin0 = 0x0,
    #[doc = "Pin 1."]
    Pin1 = 0x01,
    #[doc = "Pin 2."]
    Pin2 = 0x02,
    #[doc = "Pin 3."]
    Pin3 = 0x03,
}
impl Extipinsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel2 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel2 {
        Extipinsel2::from_bits(val)
    }
}
impl From<Extipinsel2> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel2) -> u8 {
        Extipinsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel3 {
    #[doc = "Pin 0."]
    Pin0 = 0x0,
    #[doc = "Pin 1."]
    Pin1 = 0x01,
    #[doc = "Pin 2."]
    Pin2 = 0x02,
    #[doc = "Pin 3."]
    Pin3 = 0x03,
}
impl Extipinsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel3 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel3 {
        Extipinsel3::from_bits(val)
    }
}
impl From<Extipinsel3> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel3) -> u8 {
        Extipinsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel4 {
    #[doc = "Pin 4."]
    Pin4 = 0x0,
    #[doc = "Pin 5."]
    Pin5 = 0x01,
    #[doc = "Pin 6."]
    Pin6 = 0x02,
    #[doc = "Pin 7."]
    Pin7 = 0x03,
}
impl Extipinsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel4 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel4 {
        Extipinsel4::from_bits(val)
    }
}
impl From<Extipinsel4> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel4) -> u8 {
        Extipinsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel5 {
    #[doc = "Pin 4."]
    Pin4 = 0x0,
    #[doc = "Pin 5."]
    Pin5 = 0x01,
    #[doc = "Pin 6."]
    Pin6 = 0x02,
    #[doc = "Pin 7."]
    Pin7 = 0x03,
}
impl Extipinsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel5 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel5 {
        Extipinsel5::from_bits(val)
    }
}
impl From<Extipinsel5> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel5) -> u8 {
        Extipinsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel6 {
    #[doc = "Pin 4."]
    Pin4 = 0x0,
    #[doc = "Pin 5."]
    Pin5 = 0x01,
    #[doc = "Pin 6."]
    Pin6 = 0x02,
    #[doc = "Pin 7."]
    Pin7 = 0x03,
}
impl Extipinsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel6 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel6 {
        Extipinsel6::from_bits(val)
    }
}
impl From<Extipinsel6> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel6) -> u8 {
        Extipinsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel7 {
    #[doc = "Pin 4."]
    Pin4 = 0x0,
    #[doc = "Pin 5."]
    Pin5 = 0x01,
    #[doc = "Pin 6."]
    Pin6 = 0x02,
    #[doc = "Pin 7."]
    Pin7 = 0x03,
}
impl Extipinsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel7 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel7 {
        Extipinsel7::from_bits(val)
    }
}
impl From<Extipinsel7> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel7) -> u8 {
        Extipinsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel8 {
    #[doc = "Pin 8."]
    Pin8 = 0x0,
    #[doc = "Pin 9."]
    Pin9 = 0x01,
    #[doc = "Pin 10."]
    Pin10 = 0x02,
    #[doc = "Pin 11."]
    Pin11 = 0x03,
}
impl Extipinsel8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel8 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel8 {
        Extipinsel8::from_bits(val)
    }
}
impl From<Extipinsel8> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel8) -> u8 {
        Extipinsel8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel9 {
    #[doc = "Pin 8."]
    Pin8 = 0x0,
    #[doc = "Pin 9."]
    Pin9 = 0x01,
    #[doc = "Pin 10."]
    Pin10 = 0x02,
    #[doc = "Pin 11."]
    Pin11 = 0x03,
}
impl Extipinsel9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel9 {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel9 {
        Extipinsel9::from_bits(val)
    }
}
impl From<Extipinsel9> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel9) -> u8 {
        Extipinsel9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel0 {
    #[doc = "Port A group selected for external interrupt 0."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 0."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 0."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 0."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 0."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 0."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 0."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 0."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 0."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel0 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel0 {
        Extipsel0::from_bits(val)
    }
}
impl From<Extipsel0> for u8 {
    #[inline(always)]
    fn from(val: Extipsel0) -> u8 {
        Extipsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel1 {
    #[doc = "Port A group selected for external interrupt 1."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 1."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 1."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 1."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 1."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 1."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 1."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 1."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 1."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel1 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel1 {
        Extipsel1::from_bits(val)
    }
}
impl From<Extipsel1> for u8 {
    #[inline(always)]
    fn from(val: Extipsel1) -> u8 {
        Extipsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel10 {
    #[doc = "Port A group selected for external interrupt 10."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 10."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 10."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 10."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 10."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 10."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 10."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 10."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 10."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel10 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel10 {
        Extipsel10::from_bits(val)
    }
}
impl From<Extipsel10> for u8 {
    #[inline(always)]
    fn from(val: Extipsel10) -> u8 {
        Extipsel10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel11 {
    #[doc = "Port A group selected for external interrupt 11."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 11."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 11."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 11."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 11."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 11."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 11."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 11."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 11."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel11 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel11 {
        Extipsel11::from_bits(val)
    }
}
impl From<Extipsel11> for u8 {
    #[inline(always)]
    fn from(val: Extipsel11) -> u8 {
        Extipsel11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel12 {
    #[doc = "Port A group selected for external interrupt 12."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 12."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 12."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 12."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 12."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 12."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 12."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 12."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 12."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel12 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel12 {
        Extipsel12::from_bits(val)
    }
}
impl From<Extipsel12> for u8 {
    #[inline(always)]
    fn from(val: Extipsel12) -> u8 {
        Extipsel12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel13 {
    #[doc = "Port A group selected for external interrupt 13."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 13."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 13."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 13."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 13."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 13."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 13."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 13."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 13."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel13 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel13 {
        Extipsel13::from_bits(val)
    }
}
impl From<Extipsel13> for u8 {
    #[inline(always)]
    fn from(val: Extipsel13) -> u8 {
        Extipsel13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel14 {
    #[doc = "Port A group selected for external interrupt 14."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 14."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 14."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 14."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 14."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 14."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 14."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 14."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 14."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel14 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel14 {
        Extipsel14::from_bits(val)
    }
}
impl From<Extipsel14> for u8 {
    #[inline(always)]
    fn from(val: Extipsel14) -> u8 {
        Extipsel14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel15 {
    #[doc = "Port A group selected for external interrupt 15."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 15."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 15."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 15."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 15."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 15."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 15."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 15."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 15."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel15 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel15 {
        Extipsel15::from_bits(val)
    }
}
impl From<Extipsel15> for u8 {
    #[inline(always)]
    fn from(val: Extipsel15) -> u8 {
        Extipsel15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel2 {
    #[doc = "Port A group selected for external interrupt 2."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 2."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 2."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 2."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 2."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 2."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 2."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 2."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 2."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel2 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel2 {
        Extipsel2::from_bits(val)
    }
}
impl From<Extipsel2> for u8 {
    #[inline(always)]
    fn from(val: Extipsel2) -> u8 {
        Extipsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel3 {
    #[doc = "Port A group selected for external interrupt 3."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 3."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 3."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 3."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 3."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 3."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 3."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 3."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 3."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel3 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel3 {
        Extipsel3::from_bits(val)
    }
}
impl From<Extipsel3> for u8 {
    #[inline(always)]
    fn from(val: Extipsel3) -> u8 {
        Extipsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel4 {
    #[doc = "Port A group selected for external interrupt 4."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 4."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 4."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 4."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 4."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 4."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 4."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 4."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 4."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel4 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel4 {
        Extipsel4::from_bits(val)
    }
}
impl From<Extipsel4> for u8 {
    #[inline(always)]
    fn from(val: Extipsel4) -> u8 {
        Extipsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel5 {
    #[doc = "Port A group selected for external interrupt 5."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 5."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 5."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 5."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 5."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 5."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 5."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 5."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 5."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel5 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel5 {
        Extipsel5::from_bits(val)
    }
}
impl From<Extipsel5> for u8 {
    #[inline(always)]
    fn from(val: Extipsel5) -> u8 {
        Extipsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel6 {
    #[doc = "Port A group selected for external interrupt 6."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 6."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 6."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 6."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 6."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 6."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 6."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 6."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 6."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel6 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel6 {
        Extipsel6::from_bits(val)
    }
}
impl From<Extipsel6> for u8 {
    #[inline(always)]
    fn from(val: Extipsel6) -> u8 {
        Extipsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel7 {
    #[doc = "Port A group selected for external interrupt 7."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 7."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 7."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 7."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 7."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 7."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 7."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 7."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 7."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel7 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel7 {
        Extipsel7::from_bits(val)
    }
}
impl From<Extipsel7> for u8 {
    #[inline(always)]
    fn from(val: Extipsel7) -> u8 {
        Extipsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel8 {
    #[doc = "Port A group selected for external interrupt 8."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 8."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 8."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 8."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 8."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 8."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 8."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 8."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 8."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel8 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel8 {
        Extipsel8::from_bits(val)
    }
}
impl From<Extipsel8> for u8 {
    #[inline(always)]
    fn from(val: Extipsel8) -> u8 {
        Extipsel8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel9 {
    #[doc = "Port A group selected for external interrupt 9."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 9."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 9."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 9."]
    Portd = 0x03,
    #[doc = "Port E group selected for external interrupt 9."]
    Porte = 0x04,
    #[doc = "Port F group selected for external interrupt 9."]
    Portf = 0x05,
    #[doc = "Port G group selected for external interrupt 9."]
    Portg = 0x06,
    #[doc = "Port H group selected for external interrupt 9."]
    Porth = 0x07,
    #[doc = "Port I group selected for external interrupt 9."]
    Porti = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel9 {
    #[inline(always)]
    fn from(val: u8) -> Extipsel9 {
        Extipsel9::from_bits(val)
    }
}
impl From<Extipsel9> for u8 {
    #[inline(always)]
    fn from(val: Extipsel9) -> u8 {
        Extipsel9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode10 {
        PaModehMode10::from_bits(val)
    }
}
impl From<PaModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode10) -> u8 {
        PaModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode11 {
        PaModehMode11::from_bits(val)
    }
}
impl From<PaModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode11) -> u8 {
        PaModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode12 {
        PaModehMode12::from_bits(val)
    }
}
impl From<PaModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode12) -> u8 {
        PaModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode13 {
        PaModehMode13::from_bits(val)
    }
}
impl From<PaModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode13) -> u8 {
        PaModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode14 {
        PaModehMode14::from_bits(val)
    }
}
impl From<PaModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode14) -> u8 {
        PaModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode15 {
        PaModehMode15::from_bits(val)
    }
}
impl From<PaModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode15) -> u8 {
        PaModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode8 {
        PaModehMode8::from_bits(val)
    }
}
impl From<PaModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode8) -> u8 {
        PaModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PaModehMode9 {
        PaModehMode9::from_bits(val)
    }
}
impl From<PaModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PaModehMode9) -> u8 {
        PaModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode0 {
        PaModelMode0::from_bits(val)
    }
}
impl From<PaModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode0) -> u8 {
        PaModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode1 {
        PaModelMode1::from_bits(val)
    }
}
impl From<PaModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode1) -> u8 {
        PaModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode2 {
        PaModelMode2::from_bits(val)
    }
}
impl From<PaModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode2) -> u8 {
        PaModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode3 {
        PaModelMode3::from_bits(val)
    }
}
impl From<PaModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode3) -> u8 {
        PaModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode4 {
        PaModelMode4::from_bits(val)
    }
}
impl From<PaModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode4) -> u8 {
        PaModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode5 {
        PaModelMode5::from_bits(val)
    }
}
impl From<PaModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode5) -> u8 {
        PaModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode6 {
        PaModelMode6::from_bits(val)
    }
}
impl From<PaModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode6) -> u8 {
        PaModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PaModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PaModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PaModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PaModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PaModelMode7 {
        PaModelMode7::from_bits(val)
    }
}
impl From<PaModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PaModelMode7) -> u8 {
        PaModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode10 {
        PbModehMode10::from_bits(val)
    }
}
impl From<PbModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode10) -> u8 {
        PbModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode11 {
        PbModehMode11::from_bits(val)
    }
}
impl From<PbModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode11) -> u8 {
        PbModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode12 {
        PbModehMode12::from_bits(val)
    }
}
impl From<PbModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode12) -> u8 {
        PbModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode13 {
        PbModehMode13::from_bits(val)
    }
}
impl From<PbModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode13) -> u8 {
        PbModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode14 {
        PbModehMode14::from_bits(val)
    }
}
impl From<PbModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode14) -> u8 {
        PbModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode15 {
        PbModehMode15::from_bits(val)
    }
}
impl From<PbModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode15) -> u8 {
        PbModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode8 {
        PbModehMode8::from_bits(val)
    }
}
impl From<PbModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode8) -> u8 {
        PbModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PbModehMode9 {
        PbModehMode9::from_bits(val)
    }
}
impl From<PbModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PbModehMode9) -> u8 {
        PbModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode0 {
        PbModelMode0::from_bits(val)
    }
}
impl From<PbModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode0) -> u8 {
        PbModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode1 {
        PbModelMode1::from_bits(val)
    }
}
impl From<PbModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode1) -> u8 {
        PbModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode2 {
        PbModelMode2::from_bits(val)
    }
}
impl From<PbModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode2) -> u8 {
        PbModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode3 {
        PbModelMode3::from_bits(val)
    }
}
impl From<PbModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode3) -> u8 {
        PbModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode4 {
        PbModelMode4::from_bits(val)
    }
}
impl From<PbModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode4) -> u8 {
        PbModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode5 {
        PbModelMode5::from_bits(val)
    }
}
impl From<PbModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode5) -> u8 {
        PbModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode6 {
        PbModelMode6::from_bits(val)
    }
}
impl From<PbModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode6) -> u8 {
        PbModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PbModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PbModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PbModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PbModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PbModelMode7 {
        PbModelMode7::from_bits(val)
    }
}
impl From<PbModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PbModelMode7) -> u8 {
        PbModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode10 {
        PcModehMode10::from_bits(val)
    }
}
impl From<PcModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode10) -> u8 {
        PcModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode11 {
        PcModehMode11::from_bits(val)
    }
}
impl From<PcModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode11) -> u8 {
        PcModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode12 {
        PcModehMode12::from_bits(val)
    }
}
impl From<PcModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode12) -> u8 {
        PcModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode13 {
        PcModehMode13::from_bits(val)
    }
}
impl From<PcModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode13) -> u8 {
        PcModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode14 {
        PcModehMode14::from_bits(val)
    }
}
impl From<PcModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode14) -> u8 {
        PcModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode15 {
        PcModehMode15::from_bits(val)
    }
}
impl From<PcModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode15) -> u8 {
        PcModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode8 {
        PcModehMode8::from_bits(val)
    }
}
impl From<PcModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode8) -> u8 {
        PcModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PcModehMode9 {
        PcModehMode9::from_bits(val)
    }
}
impl From<PcModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PcModehMode9) -> u8 {
        PcModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode0 {
        PcModelMode0::from_bits(val)
    }
}
impl From<PcModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode0) -> u8 {
        PcModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode1 {
        PcModelMode1::from_bits(val)
    }
}
impl From<PcModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode1) -> u8 {
        PcModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode2 {
        PcModelMode2::from_bits(val)
    }
}
impl From<PcModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode2) -> u8 {
        PcModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode3 {
        PcModelMode3::from_bits(val)
    }
}
impl From<PcModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode3) -> u8 {
        PcModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode4 {
        PcModelMode4::from_bits(val)
    }
}
impl From<PcModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode4) -> u8 {
        PcModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode5 {
        PcModelMode5::from_bits(val)
    }
}
impl From<PcModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode5) -> u8 {
        PcModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode6 {
        PcModelMode6::from_bits(val)
    }
}
impl From<PcModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode6) -> u8 {
        PcModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PcModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PcModelMode7 {
        PcModelMode7::from_bits(val)
    }
}
impl From<PcModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PcModelMode7) -> u8 {
        PcModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode10 {
        PdModehMode10::from_bits(val)
    }
}
impl From<PdModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode10) -> u8 {
        PdModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode11 {
        PdModehMode11::from_bits(val)
    }
}
impl From<PdModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode11) -> u8 {
        PdModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode12 {
        PdModehMode12::from_bits(val)
    }
}
impl From<PdModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode12) -> u8 {
        PdModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode13 {
        PdModehMode13::from_bits(val)
    }
}
impl From<PdModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode13) -> u8 {
        PdModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode14 {
        PdModehMode14::from_bits(val)
    }
}
impl From<PdModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode14) -> u8 {
        PdModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode15 {
        PdModehMode15::from_bits(val)
    }
}
impl From<PdModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode15) -> u8 {
        PdModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode8 {
        PdModehMode8::from_bits(val)
    }
}
impl From<PdModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode8) -> u8 {
        PdModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PdModehMode9 {
        PdModehMode9::from_bits(val)
    }
}
impl From<PdModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PdModehMode9) -> u8 {
        PdModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode0 {
        PdModelMode0::from_bits(val)
    }
}
impl From<PdModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode0) -> u8 {
        PdModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode1 {
        PdModelMode1::from_bits(val)
    }
}
impl From<PdModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode1) -> u8 {
        PdModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode2 {
        PdModelMode2::from_bits(val)
    }
}
impl From<PdModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode2) -> u8 {
        PdModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode3 {
        PdModelMode3::from_bits(val)
    }
}
impl From<PdModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode3) -> u8 {
        PdModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode4 {
        PdModelMode4::from_bits(val)
    }
}
impl From<PdModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode4) -> u8 {
        PdModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode5 {
        PdModelMode5::from_bits(val)
    }
}
impl From<PdModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode5) -> u8 {
        PdModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode6 {
        PdModelMode6::from_bits(val)
    }
}
impl From<PdModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode6) -> u8 {
        PdModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PdModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PdModelMode7 {
        PdModelMode7::from_bits(val)
    }
}
impl From<PdModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PdModelMode7) -> u8 {
        PdModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode10 {
        PeModehMode10::from_bits(val)
    }
}
impl From<PeModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode10) -> u8 {
        PeModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode11 {
        PeModehMode11::from_bits(val)
    }
}
impl From<PeModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode11) -> u8 {
        PeModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode12 {
        PeModehMode12::from_bits(val)
    }
}
impl From<PeModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode12) -> u8 {
        PeModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode13 {
        PeModehMode13::from_bits(val)
    }
}
impl From<PeModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode13) -> u8 {
        PeModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode14 {
        PeModehMode14::from_bits(val)
    }
}
impl From<PeModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode14) -> u8 {
        PeModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode15 {
        PeModehMode15::from_bits(val)
    }
}
impl From<PeModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode15) -> u8 {
        PeModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode8 {
        PeModehMode8::from_bits(val)
    }
}
impl From<PeModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode8) -> u8 {
        PeModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PeModehMode9 {
        PeModehMode9::from_bits(val)
    }
}
impl From<PeModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PeModehMode9) -> u8 {
        PeModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode0 {
        PeModelMode0::from_bits(val)
    }
}
impl From<PeModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode0) -> u8 {
        PeModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode1 {
        PeModelMode1::from_bits(val)
    }
}
impl From<PeModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode1) -> u8 {
        PeModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode2 {
        PeModelMode2::from_bits(val)
    }
}
impl From<PeModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode2) -> u8 {
        PeModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode3 {
        PeModelMode3::from_bits(val)
    }
}
impl From<PeModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode3) -> u8 {
        PeModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode4 {
        PeModelMode4::from_bits(val)
    }
}
impl From<PeModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode4) -> u8 {
        PeModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode5 {
        PeModelMode5::from_bits(val)
    }
}
impl From<PeModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode5) -> u8 {
        PeModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode6 {
        PeModelMode6::from_bits(val)
    }
}
impl From<PeModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode6) -> u8 {
        PeModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PeModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PeModelMode7 {
        PeModelMode7::from_bits(val)
    }
}
impl From<PeModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PeModelMode7) -> u8 {
        PeModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode10 {
        PfModehMode10::from_bits(val)
    }
}
impl From<PfModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode10) -> u8 {
        PfModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode11 {
        PfModehMode11::from_bits(val)
    }
}
impl From<PfModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode11) -> u8 {
        PfModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode12 {
        PfModehMode12::from_bits(val)
    }
}
impl From<PfModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode12) -> u8 {
        PfModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode13 {
        PfModehMode13::from_bits(val)
    }
}
impl From<PfModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode13) -> u8 {
        PfModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode14 {
        PfModehMode14::from_bits(val)
    }
}
impl From<PfModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode14) -> u8 {
        PfModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode15 {
        PfModehMode15::from_bits(val)
    }
}
impl From<PfModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode15) -> u8 {
        PfModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode8 {
        PfModehMode8::from_bits(val)
    }
}
impl From<PfModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode8) -> u8 {
        PfModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PfModehMode9 {
        PfModehMode9::from_bits(val)
    }
}
impl From<PfModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PfModehMode9) -> u8 {
        PfModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode0 {
        PfModelMode0::from_bits(val)
    }
}
impl From<PfModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode0) -> u8 {
        PfModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode1 {
        PfModelMode1::from_bits(val)
    }
}
impl From<PfModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode1) -> u8 {
        PfModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode2 {
        PfModelMode2::from_bits(val)
    }
}
impl From<PfModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode2) -> u8 {
        PfModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode3 {
        PfModelMode3::from_bits(val)
    }
}
impl From<PfModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode3) -> u8 {
        PfModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode4 {
        PfModelMode4::from_bits(val)
    }
}
impl From<PfModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode4) -> u8 {
        PfModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode5 {
        PfModelMode5::from_bits(val)
    }
}
impl From<PfModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode5) -> u8 {
        PfModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode6 {
        PfModelMode6::from_bits(val)
    }
}
impl From<PfModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode6) -> u8 {
        PfModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PfModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PfModelMode7 {
        PfModelMode7::from_bits(val)
    }
}
impl From<PfModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PfModelMode7) -> u8 {
        PfModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode10 {
        PgModehMode10::from_bits(val)
    }
}
impl From<PgModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode10) -> u8 {
        PgModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode11 {
        PgModehMode11::from_bits(val)
    }
}
impl From<PgModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode11) -> u8 {
        PgModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode12 {
        PgModehMode12::from_bits(val)
    }
}
impl From<PgModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode12) -> u8 {
        PgModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode13 {
        PgModehMode13::from_bits(val)
    }
}
impl From<PgModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode13) -> u8 {
        PgModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode14 {
        PgModehMode14::from_bits(val)
    }
}
impl From<PgModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode14) -> u8 {
        PgModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode15 {
        PgModehMode15::from_bits(val)
    }
}
impl From<PgModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode15) -> u8 {
        PgModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode8 {
        PgModehMode8::from_bits(val)
    }
}
impl From<PgModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode8) -> u8 {
        PgModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PgModehMode9 {
        PgModehMode9::from_bits(val)
    }
}
impl From<PgModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PgModehMode9) -> u8 {
        PgModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode0 {
        PgModelMode0::from_bits(val)
    }
}
impl From<PgModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode0) -> u8 {
        PgModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode1 {
        PgModelMode1::from_bits(val)
    }
}
impl From<PgModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode1) -> u8 {
        PgModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode2 {
        PgModelMode2::from_bits(val)
    }
}
impl From<PgModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode2) -> u8 {
        PgModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode3 {
        PgModelMode3::from_bits(val)
    }
}
impl From<PgModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode3) -> u8 {
        PgModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode4 {
        PgModelMode4::from_bits(val)
    }
}
impl From<PgModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode4) -> u8 {
        PgModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode5 {
        PgModelMode5::from_bits(val)
    }
}
impl From<PgModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode5) -> u8 {
        PgModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode6 {
        PgModelMode6::from_bits(val)
    }
}
impl From<PgModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode6) -> u8 {
        PgModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PgModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PgModelMode7 {
        PgModelMode7::from_bits(val)
    }
}
impl From<PgModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PgModelMode7) -> u8 {
        PgModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode10 {
        PhModehMode10::from_bits(val)
    }
}
impl From<PhModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode10) -> u8 {
        PhModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode11 {
        PhModehMode11::from_bits(val)
    }
}
impl From<PhModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode11) -> u8 {
        PhModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode12 {
        PhModehMode12::from_bits(val)
    }
}
impl From<PhModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode12) -> u8 {
        PhModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode13 {
        PhModehMode13::from_bits(val)
    }
}
impl From<PhModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode13) -> u8 {
        PhModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode14 {
        PhModehMode14::from_bits(val)
    }
}
impl From<PhModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode14) -> u8 {
        PhModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode15 {
        PhModehMode15::from_bits(val)
    }
}
impl From<PhModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode15) -> u8 {
        PhModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode8 {
        PhModehMode8::from_bits(val)
    }
}
impl From<PhModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode8) -> u8 {
        PhModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PhModehMode9 {
        PhModehMode9::from_bits(val)
    }
}
impl From<PhModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PhModehMode9) -> u8 {
        PhModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode0 {
        PhModelMode0::from_bits(val)
    }
}
impl From<PhModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode0) -> u8 {
        PhModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode1 {
        PhModelMode1::from_bits(val)
    }
}
impl From<PhModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode1) -> u8 {
        PhModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode2 {
        PhModelMode2::from_bits(val)
    }
}
impl From<PhModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode2) -> u8 {
        PhModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode3 {
        PhModelMode3::from_bits(val)
    }
}
impl From<PhModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode3) -> u8 {
        PhModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode4 {
        PhModelMode4::from_bits(val)
    }
}
impl From<PhModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode4) -> u8 {
        PhModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode5 {
        PhModelMode5::from_bits(val)
    }
}
impl From<PhModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode5) -> u8 {
        PhModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode6 {
        PhModelMode6::from_bits(val)
    }
}
impl From<PhModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode6) -> u8 {
        PhModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PhModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PhModelMode7 {
        PhModelMode7::from_bits(val)
    }
}
impl From<PhModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PhModelMode7) -> u8 {
        PhModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode10 {
        PiModehMode10::from_bits(val)
    }
}
impl From<PiModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode10) -> u8 {
        PiModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode11 {
        PiModehMode11::from_bits(val)
    }
}
impl From<PiModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode11) -> u8 {
        PiModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode12 {
        PiModehMode12::from_bits(val)
    }
}
impl From<PiModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode12) -> u8 {
        PiModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode13 {
        PiModehMode13::from_bits(val)
    }
}
impl From<PiModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode13) -> u8 {
        PiModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode14 {
        PiModehMode14::from_bits(val)
    }
}
impl From<PiModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode14) -> u8 {
        PiModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode15 {
        PiModehMode15::from_bits(val)
    }
}
impl From<PiModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode15) -> u8 {
        PiModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode8 {
        PiModehMode8::from_bits(val)
    }
}
impl From<PiModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode8) -> u8 {
        PiModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PiModehMode9 {
        PiModehMode9::from_bits(val)
    }
}
impl From<PiModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PiModehMode9) -> u8 {
        PiModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode0 {
        PiModelMode0::from_bits(val)
    }
}
impl From<PiModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode0) -> u8 {
        PiModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode1 {
        PiModelMode1::from_bits(val)
    }
}
impl From<PiModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode1) -> u8 {
        PiModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode2 {
        PiModelMode2::from_bits(val)
    }
}
impl From<PiModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode2) -> u8 {
        PiModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode3 {
        PiModelMode3::from_bits(val)
    }
}
impl From<PiModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode3) -> u8 {
        PiModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode4 {
        PiModelMode4::from_bits(val)
    }
}
impl From<PiModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode4) -> u8 {
        PiModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode5 {
        PiModelMode5::from_bits(val)
    }
}
impl From<PiModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode5) -> u8 {
        PiModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode6 {
        PiModelMode6::from_bits(val)
    }
}
impl From<PiModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode6) -> u8 {
        PiModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PiModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PiModelMode7 {
        PiModelMode7::from_bits(val)
    }
}
impl From<PiModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PiModelMode7) -> u8 {
        PiModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode10 {
        PjModehMode10::from_bits(val)
    }
}
impl From<PjModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode10) -> u8 {
        PjModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode11 {
        PjModehMode11::from_bits(val)
    }
}
impl From<PjModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode11) -> u8 {
        PjModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode12 {
        PjModehMode12::from_bits(val)
    }
}
impl From<PjModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode12) -> u8 {
        PjModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode13 {
        PjModehMode13::from_bits(val)
    }
}
impl From<PjModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode13) -> u8 {
        PjModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode14 {
        PjModehMode14::from_bits(val)
    }
}
impl From<PjModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode14) -> u8 {
        PjModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode15 {
        PjModehMode15::from_bits(val)
    }
}
impl From<PjModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode15) -> u8 {
        PjModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode8 {
        PjModehMode8::from_bits(val)
    }
}
impl From<PjModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode8) -> u8 {
        PjModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PjModehMode9 {
        PjModehMode9::from_bits(val)
    }
}
impl From<PjModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PjModehMode9) -> u8 {
        PjModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode0 {
        PjModelMode0::from_bits(val)
    }
}
impl From<PjModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode0) -> u8 {
        PjModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode1 {
        PjModelMode1::from_bits(val)
    }
}
impl From<PjModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode1) -> u8 {
        PjModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode2 {
        PjModelMode2::from_bits(val)
    }
}
impl From<PjModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode2) -> u8 {
        PjModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode3 {
        PjModelMode3::from_bits(val)
    }
}
impl From<PjModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode3) -> u8 {
        PjModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode4 {
        PjModelMode4::from_bits(val)
    }
}
impl From<PjModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode4) -> u8 {
        PjModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode5 {
        PjModelMode5::from_bits(val)
    }
}
impl From<PjModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode5) -> u8 {
        PjModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode6 {
        PjModelMode6::from_bits(val)
    }
}
impl From<PjModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode6) -> u8 {
        PjModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PjModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PjModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PjModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PjModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PjModelMode7 {
        PjModelMode7::from_bits(val)
    }
}
impl From<PjModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PjModelMode7) -> u8 {
        PjModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode10 {
        PkModehMode10::from_bits(val)
    }
}
impl From<PkModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode10) -> u8 {
        PkModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode11 {
        PkModehMode11::from_bits(val)
    }
}
impl From<PkModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode11) -> u8 {
        PkModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode12 {
        PkModehMode12::from_bits(val)
    }
}
impl From<PkModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode12) -> u8 {
        PkModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode13 {
        PkModehMode13::from_bits(val)
    }
}
impl From<PkModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode13) -> u8 {
        PkModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode14 {
        PkModehMode14::from_bits(val)
    }
}
impl From<PkModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode14) -> u8 {
        PkModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode15 {
        PkModehMode15::from_bits(val)
    }
}
impl From<PkModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode15) -> u8 {
        PkModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode8 {
        PkModehMode8::from_bits(val)
    }
}
impl From<PkModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode8) -> u8 {
        PkModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PkModehMode9 {
        PkModehMode9::from_bits(val)
    }
}
impl From<PkModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PkModehMode9) -> u8 {
        PkModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode0 {
        PkModelMode0::from_bits(val)
    }
}
impl From<PkModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode0) -> u8 {
        PkModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode1 {
        PkModelMode1::from_bits(val)
    }
}
impl From<PkModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode1) -> u8 {
        PkModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode2 {
        PkModelMode2::from_bits(val)
    }
}
impl From<PkModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode2) -> u8 {
        PkModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode3 {
        PkModelMode3::from_bits(val)
    }
}
impl From<PkModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode3) -> u8 {
        PkModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode4 {
        PkModelMode4::from_bits(val)
    }
}
impl From<PkModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode4) -> u8 {
        PkModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode5 {
        PkModelMode5::from_bits(val)
    }
}
impl From<PkModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode5) -> u8 {
        PkModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode6 {
        PkModelMode6::from_bits(val)
    }
}
impl From<PkModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode6) -> u8 {
        PkModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PkModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PkModelMode7 {
        PkModelMode7::from_bits(val)
    }
}
impl From<PkModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PkModelMode7) -> u8 {
        PkModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode10 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode10 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode10 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode10 {
        PlModehMode10::from_bits(val)
    }
}
impl From<PlModehMode10> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode10) -> u8 {
        PlModehMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode11 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode11 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode11 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode11 {
        PlModehMode11::from_bits(val)
    }
}
impl From<PlModehMode11> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode11) -> u8 {
        PlModehMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode12 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode12 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode12 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode12 {
        PlModehMode12::from_bits(val)
    }
}
impl From<PlModehMode12> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode12) -> u8 {
        PlModehMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode13 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode13 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode13 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode13 {
        PlModehMode13::from_bits(val)
    }
}
impl From<PlModehMode13> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode13) -> u8 {
        PlModehMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode14 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode14 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode14 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode14 {
        PlModehMode14::from_bits(val)
    }
}
impl From<PlModehMode14> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode14) -> u8 {
        PlModehMode14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode15 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode15 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode15 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode15 {
        PlModehMode15::from_bits(val)
    }
}
impl From<PlModehMode15> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode15) -> u8 {
        PlModehMode15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode8 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode8 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode8 {
        PlModehMode8::from_bits(val)
    }
}
impl From<PlModehMode8> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode8) -> u8 {
        PlModehMode8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModehMode9 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModehMode9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModehMode9 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModehMode9 {
    #[inline(always)]
    fn from(val: u8) -> PlModehMode9 {
        PlModehMode9::from_bits(val)
    }
}
impl From<PlModehMode9> for u8 {
    #[inline(always)]
    fn from(val: PlModehMode9) -> u8 {
        PlModehMode9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode0 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode0 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode0 {
        PlModelMode0::from_bits(val)
    }
}
impl From<PlModelMode0> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode0) -> u8 {
        PlModelMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode1 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode1 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode1 {
        PlModelMode1::from_bits(val)
    }
}
impl From<PlModelMode1> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode1) -> u8 {
        PlModelMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode2 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode2 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode2 {
        PlModelMode2::from_bits(val)
    }
}
impl From<PlModelMode2> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode2) -> u8 {
        PlModelMode2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode3 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode3 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode3 {
        PlModelMode3::from_bits(val)
    }
}
impl From<PlModelMode3> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode3) -> u8 {
        PlModelMode3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode4 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode4 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode4 {
        PlModelMode4::from_bits(val)
    }
}
impl From<PlModelMode4> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode4) -> u8 {
        PlModelMode4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode5 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode5 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode5 {
        PlModelMode5::from_bits(val)
    }
}
impl From<PlModelMode5> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode5) -> u8 {
        PlModelMode5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode6 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode6 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode6 {
        PlModelMode6::from_bits(val)
    }
}
impl From<PlModelMode6> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode6) -> u8 {
        PlModelMode6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlModelMode7 {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl PlModelMode7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlModelMode7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlModelMode7 {
    #[inline(always)]
    fn from(val: u8) -> PlModelMode7 {
        PlModelMode7::from_bits(val)
    }
}
impl From<PlModelMode7> for u8 {
    #[inline(always)]
    fn from(val: PlModelMode7) -> u8 {
        PlModelMode7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swvloc {
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
impl Swvloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swvloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swvloc {
    #[inline(always)]
    fn from(val: u8) -> Swvloc {
        Swvloc::from_bits(val)
    }
}
impl From<Swvloc> for u8 {
    #[inline(always)]
    fn from(val: Swvloc) -> u8 {
        Swvloc::to_bits(val)
    }
}
