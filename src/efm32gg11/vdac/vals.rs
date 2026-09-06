#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0ctrlTrigmode {
    #[doc = "Channel 0 is triggered by CH0DATA or COMBDATA write."]
    Sw = 0x0,
    #[doc = "Channel 0 is triggered by PRS input."]
    Prs = 0x01,
    #[doc = "Channel 0 is triggered by Refresh timer."]
    Refresh = 0x02,
    #[doc = "Channel 0 is triggered by CH0DATA/COMBDATA write or PRS input."]
    Swprs = 0x03,
    #[doc = "Channel 0 is triggered by CH0DATA/COMBDATA write or Refresh timer."]
    Swrefresh = 0x04,
    #[doc = "Channel 0 is triggered by LESENSE."]
    Lesense = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch0ctrlTrigmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0ctrlTrigmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0ctrlTrigmode {
    #[inline(always)]
    fn from(val: u8) -> Ch0ctrlTrigmode {
        Ch0ctrlTrigmode::from_bits(val)
    }
}
impl From<Ch0ctrlTrigmode> for u8 {
    #[inline(always)]
    fn from(val: Ch0ctrlTrigmode) -> u8 {
        Ch0ctrlTrigmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1ctrlTrigmode {
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write."]
    Sw = 0x0,
    #[doc = "Channel 1 is triggered by PRS input."]
    Prs = 0x01,
    #[doc = "Channel 1 is triggered by Refresh timer."]
    Refresh = 0x02,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input."]
    Swprs = 0x03,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer."]
    Swrefresh = 0x04,
    #[doc = "Channel 1 is triggered by LESENSE."]
    Lesense = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch1ctrlTrigmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1ctrlTrigmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1ctrlTrigmode {
    #[inline(always)]
    fn from(val: u8) -> Ch1ctrlTrigmode {
        Ch1ctrlTrigmode::from_bits(val)
    }
}
impl From<Ch1ctrlTrigmode> for u8 {
    #[inline(always)]
    fn from(val: Ch1ctrlTrigmode) -> u8 {
        Ch1ctrlTrigmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa0CtrlDrivestrength {
    #[doc = "Lower accuracy with Low drive strength."]
    _0 = 0x0,
    #[doc = "Low accuracy with Low drive strength."]
    _1 = 0x01,
    #[doc = "High accuracy with High drive strength."]
    _2 = 0x02,
    #[doc = "Higher accuracy with High drive strength."]
    _3 = 0x03,
}
impl Opa0CtrlDrivestrength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa0CtrlDrivestrength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa0CtrlDrivestrength {
    #[inline(always)]
    fn from(val: u8) -> Opa0CtrlDrivestrength {
        Opa0CtrlDrivestrength::from_bits(val)
    }
}
impl From<Opa0CtrlDrivestrength> for u8 {
    #[inline(always)]
    fn from(val: Opa0CtrlDrivestrength) -> u8 {
        Opa0CtrlDrivestrength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa0MuxResinmux {
    #[doc = "Set for Unity Gain."]
    Disable = 0x0,
    #[doc = "Set for NEXTOUT(x-1) input."]
    Opanext = 0x01,
    #[doc = "NEG pad connected."]
    Negpad = 0x02,
    #[doc = "POS pad connected."]
    Pospad = 0x03,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    Compad = 0x04,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    Center = 0x05,
    #[doc = "VSS connected."]
    Vss = 0x06,
    _RESERVED_7 = 0x07,
}
impl Opa0MuxResinmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa0MuxResinmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa0MuxResinmux {
    #[inline(always)]
    fn from(val: u8) -> Opa0MuxResinmux {
        Opa0MuxResinmux::from_bits(val)
    }
}
impl From<Opa0MuxResinmux> for u8 {
    #[inline(always)]
    fn from(val: Opa0MuxResinmux) -> u8 {
        Opa0MuxResinmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa0MuxRessel {
    #[doc = "Gain of 1/3."]
    Res0 = 0x0,
    #[doc = "Gain of 1."]
    Res1 = 0x01,
    #[doc = "Gain of 1 2/3."]
    Res2 = 0x02,
    #[doc = "Gain of 2 1/5."]
    Res3 = 0x03,
    #[doc = "Gain of 3."]
    Res4 = 0x04,
    #[doc = "Gain of 4 1/3."]
    Res5 = 0x05,
    #[doc = "Gain of 7."]
    Res6 = 0x06,
    #[doc = "Gain of 15."]
    Res7 = 0x07,
}
impl Opa0MuxRessel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa0MuxRessel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa0MuxRessel {
    #[inline(always)]
    fn from(val: u8) -> Opa0MuxRessel {
        Opa0MuxRessel::from_bits(val)
    }
}
impl From<Opa0MuxRessel> for u8 {
    #[inline(always)]
    fn from(val: Opa0MuxRessel) -> u8 {
        Opa0MuxRessel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa0OutAltoutpaden {
    _RESERVED_0 = 0x0,
    #[doc = "Alternate Output 0."]
    Out0 = 0x01,
    #[doc = "Alternate Output 1."]
    Out1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Alternate Output 2."]
    Out2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Alternate Output 3."]
    Out3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Alternate Output 4."]
    Out4 = 0x10,
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
impl Opa0OutAltoutpaden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa0OutAltoutpaden {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa0OutAltoutpaden {
    #[inline(always)]
    fn from(val: u8) -> Opa0OutAltoutpaden {
        Opa0OutAltoutpaden::from_bits(val)
    }
}
impl From<Opa0OutAltoutpaden> for u8 {
    #[inline(always)]
    fn from(val: Opa0OutAltoutpaden) -> u8 {
        Opa0OutAltoutpaden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa1CtrlDrivestrength {
    #[doc = "Lower accuracy with Low drive strength."]
    _0 = 0x0,
    #[doc = "Low accuracy with Low drive strength."]
    _1 = 0x01,
    #[doc = "High accuracy with High drive strength."]
    _2 = 0x02,
    #[doc = "Higher accuracy with High drive strength."]
    _3 = 0x03,
}
impl Opa1CtrlDrivestrength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa1CtrlDrivestrength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa1CtrlDrivestrength {
    #[inline(always)]
    fn from(val: u8) -> Opa1CtrlDrivestrength {
        Opa1CtrlDrivestrength::from_bits(val)
    }
}
impl From<Opa1CtrlDrivestrength> for u8 {
    #[inline(always)]
    fn from(val: Opa1CtrlDrivestrength) -> u8 {
        Opa1CtrlDrivestrength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa1MuxResinmux {
    #[doc = "Set for Unity Gain."]
    Disable = 0x0,
    #[doc = "Set for NEXTOUT(x-1) input."]
    Opanext = 0x01,
    #[doc = "NEG pad connected."]
    Negpad = 0x02,
    #[doc = "POS pad connected."]
    Pospad = 0x03,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    Compad = 0x04,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    Center = 0x05,
    #[doc = "VSS connected."]
    Vss = 0x06,
    _RESERVED_7 = 0x07,
}
impl Opa1MuxResinmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa1MuxResinmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa1MuxResinmux {
    #[inline(always)]
    fn from(val: u8) -> Opa1MuxResinmux {
        Opa1MuxResinmux::from_bits(val)
    }
}
impl From<Opa1MuxResinmux> for u8 {
    #[inline(always)]
    fn from(val: Opa1MuxResinmux) -> u8 {
        Opa1MuxResinmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa1MuxRessel {
    #[doc = "Gain of 1/3."]
    Res0 = 0x0,
    #[doc = "Gain of 1."]
    Res1 = 0x01,
    #[doc = "Gain of 1 2/3."]
    Res2 = 0x02,
    #[doc = "Gain of 2 1/5."]
    Res3 = 0x03,
    #[doc = "Gain of 3."]
    Res4 = 0x04,
    #[doc = "Gain of 4 1/3."]
    Res5 = 0x05,
    #[doc = "Gain of 7."]
    Res6 = 0x06,
    #[doc = "Gain of 15."]
    Res7 = 0x07,
}
impl Opa1MuxRessel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa1MuxRessel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa1MuxRessel {
    #[inline(always)]
    fn from(val: u8) -> Opa1MuxRessel {
        Opa1MuxRessel::from_bits(val)
    }
}
impl From<Opa1MuxRessel> for u8 {
    #[inline(always)]
    fn from(val: Opa1MuxRessel) -> u8 {
        Opa1MuxRessel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa1OutAltoutpaden {
    _RESERVED_0 = 0x0,
    #[doc = "Alternate Output 0."]
    Out0 = 0x01,
    #[doc = "Alternate Output 1."]
    Out1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Alternate Output 2."]
    Out2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Alternate Output 3."]
    Out3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Alternate Output 4."]
    Out4 = 0x10,
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
impl Opa1OutAltoutpaden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa1OutAltoutpaden {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa1OutAltoutpaden {
    #[inline(always)]
    fn from(val: u8) -> Opa1OutAltoutpaden {
        Opa1OutAltoutpaden::from_bits(val)
    }
}
impl From<Opa1OutAltoutpaden> for u8 {
    #[inline(always)]
    fn from(val: Opa1OutAltoutpaden) -> u8 {
        Opa1OutAltoutpaden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa2CtrlDrivestrength {
    #[doc = "Lower accuracy with Low drive strength."]
    _0 = 0x0,
    #[doc = "Low accuracy with Low drive strength."]
    _1 = 0x01,
    #[doc = "High accuracy with High drive strength."]
    _2 = 0x02,
    #[doc = "Higher accuracy with High drive strength."]
    _3 = 0x03,
}
impl Opa2CtrlDrivestrength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa2CtrlDrivestrength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa2CtrlDrivestrength {
    #[inline(always)]
    fn from(val: u8) -> Opa2CtrlDrivestrength {
        Opa2CtrlDrivestrength::from_bits(val)
    }
}
impl From<Opa2CtrlDrivestrength> for u8 {
    #[inline(always)]
    fn from(val: Opa2CtrlDrivestrength) -> u8 {
        Opa2CtrlDrivestrength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa2MuxResinmux {
    #[doc = "Set for Unity Gain."]
    Disable = 0x0,
    #[doc = "Set for NEXTOUT(x-1) input."]
    Opanext = 0x01,
    #[doc = "NEG pad connected."]
    Negpad = 0x02,
    #[doc = "POS pad connected."]
    Pospad = 0x03,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    Compad = 0x04,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    Center = 0x05,
    #[doc = "VSS connected."]
    Vss = 0x06,
    _RESERVED_7 = 0x07,
}
impl Opa2MuxResinmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa2MuxResinmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa2MuxResinmux {
    #[inline(always)]
    fn from(val: u8) -> Opa2MuxResinmux {
        Opa2MuxResinmux::from_bits(val)
    }
}
impl From<Opa2MuxResinmux> for u8 {
    #[inline(always)]
    fn from(val: Opa2MuxResinmux) -> u8 {
        Opa2MuxResinmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa2MuxRessel {
    #[doc = "Gain of 1/3."]
    Res0 = 0x0,
    #[doc = "Gain of 1."]
    Res1 = 0x01,
    #[doc = "Gain of 1 2/3."]
    Res2 = 0x02,
    #[doc = "Gain of 2 1/5."]
    Res3 = 0x03,
    #[doc = "Gain of 3."]
    Res4 = 0x04,
    #[doc = "Gain of 4 1/3."]
    Res5 = 0x05,
    #[doc = "Gain of 7."]
    Res6 = 0x06,
    #[doc = "Gain of 15."]
    Res7 = 0x07,
}
impl Opa2MuxRessel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa2MuxRessel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa2MuxRessel {
    #[inline(always)]
    fn from(val: u8) -> Opa2MuxRessel {
        Opa2MuxRessel::from_bits(val)
    }
}
impl From<Opa2MuxRessel> for u8 {
    #[inline(always)]
    fn from(val: Opa2MuxRessel) -> u8 {
        Opa2MuxRessel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa2OutAltoutpaden {
    _RESERVED_0 = 0x0,
    #[doc = "Alternate Output 0."]
    Out0 = 0x01,
    #[doc = "Alternate Output 1."]
    Out1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Alternate Output 2."]
    Out2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Alternate Output 3."]
    Out3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Alternate Output 4."]
    Out4 = 0x10,
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
impl Opa2OutAltoutpaden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa2OutAltoutpaden {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa2OutAltoutpaden {
    #[inline(always)]
    fn from(val: u8) -> Opa2OutAltoutpaden {
        Opa2OutAltoutpaden::from_bits(val)
    }
}
impl From<Opa2OutAltoutpaden> for u8 {
    #[inline(always)]
    fn from(val: Opa2OutAltoutpaden) -> u8 {
        Opa2OutAltoutpaden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa3CtrlDrivestrength {
    #[doc = "Lower accuracy with Low drive strength."]
    _0 = 0x0,
    #[doc = "Low accuracy with Low drive strength."]
    _1 = 0x01,
    #[doc = "High accuracy with High drive strength."]
    _2 = 0x02,
    #[doc = "Higher accuracy with High drive strength."]
    _3 = 0x03,
}
impl Opa3CtrlDrivestrength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa3CtrlDrivestrength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa3CtrlDrivestrength {
    #[inline(always)]
    fn from(val: u8) -> Opa3CtrlDrivestrength {
        Opa3CtrlDrivestrength::from_bits(val)
    }
}
impl From<Opa3CtrlDrivestrength> for u8 {
    #[inline(always)]
    fn from(val: Opa3CtrlDrivestrength) -> u8 {
        Opa3CtrlDrivestrength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa3MuxResinmux {
    #[doc = "Set for Unity Gain."]
    Disable = 0x0,
    #[doc = "Set for NEXTOUT(x-1) input."]
    Opanext = 0x01,
    #[doc = "NEG pad connected."]
    Negpad = 0x02,
    #[doc = "POS pad connected."]
    Pospad = 0x03,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    Compad = 0x04,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    Center = 0x05,
    #[doc = "VSS connected."]
    Vss = 0x06,
    _RESERVED_7 = 0x07,
}
impl Opa3MuxResinmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa3MuxResinmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa3MuxResinmux {
    #[inline(always)]
    fn from(val: u8) -> Opa3MuxResinmux {
        Opa3MuxResinmux::from_bits(val)
    }
}
impl From<Opa3MuxResinmux> for u8 {
    #[inline(always)]
    fn from(val: Opa3MuxResinmux) -> u8 {
        Opa3MuxResinmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa3MuxRessel {
    #[doc = "Gain of 1/3."]
    Res0 = 0x0,
    #[doc = "Gain of 1."]
    Res1 = 0x01,
    #[doc = "Gain of 1 2/3."]
    Res2 = 0x02,
    #[doc = "Gain of 2 1/5."]
    Res3 = 0x03,
    #[doc = "Gain of 3."]
    Res4 = 0x04,
    #[doc = "Gain of 4 1/3."]
    Res5 = 0x05,
    #[doc = "Gain of 7."]
    Res6 = 0x06,
    #[doc = "Gain of 15."]
    Res7 = 0x07,
}
impl Opa3MuxRessel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa3MuxRessel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa3MuxRessel {
    #[inline(always)]
    fn from(val: u8) -> Opa3MuxRessel {
        Opa3MuxRessel::from_bits(val)
    }
}
impl From<Opa3MuxRessel> for u8 {
    #[inline(always)]
    fn from(val: Opa3MuxRessel) -> u8 {
        Opa3MuxRessel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opa3OutAltoutpaden {
    _RESERVED_0 = 0x0,
    #[doc = "Alternate Output 0."]
    Out0 = 0x01,
    #[doc = "Alternate Output 1."]
    Out1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Alternate Output 2."]
    Out2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Alternate Output 3."]
    Out3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Alternate Output 4."]
    Out4 = 0x10,
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
impl Opa3OutAltoutpaden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opa3OutAltoutpaden {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opa3OutAltoutpaden {
    #[inline(always)]
    fn from(val: u8) -> Opa3OutAltoutpaden {
        Opa3OutAltoutpaden::from_bits(val)
    }
}
impl From<Opa3OutAltoutpaden> for u8 {
    #[inline(always)]
    fn from(val: Opa3OutAltoutpaden) -> u8 {
        Opa3OutAltoutpaden::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Presc(u8);
impl Presc {
    pub const Nodivision: Self = Self(0x0);
}
impl Presc {
    pub const fn from_bits(val: u8) -> Presc {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Presc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Nodivision"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Nodivision"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Presc {
    #[inline(always)]
    fn from(val: u8) -> Presc {
        Presc::from_bits(val)
    }
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(val: Presc) -> u8 {
        Presc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refreshperiod {
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles."]
    _8cycles = 0x0,
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles."]
    _16cycles = 0x01,
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles."]
    _32cycles = 0x02,
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles."]
    _64cycles = 0x03,
}
impl Refreshperiod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refreshperiod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refreshperiod {
    #[inline(always)]
    fn from(val: u8) -> Refreshperiod {
        Refreshperiod::from_bits(val)
    }
}
impl From<Refreshperiod> for u8 {
    #[inline(always)]
    fn from(val: Refreshperiod) -> u8 {
        Refreshperiod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "Internal low noise 1.25 V bandgap reference."]
    _1v25ln = 0x0,
    #[doc = "Internal low noise 2.5 V bandgap reference."]
    _2v5ln = 0x01,
    #[doc = "Internal 1.25 V bandgap reference."]
    _1v25 = 0x02,
    #[doc = "Internal 2.5 V bandgap reference."]
    _2v5 = 0x03,
    #[doc = "AVDD reference."]
    Vdd = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "External pin reference."]
    Ext = 0x06,
    _RESERVED_7 = 0x07,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
