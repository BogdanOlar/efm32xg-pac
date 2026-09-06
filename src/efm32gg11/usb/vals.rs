#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbthrratio {
    #[doc = "AHB threshold = MAC threshold."]
    Div1 = 0x0,
    #[doc = "AHB threshold = MAC threshold / 2."]
    Div2 = 0x01,
    #[doc = "AHB threshold = MAC threshold / 4."]
    Div4 = 0x02,
    #[doc = "AHB threshold = MAC threshold / 8."]
    Div8 = 0x03,
}
impl Ahbthrratio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbthrratio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbthrratio {
    #[inline(always)]
    fn from(val: u8) -> Ahbthrratio {
        Ahbthrratio::from_bits(val)
    }
}
impl From<Ahbthrratio> for u8 {
    #[inline(always)]
    fn from(val: Ahbthrratio) -> u8 {
        Ahbthrratio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcden {
    #[doc = "DCD is disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Only DCD timeout will be initiated."]
    Timeout = 0x02,
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    Enabled = 0x03,
}
impl Dcden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcden {
    #[inline(always)]
    fn from(val: u8) -> Dcden {
        Dcden::from_bits(val)
    }
}
impl From<Dcden> for u8 {
    #[inline(always)]
    fn from(val: Dcden) -> u8 {
        Dcden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Devspd {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    Ls = 0x02,
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    Fs = 0x03,
}
impl Devspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Devspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Devspd {
    #[inline(always)]
    fn from(val: u8) -> Devspd {
        Devspd::from_bits(val)
    }
}
impl From<Devspd> for u8 {
    #[inline(always)]
    fn from(val: Devspd) -> u8 {
        Devspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep0CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep0CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep0CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep0CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep0CtlEptype {
        Diep0CtlEptype::from_bits(val)
    }
}
impl From<Diep0CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep0CtlEptype) -> u8 {
        Diep0CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep0ctlMps {
    #[doc = "64 bytes."]
    _64b = 0x0,
    #[doc = "32 bytes."]
    _32b = 0x01,
    #[doc = "16 bytes."]
    _16b = 0x02,
    #[doc = "8 bytes."]
    _8b = 0x03,
}
impl Diep0ctlMps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep0ctlMps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep0ctlMps {
    #[inline(always)]
    fn from(val: u8) -> Diep0ctlMps {
        Diep0ctlMps::from_bits(val)
    }
}
impl From<Diep0ctlMps> for u8 {
    #[inline(always)]
    fn from(val: Diep0ctlMps) -> u8 {
        Diep0ctlMps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep1CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep1CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep1CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep1CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep1CtlEptype {
        Diep1CtlEptype::from_bits(val)
    }
}
impl From<Diep1CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep1CtlEptype) -> u8 {
        Diep1CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep2CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep2CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep2CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep2CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep2CtlEptype {
        Diep2CtlEptype::from_bits(val)
    }
}
impl From<Diep2CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep2CtlEptype) -> u8 {
        Diep2CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep3CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep3CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep3CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep3CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep3CtlEptype {
        Diep3CtlEptype::from_bits(val)
    }
}
impl From<Diep3CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep3CtlEptype) -> u8 {
        Diep3CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep4CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep4CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep4CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep4CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep4CtlEptype {
        Diep4CtlEptype::from_bits(val)
    }
}
impl From<Diep4CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep4CtlEptype) -> u8 {
        Diep4CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diep5CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Diep5CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diep5CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diep5CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Diep5CtlEptype {
        Diep5CtlEptype::from_bits(val)
    }
}
impl From<Diep5CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Diep5CtlEptype) -> u8 {
        Diep5CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep0CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep0CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep0CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep0CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep0CtlEptype {
        Doep0CtlEptype::from_bits(val)
    }
}
impl From<Doep0CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep0CtlEptype) -> u8 {
        Doep0CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep0TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep0TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep0TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep0TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep0TsizRxdpidsupcnt {
        Doep0TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep0TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep0TsizRxdpidsupcnt) -> u8 {
        Doep0TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep0ctlMps {
    #[doc = "64 bytes."]
    _64b = 0x0,
    #[doc = "32 bytes."]
    _32b = 0x01,
    #[doc = "16 bytes."]
    _16b = 0x02,
    #[doc = "8 bytes."]
    _8b = 0x03,
}
impl Doep0ctlMps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep0ctlMps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep0ctlMps {
    #[inline(always)]
    fn from(val: u8) -> Doep0ctlMps {
        Doep0ctlMps::from_bits(val)
    }
}
impl From<Doep0ctlMps> for u8 {
    #[inline(always)]
    fn from(val: Doep0ctlMps) -> u8 {
        Doep0ctlMps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep1CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep1CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep1CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep1CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep1CtlEptype {
        Doep1CtlEptype::from_bits(val)
    }
}
impl From<Doep1CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep1CtlEptype) -> u8 {
        Doep1CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep1TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep1TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep1TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep1TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep1TsizRxdpidsupcnt {
        Doep1TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep1TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep1TsizRxdpidsupcnt) -> u8 {
        Doep1TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep2CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep2CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep2CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep2CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep2CtlEptype {
        Doep2CtlEptype::from_bits(val)
    }
}
impl From<Doep2CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep2CtlEptype) -> u8 {
        Doep2CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep2TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep2TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep2TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep2TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep2TsizRxdpidsupcnt {
        Doep2TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep2TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep2TsizRxdpidsupcnt) -> u8 {
        Doep2TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep3CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep3CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep3CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep3CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep3CtlEptype {
        Doep3CtlEptype::from_bits(val)
    }
}
impl From<Doep3CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep3CtlEptype) -> u8 {
        Doep3CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep3TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep3TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep3TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep3TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep3TsizRxdpidsupcnt {
        Doep3TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep3TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep3TsizRxdpidsupcnt) -> u8 {
        Doep3TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep4CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep4CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep4CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep4CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep4CtlEptype {
        Doep4CtlEptype::from_bits(val)
    }
}
impl From<Doep4CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep4CtlEptype) -> u8 {
        Doep4CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep4TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep4TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep4TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep4TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep4TsizRxdpidsupcnt {
        Doep4TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep4TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep4TsizRxdpidsupcnt) -> u8 {
        Doep4TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep5CtlEptype {
    #[doc = "Control Endpoint."]
    Control = 0x0,
    #[doc = "Isochronous Endpoint."]
    Iso = 0x01,
    #[doc = "Bulk Endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt Endpoint."]
    Int = 0x03,
}
impl Doep5CtlEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep5CtlEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep5CtlEptype {
    #[inline(always)]
    fn from(val: u8) -> Doep5CtlEptype {
        Doep5CtlEptype::from_bits(val)
    }
}
impl From<Doep5CtlEptype> for u8 {
    #[inline(always)]
    fn from(val: Doep5CtlEptype) -> u8 {
        Doep5CtlEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doep5TsizRxdpidsupcnt {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID / 1 Packet."]
    Data2 = 0x01,
    #[doc = "DATA1 PID / 2 Packets."]
    Data1 = 0x02,
    #[doc = "MDATA PID / 3 Packets."]
    Mdata = 0x03,
}
impl Doep5TsizRxdpidsupcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doep5TsizRxdpidsupcnt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doep5TsizRxdpidsupcnt {
    #[inline(always)]
    fn from(val: u8) -> Doep5TsizRxdpidsupcnt {
        Doep5TsizRxdpidsupcnt::from_bits(val)
    }
}
impl From<Doep5TsizRxdpidsupcnt> for u8 {
    #[inline(always)]
    fn from(val: Doep5TsizRxdpidsupcnt) -> u8 {
        Doep5TsizRxdpidsupcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enumspd {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Low speed (PHY clock is running at 6 MHz)."]
    Ls = 0x02,
    #[doc = "Full speed (PHY clock is running at 48 MHz)."]
    Fs = 0x03,
}
impl Enumspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enumspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enumspd {
    #[inline(always)]
    fn from(val: u8) -> Enumspd {
        Enumspd::from_bits(val)
    }
}
impl From<Enumspd> for u8 {
    #[inline(always)]
    fn from(val: Enumspd) -> u8 {
        Enumspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fslspclksel {
    _RESERVED_0 = 0x0,
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    Div1 = 0x01,
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    Div8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fslspclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fslspclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fslspclksel {
    #[inline(always)]
    fn from(val: u8) -> Fslspclksel {
        Fslspclksel::from_bits(val)
    }
}
impl From<Fslspclksel> for u8 {
    #[inline(always)]
    fn from(val: Fslspclksel) -> u8 {
        Fslspclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GrxstspDpid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA1 PID."]
    Data1 = 0x01,
    #[doc = "DATA2 PID."]
    Data2 = 0x02,
    #[doc = "MDATA PID."]
    Mdata = 0x03,
}
impl GrxstspDpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GrxstspDpid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GrxstspDpid {
    #[inline(always)]
    fn from(val: u8) -> GrxstspDpid {
        GrxstspDpid::from_bits(val)
    }
}
impl From<GrxstspDpid> for u8 {
    #[inline(always)]
    fn from(val: GrxstspDpid) -> u8 {
        GrxstspDpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GrxstspPktsts {
    _RESERVED_0 = 0x0,
    #[doc = "Device mode: Global OUT NAK (triggers an interrupt)."]
    Goutnak = 0x01,
    #[doc = "Host mode: IN data packet received. Device mode: OUT data packet received."]
    Pktrcv = 0x02,
    #[doc = "Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    Xfercompl = 0x03,
    #[doc = "Device mode: SETUP transaction completed (triggers an interrupt)."]
    Setupcompl = 0x04,
    #[doc = "Host mode: Data toggle error (triggers an interrupt)."]
    Tglerr = 0x05,
    #[doc = "Device mode: SETUP data packet received."]
    Setuprcv = 0x06,
    #[doc = "Host mode: Channel halted (triggers an interrupt)."]
    Chlt = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GrxstspPktsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GrxstspPktsts {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GrxstspPktsts {
    #[inline(always)]
    fn from(val: u8) -> GrxstspPktsts {
        GrxstspPktsts::from_bits(val)
    }
}
impl From<GrxstspPktsts> for u8 {
    #[inline(always)]
    fn from(val: GrxstspPktsts) -> u8 {
        GrxstspPktsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GrxstsrDpid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA1 PID."]
    Data1 = 0x01,
    #[doc = "DATA2 PID."]
    Data2 = 0x02,
    #[doc = "MDATA PID."]
    Mdata = 0x03,
}
impl GrxstsrDpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GrxstsrDpid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GrxstsrDpid {
    #[inline(always)]
    fn from(val: u8) -> GrxstsrDpid {
        GrxstsrDpid::from_bits(val)
    }
}
impl From<GrxstsrDpid> for u8 {
    #[inline(always)]
    fn from(val: GrxstsrDpid) -> u8 {
        GrxstsrDpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GrxstsrPktsts {
    _RESERVED_0 = 0x0,
    #[doc = "Device mode: Global OUT NAK (triggers an interrupt)."]
    Goutnak = 0x01,
    #[doc = "Host mode: IN data packet received. Device mode: OUT data packet received."]
    Pktrcv = 0x02,
    #[doc = "Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    Xfercompl = 0x03,
    #[doc = "Device mode: SETUP transaction completed (triggers an interrupt)."]
    Setupcompl = 0x04,
    #[doc = "Host mode: Data toggle error (triggers an interrupt)."]
    Tglerr = 0x05,
    #[doc = "Device mode: SETUP data packet received."]
    Setuprcv = 0x06,
    #[doc = "Host mode: Channel halted (triggers an interrupt)."]
    Chlt = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GrxstsrPktsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GrxstsrPktsts {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GrxstsrPktsts {
    #[inline(always)]
    fn from(val: u8) -> GrxstsrPktsts {
        GrxstsrPktsts::from_bits(val)
    }
}
impl From<GrxstsrPktsts> for u8 {
    #[inline(always)]
    fn from(val: GrxstsrPktsts) -> u8 {
        GrxstsrPktsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hbstlen {
    #[doc = "Single transfer."]
    Single = 0x0,
    #[doc = "Incrementing burst of unspecified length."]
    Incr = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4-beat incrementing burst."]
    Incr4 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "8-beat incrementing burst."]
    Incr8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "16-beat incrementing burst."]
    Incr16 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Hbstlen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hbstlen {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hbstlen {
    #[inline(always)]
    fn from(val: u8) -> Hbstlen {
        Hbstlen::from_bits(val)
    }
}
impl From<Hbstlen> for u8 {
    #[inline(always)]
    fn from(val: Hbstlen) -> u8 {
        Hbstlen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc0CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc0CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc0CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc0CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc0CharEptype {
        Hc0CharEptype::from_bits(val)
    }
}
impl From<Hc0CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc0CharEptype) -> u8 {
        Hc0CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc0TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc0TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc0TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc0TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc0TsizPid {
        Hc0TsizPid::from_bits(val)
    }
}
impl From<Hc0TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc0TsizPid) -> u8 {
        Hc0TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc10CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc10CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc10CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc10CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc10CharEptype {
        Hc10CharEptype::from_bits(val)
    }
}
impl From<Hc10CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc10CharEptype) -> u8 {
        Hc10CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc10TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc10TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc10TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc10TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc10TsizPid {
        Hc10TsizPid::from_bits(val)
    }
}
impl From<Hc10TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc10TsizPid) -> u8 {
        Hc10TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc11CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc11CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc11CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc11CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc11CharEptype {
        Hc11CharEptype::from_bits(val)
    }
}
impl From<Hc11CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc11CharEptype) -> u8 {
        Hc11CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc11TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc11TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc11TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc11TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc11TsizPid {
        Hc11TsizPid::from_bits(val)
    }
}
impl From<Hc11TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc11TsizPid) -> u8 {
        Hc11TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc12CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc12CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc12CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc12CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc12CharEptype {
        Hc12CharEptype::from_bits(val)
    }
}
impl From<Hc12CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc12CharEptype) -> u8 {
        Hc12CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc12TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc12TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc12TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc12TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc12TsizPid {
        Hc12TsizPid::from_bits(val)
    }
}
impl From<Hc12TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc12TsizPid) -> u8 {
        Hc12TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc13CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc13CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc13CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc13CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc13CharEptype {
        Hc13CharEptype::from_bits(val)
    }
}
impl From<Hc13CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc13CharEptype) -> u8 {
        Hc13CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc13TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc13TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc13TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc13TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc13TsizPid {
        Hc13TsizPid::from_bits(val)
    }
}
impl From<Hc13TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc13TsizPid) -> u8 {
        Hc13TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc1CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc1CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc1CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc1CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc1CharEptype {
        Hc1CharEptype::from_bits(val)
    }
}
impl From<Hc1CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc1CharEptype) -> u8 {
        Hc1CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc1TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc1TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc1TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc1TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc1TsizPid {
        Hc1TsizPid::from_bits(val)
    }
}
impl From<Hc1TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc1TsizPid) -> u8 {
        Hc1TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc2CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc2CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc2CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc2CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc2CharEptype {
        Hc2CharEptype::from_bits(val)
    }
}
impl From<Hc2CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc2CharEptype) -> u8 {
        Hc2CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc2TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc2TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc2TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc2TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc2TsizPid {
        Hc2TsizPid::from_bits(val)
    }
}
impl From<Hc2TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc2TsizPid) -> u8 {
        Hc2TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc3CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc3CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc3CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc3CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc3CharEptype {
        Hc3CharEptype::from_bits(val)
    }
}
impl From<Hc3CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc3CharEptype) -> u8 {
        Hc3CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc3TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc3TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc3TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc3TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc3TsizPid {
        Hc3TsizPid::from_bits(val)
    }
}
impl From<Hc3TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc3TsizPid) -> u8 {
        Hc3TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc4CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc4CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc4CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc4CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc4CharEptype {
        Hc4CharEptype::from_bits(val)
    }
}
impl From<Hc4CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc4CharEptype) -> u8 {
        Hc4CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc4TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc4TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc4TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc4TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc4TsizPid {
        Hc4TsizPid::from_bits(val)
    }
}
impl From<Hc4TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc4TsizPid) -> u8 {
        Hc4TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc5CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc5CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc5CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc5CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc5CharEptype {
        Hc5CharEptype::from_bits(val)
    }
}
impl From<Hc5CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc5CharEptype) -> u8 {
        Hc5CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc5TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc5TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc5TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc5TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc5TsizPid {
        Hc5TsizPid::from_bits(val)
    }
}
impl From<Hc5TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc5TsizPid) -> u8 {
        Hc5TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc6CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc6CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc6CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc6CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc6CharEptype {
        Hc6CharEptype::from_bits(val)
    }
}
impl From<Hc6CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc6CharEptype) -> u8 {
        Hc6CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc6TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc6TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc6TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc6TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc6TsizPid {
        Hc6TsizPid::from_bits(val)
    }
}
impl From<Hc6TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc6TsizPid) -> u8 {
        Hc6TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc7CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc7CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc7CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc7CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc7CharEptype {
        Hc7CharEptype::from_bits(val)
    }
}
impl From<Hc7CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc7CharEptype) -> u8 {
        Hc7CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc7TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc7TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc7TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc7TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc7TsizPid {
        Hc7TsizPid::from_bits(val)
    }
}
impl From<Hc7TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc7TsizPid) -> u8 {
        Hc7TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc8CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc8CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc8CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc8CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc8CharEptype {
        Hc8CharEptype::from_bits(val)
    }
}
impl From<Hc8CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc8CharEptype) -> u8 {
        Hc8CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc8TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc8TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc8TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc8TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc8TsizPid {
        Hc8TsizPid::from_bits(val)
    }
}
impl From<Hc8TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc8TsizPid) -> u8 {
        Hc8TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc9CharEptype {
    #[doc = "Control endpoint."]
    Control = 0x0,
    #[doc = "Isochronous endpoint."]
    Iso = 0x01,
    #[doc = "Bulk endpoint."]
    Bulk = 0x02,
    #[doc = "Interrupt endpoint."]
    Int = 0x03,
}
impl Hc9CharEptype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc9CharEptype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc9CharEptype {
    #[inline(always)]
    fn from(val: u8) -> Hc9CharEptype {
        Hc9CharEptype::from_bits(val)
    }
}
impl From<Hc9CharEptype> for u8 {
    #[inline(always)]
    fn from(val: Hc9CharEptype) -> u8 {
        Hc9CharEptype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc9TsizPid {
    #[doc = "DATA0 PID."]
    Data0 = 0x0,
    #[doc = "DATA2 PID."]
    Data2 = 0x01,
    #[doc = "DATA1 PID."]
    Data1 = 0x02,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    Mdata = 0x03,
}
impl Hc9TsizPid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc9TsizPid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc9TsizPid {
    #[inline(always)]
    fn from(val: u8) -> Hc9TsizPid {
        Hc9TsizPid::from_bits(val)
    }
}
impl From<Hc9TsizPid> for u8 {
    #[inline(always)]
    fn from(val: Hc9TsizPid) -> u8 {
        Hc9TsizPid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lemoscctrl {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    None = 0x0,
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    Gate = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lemoscctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lemoscctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lemoscctrl {
    #[inline(always)]
    fn from(val: u8) -> Lemoscctrl {
        Lemoscctrl::from_bits(val)
    }
}
impl From<Lemoscctrl> for u8 {
    #[inline(always)]
    fn from(val: Lemoscctrl) -> u8 {
        Lemoscctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perfrint {
    #[doc = "80% of the frame interval."]
    _80pcnt = 0x0,
    #[doc = "85% of the frame interval."]
    _85pcnt = 0x01,
    #[doc = "90% of the frame interval."]
    _90pcnt = 0x02,
    #[doc = "95% of the frame interval."]
    _95pcnt = 0x03,
}
impl Perfrint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perfrint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perfrint {
    #[inline(always)]
    fn from(val: u8) -> Perfrint {
        Perfrint::from_bits(val)
    }
}
impl From<Perfrint> for u8 {
    #[inline(always)]
    fn from(val: Perfrint) -> u8 {
        Perfrint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prtspd {
    _RESERVED_0 = 0x0,
    #[doc = "Full speed."]
    Fs = 0x01,
    #[doc = "Low speed."]
    Ls = 0x02,
    _RESERVED_3 = 0x03,
}
impl Prtspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prtspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prtspd {
    #[inline(always)]
    fn from(val: u8) -> Prtspd {
        Prtspd::from_bits(val)
    }
}
impl From<Prtspd> for u8 {
    #[inline(always)]
    fn from(val: Prtspd) -> u8 {
        Prtspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prttstctl {
    #[doc = "Test mode disabled."]
    Disable = 0x0,
    #[doc = "Test_J mode."]
    J = 0x01,
    #[doc = "Test_K mode."]
    K = 0x02,
    #[doc = "Test_SE0_NAK mode."]
    Se0nak = 0x03,
    #[doc = "Test_Packet mode."]
    Packet = 0x04,
    #[doc = "Test_Force_Enable."]
    Force = 0x05,
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
impl Prttstctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prttstctl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prttstctl {
    #[inline(always)]
    fn from(val: u8) -> Prttstctl {
        Prttstctl::from_bits(val)
    }
}
impl From<Prttstctl> for u8 {
    #[inline(always)]
    fn from(val: Prttstctl) -> u8 {
        Prttstctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstctl {
    #[doc = "Test mode disabled."]
    Disable = 0x0,
    #[doc = "Test_J mode."]
    J = 0x01,
    #[doc = "Test_K mode."]
    K = 0x02,
    #[doc = "Test_SE0_NAK mode."]
    Se0nak = 0x03,
    #[doc = "Test_Packet mode."]
    Packet = 0x04,
    #[doc = "Test_Force_Enable."]
    Force = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tstctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstctl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstctl {
    #[inline(always)]
    fn from(val: u8) -> Tstctl {
        Tstctl::from_bits(val)
    }
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(val: Tstctl) -> u8 {
        Tstctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfnum {
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush."]
    F0 = 0x0,
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1 = 0x01,
    #[doc = "Device mode: TXFIFO 2 flush."]
    F2 = 0x02,
    #[doc = "Device mode: TXFIFO 3 flush."]
    F3 = 0x03,
    #[doc = "Device mode: TXFIFO 4 flush."]
    F4 = 0x04,
    #[doc = "Device mode: TXFIFO 5 flush."]
    F5 = 0x05,
    #[doc = "Device mode: TXFIFO 6 flush."]
    F6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    Fall = 0x10,
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
impl Txfnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfnum {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfnum {
    #[inline(always)]
    fn from(val: u8) -> Txfnum {
        Txfnum::from_bits(val)
    }
}
impl From<Txfnum> for u8 {
    #[inline(always)]
    fn from(val: Txfnum) -> u8 {
        Txfnum::to_bits(val)
    }
}
