#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU = 0,
    #[doc = "1 - WDOG0"]
    WDOG0 = 1,
    #[doc = "2 - LDMA"]
    LDMA = 2,
    #[doc = "3 - GPIO_EVEN"]
    GPIO_EVEN = 3,
    #[doc = "4 - SMU"]
    SMU = 4,
    #[doc = "5 - TIMER0"]
    TIMER0 = 5,
    #[doc = "6 - USART0_RX"]
    USART0_RX = 6,
    #[doc = "7 - USART0_TX"]
    USART0_TX = 7,
    #[doc = "8 - ACMP0"]
    ACMP0 = 8,
    #[doc = "9 - ADC0"]
    ADC0 = 9,
    #[doc = "10 - IDAC0"]
    IDAC0 = 10,
    #[doc = "11 - I2C0"]
    I2C0 = 11,
    #[doc = "12 - I2C1"]
    I2C1 = 12,
    #[doc = "13 - GPIO_ODD"]
    GPIO_ODD = 13,
    #[doc = "14 - TIMER1"]
    TIMER1 = 14,
    #[doc = "15 - TIMER2"]
    TIMER2 = 15,
    #[doc = "16 - TIMER3"]
    TIMER3 = 16,
    #[doc = "17 - USART1_RX"]
    USART1_RX = 17,
    #[doc = "18 - USART1_TX"]
    USART1_TX = 18,
    #[doc = "19 - USART2_RX"]
    USART2_RX = 19,
    #[doc = "20 - USART2_TX"]
    USART2_TX = 20,
    #[doc = "21 - UART0_RX"]
    UART0_RX = 21,
    #[doc = "22 - UART0_TX"]
    UART0_TX = 22,
    #[doc = "23 - UART1_RX"]
    UART1_RX = 23,
    #[doc = "24 - UART1_TX"]
    UART1_TX = 24,
    #[doc = "25 - LEUART0"]
    LEUART0 = 25,
    #[doc = "26 - LEUART1"]
    LEUART1 = 26,
    #[doc = "27 - LETIMER0"]
    LETIMER0 = 27,
    #[doc = "28 - PCNT0"]
    PCNT0 = 28,
    #[doc = "29 - PCNT1"]
    PCNT1 = 29,
    #[doc = "30 - PCNT2"]
    PCNT2 = 30,
    #[doc = "31 - RTCC"]
    RTCC = 31,
    #[doc = "32 - CMU"]
    CMU = 32,
    #[doc = "33 - MSC"]
    MSC = 33,
    #[doc = "34 - CRYPTO0"]
    CRYPTO0 = 34,
    #[doc = "35 - CRYOTIMER"]
    CRYOTIMER = 35,
    #[doc = "36 - FPUEH"]
    FPUEH = 36,
    #[doc = "37 - USART3_RX"]
    USART3_RX = 37,
    #[doc = "38 - USART3_TX"]
    USART3_TX = 38,
    #[doc = "39 - USART4_RX"]
    USART4_RX = 39,
    #[doc = "40 - USART4_TX"]
    USART4_TX = 40,
    #[doc = "41 - WTIMER0"]
    WTIMER0 = 41,
    #[doc = "42 - WTIMER1"]
    WTIMER1 = 42,
    #[doc = "43 - WTIMER2"]
    WTIMER2 = 43,
    #[doc = "44 - WTIMER3"]
    WTIMER3 = 44,
    #[doc = "45 - I2C2"]
    I2C2 = 45,
    #[doc = "46 - VDAC0"]
    VDAC0 = 46,
    #[doc = "47 - TIMER4"]
    TIMER4 = 47,
    #[doc = "48 - TIMER5"]
    TIMER5 = 48,
    #[doc = "49 - TIMER6"]
    TIMER6 = 49,
    #[doc = "50 - USART5_RX"]
    USART5_RX = 50,
    #[doc = "51 - USART5_TX"]
    USART5_TX = 51,
    #[doc = "52 - CSEN"]
    CSEN = 52,
    #[doc = "53 - LESENSE"]
    LESENSE = 53,
    #[doc = "54 - EBI"]
    EBI = 54,
    #[doc = "55 - ACMP2"]
    ACMP2 = 55,
    #[doc = "56 - ADC1"]
    ADC1 = 56,
    #[doc = "57 - LCD"]
    LCD = 57,
    #[doc = "58 - SDIO"]
    SDIO = 58,
    #[doc = "59 - ETH"]
    ETH = 59,
    #[doc = "60 - CAN0"]
    CAN0 = 60,
    #[doc = "61 - CAN1"]
    CAN1 = 61,
    #[doc = "62 - USB"]
    USB = 62,
    #[doc = "63 - RTC"]
    RTC = 63,
    #[doc = "64 - WDOG1"]
    WDOG1 = 64,
    #[doc = "65 - LETIMER1"]
    LETIMER1 = 65,
    #[doc = "66 - TRNG0"]
    TRNG0 = 66,
    #[doc = "67 - QSPI0"]
    QSPI0 = 67,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "MSC"]
pub const MSC: msc::Msc = unsafe { msc::Msc::from_ptr(0x4000_0000usize as _) };
#[doc = "FPUEH"]
pub const FPUEH: fpueh::Fpueh = unsafe { fpueh::Fpueh::from_ptr(0x4000_1000usize as _) };
#[doc = "LDMA"]
pub const LDMA: ldma::Ldma = unsafe { ldma::Ldma::from_ptr(0x4000_2000usize as _) };
#[doc = "CAN0"]
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x4000_4000usize as _) };
#[doc = "CAN1"]
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_4400usize as _) };
#[doc = "EBI"]
pub const EBI: ebi::Ebi = unsafe { ebi::Ebi::from_ptr(0x4000_b000usize as _) };
#[doc = "USART0"]
pub const USART0: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0000usize as _) };
#[doc = "USART1"]
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0400usize as _) };
#[doc = "USART2"]
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0800usize as _) };
#[doc = "USART3"]
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_0c00usize as _) };
#[doc = "USART4"]
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1000usize as _) };
#[doc = "USART5"]
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1400usize as _) };
#[doc = "UART0"]
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4001_4000usize as _) };
#[doc = "UART1"]
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x4001_4400usize as _) };
#[doc = "TIMER0"]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8000usize as _) };
#[doc = "TIMER1"]
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8400usize as _) };
#[doc = "TIMER2"]
pub const TIMER2: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8800usize as _) };
#[doc = "TIMER3"]
pub const TIMER3: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_8c00usize as _) };
#[doc = "TIMER4"]
pub const TIMER4: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_9000usize as _) };
#[doc = "TIMER5"]
pub const TIMER5: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_9400usize as _) };
#[doc = "TIMER6"]
pub const TIMER6: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_9800usize as _) };
#[doc = "WTIMER0"]
pub const WTIMER0: wtimer::Wtimer = unsafe { wtimer::Wtimer::from_ptr(0x4001_a000usize as _) };
#[doc = "WTIMER1"]
pub const WTIMER1: wtimer::Wtimer = unsafe { wtimer::Wtimer::from_ptr(0x4001_a400usize as _) };
#[doc = "WTIMER2"]
pub const WTIMER2: wtimer::Wtimer = unsafe { wtimer::Wtimer::from_ptr(0x4001_a800usize as _) };
#[doc = "WTIMER3"]
pub const WTIMER3: wtimer::Wtimer = unsafe { wtimer::Wtimer::from_ptr(0x4001_ac00usize as _) };
#[doc = "GPCRC"]
pub const GPCRC: gpcrc::Gpcrc = unsafe { gpcrc::Gpcrc::from_ptr(0x4001_c000usize as _) };
#[doc = "QSPI0"]
pub const QSPI: qspi::Qspi = unsafe { qspi::Qspi::from_ptr(0x4001_c400usize as _) };
#[doc = "TRNG0"]
pub const TRNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x4001_d000usize as _) };
#[doc = "SMU"]
pub const SMU: smu::Smu = unsafe { smu::Smu::from_ptr(0x4002_0000usize as _) };
#[doc = "USB"]
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4002_2000usize as _) };
#[doc = "ETH"]
pub const ETH: eth::Eth = unsafe { eth::Eth::from_ptr(0x4002_4000usize as _) };
#[doc = "WDOG0"]
pub const WDOG0: wdog::Wdog = unsafe { wdog::Wdog::from_ptr(0x4005_2000usize as _) };
#[doc = "WDOG1"]
pub const WDOG1: wdog::Wdog = unsafe { wdog::Wdog::from_ptr(0x4005_2400usize as _) };
#[doc = "LCD"]
pub const LCD: lcd::Lcd = unsafe { lcd::Lcd::from_ptr(0x4005_4000usize as _) };
#[doc = "LESENSE"]
pub const LESENSE: lesense::Lesense = unsafe { lesense::Lesense::from_ptr(0x4005_5000usize as _) };
#[doc = "RTC"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4006_0000usize as _) };
#[doc = "RTCC"]
pub const RTCC: rtcc::Rtcc = unsafe { rtcc::Rtcc::from_ptr(0x4006_2000usize as _) };
#[doc = "LETIMER0"]
pub const LETIMER0: letimer::Letimer = unsafe { letimer::Letimer::from_ptr(0x4006_6000usize as _) };
#[doc = "LETIMER1"]
pub const LETIMER1: letimer::Letimer = unsafe { letimer::Letimer::from_ptr(0x4006_6400usize as _) };
#[doc = "LEUART0"]
pub const LEUART0: leuart::Leuart = unsafe { leuart::Leuart::from_ptr(0x4006_a000usize as _) };
#[doc = "LEUART1"]
pub const LEUART1: leuart::Leuart = unsafe { leuart::Leuart::from_ptr(0x4006_a400usize as _) };
#[doc = "PCNT0"]
pub const PCNT0: pcnt::Pcnt = unsafe { pcnt::Pcnt::from_ptr(0x4006_e000usize as _) };
#[doc = "PCNT1"]
pub const PCNT1: pcnt::Pcnt = unsafe { pcnt::Pcnt::from_ptr(0x4006_e400usize as _) };
#[doc = "PCNT2"]
pub const PCNT2: pcnt::Pcnt = unsafe { pcnt::Pcnt::from_ptr(0x4006_e800usize as _) };
#[doc = "ACMP0"]
pub const ACMP0: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4008_0000usize as _) };
#[doc = "ACMP1"]
pub const ACMP1: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4008_0400usize as _) };
#[doc = "ACMP2"]
pub const ACMP2: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4008_0800usize as _) };
#[doc = "ACMP3"]
pub const ACMP3: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4008_0c00usize as _) };
#[doc = "ADC0"]
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x4008_2000usize as _) };
#[doc = "ADC1"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4008_2400usize as _) };
#[doc = "IDAC0"]
pub const IDAC: idac::Idac = unsafe { idac::Idac::from_ptr(0x4008_4000usize as _) };
#[doc = "VDAC0"]
pub const VDAC: vdac::Vdac = unsafe { vdac::Vdac::from_ptr(0x4008_6000usize as _) };
#[doc = "GPIO"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4008_8000usize as _) };
#[doc = "I2C0"]
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_9000usize as _) };
#[doc = "I2C1"]
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_9400usize as _) };
#[doc = "I2C2"]
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_9800usize as _) };
#[doc = "CSEN"]
pub const CSEN: csen::Csen = unsafe { csen::Csen::from_ptr(0x4008_e000usize as _) };
#[doc = "CRYOTIMER"]
pub const CRYOTIMER: cryotimer::Cryotimer =
    unsafe { cryotimer::Cryotimer::from_ptr(0x4008_f000usize as _) };
#[doc = "EMU"]
pub const EMU: emu::Emu = unsafe { emu::Emu::from_ptr(0x400e_3000usize as _) };
#[doc = "CMU"]
pub const CMU: cmu::Cmu = unsafe { cmu::Cmu::from_ptr(0x400e_4000usize as _) };
#[doc = "RMU"]
pub const RMU: rmu::Rmu = unsafe { rmu::Rmu::from_ptr(0x400e_5000usize as _) };
#[doc = "PRS"]
pub const PRS: prs::Prs = unsafe { prs::Prs::from_ptr(0x400e_6000usize as _) };
#[doc = "CRYPTO0"]
pub const CRYPTO: crypto::Crypto = unsafe { crypto::Crypto::from_ptr(0x400f_0000usize as _) };
#[doc = "SDIO"]
pub const SDIO: sdio::Sdio = unsafe { sdio::Sdio::from_ptr(0x400f_1000usize as _) };
#[doc = "ETM"]
pub const ETM: etm::Etm = unsafe { etm::Etm::from_ptr(0xe004_1000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod acmp;
pub mod adc;
pub mod can;
pub mod cmu;
pub mod cryotimer;
pub mod crypto;
pub mod csen;
pub mod ebi;
pub mod emu;
pub mod eth;
pub mod etm;
pub mod fpueh;
pub mod gpcrc;
pub mod gpio;
pub mod i2c;
pub mod idac;
pub mod lcd;
pub mod ldma;
pub mod lesense;
pub mod letimer;
pub mod leuart;
pub mod msc;
pub mod pcnt;
pub mod prs;
pub mod qspi;
pub mod rmu;
pub mod rtc;
pub mod rtcc;
pub mod sdio;
pub mod smu;
pub mod timer;
pub mod trng;
pub mod uart;
pub mod usart;
pub mod usb;
pub mod vdac;
pub mod wdog;
pub mod wtimer;
