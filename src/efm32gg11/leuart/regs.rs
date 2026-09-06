#[doc = "Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "Fractional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x3fff;
        val as u16
    }
    #[doc = "Fractional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 3usize)) | (((val as u32) & 0x3fff) << 3usize);
    }
}
impl Default for Clkdiv {
    #[inline(always)]
    fn default() -> Clkdiv {
        Clkdiv(0)
    }
}
impl core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkdiv").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkdiv {{ div: {=u16:?} }}", self.div())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Receiver Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable."]
    #[inline(always)]
    pub const fn set_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receiver Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Disable."]
    #[inline(always)]
    pub const fn set_rxdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable."]
    #[inline(always)]
    pub const fn set_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmitter Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn txdis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Disable."]
    #[inline(always)]
    pub const fn set_txdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receiver Block Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblocken(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Block Enable."]
    #[inline(always)]
    pub const fn set_rxblocken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receiver Block Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblockdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Block Disable."]
    #[inline(always)]
    pub const fn set_rxblockdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clear TX."]
    #[must_use]
    #[inline(always)]
    pub const fn cleartx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear TX."]
    #[inline(always)]
    pub const fn set_cleartx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Clear RX."]
    #[must_use]
    #[inline(always)]
    pub const fn clearrx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clear RX."]
    #[inline(always)]
    pub const fn set_clearrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd")
            .field("rxen", &self.rxen())
            .field("rxdis", &self.rxdis())
            .field("txen", &self.txen())
            .field("txdis", &self.txdis())
            .field("rxblocken", &self.rxblocken())
            .field("rxblockdis", &self.rxblockdis())
            .field("cleartx", &self.cleartx())
            .field("clearrx", &self.clearrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ rxen: {=bool:?}, rxdis: {=bool:?}, txen: {=bool:?}, txdis: {=bool:?}, rxblocken: {=bool:?}, rxblockdis: {=bool:?}, cleartx: {=bool:?}, clearrx: {=bool:?} }}" , self . rxen () , self . rxdis () , self . txen () , self . txdis () , self . rxblocken () , self . rxblockdis () , self . cleartx () , self . clearrx ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Automatic Transmitter Tristate."]
    #[must_use]
    #[inline(always)]
    pub const fn autotri(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Transmitter Tristate."]
    #[inline(always)]
    pub const fn set_autotri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn databits(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data-Bit Mode."]
    #[inline(always)]
    pub const fn set_databits(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Parity-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn parity(&self) -> super::vals::Parity {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Parity::from_bits(val as u8)
    }
    #[doc = "Parity-Bit Mode."]
    #[inline(always)]
    pub const fn set_parity(&mut self, val: super::vals::Parity) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Stop-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stopbits(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Stop-Bit Mode."]
    #[inline(always)]
    pub const fn set_stopbits(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Invert Input and Output."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Input and Output."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clear RX DMA on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn errsdma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear RX DMA on Error."]
    #[inline(always)]
    pub const fn set_errsdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Loopback Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn loopbk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Enable."]
    #[inline(always)]
    pub const fn set_loopbk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Start-Frame UnBlock RX."]
    #[must_use]
    #[inline(always)]
    pub const fn sfubrx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Start-Frame UnBlock RX."]
    #[inline(always)]
    pub const fn set_sfubrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Multi-Processor Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mpm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Mode."]
    #[inline(always)]
    pub const fn set_mpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Multi-Processor Address-Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn mpab(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Address-Bit."]
    #[inline(always)]
    pub const fn set_mpab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit 8 Default Value."]
    #[must_use]
    #[inline(always)]
    pub const fn bit8dv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bit 8 Default Value."]
    #[inline(always)]
    pub const fn set_bit8dv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RX DMA Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmawu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RX DMA Wakeup."]
    #[inline(always)]
    pub const fn set_rxdmawu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TX DMA Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn txdmawu(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TX DMA Wakeup."]
    #[inline(always)]
    pub const fn set_txdmawu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TX Delay Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txdelay(&self) -> super::vals::Txdelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Txdelay::from_bits(val as u8)
    }
    #[doc = "TX Delay Transmission."]
    #[inline(always)]
    pub const fn set_txdelay(&mut self, val: super::vals::Txdelay) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("autotri", &self.autotri())
            .field("databits", &self.databits())
            .field("parity", &self.parity())
            .field("stopbits", &self.stopbits())
            .field("inv", &self.inv())
            .field("errsdma", &self.errsdma())
            .field("loopbk", &self.loopbk())
            .field("sfubrx", &self.sfubrx())
            .field("mpm", &self.mpm())
            .field("mpab", &self.mpab())
            .field("bit8dv", &self.bit8dv())
            .field("rxdmawu", &self.rxdmawu())
            .field("txdmawu", &self.txdmawu())
            .field("txdelay", &self.txdelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ autotri: {=bool:?}, databits: {=bool:?}, parity: {:?}, stopbits: {=bool:?}, inv: {=bool:?}, errsdma: {=bool:?}, loopbk: {=bool:?}, sfubrx: {=bool:?}, mpm: {=bool:?}, mpab: {=bool:?}, bit8dv: {=bool:?}, rxdmawu: {=bool:?}, txdmawu: {=bool:?}, txdelay: {:?} }}" , self . autotri () , self . databits () , self . parity () , self . stopbits () , self . inv () , self . errsdma () , self . loopbk () , self . sfubrx () , self . mpm () , self . mpab () , self . bit8dv () , self . rxdmawu () , self . txdmawu () , self . txdelay ())
    }
}
#[doc = "Freeze Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freeze(pub u32);
impl Freeze {
    #[doc = "Register Update Freeze."]
    #[must_use]
    #[inline(always)]
    pub const fn regfreeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Update Freeze."]
    #[inline(always)]
    pub const fn set_regfreeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Freeze {
    #[inline(always)]
    fn default() -> Freeze {
        Freeze(0)
    }
}
impl core::fmt::Debug for Freeze {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freeze")
            .field("regfreeze", &self.regfreeze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freeze {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Freeze {{ regfreeze: {=bool:?} }}", self.regfreeze())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "TXC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TXC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TXBL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXBL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RXDATAV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RXDATAV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RXUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RXUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "MPAF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MPAF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STARTF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn startf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STARTF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_startf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SIGF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sigf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SIGF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sigf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ien {
    #[inline(always)]
    fn default() -> Ien {
        Ien(0)
    }
}
impl core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ien")
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("startf", &self.startf())
            .field("sigf", &self.sigf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, startf: {=bool:?}, sigf: {=bool:?} }}" , self . txc () , self . txbl () , self . rxdatav () , self . rxof () , self . rxuf () , self . txof () , self . perr () , self . ferr () , self . mpaf () , self . startf () , self . sigf ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "TX Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX Buffer Level Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Level Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Data Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RX Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RX Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RX Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RX Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TX Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TX Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Parity Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Framing Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Multi-Processor Address Frame Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Address Frame Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Start Frame Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn startf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Start Frame Interrupt Flag."]
    #[inline(always)]
    pub const fn set_startf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Signal Frame Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sigf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Signal Frame Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sigf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for If {
    #[inline(always)]
    fn default() -> If {
        If(0)
    }
}
impl core::fmt::Debug for If {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If")
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("startf", &self.startf())
            .field("sigf", &self.sigf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, startf: {=bool:?}, sigf: {=bool:?} }}" , self . txc () , self . txbl () , self . rxdatav () , self . rxof () , self . rxuf () , self . txof () , self . perr () , self . ferr () , self . mpaf () , self . startf () , self . sigf ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set TXC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set RXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set PERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set PERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set FERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set FERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set MPAF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set MPAF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set STARTF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn startf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set STARTF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_startf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set SIGF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sigf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set SIGF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sigf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ifs {
    #[inline(always)]
    fn default() -> Ifs {
        Ifs(0)
    }
}
impl core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifs")
            .field("txc", &self.txc())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("startf", &self.startf())
            .field("sigf", &self.sigf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ txc: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, startf: {=bool:?}, sigf: {=bool:?} }}" , self . txc () , self . rxof () , self . rxuf () , self . txof () , self . perr () , self . ferr () , self . mpaf () , self . startf () , self . sigf ())
    }
}
#[doc = "LEUART Input Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "RX PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rxprssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "RX PRS Channel Select."]
    #[inline(always)]
    pub const fn set_rxprssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "PRS RX Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxprs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PRS RX Enable."]
    #[inline(always)]
    pub const fn set_rxprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Input {
    #[inline(always)]
    fn default() -> Input {
        Input(0)
    }
}
impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Input")
            .field("rxprssel", &self.rxprssel())
            .field("rxprs", &self.rxprs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ rxprssel: {:?}, rxprs: {=bool:?} }}",
            self.rxprssel(),
            self.rxprs()
        )
    }
}
#[doc = "Pulse Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pulsectrl(pub u32);
impl Pulsectrl {
    #[doc = "Pulse Width."]
    #[must_use]
    #[inline(always)]
    pub const fn pulsew(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse Width."]
    #[inline(always)]
    pub const fn set_pulsew(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pulse Generator/Extender Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pulseen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pulse Generator/Extender Enable."]
    #[inline(always)]
    pub const fn set_pulseen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pulse Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn pulsefilt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pulse Filter."]
    #[inline(always)]
    pub const fn set_pulsefilt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pulsectrl {
    #[inline(always)]
    fn default() -> Pulsectrl {
        Pulsectrl(0)
    }
}
impl core::fmt::Debug for Pulsectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pulsectrl")
            .field("pulsew", &self.pulsew())
            .field("pulseen", &self.pulseen())
            .field("pulsefilt", &self.pulsefilt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pulsectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pulsectrl {{ pulsew: {=u8:?}, pulseen: {=bool:?}, pulsefilt: {=bool:?} }}",
            self.pulsew(),
            self.pulseen(),
            self.pulsefilt()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn rxloc(&self) -> super::vals::Rxloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Rxloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_rxloc(&mut self, val: super::vals::Rxloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn txloc(&self) -> super::vals::Txloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Txloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_txloc(&mut self, val: super::vals::Txloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
}
impl Default for Routeloc0 {
    #[inline(always)]
    fn default() -> Routeloc0 {
        Routeloc0(0)
    }
}
impl core::fmt::Debug for Routeloc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc0")
            .field("rxloc", &self.rxloc())
            .field("txloc", &self.txloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ rxloc: {:?}, txloc: {:?} }}",
            self.rxloc(),
            self.txloc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "RX Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Pin Enable."]
    #[inline(always)]
    pub const fn set_rxpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX Pin Enable."]
    #[inline(always)]
    pub const fn set_txpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Routepen {
    #[inline(always)]
    fn default() -> Routepen {
        Routepen(0)
    }
}
impl core::fmt::Debug for Routepen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routepen")
            .field("rxpen", &self.rxpen())
            .field("txpen", &self.txpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routepen {{ rxpen: {=bool:?}, txpen: {=bool:?} }}",
            self.rxpen(),
            self.txpen()
        )
    }
}
#[doc = "Receive Buffer Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdata(pub u32);
impl Rxdata {
    #[doc = "RX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxdata {
    #[inline(always)]
    fn default() -> Rxdata {
        Rxdata(0)
    }
}
impl core::fmt::Debug for Rxdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdata")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxdata {{ rxdata: {=u8:?} }}", self.rxdata())
    }
}
#[doc = "Receive Buffer Data Extended Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdatax(pub u32);
impl Rxdatax {
    #[doc = "RX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Receive Data Parity Error."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Parity Error."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Receive Data Framing Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Framing Error."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Rxdatax {
    #[inline(always)]
    fn default() -> Rxdatax {
        Rxdatax(0)
    }
}
impl core::fmt::Debug for Rxdatax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdatax")
            .field("rxdata", &self.rxdata())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdatax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxdatax {{ rxdata: {=u16:?}, perr: {=bool:?}, ferr: {=bool:?} }}",
            self.rxdata(),
            self.perr(),
            self.ferr()
        )
    }
}
#[doc = "Receive Buffer Data Extended Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdataxp(pub u32);
impl Rxdataxp {
    #[doc = "RX Data Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data Peek."]
    #[inline(always)]
    pub const fn set_rxdatap(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Receive Data Parity Error Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn perrp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Parity Error Peek."]
    #[inline(always)]
    pub const fn set_perrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Receive Data Framing Error Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn ferrp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Framing Error Peek."]
    #[inline(always)]
    pub const fn set_ferrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Rxdataxp {
    #[inline(always)]
    fn default() -> Rxdataxp {
        Rxdataxp(0)
    }
}
impl core::fmt::Debug for Rxdataxp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdataxp")
            .field("rxdatap", &self.rxdatap())
            .field("perrp", &self.perrp())
            .field("ferrp", &self.ferrp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdataxp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxdataxp {{ rxdatap: {=u16:?}, perrp: {=bool:?}, ferrp: {=bool:?} }}",
            self.rxdatap(),
            self.perrp(),
            self.ferrp()
        )
    }
}
#[doc = "Signal Frame Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sigframe(pub u32);
impl Sigframe {
    #[doc = "Signal Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn sigframe(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Signal Frame."]
    #[inline(always)]
    pub const fn set_sigframe(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Sigframe {
    #[inline(always)]
    fn default() -> Sigframe {
        Sigframe(0)
    }
}
impl core::fmt::Debug for Sigframe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sigframe")
            .field("sigframe", &self.sigframe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sigframe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sigframe {{ sigframe: {=u16:?} }}", self.sigframe())
    }
}
#[doc = "Start Frame Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startframe(pub u32);
impl Startframe {
    #[doc = "Start Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn startframe(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Start Frame."]
    #[inline(always)]
    pub const fn set_startframe(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Startframe {
    #[inline(always)]
    fn default() -> Startframe {
        Startframe(0)
    }
}
impl core::fmt::Debug for Startframe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startframe")
            .field("startframe", &self.startframe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startframe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Startframe {{ startframe: {=u16:?} }}",
            self.startframe()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Receiver Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxens(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable Status."]
    #[inline(always)]
    pub const fn set_rxens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txens(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable Status."]
    #[inline(always)]
    pub const fn set_txens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Incoming Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Block Incoming Data."]
    #[inline(always)]
    pub const fn set_rxblock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TX Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TX Complete."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TX Buffer Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Level."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Valid."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TX Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TX Idle."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("rxens", &self.rxens())
            .field("txens", &self.txens())
            .field("rxblock", &self.rxblock())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("txidle", &self.txidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ rxens: {=bool:?}, txens: {=bool:?}, rxblock: {=bool:?}, txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, txidle: {=bool:?} }}" , self . rxens () , self . txens () , self . rxblock () , self . txc () , self . txbl () , self . rxdatav () , self . txidle ())
    }
}
#[doc = "Synchronization Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc = "CTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CTRL Register Busy."]
    #[inline(always)]
    pub const fn set_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CMD Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CMD Register Busy."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CLKDIV Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CLKDIV Register Busy."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STARTFRAME Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn startframe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "STARTFRAME Register Busy."]
    #[inline(always)]
    pub const fn set_startframe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SIGFRAME Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn sigframe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SIGFRAME Register Busy."]
    #[inline(always)]
    pub const fn set_sigframe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TXDATAX Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn txdatax(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXDATAX Register Busy."]
    #[inline(always)]
    pub const fn set_txdatax(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TXDATA Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TXDATA Register Busy."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PULSECTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn pulsectrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PULSECTRL Register Busy."]
    #[inline(always)]
    pub const fn set_pulsectrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Syncbusy {
    #[inline(always)]
    fn default() -> Syncbusy {
        Syncbusy(0)
    }
}
impl core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syncbusy")
            .field("ctrl", &self.ctrl())
            .field("cmd", &self.cmd())
            .field("clkdiv", &self.clkdiv())
            .field("startframe", &self.startframe())
            .field("sigframe", &self.sigframe())
            .field("txdatax", &self.txdatax())
            .field("txdata", &self.txdata())
            .field("pulsectrl", &self.pulsectrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Syncbusy {{ ctrl: {=bool:?}, cmd: {=bool:?}, clkdiv: {=bool:?}, startframe: {=bool:?}, sigframe: {=bool:?}, txdatax: {=bool:?}, txdata: {=bool:?}, pulsectrl: {=bool:?} }}" , self . ctrl () , self . cmd () , self . clkdiv () , self . startframe () , self . sigframe () , self . txdatax () , self . txdata () , self . pulsectrl ())
    }
}
#[doc = "Transmit Buffer Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata(pub u32);
impl Txdata {
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata {
    #[inline(always)]
    fn default() -> Txdata {
        Txdata(0)
    }
}
impl core::fmt::Debug for Txdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata")
            .field("txdata", &self.txdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata {{ txdata: {=u8:?} }}", self.txdata())
    }
}
#[doc = "Transmit Buffer Data Extended Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdatax(pub u32);
impl Txdatax {
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Transmit Data as Break."]
    #[must_use]
    #[inline(always)]
    pub const fn txbreak(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data as Break."]
    #[inline(always)]
    pub const fn set_txbreak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Disable TX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisat(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Disable TX After Transmission."]
    #[inline(always)]
    pub const fn set_txdisat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn rxenat(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX After Transmission."]
    #[inline(always)]
    pub const fn set_rxenat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Txdatax {
    #[inline(always)]
    fn default() -> Txdatax {
        Txdatax(0)
    }
}
impl core::fmt::Debug for Txdatax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdatax")
            .field("txdata", &self.txdata())
            .field("txbreak", &self.txbreak())
            .field("txdisat", &self.txdisat())
            .field("rxenat", &self.rxenat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdatax {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Txdatax {{ txdata: {=u16:?}, txbreak: {=bool:?}, txdisat: {=bool:?}, rxenat: {=bool:?} }}" , self . txdata () , self . txbreak () , self . txdisat () , self . rxenat ())
    }
}
