#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU = 0,
    #[doc = "2 - WDOG0"]
    WDOG0 = 2,
    #[doc = "8 - LDMA"]
    LDMA = 8,
    #[doc = "9 - GPIO_EVEN"]
    GPIO_EVEN = 9,
    #[doc = "10 - TIMER0"]
    TIMER0 = 10,
    #[doc = "11 - USART0_RX"]
    USART0_RX = 11,
    #[doc = "12 - USART0_TX"]
    USART0_TX = 12,
    #[doc = "13 - ACMP0"]
    ACMP0 = 13,
    #[doc = "14 - ADC0"]
    ADC0 = 14,
    #[doc = "15 - IDAC0"]
    IDAC0 = 15,
    #[doc = "16 - I2C0"]
    I2C0 = 16,
    #[doc = "17 - GPIO_ODD"]
    GPIO_ODD = 17,
    #[doc = "18 - TIMER1"]
    TIMER1 = 18,
    #[doc = "19 - USART1_RX"]
    USART1_RX = 19,
    #[doc = "20 - USART1_TX"]
    USART1_TX = 20,
    #[doc = "21 - LEUART0"]
    LEUART0 = 21,
    #[doc = "22 - PCNT0"]
    PCNT0 = 22,
    #[doc = "23 - CMU"]
    CMU = 23,
    #[doc = "24 - MSC"]
    MSC = 24,
    #[doc = "25 - CRYPTO"]
    CRYPTO = 25,
    #[doc = "26 - LETIMER0"]
    LETIMER0 = 26,
    #[doc = "29 - RTCC"]
    RTCC = 29,
    #[doc = "31 - CRYOTIMER"]
    CRYOTIMER = 31,
    #[doc = "33 - FPUEH"]
    FPUEH = 33,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "ACMP0"]
pub const ACMP0: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4000_0000usize as _) };
#[doc = "ACMP1"]
pub const ACMP1: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4000_0400usize as _) };
#[doc = "ADC0"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_2000usize as _) };
#[doc = "IDAC0"]
pub const IDAC: idac::Idac = unsafe { idac::Idac::from_ptr(0x4000_6000usize as _) };
#[doc = "GPIO"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4000_a000usize as _) };
#[doc = "I2C0"]
pub const I2C: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_c000usize as _) };
#[doc = "USART0"]
pub const USART0: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0000usize as _) };
#[doc = "USART1"]
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0400usize as _) };
#[doc = "TIMER0"]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8000usize as _) };
#[doc = "TIMER1"]
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8400usize as _) };
#[doc = "GPCRC"]
pub const GPCRC: gpcrc::Gpcrc = unsafe { gpcrc::Gpcrc::from_ptr(0x4001_c000usize as _) };
#[doc = "CRYOTIMER"]
pub const CRYOTIMER: cryotimer::Cryotimer =
    unsafe { cryotimer::Cryotimer::from_ptr(0x4001_e000usize as _) };
#[doc = "RTCC"]
pub const RTCC: rtcc::Rtcc = unsafe { rtcc::Rtcc::from_ptr(0x4004_2000usize as _) };
#[doc = "LETIMER0"]
pub const LETIMER: letimer::Letimer = unsafe { letimer::Letimer::from_ptr(0x4004_6000usize as _) };
#[doc = "LEUART0"]
pub const LEUART: leuart::Leuart = unsafe { leuart::Leuart::from_ptr(0x4004_a000usize as _) };
#[doc = "PCNT0"]
pub const PCNT: pcnt::Pcnt = unsafe { pcnt::Pcnt::from_ptr(0x4004_e000usize as _) };
#[doc = "WDOG0"]
pub const WDOG: wdog::Wdog = unsafe { wdog::Wdog::from_ptr(0x4005_2000usize as _) };
#[doc = "MSC"]
pub const MSC: msc::Msc = unsafe { msc::Msc::from_ptr(0x400e_0000usize as _) };
#[doc = "FPUEH"]
pub const FPUEH: fpueh::Fpueh = unsafe { fpueh::Fpueh::from_ptr(0x400e_1000usize as _) };
#[doc = "LDMA"]
pub const LDMA: ldma::Ldma = unsafe { ldma::Ldma::from_ptr(0x400e_2000usize as _) };
#[doc = "EMU"]
pub const EMU: emu::Emu = unsafe { emu::Emu::from_ptr(0x400e_3000usize as _) };
#[doc = "CMU"]
pub const CMU: cmu::Cmu = unsafe { cmu::Cmu::from_ptr(0x400e_4000usize as _) };
#[doc = "RMU"]
pub const RMU: rmu::Rmu = unsafe { rmu::Rmu::from_ptr(0x400e_5000usize as _) };
#[doc = "PRS"]
pub const PRS: prs::Prs = unsafe { prs::Prs::from_ptr(0x400e_6000usize as _) };
#[doc = "CRYPTO"]
pub const CRYPTO: crypto::Crypto = unsafe { crypto::Crypto::from_ptr(0x400f_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod acmp;
pub mod adc;
pub mod cmu;
pub mod cryotimer;
pub mod crypto;
pub mod emu;
pub mod fpueh;
pub mod gpcrc;
pub mod gpio;
pub mod i2c;
pub mod idac;
pub mod ldma;
pub mod letimer;
pub mod leuart;
pub mod msc;
pub mod pcnt;
pub mod prs;
pub mod rmu;
pub mod rtcc;
pub mod timer;
pub mod usart;
pub mod wdog;
