#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Periphid {
    #[doc = "Analog Comparator 0."]
    Acmp0 = 0x0,
    #[doc = "Analog Comparator 1."]
    Acmp1 = 0x01,
    #[doc = "Analog Comparator 1."]
    Acmp2 = 0x02,
    #[doc = "Analog Comparator 3."]
    Acmp3 = 0x03,
    #[doc = "Analog to Digital Converter 0."]
    Adc0 = 0x04,
    #[doc = "Analog to Digital Converter 0."]
    Adc1 = 0x05,
    #[doc = "CAN 0."]
    Can0 = 0x06,
    #[doc = "CAN 1."]
    Can1 = 0x07,
    #[doc = "Clock Management Unit."]
    Cmu = 0x08,
    #[doc = "CRYOTIMER."]
    Cryotimer = 0x09,
    #[doc = "Advanced Encryption Standard Accelerator."]
    Crypto0 = 0x0a,
    #[doc = "Capacitive touch sense module."]
    Csen = 0x0b,
    #[doc = "Digital to Analog Converter 0."]
    Vdac0 = 0x0c,
    #[doc = "Peripheral Reflex System."]
    Prs = 0x0d,
    #[doc = "External Bus Interface."]
    Ebi = 0x0e,
    #[doc = "Energy Management Unit."]
    Emu = 0x0f,
    #[doc = "Ethernet Controller."]
    Eth = 0x10,
    #[doc = "FPU Exception Handler."]
    Fpueh = 0x11,
    #[doc = "General Purpose CRC."]
    Gpcrc = 0x12,
    #[doc = "General purpose Input/Output."]
    Gpio = 0x13,
    #[doc = "I2C 0."]
    I2c0 = 0x14,
    #[doc = "I2C 1."]
    I2c1 = 0x15,
    #[doc = "I2C 2."]
    I2c2 = 0x16,
    #[doc = "Current Digital to Analog Converter 0."]
    Idac0 = 0x17,
    #[doc = "Memory System Controller."]
    Msc = 0x18,
    #[doc = "Liquid Crystal Display Controller."]
    Lcd = 0x19,
    #[doc = "Linked Direct Memory Access Controller."]
    Ldma = 0x1a,
    #[doc = "Low Energy Sensor Interface."]
    Lesense = 0x1b,
    #[doc = "Low Energy Timer 0."]
    Letimer0 = 0x1c,
    #[doc = "Low Energy Timer 1."]
    Letimer1 = 0x1d,
    #[doc = "Low Energy UART 0."]
    Leuart0 = 0x1e,
    #[doc = "Low Energy UART 1."]
    Leuart1 = 0x1f,
    #[doc = "Pulse Counter 0."]
    Pcnt0 = 0x20,
    #[doc = "Pulse Counter 1."]
    Pcnt1 = 0x21,
    #[doc = "Pulse Counter 2."]
    Pcnt2 = 0x22,
    #[doc = "Quad-SPI."]
    Qspi0 = 0x23,
    #[doc = "Reset Management Unit."]
    Rmu = 0x24,
    #[doc = "Real-Time Counter."]
    Rtc = 0x25,
    #[doc = "Real-Time Counter and Calendar."]
    Rtcc = 0x26,
    #[doc = "SDIO Controller."]
    Sdio = 0x27,
    #[doc = "Security Management Unit."]
    Smu = 0x28,
    #[doc = "Timer 0."]
    Timer0 = 0x29,
    #[doc = "Timer 1."]
    Timer1 = 0x2a,
    #[doc = "Timer 2."]
    Timer2 = 0x2b,
    #[doc = "Timer 3."]
    Timer3 = 0x2c,
    #[doc = "Timer 4."]
    Timer4 = 0x2d,
    #[doc = "Timer 5."]
    Timer5 = 0x2e,
    #[doc = "Timer 6."]
    Timer6 = 0x2f,
    #[doc = "True Random Number Generator 0."]
    Trng0 = 0x30,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0."]
    Uart0 = 0x31,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1."]
    Uart1 = 0x32,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0."]
    Usart0 = 0x33,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1."]
    Usart1 = 0x34,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2."]
    Usart2 = 0x35,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3."]
    Usart3 = 0x36,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4."]
    Usart4 = 0x37,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5."]
    Usart5 = 0x38,
    #[doc = "Universal Serial Bus Interface."]
    Usb = 0x39,
    #[doc = "Watchdog."]
    Wdog0 = 0x3a,
    #[doc = "Watchdog."]
    Wdog1 = 0x3b,
    #[doc = "Wide Timer 0."]
    Wtimer0 = 0x3c,
    #[doc = "Wide Timer 0."]
    Wtimer1 = 0x3d,
    #[doc = "Wide Timer 2."]
    Wtimer2 = 0x3e,
    #[doc = "Wide Timer 3."]
    Wtimer3 = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
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
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
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
impl Periphid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Periphid {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Periphid {
    #[inline(always)]
    fn from(val: u8) -> Periphid {
        Periphid::from_bits(val)
    }
}
impl From<Periphid> for u8 {
    #[inline(always)]
    fn from(val: Periphid) -> u8 {
        Periphid::to_bits(val)
    }
}
