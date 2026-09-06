#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Amuxsel {
    #[doc = "VBUS divided by 10."]
    Vbusdiv10 = 0x0,
    #[doc = "VREGI divided by 10."]
    Vregidiv10 = 0x01,
    #[doc = "VREGO divided by 6."]
    Vregodiv6 = 0x02,
    #[doc = "VREGI current monitor."]
    Vregiimon = 0x03,
    #[doc = "VBUS current monitor."]
    Vbusimon = 0x04,
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
impl Amuxsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Amuxsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Amuxsel {
    #[inline(always)]
    fn from(val: u8) -> Amuxsel {
        Amuxsel::from_bits(val)
    }
}
impl From<Amuxsel> for u8 {
    #[inline(always)]
    fn from(val: Amuxsel) -> u8 {
        Amuxsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buactpwrcon {
    #[doc = "No connection."]
    None = 0x0,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    Mainbu = 0x01,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    Bumain = 0x02,
    #[doc = "Main power and backup power are connected without diode."]
    Nodiode = 0x03,
}
impl Buactpwrcon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buactpwrcon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buactpwrcon {
    #[inline(always)]
    fn from(val: u8) -> Buactpwrcon {
        Buactpwrcon::from_bits(val)
    }
}
impl From<Buactpwrcon> for u8 {
    #[inline(always)]
    fn from(val: Buactpwrcon) -> u8 {
        Buactpwrcon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buinactpwrcon {
    #[doc = "No connection."]
    None = 0x0,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    Mainbu = 0x01,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    Bumain = 0x02,
    #[doc = "Main power and backup power are connected without diode."]
    Nodiode = 0x03,
}
impl Buinactpwrcon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buinactpwrcon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buinactpwrcon {
    #[inline(always)]
    fn from(val: u8) -> Buinactpwrcon {
        Buinactpwrcon::from_bits(val)
    }
}
impl From<Buinactpwrcon> for u8 {
    #[inline(always)]
    fn from(val: Buinactpwrcon) -> u8 {
        Buinactpwrcon::to_bits(val)
    }
}
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
pub enum Em23vscale {
    #[doc = "Voltage Scale Level 2."]
    Vscale2 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Voltage Scale Level 0."]
    Vscale0 = 0x02,
    #[doc = "RESV."]
    Resv = 0x03,
}
impl Em23vscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em23vscale {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em23vscale {
    #[inline(always)]
    fn from(val: u8) -> Em23vscale {
        Em23vscale::from_bits(val)
    }
}
impl From<Em23vscale> for u8 {
    #[inline(always)]
    fn from(val: Em23vscale) -> u8 {
        Em23vscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em4hvscale {
    #[doc = "Voltage Scale Level 2."]
    Vscale2 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Voltage Scale Level 0."]
    Vscale0 = 0x02,
    #[doc = "RESV."]
    Resv = 0x03,
}
impl Em4hvscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em4hvscale {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em4hvscale {
    #[inline(always)]
    fn from(val: u8) -> Em4hvscale {
        Em4hvscale::from_bits(val)
    }
}
impl From<Em4hvscale> for u8 {
    #[inline(always)]
    fn from(val: Em4hvscale) -> u8 {
        Em4hvscale::to_bits(val)
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
pub enum Inputmode {
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI."]
    Auto = 0x0,
    #[doc = "Force VBUS pin as the regulator input."]
    Vbus = 0x01,
    #[doc = "Force VREGI pin as the regulator input."]
    Vregi = 0x02,
    _RESERVED_3 = 0x03,
}
impl Inputmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputmode {
    #[inline(always)]
    fn from(val: u8) -> Inputmode {
        Inputmode::from_bits(val)
    }
}
impl From<Inputmode> for u8 {
    #[inline(always)]
    fn from(val: Inputmode) -> u8 {
        Inputmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpcmpbiasem01 {
    #[doc = "Maximum load current less than 75uA."]
    Bias0 = 0x0,
    #[doc = "Maximum load current less than 500uA."]
    Bias1 = 0x01,
    #[doc = "Maximum load current less than 2.5mA."]
    Bias2 = 0x02,
    #[doc = "Maximum load current less than 10mA."]
    Bias3 = 0x03,
}
impl Lpcmpbiasem01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcmpbiasem01 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcmpbiasem01 {
    #[inline(always)]
    fn from(val: u8) -> Lpcmpbiasem01 {
        Lpcmpbiasem01::from_bits(val)
    }
}
impl From<Lpcmpbiasem01> for u8 {
    #[inline(always)]
    fn from(val: Lpcmpbiasem01) -> u8 {
        Lpcmpbiasem01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpcmpbiasem234h {
    #[doc = "Maximum load current less than 75uA."]
    Bias0 = 0x0,
    #[doc = "Maximum load current less than 500uA."]
    Bias1 = 0x01,
    #[doc = "Maximum load current less than 2.5mA."]
    Bias2 = 0x02,
    #[doc = "Maximum load current less than 10mA."]
    Bias3 = 0x03,
}
impl Lpcmpbiasem234h {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcmpbiasem234h {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcmpbiasem234h {
    #[inline(always)]
    fn from(val: u8) -> Lpcmpbiasem234h {
        Lpcmpbiasem234h::from_bits(val)
    }
}
impl From<Lpcmpbiasem234h> for u8 {
    #[inline(always)]
    fn from(val: Lpcmpbiasem234h) -> u8 {
        Lpcmpbiasem234h::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrres {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    Res0 = 0x0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    Res1 = 0x01,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    Res2 = 0x02,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    Res3 = 0x03,
}
impl Pwrres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrres {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrres {
    #[inline(always)]
    fn from(val: u8) -> Pwrres {
        Pwrres::from_bits(val)
    }
}
impl From<Pwrres> for u8 {
    #[inline(always)]
    fn from(val: Pwrres) -> u8 {
        Pwrres::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ram0ctrlRampowerdown(u8);
impl Ram0ctrlRampowerdown {
    #[doc = "None of the RAM blocks powered down."]
    pub const None: Self = Self(0x0);
    #[doc = "Power down RAM block 7 and above."]
    pub const Blk7: Self = Self(0x40);
    #[doc = "Power down RAM block 6 and above."]
    pub const Blk6to7: Self = Self(0x60);
    #[doc = "Power down RAM block 5 and above."]
    pub const Blk5to7: Self = Self(0x70);
    #[doc = "Power down RAM blocks 4 and above."]
    pub const Blk4to7: Self = Self(0x78);
    #[doc = "Power down RAM blocks 3 and above."]
    pub const Blk3to7: Self = Self(0x7c);
    #[doc = "Power down RAM blocks 2 and above."]
    pub const Blk2to7: Self = Self(0x7e);
    #[doc = "Power down RAM blocks 1 and above."]
    pub const Blk1to7: Self = Self(0x7f);
}
impl Ram0ctrlRampowerdown {
    pub const fn from_bits(val: u8) -> Ram0ctrlRampowerdown {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ram0ctrlRampowerdown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x40 => f.write_str("Blk7"),
            0x60 => f.write_str("Blk6to7"),
            0x70 => f.write_str("Blk5to7"),
            0x78 => f.write_str("Blk4to7"),
            0x7c => f.write_str("Blk3to7"),
            0x7e => f.write_str("Blk2to7"),
            0x7f => f.write_str("Blk1to7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram0ctrlRampowerdown {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x40 => defmt::write!(f, "Blk7"),
            0x60 => defmt::write!(f, "Blk6to7"),
            0x70 => defmt::write!(f, "Blk5to7"),
            0x78 => defmt::write!(f, "Blk4to7"),
            0x7c => defmt::write!(f, "Blk3to7"),
            0x7e => defmt::write!(f, "Blk2to7"),
            0x7f => defmt::write!(f, "Blk1to7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ram0ctrlRampowerdown {
    #[inline(always)]
    fn from(val: u8) -> Ram0ctrlRampowerdown {
        Ram0ctrlRampowerdown::from_bits(val)
    }
}
impl From<Ram0ctrlRampowerdown> for u8 {
    #[inline(always)]
    fn from(val: Ram0ctrlRampowerdown) -> u8 {
        Ram0ctrlRampowerdown::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ram1ctrlRampowerdown(u8);
impl Ram1ctrlRampowerdown {
    #[doc = "None of the RAM blocks powered down."]
    pub const None: Self = Self(0x0);
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)."]
    pub const Blk7: Self = Self(0x80);
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)."]
    pub const Blk6to7: Self = Self(0xc0);
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)."]
    pub const Blk5to7: Self = Self(0xe0);
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)."]
    pub const Blk4to7: Self = Self(0xf0);
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)."]
    pub const Blk3to7: Self = Self(0xf8);
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)."]
    pub const Blk2to7: Self = Self(0xfc);
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)."]
    pub const Blk1to7: Self = Self(0xfe);
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)."]
    pub const Blk0to7: Self = Self(0xff);
}
impl Ram1ctrlRampowerdown {
    pub const fn from_bits(val: u8) -> Ram1ctrlRampowerdown {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ram1ctrlRampowerdown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x80 => f.write_str("Blk7"),
            0xc0 => f.write_str("Blk6to7"),
            0xe0 => f.write_str("Blk5to7"),
            0xf0 => f.write_str("Blk4to7"),
            0xf8 => f.write_str("Blk3to7"),
            0xfc => f.write_str("Blk2to7"),
            0xfe => f.write_str("Blk1to7"),
            0xff => f.write_str("Blk0to7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram1ctrlRampowerdown {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x80 => defmt::write!(f, "Blk7"),
            0xc0 => defmt::write!(f, "Blk6to7"),
            0xe0 => defmt::write!(f, "Blk5to7"),
            0xf0 => defmt::write!(f, "Blk4to7"),
            0xf8 => defmt::write!(f, "Blk3to7"),
            0xfc => defmt::write!(f, "Blk2to7"),
            0xfe => defmt::write!(f, "Blk1to7"),
            0xff => defmt::write!(f, "Blk0to7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ram1ctrlRampowerdown {
    #[inline(always)]
    fn from(val: u8) -> Ram1ctrlRampowerdown {
        Ram1ctrlRampowerdown::from_bits(val)
    }
}
impl From<Ram1ctrlRampowerdown> for u8 {
    #[inline(always)]
    fn from(val: Ram1ctrlRampowerdown) -> u8 {
        Ram1ctrlRampowerdown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram2ctrlRampowerdown {
    #[doc = "None of the RAM blocks powered down."]
    None = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Power down RAM block 3."]
    Blk3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Power down RAM blocks 2-3."]
    Blk2to3 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Power down RAM blocks 1-3."]
    Blk1to3 = 0x0e,
    #[doc = "Power down RAM blocks 0-3."]
    Blk0to3 = 0x0f,
}
impl Ram2ctrlRampowerdown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram2ctrlRampowerdown {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram2ctrlRampowerdown {
    #[inline(always)]
    fn from(val: u8) -> Ram2ctrlRampowerdown {
        Ram2ctrlRampowerdown::from_bits(val)
    }
}
impl From<Ram2ctrlRampowerdown> for u8 {
    #[inline(always)]
    fn from(val: Ram2ctrlRampowerdown) -> u8 {
        Ram2ctrlRampowerdown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Voutres {
    #[doc = "BU_VOUT is not connected."]
    Dis = 0x0,
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    Weak = 0x01,
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    Med = 0x02,
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    Strong = 0x03,
}
impl Voutres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Voutres {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Voutres {
    #[inline(always)]
    fn from(val: u8) -> Voutres {
        Voutres::from_bits(val)
    }
}
impl From<Voutres> for u8 {
    #[inline(always)]
    fn from(val: Voutres) -> u8 {
        Voutres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vscale {
    #[doc = "Voltage Scale Level 2."]
    Vscale2 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Voltage Scale Level 0."]
    Vscale0 = 0x02,
    #[doc = "RESV."]
    Resv = 0x03,
}
impl Vscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vscale {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vscale {
    #[inline(always)]
    fn from(val: u8) -> Vscale {
        Vscale::from_bits(val)
    }
}
impl From<Vscale> for u8 {
    #[inline(always)]
    fn from(val: Vscale) -> u8 {
        Vscale::to_bits(val)
    }
}
