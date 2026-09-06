#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch0CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlEdsel {
        Ch0CtrlEdsel::from_bits(val)
    }
}
impl From<Ch0CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlEdsel) -> u8 {
        Ch0CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch0CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlSourcesel {
        Ch0CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch0CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlSourcesel) -> u8 {
        Ch0CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0loc {
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
impl Ch0loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0loc {
    #[inline(always)]
    fn from(val: u8) -> Ch0loc {
        Ch0loc::from_bits(val)
    }
}
impl From<Ch0loc> for u8 {
    #[inline(always)]
    fn from(val: Ch0loc) -> u8 {
        Ch0loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch10CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlEdsel {
        Ch10CtrlEdsel::from_bits(val)
    }
}
impl From<Ch10CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlEdsel) -> u8 {
        Ch10CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch10CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlSourcesel {
        Ch10CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch10CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlSourcesel) -> u8 {
        Ch10CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch10loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10loc {
    #[inline(always)]
    fn from(val: u8) -> Ch10loc {
        Ch10loc::from_bits(val)
    }
}
impl From<Ch10loc> for u8 {
    #[inline(always)]
    fn from(val: Ch10loc) -> u8 {
        Ch10loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch11CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlEdsel {
        Ch11CtrlEdsel::from_bits(val)
    }
}
impl From<Ch11CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlEdsel) -> u8 {
        Ch11CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch11CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlSourcesel {
        Ch11CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch11CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlSourcesel) -> u8 {
        Ch11CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch11loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11loc {
    #[inline(always)]
    fn from(val: u8) -> Ch11loc {
        Ch11loc::from_bits(val)
    }
}
impl From<Ch11loc> for u8 {
    #[inline(always)]
    fn from(val: Ch11loc) -> u8 {
        Ch11loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch12CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlEdsel {
        Ch12CtrlEdsel::from_bits(val)
    }
}
impl From<Ch12CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlEdsel) -> u8 {
        Ch12CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch12CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlSourcesel {
        Ch12CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch12CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlSourcesel) -> u8 {
        Ch12CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch12loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12loc {
    #[inline(always)]
    fn from(val: u8) -> Ch12loc {
        Ch12loc::from_bits(val)
    }
}
impl From<Ch12loc> for u8 {
    #[inline(always)]
    fn from(val: Ch12loc) -> u8 {
        Ch12loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch13CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlEdsel {
        Ch13CtrlEdsel::from_bits(val)
    }
}
impl From<Ch13CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlEdsel) -> u8 {
        Ch13CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch13CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlSourcesel {
        Ch13CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch13CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlSourcesel) -> u8 {
        Ch13CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch13loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13loc {
    #[inline(always)]
    fn from(val: u8) -> Ch13loc {
        Ch13loc::from_bits(val)
    }
}
impl From<Ch13loc> for u8 {
    #[inline(always)]
    fn from(val: Ch13loc) -> u8 {
        Ch13loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch14CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlEdsel {
        Ch14CtrlEdsel::from_bits(val)
    }
}
impl From<Ch14CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlEdsel) -> u8 {
        Ch14CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch14CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlSourcesel {
        Ch14CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch14CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlSourcesel) -> u8 {
        Ch14CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch14loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14loc {
    #[inline(always)]
    fn from(val: u8) -> Ch14loc {
        Ch14loc::from_bits(val)
    }
}
impl From<Ch14loc> for u8 {
    #[inline(always)]
    fn from(val: Ch14loc) -> u8 {
        Ch14loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch15CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlEdsel {
        Ch15CtrlEdsel::from_bits(val)
    }
}
impl From<Ch15CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlEdsel) -> u8 {
        Ch15CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch15CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlSourcesel {
        Ch15CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch15CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlSourcesel) -> u8 {
        Ch15CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch15loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15loc {
    #[inline(always)]
    fn from(val: u8) -> Ch15loc {
        Ch15loc::from_bits(val)
    }
}
impl From<Ch15loc> for u8 {
    #[inline(always)]
    fn from(val: Ch15loc) -> u8 {
        Ch15loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch16CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlEdsel {
        Ch16CtrlEdsel::from_bits(val)
    }
}
impl From<Ch16CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlEdsel) -> u8 {
        Ch16CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch16CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlSourcesel {
        Ch16CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch16CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlSourcesel) -> u8 {
        Ch16CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch16loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16loc {
    #[inline(always)]
    fn from(val: u8) -> Ch16loc {
        Ch16loc::from_bits(val)
    }
}
impl From<Ch16loc> for u8 {
    #[inline(always)]
    fn from(val: Ch16loc) -> u8 {
        Ch16loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch17CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlEdsel {
        Ch17CtrlEdsel::from_bits(val)
    }
}
impl From<Ch17CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlEdsel) -> u8 {
        Ch17CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch17CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlSourcesel {
        Ch17CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch17CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlSourcesel) -> u8 {
        Ch17CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch17loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17loc {
    #[inline(always)]
    fn from(val: u8) -> Ch17loc {
        Ch17loc::from_bits(val)
    }
}
impl From<Ch17loc> for u8 {
    #[inline(always)]
    fn from(val: Ch17loc) -> u8 {
        Ch17loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch18CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlEdsel {
        Ch18CtrlEdsel::from_bits(val)
    }
}
impl From<Ch18CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlEdsel) -> u8 {
        Ch18CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch18CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlSourcesel {
        Ch18CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch18CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlSourcesel) -> u8 {
        Ch18CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch18loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18loc {
    #[inline(always)]
    fn from(val: u8) -> Ch18loc {
        Ch18loc::from_bits(val)
    }
}
impl From<Ch18loc> for u8 {
    #[inline(always)]
    fn from(val: Ch18loc) -> u8 {
        Ch18loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch19CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlEdsel {
        Ch19CtrlEdsel::from_bits(val)
    }
}
impl From<Ch19CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlEdsel) -> u8 {
        Ch19CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch19CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlSourcesel {
        Ch19CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch19CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlSourcesel) -> u8 {
        Ch19CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch19loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19loc {
    #[inline(always)]
    fn from(val: u8) -> Ch19loc {
        Ch19loc::from_bits(val)
    }
}
impl From<Ch19loc> for u8 {
    #[inline(always)]
    fn from(val: Ch19loc) -> u8 {
        Ch19loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch1CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlEdsel {
        Ch1CtrlEdsel::from_bits(val)
    }
}
impl From<Ch1CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlEdsel) -> u8 {
        Ch1CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch1CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlSourcesel {
        Ch1CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch1CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlSourcesel) -> u8 {
        Ch1CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1loc {
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
impl Ch1loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1loc {
    #[inline(always)]
    fn from(val: u8) -> Ch1loc {
        Ch1loc::from_bits(val)
    }
}
impl From<Ch1loc> for u8 {
    #[inline(always)]
    fn from(val: Ch1loc) -> u8 {
        Ch1loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch20CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlEdsel {
        Ch20CtrlEdsel::from_bits(val)
    }
}
impl From<Ch20CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlEdsel) -> u8 {
        Ch20CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch20CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlSourcesel {
        Ch20CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch20CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlSourcesel) -> u8 {
        Ch20CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch20loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20loc {
    #[inline(always)]
    fn from(val: u8) -> Ch20loc {
        Ch20loc::from_bits(val)
    }
}
impl From<Ch20loc> for u8 {
    #[inline(always)]
    fn from(val: Ch20loc) -> u8 {
        Ch20loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch21CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlEdsel {
        Ch21CtrlEdsel::from_bits(val)
    }
}
impl From<Ch21CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlEdsel) -> u8 {
        Ch21CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch21CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlSourcesel {
        Ch21CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch21CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlSourcesel) -> u8 {
        Ch21CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch21loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21loc {
    #[inline(always)]
    fn from(val: u8) -> Ch21loc {
        Ch21loc::from_bits(val)
    }
}
impl From<Ch21loc> for u8 {
    #[inline(always)]
    fn from(val: Ch21loc) -> u8 {
        Ch21loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch22CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlEdsel {
        Ch22CtrlEdsel::from_bits(val)
    }
}
impl From<Ch22CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlEdsel) -> u8 {
        Ch22CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch22CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlSourcesel {
        Ch22CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch22CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlSourcesel) -> u8 {
        Ch22CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch22loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22loc {
    #[inline(always)]
    fn from(val: u8) -> Ch22loc {
        Ch22loc::from_bits(val)
    }
}
impl From<Ch22loc> for u8 {
    #[inline(always)]
    fn from(val: Ch22loc) -> u8 {
        Ch22loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch23CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlEdsel {
        Ch23CtrlEdsel::from_bits(val)
    }
}
impl From<Ch23CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlEdsel) -> u8 {
        Ch23CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch23CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlSourcesel {
        Ch23CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch23CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlSourcesel) -> u8 {
        Ch23CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch23loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23loc {
    #[inline(always)]
    fn from(val: u8) -> Ch23loc {
        Ch23loc::from_bits(val)
    }
}
impl From<Ch23loc> for u8 {
    #[inline(always)]
    fn from(val: Ch23loc) -> u8 {
        Ch23loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch2CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlEdsel {
        Ch2CtrlEdsel::from_bits(val)
    }
}
impl From<Ch2CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlEdsel) -> u8 {
        Ch2CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch2CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlSourcesel {
        Ch2CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch2CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlSourcesel) -> u8 {
        Ch2CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2loc {
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
impl Ch2loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2loc {
    #[inline(always)]
    fn from(val: u8) -> Ch2loc {
        Ch2loc::from_bits(val)
    }
}
impl From<Ch2loc> for u8 {
    #[inline(always)]
    fn from(val: Ch2loc) -> u8 {
        Ch2loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch3CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlEdsel {
        Ch3CtrlEdsel::from_bits(val)
    }
}
impl From<Ch3CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlEdsel) -> u8 {
        Ch3CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch3CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlSourcesel {
        Ch3CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch3CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlSourcesel) -> u8 {
        Ch3CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3loc {
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
impl Ch3loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3loc {
    #[inline(always)]
    fn from(val: u8) -> Ch3loc {
        Ch3loc::from_bits(val)
    }
}
impl From<Ch3loc> for u8 {
    #[inline(always)]
    fn from(val: Ch3loc) -> u8 {
        Ch3loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch4CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlEdsel {
        Ch4CtrlEdsel::from_bits(val)
    }
}
impl From<Ch4CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlEdsel) -> u8 {
        Ch4CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch4CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlSourcesel {
        Ch4CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch4CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlSourcesel) -> u8 {
        Ch4CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch4loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4loc {
    #[inline(always)]
    fn from(val: u8) -> Ch4loc {
        Ch4loc::from_bits(val)
    }
}
impl From<Ch4loc> for u8 {
    #[inline(always)]
    fn from(val: Ch4loc) -> u8 {
        Ch4loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch5CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlEdsel {
        Ch5CtrlEdsel::from_bits(val)
    }
}
impl From<Ch5CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlEdsel) -> u8 {
        Ch5CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch5CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlSourcesel {
        Ch5CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch5CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlSourcesel) -> u8 {
        Ch5CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch5loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5loc {
    #[inline(always)]
    fn from(val: u8) -> Ch5loc {
        Ch5loc::from_bits(val)
    }
}
impl From<Ch5loc> for u8 {
    #[inline(always)]
    fn from(val: Ch5loc) -> u8 {
        Ch5loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch6CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlEdsel {
        Ch6CtrlEdsel::from_bits(val)
    }
}
impl From<Ch6CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlEdsel) -> u8 {
        Ch6CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch6CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlSourcesel {
        Ch6CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch6CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlSourcesel) -> u8 {
        Ch6CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch6loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6loc {
    #[inline(always)]
    fn from(val: u8) -> Ch6loc {
        Ch6loc::from_bits(val)
    }
}
impl From<Ch6loc> for u8 {
    #[inline(always)]
    fn from(val: Ch6loc) -> u8 {
        Ch6loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch7CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlEdsel {
        Ch7CtrlEdsel::from_bits(val)
    }
}
impl From<Ch7CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlEdsel) -> u8 {
        Ch7CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch7CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlSourcesel {
        Ch7CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch7CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlSourcesel) -> u8 {
        Ch7CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch7loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7loc {
    #[inline(always)]
    fn from(val: u8) -> Ch7loc {
        Ch7loc::from_bits(val)
    }
}
impl From<Ch7loc> for u8 {
    #[inline(always)]
    fn from(val: Ch7loc) -> u8 {
        Ch7loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch8CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlEdsel {
        Ch8CtrlEdsel::from_bits(val)
    }
}
impl From<Ch8CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlEdsel) -> u8 {
        Ch8CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch8CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlSourcesel {
        Ch8CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch8CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlSourcesel) -> u8 {
        Ch8CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch8loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8loc {
    #[inline(always)]
    fn from(val: u8) -> Ch8loc {
        Ch8loc::from_bits(val)
    }
}
impl From<Ch8loc> for u8 {
    #[inline(always)]
    fn from(val: Ch8loc) -> u8 {
        Ch8loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlEdsel {
    #[doc = "Signal is left as it is."]
    Off = 0x0,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal."]
    Posedge = 0x01,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal."]
    Negedge = 0x02,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal."]
    Bothedges = 0x03,
}
impl Ch9CtrlEdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlEdsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlEdsel {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlEdsel {
        Ch9CtrlEdsel::from_bits(val)
    }
}
impl From<Ch9CtrlEdsel> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlEdsel) -> u8 {
        Ch9CtrlEdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prsl = 0x01,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x02,
    #[doc = "Peripheral Reflex System."]
    Prsh = 0x03,
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x04,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x05,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x06,
    #[doc = "Real-Time Counter."]
    Rtc = 0x07,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x08,
    #[doc = "General purpose Input/Output."]
    Gpiol = 0x09,
    #[doc = "General purpose Input/Output."]
    Gpioh = 0x0a,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x0b,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x0c,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x0d,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x0e,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x0f,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x10,
    #[doc = "Clock Management Unit."]
    Cmu = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x17,
    #[doc = "Low Energy Sensor Interface."]
    Lesensel = 0x18,
    #[doc = "Low Energy Sensor Interface."]
    Lesenseh = 0x19,
    #[doc = "Low Energy Sensor Interface."]
    Lesensed = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x1c,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x1d,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x1e,
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
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x30,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x31,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x35,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x36,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "Timer 0."]
    Timer0 = 0x3c,
    #[doc = "Timer 1."]
    Timer1 = 0x3d,
    #[doc = "Timer 2."]
    Timer2 = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    Cm4 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Timer 3."]
    Timer3 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x52,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x53,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x54,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    #[doc = "Timer 4."]
    Timer4 = 0x62,
    #[doc = "Timer 5."]
    Timer5 = 0x63,
    #[doc = "Timer 6."]
    Timer6 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ch9CtrlSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlSourcesel {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlSourcesel {
        Ch9CtrlSourcesel::from_bits(val)
    }
}
impl From<Ch9CtrlSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlSourcesel) -> u8 {
        Ch9CtrlSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9loc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
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
impl Ch9loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9loc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9loc {
    #[inline(always)]
    fn from(val: u8) -> Ch9loc {
        Ch9loc::from_bits(val)
    }
}
impl From<Ch9loc> for u8 {
    #[inline(always)]
    fn from(val: Ch9loc) -> u8 {
        Ch9loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prssel {
    #[doc = "PRS ch 0 triggers scan sequence."]
    Prsch0 = 0x0,
    #[doc = "PRS ch 1 triggers scan sequence."]
    Prsch1 = 0x01,
    #[doc = "PRS ch 2 triggers scan sequence."]
    Prsch2 = 0x02,
    #[doc = "PRS ch 3 triggers scan sequence."]
    Prsch3 = 0x03,
    #[doc = "PRS ch 4 triggers scan sequence."]
    Prsch4 = 0x04,
    #[doc = "PRS ch 5 triggers scan sequence."]
    Prsch5 = 0x05,
    #[doc = "PRS ch 6 triggers scan sequence."]
    Prsch6 = 0x06,
    #[doc = "PRS ch 7 triggers scan sequence."]
    Prsch7 = 0x07,
    #[doc = "PRS ch 8 triggers scan sequence."]
    Prsch8 = 0x08,
    #[doc = "PRS ch 9 triggers scan sequence."]
    Prsch9 = 0x09,
    #[doc = "PRS ch 10 triggers scan sequence."]
    Prsch10 = 0x0a,
    #[doc = "PRS ch 11 triggers scan sequence."]
    Prsch11 = 0x0b,
    #[doc = "PRS ch 12 triggers scan sequence."]
    Prsch12 = 0x0c,
    #[doc = "PRS ch 13 triggers scan sequence."]
    Prsch13 = 0x0d,
    #[doc = "PRS ch 14 triggers scan sequence."]
    Prsch14 = 0x0e,
    #[doc = "PRS ch 15 triggers scan sequence."]
    Prsch15 = 0x0f,
    #[doc = "PRS ch 16 triggers scan sequence."]
    Prsch16 = 0x10,
    #[doc = "PRS ch 17 triggers scan sequence."]
    Prsch17 = 0x11,
    #[doc = "PRS ch 18 triggers scan sequence."]
    Prsch18 = 0x12,
    #[doc = "PRS ch 19 triggers scan sequence."]
    Prsch19 = 0x13,
    #[doc = "PRS ch 20 triggers scan sequence."]
    Prsch20 = 0x14,
    #[doc = "PRS ch 21 triggers scan sequence."]
    Prsch21 = 0x15,
    #[doc = "PRS ch 22 triggers scan sequence."]
    Prsch22 = 0x16,
    #[doc = "PRS ch 23 triggers scan sequence."]
    Prsch23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Prssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prssel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prssel {
    #[inline(always)]
    fn from(val: u8) -> Prssel {
        Prssel::from_bits(val)
    }
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(val: Prssel) -> u8 {
        Prssel::to_bits(val)
    }
}
