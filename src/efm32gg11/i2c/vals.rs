#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bito {
    #[doc = "Timeout disabled."]
    Off = 0x0,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40pcc = 0x01,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80pcc = 0x02,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160pcc = 0x03,
}
impl Bito {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bito {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bito {
    #[inline(always)]
    fn from(val: u8) -> Bito {
        Bito::from_bits(val)
    }
}
impl From<Bito> for u8 {
    #[inline(always)]
    fn from(val: Bito) -> u8 {
        Bito::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clhr {
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4."]
    Standard = 0x0,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3."]
    Asymmetric = 0x01,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6."]
    Fast = 0x02,
    _RESERVED_3 = 0x03,
}
impl Clhr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clhr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clhr {
    #[inline(always)]
    fn from(val: u8) -> Clhr {
        Clhr::from_bits(val)
    }
}
impl From<Clhr> for u8 {
    #[inline(always)]
    fn from(val: Clhr) -> u8 {
        Clhr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clto {
    #[doc = "Timeout disabled."]
    Off = 0x0,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40pcc = 0x01,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80pcc = 0x02,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160pcc = 0x03,
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320pcc = 0x04,
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024pcc = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Clto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clto {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clto {
    #[inline(always)]
    fn from(val: u8) -> Clto {
        Clto::from_bits(val)
    }
}
impl From<Clto> for u8 {
    #[inline(always)]
    fn from(val: Clto) -> u8 {
        Clto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sclloc {
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
impl Sclloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sclloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sclloc {
    #[inline(always)]
    fn from(val: u8) -> Sclloc {
        Sclloc::from_bits(val)
    }
}
impl From<Sclloc> for u8 {
    #[inline(always)]
    fn from(val: Sclloc) -> u8 {
        Sclloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdaloc {
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
impl Sdaloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdaloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdaloc {
    #[inline(always)]
    fn from(val: u8) -> Sdaloc {
        Sdaloc::from_bits(val)
    }
}
impl From<Sdaloc> for u8 {
    #[inline(always)]
    fn from(val: Sdaloc) -> u8 {
        Sdaloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    #[doc = "No transmission is being performed."]
    Idle = 0x0,
    #[doc = "Waiting for idle. Will send a start condition as soon as the bus is idle."]
    Wait = 0x01,
    #[doc = "Start transmitted or received."]
    Start = 0x02,
    #[doc = "Address transmitted or received."]
    Addr = 0x03,
    #[doc = "Address ack/nack transmitted or received."]
    Addrack = 0x04,
    #[doc = "Data transmitted or received."]
    Data = 0x05,
    #[doc = "Data ack/nack transmitted or received."]
    Dataack = 0x06,
    _RESERVED_7 = 0x07,
}
impl State {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State {
    #[inline(always)]
    fn from(val: u8) -> State {
        State::from_bits(val)
    }
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(val: State) -> u8 {
        State::to_bits(val)
    }
}
