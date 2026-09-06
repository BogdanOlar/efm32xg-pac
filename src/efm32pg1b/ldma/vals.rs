#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch0CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch0CfgArbslots {
        Ch0CfgArbslots::from_bits(val)
    }
}
impl From<Ch0CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch0CfgArbslots) -> u8 {
        Ch0CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch0CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlBlocksize {
        Ch0CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch0CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlBlocksize) -> u8 {
        Ch0CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch0CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlDstinc {
        Ch0CtrlDstinc::from_bits(val)
    }
}
impl From<Ch0CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlDstinc) -> u8 {
        Ch0CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch0CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlSize {
        Ch0CtrlSize::from_bits(val)
    }
}
impl From<Ch0CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlSize) -> u8 {
        Ch0CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch0CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlSrcinc {
        Ch0CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch0CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlSrcinc) -> u8 {
        Ch0CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch0CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch0CtrlStructtype {
        Ch0CtrlStructtype::from_bits(val)
    }
}
impl From<Ch0CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch0CtrlStructtype) -> u8 {
        Ch0CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch0ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch0ReqselSourcesel {
        Ch0ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch0ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch0ReqselSourcesel) -> u8 {
        Ch0ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch1CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch1CfgArbslots {
        Ch1CfgArbslots::from_bits(val)
    }
}
impl From<Ch1CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch1CfgArbslots) -> u8 {
        Ch1CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch1CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlBlocksize {
        Ch1CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch1CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlBlocksize) -> u8 {
        Ch1CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch1CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlDstinc {
        Ch1CtrlDstinc::from_bits(val)
    }
}
impl From<Ch1CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlDstinc) -> u8 {
        Ch1CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch1CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlSize {
        Ch1CtrlSize::from_bits(val)
    }
}
impl From<Ch1CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlSize) -> u8 {
        Ch1CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch1CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlSrcinc {
        Ch1CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch1CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlSrcinc) -> u8 {
        Ch1CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch1CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch1CtrlStructtype {
        Ch1CtrlStructtype::from_bits(val)
    }
}
impl From<Ch1CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch1CtrlStructtype) -> u8 {
        Ch1CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch1ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch1ReqselSourcesel {
        Ch1ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch1ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch1ReqselSourcesel) -> u8 {
        Ch1ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch2CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch2CfgArbslots {
        Ch2CfgArbslots::from_bits(val)
    }
}
impl From<Ch2CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch2CfgArbslots) -> u8 {
        Ch2CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch2CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlBlocksize {
        Ch2CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch2CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlBlocksize) -> u8 {
        Ch2CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch2CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlDstinc {
        Ch2CtrlDstinc::from_bits(val)
    }
}
impl From<Ch2CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlDstinc) -> u8 {
        Ch2CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch2CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlSize {
        Ch2CtrlSize::from_bits(val)
    }
}
impl From<Ch2CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlSize) -> u8 {
        Ch2CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch2CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlSrcinc {
        Ch2CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch2CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlSrcinc) -> u8 {
        Ch2CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch2CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch2CtrlStructtype {
        Ch2CtrlStructtype::from_bits(val)
    }
}
impl From<Ch2CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch2CtrlStructtype) -> u8 {
        Ch2CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch2ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch2ReqselSourcesel {
        Ch2ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch2ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch2ReqselSourcesel) -> u8 {
        Ch2ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch3CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch3CfgArbslots {
        Ch3CfgArbslots::from_bits(val)
    }
}
impl From<Ch3CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch3CfgArbslots) -> u8 {
        Ch3CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch3CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlBlocksize {
        Ch3CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch3CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlBlocksize) -> u8 {
        Ch3CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch3CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlDstinc {
        Ch3CtrlDstinc::from_bits(val)
    }
}
impl From<Ch3CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlDstinc) -> u8 {
        Ch3CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch3CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlSize {
        Ch3CtrlSize::from_bits(val)
    }
}
impl From<Ch3CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlSize) -> u8 {
        Ch3CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch3CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlSrcinc {
        Ch3CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch3CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlSrcinc) -> u8 {
        Ch3CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch3CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch3CtrlStructtype {
        Ch3CtrlStructtype::from_bits(val)
    }
}
impl From<Ch3CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch3CtrlStructtype) -> u8 {
        Ch3CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch3ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch3ReqselSourcesel {
        Ch3ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch3ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch3ReqselSourcesel) -> u8 {
        Ch3ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch4CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch4CfgArbslots {
        Ch4CfgArbslots::from_bits(val)
    }
}
impl From<Ch4CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch4CfgArbslots) -> u8 {
        Ch4CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch4CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlBlocksize {
        Ch4CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch4CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlBlocksize) -> u8 {
        Ch4CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch4CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlDstinc {
        Ch4CtrlDstinc::from_bits(val)
    }
}
impl From<Ch4CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlDstinc) -> u8 {
        Ch4CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch4CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlSize {
        Ch4CtrlSize::from_bits(val)
    }
}
impl From<Ch4CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlSize) -> u8 {
        Ch4CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch4CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlSrcinc {
        Ch4CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch4CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlSrcinc) -> u8 {
        Ch4CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch4CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch4CtrlStructtype {
        Ch4CtrlStructtype::from_bits(val)
    }
}
impl From<Ch4CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch4CtrlStructtype) -> u8 {
        Ch4CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch4ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch4ReqselSourcesel {
        Ch4ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch4ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch4ReqselSourcesel) -> u8 {
        Ch4ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch5CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch5CfgArbslots {
        Ch5CfgArbslots::from_bits(val)
    }
}
impl From<Ch5CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch5CfgArbslots) -> u8 {
        Ch5CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch5CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlBlocksize {
        Ch5CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch5CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlBlocksize) -> u8 {
        Ch5CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch5CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlDstinc {
        Ch5CtrlDstinc::from_bits(val)
    }
}
impl From<Ch5CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlDstinc) -> u8 {
        Ch5CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch5CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlSize {
        Ch5CtrlSize::from_bits(val)
    }
}
impl From<Ch5CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlSize) -> u8 {
        Ch5CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch5CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlSrcinc {
        Ch5CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch5CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlSrcinc) -> u8 {
        Ch5CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch5CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch5CtrlStructtype {
        Ch5CtrlStructtype::from_bits(val)
    }
}
impl From<Ch5CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch5CtrlStructtype) -> u8 {
        Ch5CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch5ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch5ReqselSourcesel {
        Ch5ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch5ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch5ReqselSourcesel) -> u8 {
        Ch5ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch6CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch6CfgArbslots {
        Ch6CfgArbslots::from_bits(val)
    }
}
impl From<Ch6CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch6CfgArbslots) -> u8 {
        Ch6CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch6CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlBlocksize {
        Ch6CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch6CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlBlocksize) -> u8 {
        Ch6CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch6CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlDstinc {
        Ch6CtrlDstinc::from_bits(val)
    }
}
impl From<Ch6CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlDstinc) -> u8 {
        Ch6CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch6CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlSize {
        Ch6CtrlSize::from_bits(val)
    }
}
impl From<Ch6CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlSize) -> u8 {
        Ch6CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch6CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlSrcinc {
        Ch6CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch6CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlSrcinc) -> u8 {
        Ch6CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch6CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch6CtrlStructtype {
        Ch6CtrlStructtype::from_bits(val)
    }
}
impl From<Ch6CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch6CtrlStructtype) -> u8 {
        Ch6CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch6ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch6ReqselSourcesel {
        Ch6ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch6ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch6ReqselSourcesel) -> u8 {
        Ch6ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch7CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch7CfgArbslots {
        Ch7CfgArbslots::from_bits(val)
    }
}
impl From<Ch7CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch7CfgArbslots) -> u8 {
        Ch7CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlBlocksize {
    #[doc = "One unit transfer per arbitration."]
    Unit1 = 0x0,
    #[doc = "Two unit transfers per arbitration."]
    Unit2 = 0x01,
    #[doc = "Three unit transfers per arbitration."]
    Unit3 = 0x02,
    #[doc = "Four unit transfers per arbitration."]
    Unit4 = 0x03,
    #[doc = "Six unit transfers per arbitration."]
    Unit6 = 0x04,
    #[doc = "Eight unit transfers per arbitration."]
    Unit8 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Sixteen unit transfers per arbitration."]
    Unit16 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "32 unit transfers per arbitration."]
    Unit32 = 0x09,
    #[doc = "64 unit transfers per arbitration."]
    Unit64 = 0x0a,
    #[doc = "128 unit transfers per arbitration."]
    Unit128 = 0x0b,
    #[doc = "256 unit transfers per arbitration."]
    Unit256 = 0x0c,
    #[doc = "512 unit transfers per arbitration."]
    Unit512 = 0x0d,
    #[doc = "1024 unit transfers per arbitration."]
    Unit1024 = 0x0e,
    #[doc = "Transfer all units as specified by the XFRCNT field."]
    All = 0x0f,
}
impl Ch7CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlBlocksize {
        Ch7CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch7CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlBlocksize) -> u8 {
        Ch7CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch7CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlDstinc {
        Ch7CtrlDstinc::from_bits(val)
    }
}
impl From<Ch7CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlDstinc) -> u8 {
        Ch7CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch7CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlSize {
        Ch7CtrlSize::from_bits(val)
    }
}
impl From<Ch7CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlSize) -> u8 {
        Ch7CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch7CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlSrcinc {
        Ch7CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch7CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlSrcinc) -> u8 {
        Ch7CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch7CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch7CtrlStructtype {
        Ch7CtrlStructtype::from_bits(val)
    }
}
impl From<Ch7CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch7CtrlStructtype) -> u8 {
        Ch7CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7ReqselSourcesel {
    #[doc = "No source selected."]
    None = 0x0,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Timer 0."]
    Timer0 = 0x18,
    #[doc = "Timer 1."]
    Timer1 = 0x19,
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
    #[doc = "Memory System Controller."]
    Msc = 0x30,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto = 0x31,
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
impl Ch7ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch7ReqselSourcesel {
        Ch7ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch7ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch7ReqselSourcesel) -> u8 {
        Ch7ReqselSourcesel::to_bits(val)
    }
}
