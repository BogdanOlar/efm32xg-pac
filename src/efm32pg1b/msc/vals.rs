#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lockkey(u16);
impl Lockkey {
    pub const Unlocked: Self = Self(0x0);
    pub const Locked: Self = Self(0x01);
}
impl Lockkey {
    pub const fn from_bits(val: u16) -> Lockkey {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Lockkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Unlocked"),
            0x01 => f.write_str("Locked"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lockkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Unlocked"),
            0x01 => defmt::write!(f, "Locked"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Lockkey {
    #[inline(always)]
    fn from(val: u16) -> Lockkey {
        Lockkey::from_bits(val)
    }
}
impl From<Lockkey> for u16 {
    #[inline(always)]
    fn from(val: Lockkey) -> u16 {
        Lockkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    Ws0 = 0x0,
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details."]
    Ws1 = 0x01,
    _RESERVED_2 = 0x02,
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
