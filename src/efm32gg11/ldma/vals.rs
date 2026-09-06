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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
pub enum Ch10CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch10CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch10CfgArbslots {
        Ch10CfgArbslots::from_bits(val)
    }
}
impl From<Ch10CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch10CfgArbslots) -> u8 {
        Ch10CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlBlocksize {
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
impl Ch10CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlBlocksize {
        Ch10CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch10CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlBlocksize) -> u8 {
        Ch10CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch10CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlDstinc {
        Ch10CtrlDstinc::from_bits(val)
    }
}
impl From<Ch10CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlDstinc) -> u8 {
        Ch10CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch10CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlSize {
        Ch10CtrlSize::from_bits(val)
    }
}
impl From<Ch10CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlSize) -> u8 {
        Ch10CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch10CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlSrcinc {
        Ch10CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch10CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlSrcinc) -> u8 {
        Ch10CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch10CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch10CtrlStructtype {
        Ch10CtrlStructtype::from_bits(val)
    }
}
impl From<Ch10CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch10CtrlStructtype) -> u8 {
        Ch10CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch10ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch10ReqselSourcesel {
        Ch10ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch10ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch10ReqselSourcesel) -> u8 {
        Ch10ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch11CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch11CfgArbslots {
        Ch11CfgArbslots::from_bits(val)
    }
}
impl From<Ch11CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch11CfgArbslots) -> u8 {
        Ch11CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlBlocksize {
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
impl Ch11CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlBlocksize {
        Ch11CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch11CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlBlocksize) -> u8 {
        Ch11CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch11CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlDstinc {
        Ch11CtrlDstinc::from_bits(val)
    }
}
impl From<Ch11CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlDstinc) -> u8 {
        Ch11CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch11CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlSize {
        Ch11CtrlSize::from_bits(val)
    }
}
impl From<Ch11CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlSize) -> u8 {
        Ch11CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch11CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlSrcinc {
        Ch11CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch11CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlSrcinc) -> u8 {
        Ch11CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch11CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch11CtrlStructtype {
        Ch11CtrlStructtype::from_bits(val)
    }
}
impl From<Ch11CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch11CtrlStructtype) -> u8 {
        Ch11CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch11ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch11ReqselSourcesel {
        Ch11ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch11ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch11ReqselSourcesel) -> u8 {
        Ch11ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch12CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch12CfgArbslots {
        Ch12CfgArbslots::from_bits(val)
    }
}
impl From<Ch12CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch12CfgArbslots) -> u8 {
        Ch12CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlBlocksize {
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
impl Ch12CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlBlocksize {
        Ch12CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch12CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlBlocksize) -> u8 {
        Ch12CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch12CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlDstinc {
        Ch12CtrlDstinc::from_bits(val)
    }
}
impl From<Ch12CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlDstinc) -> u8 {
        Ch12CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch12CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlSize {
        Ch12CtrlSize::from_bits(val)
    }
}
impl From<Ch12CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlSize) -> u8 {
        Ch12CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch12CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlSrcinc {
        Ch12CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch12CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlSrcinc) -> u8 {
        Ch12CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch12CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch12CtrlStructtype {
        Ch12CtrlStructtype::from_bits(val)
    }
}
impl From<Ch12CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch12CtrlStructtype) -> u8 {
        Ch12CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch12ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch12ReqselSourcesel {
        Ch12ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch12ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch12ReqselSourcesel) -> u8 {
        Ch12ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch13CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch13CfgArbslots {
        Ch13CfgArbslots::from_bits(val)
    }
}
impl From<Ch13CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch13CfgArbslots) -> u8 {
        Ch13CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlBlocksize {
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
impl Ch13CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlBlocksize {
        Ch13CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch13CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlBlocksize) -> u8 {
        Ch13CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch13CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlDstinc {
        Ch13CtrlDstinc::from_bits(val)
    }
}
impl From<Ch13CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlDstinc) -> u8 {
        Ch13CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch13CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlSize {
        Ch13CtrlSize::from_bits(val)
    }
}
impl From<Ch13CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlSize) -> u8 {
        Ch13CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch13CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlSrcinc {
        Ch13CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch13CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlSrcinc) -> u8 {
        Ch13CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch13CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch13CtrlStructtype {
        Ch13CtrlStructtype::from_bits(val)
    }
}
impl From<Ch13CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch13CtrlStructtype) -> u8 {
        Ch13CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch13ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch13ReqselSourcesel {
        Ch13ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch13ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch13ReqselSourcesel) -> u8 {
        Ch13ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch14CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch14CfgArbslots {
        Ch14CfgArbslots::from_bits(val)
    }
}
impl From<Ch14CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch14CfgArbslots) -> u8 {
        Ch14CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlBlocksize {
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
impl Ch14CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlBlocksize {
        Ch14CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch14CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlBlocksize) -> u8 {
        Ch14CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch14CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlDstinc {
        Ch14CtrlDstinc::from_bits(val)
    }
}
impl From<Ch14CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlDstinc) -> u8 {
        Ch14CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch14CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlSize {
        Ch14CtrlSize::from_bits(val)
    }
}
impl From<Ch14CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlSize) -> u8 {
        Ch14CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch14CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlSrcinc {
        Ch14CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch14CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlSrcinc) -> u8 {
        Ch14CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch14CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch14CtrlStructtype {
        Ch14CtrlStructtype::from_bits(val)
    }
}
impl From<Ch14CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch14CtrlStructtype) -> u8 {
        Ch14CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch14ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch14ReqselSourcesel {
        Ch14ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch14ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch14ReqselSourcesel) -> u8 {
        Ch14ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch15CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch15CfgArbslots {
        Ch15CfgArbslots::from_bits(val)
    }
}
impl From<Ch15CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch15CfgArbslots) -> u8 {
        Ch15CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlBlocksize {
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
impl Ch15CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlBlocksize {
        Ch15CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch15CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlBlocksize) -> u8 {
        Ch15CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch15CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlDstinc {
        Ch15CtrlDstinc::from_bits(val)
    }
}
impl From<Ch15CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlDstinc) -> u8 {
        Ch15CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch15CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlSize {
        Ch15CtrlSize::from_bits(val)
    }
}
impl From<Ch15CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlSize) -> u8 {
        Ch15CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch15CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlSrcinc {
        Ch15CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch15CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlSrcinc) -> u8 {
        Ch15CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch15CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch15CtrlStructtype {
        Ch15CtrlStructtype::from_bits(val)
    }
}
impl From<Ch15CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch15CtrlStructtype) -> u8 {
        Ch15CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch15ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch15ReqselSourcesel {
        Ch15ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch15ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch15ReqselSourcesel) -> u8 {
        Ch15ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch16CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch16CfgArbslots {
        Ch16CfgArbslots::from_bits(val)
    }
}
impl From<Ch16CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch16CfgArbslots) -> u8 {
        Ch16CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlBlocksize {
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
impl Ch16CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlBlocksize {
        Ch16CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch16CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlBlocksize) -> u8 {
        Ch16CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch16CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlDstinc {
        Ch16CtrlDstinc::from_bits(val)
    }
}
impl From<Ch16CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlDstinc) -> u8 {
        Ch16CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch16CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlSize {
        Ch16CtrlSize::from_bits(val)
    }
}
impl From<Ch16CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlSize) -> u8 {
        Ch16CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch16CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlSrcinc {
        Ch16CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch16CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlSrcinc) -> u8 {
        Ch16CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch16CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch16CtrlStructtype {
        Ch16CtrlStructtype::from_bits(val)
    }
}
impl From<Ch16CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch16CtrlStructtype) -> u8 {
        Ch16CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch16ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch16ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch16ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch16ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch16ReqselSourcesel {
        Ch16ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch16ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch16ReqselSourcesel) -> u8 {
        Ch16ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch17CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch17CfgArbslots {
        Ch17CfgArbslots::from_bits(val)
    }
}
impl From<Ch17CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch17CfgArbslots) -> u8 {
        Ch17CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlBlocksize {
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
impl Ch17CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlBlocksize {
        Ch17CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch17CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlBlocksize) -> u8 {
        Ch17CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch17CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlDstinc {
        Ch17CtrlDstinc::from_bits(val)
    }
}
impl From<Ch17CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlDstinc) -> u8 {
        Ch17CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch17CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlSize {
        Ch17CtrlSize::from_bits(val)
    }
}
impl From<Ch17CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlSize) -> u8 {
        Ch17CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch17CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlSrcinc {
        Ch17CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch17CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlSrcinc) -> u8 {
        Ch17CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch17CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch17CtrlStructtype {
        Ch17CtrlStructtype::from_bits(val)
    }
}
impl From<Ch17CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch17CtrlStructtype) -> u8 {
        Ch17CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch17ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch17ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch17ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch17ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch17ReqselSourcesel {
        Ch17ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch17ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch17ReqselSourcesel) -> u8 {
        Ch17ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch18CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch18CfgArbslots {
        Ch18CfgArbslots::from_bits(val)
    }
}
impl From<Ch18CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch18CfgArbslots) -> u8 {
        Ch18CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlBlocksize {
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
impl Ch18CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlBlocksize {
        Ch18CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch18CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlBlocksize) -> u8 {
        Ch18CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch18CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlDstinc {
        Ch18CtrlDstinc::from_bits(val)
    }
}
impl From<Ch18CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlDstinc) -> u8 {
        Ch18CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch18CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlSize {
        Ch18CtrlSize::from_bits(val)
    }
}
impl From<Ch18CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlSize) -> u8 {
        Ch18CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch18CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlSrcinc {
        Ch18CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch18CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlSrcinc) -> u8 {
        Ch18CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch18CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch18CtrlStructtype {
        Ch18CtrlStructtype::from_bits(val)
    }
}
impl From<Ch18CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch18CtrlStructtype) -> u8 {
        Ch18CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch18ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch18ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch18ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch18ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch18ReqselSourcesel {
        Ch18ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch18ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch18ReqselSourcesel) -> u8 {
        Ch18ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch19CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch19CfgArbslots {
        Ch19CfgArbslots::from_bits(val)
    }
}
impl From<Ch19CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch19CfgArbslots) -> u8 {
        Ch19CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlBlocksize {
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
impl Ch19CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlBlocksize {
        Ch19CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch19CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlBlocksize) -> u8 {
        Ch19CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch19CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlDstinc {
        Ch19CtrlDstinc::from_bits(val)
    }
}
impl From<Ch19CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlDstinc) -> u8 {
        Ch19CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch19CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlSize {
        Ch19CtrlSize::from_bits(val)
    }
}
impl From<Ch19CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlSize) -> u8 {
        Ch19CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch19CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlSrcinc {
        Ch19CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch19CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlSrcinc) -> u8 {
        Ch19CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch19CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch19CtrlStructtype {
        Ch19CtrlStructtype::from_bits(val)
    }
}
impl From<Ch19CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch19CtrlStructtype) -> u8 {
        Ch19CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch19ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch19ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch19ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch19ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch19ReqselSourcesel {
        Ch19ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch19ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch19ReqselSourcesel) -> u8 {
        Ch19ReqselSourcesel::to_bits(val)
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
pub enum Ch20CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch20CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch20CfgArbslots {
        Ch20CfgArbslots::from_bits(val)
    }
}
impl From<Ch20CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch20CfgArbslots) -> u8 {
        Ch20CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlBlocksize {
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
impl Ch20CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlBlocksize {
        Ch20CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch20CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlBlocksize) -> u8 {
        Ch20CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch20CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlDstinc {
        Ch20CtrlDstinc::from_bits(val)
    }
}
impl From<Ch20CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlDstinc) -> u8 {
        Ch20CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch20CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlSize {
        Ch20CtrlSize::from_bits(val)
    }
}
impl From<Ch20CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlSize) -> u8 {
        Ch20CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch20CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlSrcinc {
        Ch20CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch20CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlSrcinc) -> u8 {
        Ch20CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch20CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch20CtrlStructtype {
        Ch20CtrlStructtype::from_bits(val)
    }
}
impl From<Ch20CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch20CtrlStructtype) -> u8 {
        Ch20CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch20ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch20ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch20ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch20ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch20ReqselSourcesel {
        Ch20ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch20ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch20ReqselSourcesel) -> u8 {
        Ch20ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch21CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch21CfgArbslots {
        Ch21CfgArbslots::from_bits(val)
    }
}
impl From<Ch21CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch21CfgArbslots) -> u8 {
        Ch21CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlBlocksize {
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
impl Ch21CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlBlocksize {
        Ch21CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch21CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlBlocksize) -> u8 {
        Ch21CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch21CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlDstinc {
        Ch21CtrlDstinc::from_bits(val)
    }
}
impl From<Ch21CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlDstinc) -> u8 {
        Ch21CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch21CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlSize {
        Ch21CtrlSize::from_bits(val)
    }
}
impl From<Ch21CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlSize) -> u8 {
        Ch21CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch21CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlSrcinc {
        Ch21CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch21CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlSrcinc) -> u8 {
        Ch21CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch21CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch21CtrlStructtype {
        Ch21CtrlStructtype::from_bits(val)
    }
}
impl From<Ch21CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch21CtrlStructtype) -> u8 {
        Ch21CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch21ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch21ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch21ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch21ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch21ReqselSourcesel {
        Ch21ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch21ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch21ReqselSourcesel) -> u8 {
        Ch21ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch22CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch22CfgArbslots {
        Ch22CfgArbslots::from_bits(val)
    }
}
impl From<Ch22CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch22CfgArbslots) -> u8 {
        Ch22CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlBlocksize {
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
impl Ch22CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlBlocksize {
        Ch22CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch22CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlBlocksize) -> u8 {
        Ch22CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch22CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlDstinc {
        Ch22CtrlDstinc::from_bits(val)
    }
}
impl From<Ch22CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlDstinc) -> u8 {
        Ch22CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch22CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlSize {
        Ch22CtrlSize::from_bits(val)
    }
}
impl From<Ch22CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlSize) -> u8 {
        Ch22CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch22CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlSrcinc {
        Ch22CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch22CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlSrcinc) -> u8 {
        Ch22CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch22CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch22CtrlStructtype {
        Ch22CtrlStructtype::from_bits(val)
    }
}
impl From<Ch22CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch22CtrlStructtype) -> u8 {
        Ch22CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch22ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch22ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch22ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch22ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch22ReqselSourcesel {
        Ch22ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch22ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch22ReqselSourcesel) -> u8 {
        Ch22ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch23CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch23CfgArbslots {
        Ch23CfgArbslots::from_bits(val)
    }
}
impl From<Ch23CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch23CfgArbslots) -> u8 {
        Ch23CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlBlocksize {
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
impl Ch23CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlBlocksize {
        Ch23CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch23CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlBlocksize) -> u8 {
        Ch23CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch23CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlDstinc {
        Ch23CtrlDstinc::from_bits(val)
    }
}
impl From<Ch23CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlDstinc) -> u8 {
        Ch23CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch23CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlSize {
        Ch23CtrlSize::from_bits(val)
    }
}
impl From<Ch23CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlSize) -> u8 {
        Ch23CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch23CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlSrcinc {
        Ch23CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch23CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlSrcinc) -> u8 {
        Ch23CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch23CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch23CtrlStructtype {
        Ch23CtrlStructtype::from_bits(val)
    }
}
impl From<Ch23CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch23CtrlStructtype) -> u8 {
        Ch23CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch23ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch23ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch23ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch23ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch23ReqselSourcesel {
        Ch23ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch23ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch23ReqselSourcesel) -> u8 {
        Ch23ReqselSourcesel::to_bits(val)
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch8CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch8CfgArbslots {
        Ch8CfgArbslots::from_bits(val)
    }
}
impl From<Ch8CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch8CfgArbslots) -> u8 {
        Ch8CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlBlocksize {
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
impl Ch8CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlBlocksize {
        Ch8CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch8CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlBlocksize) -> u8 {
        Ch8CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch8CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlDstinc {
        Ch8CtrlDstinc::from_bits(val)
    }
}
impl From<Ch8CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlDstinc) -> u8 {
        Ch8CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch8CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlSize {
        Ch8CtrlSize::from_bits(val)
    }
}
impl From<Ch8CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlSize) -> u8 {
        Ch8CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch8CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlSrcinc {
        Ch8CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch8CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlSrcinc) -> u8 {
        Ch8CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch8CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch8CtrlStructtype {
        Ch8CtrlStructtype::from_bits(val)
    }
}
impl From<Ch8CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch8CtrlStructtype) -> u8 {
        Ch8CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch8ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch8ReqselSourcesel {
        Ch8ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch8ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch8ReqselSourcesel) -> u8 {
        Ch8ReqselSourcesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CfgArbslots {
    #[doc = "One arbitration slot selected."]
    One = 0x0,
    #[doc = "Two arbitration slots selected."]
    Two = 0x01,
    #[doc = "Four arbitration slots selected."]
    Four = 0x02,
    #[doc = "Eight arbitration slots selected."]
    Eight = 0x03,
}
impl Ch9CfgArbslots {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CfgArbslots {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CfgArbslots {
    #[inline(always)]
    fn from(val: u8) -> Ch9CfgArbslots {
        Ch9CfgArbslots::from_bits(val)
    }
}
impl From<Ch9CfgArbslots> for u8 {
    #[inline(always)]
    fn from(val: Ch9CfgArbslots) -> u8 {
        Ch9CfgArbslots::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlBlocksize {
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
impl Ch9CtrlBlocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlBlocksize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlBlocksize {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlBlocksize {
        Ch9CtrlBlocksize::from_bits(val)
    }
}
impl From<Ch9CtrlBlocksize> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlBlocksize) -> u8 {
        Ch9CtrlBlocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlDstinc {
    #[doc = "Increment destination address by one unit data size after each write."]
    One = 0x0,
    #[doc = "Increment destination address by two unit data sizes after each write."]
    Two = 0x01,
    #[doc = "Increment destination address by four unit data sizes after each write."]
    Four = 0x02,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 0x03,
}
impl Ch9CtrlDstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlDstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlDstinc {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlDstinc {
        Ch9CtrlDstinc::from_bits(val)
    }
}
impl From<Ch9CtrlDstinc> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlDstinc) -> u8 {
        Ch9CtrlDstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlSize {
    #[doc = "Each unit transfer is a byte."]
    Byte = 0x0,
    #[doc = "Each unit transfer is a half-word."]
    Halfword = 0x01,
    #[doc = "Each unit transfer is a word."]
    Word = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch9CtrlSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlSize {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlSize {
        Ch9CtrlSize::from_bits(val)
    }
}
impl From<Ch9CtrlSize> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlSize) -> u8 {
        Ch9CtrlSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlSrcinc {
    #[doc = "Increment source address by one unit data size after each read."]
    One = 0x0,
    #[doc = "Increment source address by two unit data sizes after each read."]
    Two = 0x01,
    #[doc = "Increment source address by four unit data sizes after each read."]
    Four = 0x02,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 0x03,
}
impl Ch9CtrlSrcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlSrcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlSrcinc {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlSrcinc {
        Ch9CtrlSrcinc::from_bits(val)
    }
}
impl From<Ch9CtrlSrcinc> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlSrcinc) -> u8 {
        Ch9CtrlSrcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9CtrlStructtype {
    #[doc = "DMA transfer structure type selected."]
    Transfer = 0x0,
    #[doc = "Synchronization structure type selected."]
    Synchronize = 0x01,
    #[doc = "Write immediate value structure type selected."]
    Write = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch9CtrlStructtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9CtrlStructtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9CtrlStructtype {
    #[inline(always)]
    fn from(val: u8) -> Ch9CtrlStructtype {
        Ch9CtrlStructtype::from_bits(val)
    }
}
impl From<Ch9CtrlStructtype> for u8 {
    #[inline(always)]
    fn from(val: Ch9CtrlStructtype) -> u8 {
        Ch9CtrlStructtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9ReqselSourcesel {
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
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x09,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x0c,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x0d,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x0e,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x0f,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x10,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x11,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x12,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x13,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x14,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x15,
    #[doc = "I2C 0."]
    I2c0 = 0x16,
    #[doc = "I2C 1."]
    I2c1 = 0x17,
    #[doc = "I2C 2."]
    I2c2 = 0x18,
    #[doc = "Timer 0."]
    Timer0 = 0x19,
    #[doc = "Timer 1."]
    Timer1 = 0x1a,
    #[doc = "Timer 2."]
    Timer2 = 0x1b,
    #[doc = "Timer 3."]
    Timer3 = 0x1c,
    #[doc = "Timer 4."]
    Timer4 = 0x1d,
    #[doc = "Timer 5."]
    Timer5 = 0x1e,
    #[doc = "Timer 6."]
    Timer6 = 0x1f,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x20,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x21,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x22,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x23,
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
    Crypto0 = 0x31,
    #[doc = "External Bus Interface."]
    Ebi = 0x32,
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
    #[doc = "Capacitive touch sense module."]
    Csen = 0x3d,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Ch9ReqselSourcesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9ReqselSourcesel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9ReqselSourcesel {
    #[inline(always)]
    fn from(val: u8) -> Ch9ReqselSourcesel {
        Ch9ReqselSourcesel::from_bits(val)
    }
}
impl From<Ch9ReqselSourcesel> for u8 {
    #[inline(always)]
    fn from(val: Ch9ReqselSourcesel) -> u8 {
        Ch9ReqselSourcesel::to_bits(val)
    }
}
