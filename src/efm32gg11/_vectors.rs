unsafe extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn SMU();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn ACMP0();
    fn ADC0();
    fn IDAC0();
    fn I2C0();
    fn I2C1();
    fn GPIO_ODD();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn USART1_RX();
    fn USART1_TX();
    fn USART2_RX();
    fn USART2_TX();
    fn UART0_RX();
    fn UART0_TX();
    fn UART1_RX();
    fn UART1_TX();
    fn LEUART0();
    fn LEUART1();
    fn LETIMER0();
    fn PCNT0();
    fn PCNT1();
    fn PCNT2();
    fn RTCC();
    fn CMU();
    fn MSC();
    fn CRYPTO0();
    fn CRYOTIMER();
    fn FPUEH();
    fn USART3_RX();
    fn USART3_TX();
    fn USART4_RX();
    fn USART4_TX();
    fn WTIMER0();
    fn WTIMER1();
    fn WTIMER2();
    fn WTIMER3();
    fn I2C2();
    fn VDAC0();
    fn TIMER4();
    fn TIMER5();
    fn TIMER6();
    fn USART5_RX();
    fn USART5_TX();
    fn CSEN();
    fn LESENSE();
    fn EBI();
    fn ACMP2();
    fn ADC1();
    fn LCD();
    fn SDIO();
    fn ETH();
    fn CAN0();
    fn CAN1();
    fn USB();
    fn RTC();
    fn WDOG1();
    fn LETIMER1();
    fn TRNG0();
    fn QSPI0();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 68] = [
    Vector { _handler: EMU },
    Vector { _handler: WDOG0 },
    Vector { _handler: LDMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: SMU },
    Vector { _handler: TIMER0 },
    Vector {
        _handler: USART0_RX,
    },
    Vector {
        _handler: USART0_TX,
    },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: IDAC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector {
        _handler: USART2_RX,
    },
    Vector {
        _handler: USART2_TX,
    },
    Vector { _handler: UART0_RX },
    Vector { _handler: UART0_TX },
    Vector { _handler: UART1_RX },
    Vector { _handler: UART1_TX },
    Vector { _handler: LEUART0 },
    Vector { _handler: LEUART1 },
    Vector { _handler: LETIMER0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: PCNT1 },
    Vector { _handler: PCNT2 },
    Vector { _handler: RTCC },
    Vector { _handler: CMU },
    Vector { _handler: MSC },
    Vector { _handler: CRYPTO0 },
    Vector {
        _handler: CRYOTIMER,
    },
    Vector { _handler: FPUEH },
    Vector {
        _handler: USART3_RX,
    },
    Vector {
        _handler: USART3_TX,
    },
    Vector {
        _handler: USART4_RX,
    },
    Vector {
        _handler: USART4_TX,
    },
    Vector { _handler: WTIMER0 },
    Vector { _handler: WTIMER1 },
    Vector { _handler: WTIMER2 },
    Vector { _handler: WTIMER3 },
    Vector { _handler: I2C2 },
    Vector { _handler: VDAC0 },
    Vector { _handler: TIMER4 },
    Vector { _handler: TIMER5 },
    Vector { _handler: TIMER6 },
    Vector {
        _handler: USART5_RX,
    },
    Vector {
        _handler: USART5_TX,
    },
    Vector { _handler: CSEN },
    Vector { _handler: LESENSE },
    Vector { _handler: EBI },
    Vector { _handler: ACMP2 },
    Vector { _handler: ADC1 },
    Vector { _handler: LCD },
    Vector { _handler: SDIO },
    Vector { _handler: ETH },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: USB },
    Vector { _handler: RTC },
    Vector { _handler: WDOG1 },
    Vector { _handler: LETIMER1 },
    Vector { _handler: TRNG0 },
    Vector { _handler: QSPI0 },
];
