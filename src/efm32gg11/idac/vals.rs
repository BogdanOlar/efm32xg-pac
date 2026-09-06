#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Aportoutsel(u8);
impl Aportoutsel {
    #[doc = "APORT1X Channel 0."]
    pub const Aport1xch0: Self = Self(0x20);
    #[doc = "APORT1Y Channel 1."]
    pub const Aport1ych1: Self = Self(0x21);
    #[doc = "APORT1X Channel 2."]
    pub const Aport1xch2: Self = Self(0x22);
    #[doc = "APORT1Y Channel 3."]
    pub const Aport1ych3: Self = Self(0x23);
    #[doc = "APORT1X Channel 4."]
    pub const Aport1xch4: Self = Self(0x24);
    #[doc = "APORT1Y Channel 5."]
    pub const Aport1ych5: Self = Self(0x25);
    #[doc = "APORT1X Channel 6."]
    pub const Aport1xch6: Self = Self(0x26);
    #[doc = "APORT1Y Channel 7."]
    pub const Aport1ych7: Self = Self(0x27);
    #[doc = "APORT1X Channel 8."]
    pub const Aport1xch8: Self = Self(0x28);
    #[doc = "APORT1Y Channel 9."]
    pub const Aport1ych9: Self = Self(0x29);
    #[doc = "APORT1X Channel 10."]
    pub const Aport1xch10: Self = Self(0x2a);
    #[doc = "APORT1Y Channel 11."]
    pub const Aport1ych11: Self = Self(0x2b);
    #[doc = "APORT1X Channel 12."]
    pub const Aport1xch12: Self = Self(0x2c);
    #[doc = "APORT1Y Channel 13."]
    pub const Aport1ych13: Self = Self(0x2d);
    #[doc = "APORT1X Channel 14."]
    pub const Aport1xch14: Self = Self(0x2e);
    #[doc = "APORT1Y Channel 15."]
    pub const Aport1ych15: Self = Self(0x2f);
    #[doc = "APORT1X Channel 16."]
    pub const Aport1xch16: Self = Self(0x30);
    #[doc = "APORT1Y Channel 17."]
    pub const Aport1ych17: Self = Self(0x31);
    #[doc = "APORT1X Channel 18."]
    pub const Aport1xch18: Self = Self(0x32);
    #[doc = "APORT1Y Channel 19."]
    pub const Aport1ych19: Self = Self(0x33);
    #[doc = "APORT1X Channel 20."]
    pub const Aport1xch20: Self = Self(0x34);
    #[doc = "APORT1Y Channel 21."]
    pub const Aport1ych21: Self = Self(0x35);
    #[doc = "APORT1X Channel 22."]
    pub const Aport1xch22: Self = Self(0x36);
    #[doc = "APORT1Y Channel 23."]
    pub const Aport1ych23: Self = Self(0x37);
    #[doc = "APORT1X Channel 24."]
    pub const Aport1xch24: Self = Self(0x38);
    #[doc = "APORT1Y Channel 25."]
    pub const Aport1ych25: Self = Self(0x39);
    #[doc = "APORT1X Channel 26."]
    pub const Aport1xch26: Self = Self(0x3a);
    #[doc = "APORT1Y Channel 27."]
    pub const Aport1ych27: Self = Self(0x3b);
    #[doc = "APORT1X Channel 28."]
    pub const Aport1xch28: Self = Self(0x3c);
    #[doc = "APORT1Y Channel 29."]
    pub const Aport1ych29: Self = Self(0x3d);
    #[doc = "APORT1X Channel 30."]
    pub const Aport1xch30: Self = Self(0x3e);
    #[doc = "APORT1Y Channel 31."]
    pub const Aport1ych31: Self = Self(0x3f);
}
impl Aportoutsel {
    pub const fn from_bits(val: u8) -> Aportoutsel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Aportoutsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x20 => f.write_str("Aport1xch0"),
            0x21 => f.write_str("Aport1ych1"),
            0x22 => f.write_str("Aport1xch2"),
            0x23 => f.write_str("Aport1ych3"),
            0x24 => f.write_str("Aport1xch4"),
            0x25 => f.write_str("Aport1ych5"),
            0x26 => f.write_str("Aport1xch6"),
            0x27 => f.write_str("Aport1ych7"),
            0x28 => f.write_str("Aport1xch8"),
            0x29 => f.write_str("Aport1ych9"),
            0x2a => f.write_str("Aport1xch10"),
            0x2b => f.write_str("Aport1ych11"),
            0x2c => f.write_str("Aport1xch12"),
            0x2d => f.write_str("Aport1ych13"),
            0x2e => f.write_str("Aport1xch14"),
            0x2f => f.write_str("Aport1ych15"),
            0x30 => f.write_str("Aport1xch16"),
            0x31 => f.write_str("Aport1ych17"),
            0x32 => f.write_str("Aport1xch18"),
            0x33 => f.write_str("Aport1ych19"),
            0x34 => f.write_str("Aport1xch20"),
            0x35 => f.write_str("Aport1ych21"),
            0x36 => f.write_str("Aport1xch22"),
            0x37 => f.write_str("Aport1ych23"),
            0x38 => f.write_str("Aport1xch24"),
            0x39 => f.write_str("Aport1ych25"),
            0x3a => f.write_str("Aport1xch26"),
            0x3b => f.write_str("Aport1ych27"),
            0x3c => f.write_str("Aport1xch28"),
            0x3d => f.write_str("Aport1ych29"),
            0x3e => f.write_str("Aport1xch30"),
            0x3f => f.write_str("Aport1ych31"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportoutsel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x20 => defmt::write!(f, "Aport1xch0"),
            0x21 => defmt::write!(f, "Aport1ych1"),
            0x22 => defmt::write!(f, "Aport1xch2"),
            0x23 => defmt::write!(f, "Aport1ych3"),
            0x24 => defmt::write!(f, "Aport1xch4"),
            0x25 => defmt::write!(f, "Aport1ych5"),
            0x26 => defmt::write!(f, "Aport1xch6"),
            0x27 => defmt::write!(f, "Aport1ych7"),
            0x28 => defmt::write!(f, "Aport1xch8"),
            0x29 => defmt::write!(f, "Aport1ych9"),
            0x2a => defmt::write!(f, "Aport1xch10"),
            0x2b => defmt::write!(f, "Aport1ych11"),
            0x2c => defmt::write!(f, "Aport1xch12"),
            0x2d => defmt::write!(f, "Aport1ych13"),
            0x2e => defmt::write!(f, "Aport1xch14"),
            0x2f => defmt::write!(f, "Aport1ych15"),
            0x30 => defmt::write!(f, "Aport1xch16"),
            0x31 => defmt::write!(f, "Aport1ych17"),
            0x32 => defmt::write!(f, "Aport1xch18"),
            0x33 => defmt::write!(f, "Aport1ych19"),
            0x34 => defmt::write!(f, "Aport1xch20"),
            0x35 => defmt::write!(f, "Aport1ych21"),
            0x36 => defmt::write!(f, "Aport1xch22"),
            0x37 => defmt::write!(f, "Aport1ych23"),
            0x38 => defmt::write!(f, "Aport1xch24"),
            0x39 => defmt::write!(f, "Aport1ych25"),
            0x3a => defmt::write!(f, "Aport1xch26"),
            0x3b => defmt::write!(f, "Aport1ych27"),
            0x3c => defmt::write!(f, "Aport1xch28"),
            0x3d => defmt::write!(f, "Aport1ych29"),
            0x3e => defmt::write!(f, "Aport1xch30"),
            0x3f => defmt::write!(f, "Aport1ych31"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Aportoutsel {
    #[inline(always)]
    fn from(val: u8) -> Aportoutsel {
        Aportoutsel::from_bits(val)
    }
}
impl From<Aportoutsel> for u8 {
    #[inline(always)]
    fn from(val: Aportoutsel) -> u8 {
        Aportoutsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangesel {
    #[doc = "Current range set to 0 - 1.6 uA."]
    Range0 = 0x0,
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    Range1 = 0x01,
    #[doc = "Current range set to 0.5 - 16 uA."]
    Range2 = 0x02,
    #[doc = "Current range set to 2 - 64 uA."]
    Range3 = 0x03,
}
impl Rangesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangesel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangesel {
    #[inline(always)]
    fn from(val: u8) -> Rangesel {
        Rangesel::from_bits(val)
    }
}
impl From<Rangesel> for u8 {
    #[inline(always)]
    fn from(val: Rangesel) -> u8 {
        Rangesel::to_bits(val)
    }
}
