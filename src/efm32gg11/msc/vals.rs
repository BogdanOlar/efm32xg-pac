#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bankswitchlockkey(u16);
impl Bankswitchlockkey {
    pub const Unlocked: Self = Self(0x0);
    pub const Locked: Self = Self(0x01);
}
impl Bankswitchlockkey {
    pub const fn from_bits(val: u16) -> Bankswitchlockkey {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Bankswitchlockkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Unlocked"),
            0x01 => f.write_str("Locked"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bankswitchlockkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Unlocked"),
            0x01 => defmt::write!(f, "Locked"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Bankswitchlockkey {
    #[inline(always)]
    fn from(val: u16) -> Bankswitchlockkey {
        Bankswitchlockkey::from_bits(val)
    }
}
impl From<Bankswitchlockkey> for u16 {
    #[inline(always)]
    fn from(val: Bankswitchlockkey) -> u16 {
        Bankswitchlockkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cachelplevel {
    #[doc = "Base instruction cache functionality."]
    Base = 0x0,
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    Advanced = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    Minactivity = 0x03,
}
impl Cachelplevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cachelplevel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cachelplevel {
    #[inline(always)]
    fn from(val: u8) -> Cachelplevel {
        Cachelplevel::from_bits(val)
    }
}
impl From<Cachelplevel> for u8 {
    #[inline(always)]
    fn from(val: Cachelplevel) -> u8 {
        Cachelplevel::to_bits(val)
    }
}
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
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details."]
    Ws2 = 0x02,
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details."]
    Ws3 = 0x03,
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
