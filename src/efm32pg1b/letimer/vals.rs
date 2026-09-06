#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out0loc {
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
    #[doc = "Location 18."]
    Loc18 = 0x12,
    #[doc = "Location 19."]
    Loc19 = 0x13,
    #[doc = "Location 20."]
    Loc20 = 0x14,
    #[doc = "Location 21."]
    Loc21 = 0x15,
    #[doc = "Location 22."]
    Loc22 = 0x16,
    #[doc = "Location 23."]
    Loc23 = 0x17,
    #[doc = "Location 24."]
    Loc24 = 0x18,
    #[doc = "Location 25."]
    Loc25 = 0x19,
    #[doc = "Location 26."]
    Loc26 = 0x1a,
    #[doc = "Location 27."]
    Loc27 = 0x1b,
    #[doc = "Location 28."]
    Loc28 = 0x1c,
    #[doc = "Location 29."]
    Loc29 = 0x1d,
    #[doc = "Location 30."]
    Loc30 = 0x1e,
    #[doc = "Location 31."]
    Loc31 = 0x1f,
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
impl Out0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out0loc {
    #[inline(always)]
    fn from(val: u8) -> Out0loc {
        Out0loc::from_bits(val)
    }
}
impl From<Out0loc> for u8 {
    #[inline(always)]
    fn from(val: Out0loc) -> u8 {
        Out0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out1loc {
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
    #[doc = "Location 18."]
    Loc18 = 0x12,
    #[doc = "Location 19."]
    Loc19 = 0x13,
    #[doc = "Location 20."]
    Loc20 = 0x14,
    #[doc = "Location 21."]
    Loc21 = 0x15,
    #[doc = "Location 22."]
    Loc22 = 0x16,
    #[doc = "Location 23."]
    Loc23 = 0x17,
    #[doc = "Location 24."]
    Loc24 = 0x18,
    #[doc = "Location 25."]
    Loc25 = 0x19,
    #[doc = "Location 26."]
    Loc26 = 0x1a,
    #[doc = "Location 27."]
    Loc27 = 0x1b,
    #[doc = "Location 28."]
    Loc28 = 0x1c,
    #[doc = "Location 29."]
    Loc29 = 0x1d,
    #[doc = "Location 30."]
    Loc30 = 0x1e,
    #[doc = "Location 31."]
    Loc31 = 0x1f,
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
impl Out1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out1loc {
    #[inline(always)]
    fn from(val: u8) -> Out1loc {
        Out1loc::from_bits(val)
    }
}
impl From<Out1loc> for u8 {
    #[inline(always)]
    fn from(val: Out1loc) -> u8 {
        Out1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prsclearmode {
    #[doc = "PRS cannot clear the LETIMER."]
    None = 0x0,
    #[doc = "Rising edge of selected PRS input can clear the LETIMER."]
    Rising = 0x01,
    #[doc = "Falling edge of selected PRS input can clear the LETIMER."]
    Falling = 0x02,
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER."]
    Both = 0x03,
}
impl Prsclearmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prsclearmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prsclearmode {
    #[inline(always)]
    fn from(val: u8) -> Prsclearmode {
        Prsclearmode::from_bits(val)
    }
}
impl From<Prsclearmode> for u8 {
    #[inline(always)]
    fn from(val: Prsclearmode) -> u8 {
        Prsclearmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prsstartmode {
    #[doc = "PRS cannot start the LETIMER."]
    None = 0x0,
    #[doc = "Rising edge of selected PRS input can start the LETIMER."]
    Rising = 0x01,
    #[doc = "Falling edge of selected PRS input can start the LETIMER."]
    Falling = 0x02,
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER."]
    Both = 0x03,
}
impl Prsstartmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prsstartmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prsstartmode {
    #[inline(always)]
    fn from(val: u8) -> Prsstartmode {
        Prsstartmode::from_bits(val)
    }
}
impl From<Prsstartmode> for u8 {
    #[inline(always)]
    fn from(val: Prsstartmode) -> u8 {
        Prsstartmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prsstopmode {
    #[doc = "PRS cannot stop the LETIMER."]
    None = 0x0,
    #[doc = "Rising edge of selected PRS input can stop the LETIMER."]
    Rising = 0x01,
    #[doc = "Falling edge of selected PRS input can stop the LETIMER."]
    Falling = 0x02,
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER."]
    Both = 0x03,
}
impl Prsstopmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prsstopmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prsstopmode {
    #[inline(always)]
    fn from(val: u8) -> Prsstopmode {
        Prsstopmode::from_bits(val)
    }
}
impl From<Prsstopmode> for u8 {
    #[inline(always)]
    fn from(val: Prsstopmode) -> u8 {
        Prsstopmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repmode {
    #[doc = "When started, the LETIMER counts down until it is stopped by software."]
    Free = 0x0,
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    Oneshot = 0x01,
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops."]
    Buffered = 0x02,
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero."]
    Double = 0x03,
}
impl Repmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repmode {
    #[inline(always)]
    fn from(val: u8) -> Repmode {
        Repmode::from_bits(val)
    }
}
impl From<Repmode> for u8 {
    #[inline(always)]
    fn from(val: Repmode) -> u8 {
        Repmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ufoa0 {
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0."]
    None = 0x0,
    #[doc = "LETn_O0 is toggled on CNT underflow."]
    Toggle = 0x01,
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    Pulse = 0x02,
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1."]
    Pwm = 0x03,
}
impl Ufoa0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ufoa0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ufoa0 {
    #[inline(always)]
    fn from(val: u8) -> Ufoa0 {
        Ufoa0::from_bits(val)
    }
}
impl From<Ufoa0> for u8 {
    #[inline(always)]
    fn from(val: Ufoa0) -> u8 {
        Ufoa0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ufoa1 {
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1."]
    None = 0x0,
    #[doc = "LETn_O1 is toggled on CNT underflow."]
    Toggle = 0x01,
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    Pulse = 0x02,
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1."]
    Pwm = 0x03,
}
impl Ufoa1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ufoa1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ufoa1 {
    #[inline(always)]
    fn from(val: u8) -> Ufoa1 {
        Ufoa1::from_bits(val)
    }
}
impl From<Ufoa1> for u8 {
    #[inline(always)]
    fn from(val: Ufoa1) -> u8 {
        Ufoa1::to_bits(val)
    }
}
