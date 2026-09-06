#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcdcmode {
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    Bypass = 0x0,
    #[doc = "DCDC regulator is operating in low noise mode."]
    Lownoise = 0x01,
    #[doc = "DCDC regulator is operating in low power mode."]
    Lowpower = 0x02,
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally."]
    Off = 0x03,
}
impl Dcdcmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcdcmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcdcmode {
    #[inline(always)]
    fn from(val: u8) -> Dcdcmode {
        Dcdcmode::from_bits(val)
    }
}
impl From<Dcdcmode> for u8 {
    #[inline(always)]
    fn from(val: Dcdcmode) -> u8 {
        Dcdcmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em4ioretmode {
    #[doc = "No Retention: Pads enter reset state when entering EM4."]
    Disable = 0x0,
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4."]
    Em4exit = 0x01,
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention."]
    Swunlatch = 0x02,
    _RESERVED_3 = 0x03,
}
impl Em4ioretmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em4ioretmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em4ioretmode {
    #[inline(always)]
    fn from(val: u8) -> Em4ioretmode {
        Em4ioretmode::from_bits(val)
    }
}
impl From<Em4ioretmode> for u8 {
    #[inline(always)]
    fn from(val: Em4ioretmode) -> u8 {
        Em4ioretmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpcmpbias {
    #[doc = "Maximum load current less than 75uA."]
    Bias0 = 0x0,
    #[doc = "Maximum load current less than 500uA."]
    Bias1 = 0x01,
    #[doc = "Maximum load current less than 2.5mA."]
    Bias2 = 0x02,
    #[doc = "Maximum load current less than 10mA."]
    Bias3 = 0x03,
}
impl Lpcmpbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcmpbias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcmpbias {
    #[inline(always)]
    fn from(val: u8) -> Lpcmpbias {
        Lpcmpbias::from_bits(val)
    }
}
impl From<Lpcmpbias> for u8 {
    #[inline(always)]
    fn from(val: Lpcmpbias) -> u8 {
        Lpcmpbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrcfg {
    #[doc = "Power up configuration. Works with any external configuration."]
    Startup = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "DCDC is enabled and routed to DVDD."]
    Dcdctodvdd = 0x02,
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
impl Pwrcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrcfg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrcfg {
    #[inline(always)]
    fn from(val: u8) -> Pwrcfg {
        Pwrcfg::from_bits(val)
    }
}
impl From<Pwrcfg> for u8 {
    #[inline(always)]
    fn from(val: Pwrcfg) -> u8 {
        Pwrcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rampowerdown {
    #[doc = "None of the RAM blocks powered down."]
    None = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Power down RAM blocks 4 and above."]
    Blk4 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Power down RAM blocks 3 and above."]
    Blk3to4 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Power down RAM blocks 2 and above."]
    Blk2to4 = 0x0e,
    #[doc = "Power down RAM blocks 1 and above."]
    Blk1to4 = 0x0f,
}
impl Rampowerdown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rampowerdown {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rampowerdown {
    #[inline(always)]
    fn from(val: u8) -> Rampowerdown {
        Rampowerdown::from_bits(val)
    }
}
impl From<Rampowerdown> for u8 {
    #[inline(always)]
    fn from(val: Rampowerdown) -> u8 {
        Rampowerdown::to_bits(val)
    }
}
