unsafe extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn ACMP0();
    fn ADC0();
    fn IDAC0();
    fn I2C0();
    fn GPIO_ODD();
    fn TIMER1();
    fn USART1_RX();
    fn USART1_TX();
    fn LEUART0();
    fn PCNT0();
    fn CMU();
    fn MSC();
    fn CRYPTO();
    fn LETIMER0();
    fn RTCC();
    fn CRYOTIMER();
    fn FPUEH();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 34] = [
    Vector { _handler: EMU },
    Vector { _reserved: 0 },
    Vector { _handler: WDOG0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LDMA },
    Vector {
        _handler: GPIO_EVEN,
    },
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
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector { _handler: LEUART0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: CMU },
    Vector { _handler: MSC },
    Vector { _handler: CRYPTO },
    Vector { _handler: LETIMER0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTCC },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYOTIMER,
    },
    Vector { _reserved: 0 },
    Vector { _handler: FPUEH },
];
