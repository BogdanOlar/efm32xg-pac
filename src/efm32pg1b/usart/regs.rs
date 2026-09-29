#[doc = "Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "Fractional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Fractional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 3usize)) | (((val as u32) & 0x000f_ffff) << 3usize);
    }
    #[doc = "AUTOBAUD Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autobauden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AUTOBAUD Detection Enable."]
    #[inline(always)]
    pub const fn set_autobauden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        f.debug_struct("Clkdiv")
            .field("div", &self.div())
            .field("autobauden", &self.autobauden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkdiv {{ div: {=u32:?}, autobauden: {=bool:?} }}",
            self.div(),
            self.autobauden()
        )
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
    #[doc = "Master Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn masteren(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Enable."]
    #[inline(always)]
    pub const fn set_masteren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn masterdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Disable."]
    #[inline(always)]
    pub const fn set_masterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receiver Block Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblocken(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Block Enable."]
    #[inline(always)]
    pub const fn set_rxblocken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receiver Block Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblockdis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Block Disable."]
    #[inline(always)]
    pub const fn set_rxblockdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmitter Tristate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txtrien(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Tristate Enable."]
    #[inline(always)]
    pub const fn set_txtrien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmitter Tristate Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn txtridis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Tristate Disable."]
    #[inline(always)]
    pub const fn set_txtridis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clear TX."]
    #[must_use]
    #[inline(always)]
    pub const fn cleartx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clear TX."]
    #[inline(always)]
    pub const fn set_cleartx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Clear RX."]
    #[must_use]
    #[inline(always)]
    pub const fn clearrx(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Clear RX."]
    #[inline(always)]
    pub const fn set_clearrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("masteren", &self.masteren())
            .field("masterdis", &self.masterdis())
            .field("rxblocken", &self.rxblocken())
            .field("rxblockdis", &self.rxblockdis())
            .field("txtrien", &self.txtrien())
            .field("txtridis", &self.txtridis())
            .field("cleartx", &self.cleartx())
            .field("clearrx", &self.clearrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ rxen: {=bool:?}, rxdis: {=bool:?}, txen: {=bool:?}, txdis: {=bool:?}, masteren: {=bool:?}, masterdis: {=bool:?}, rxblocken: {=bool:?}, rxblockdis: {=bool:?}, txtrien: {=bool:?}, txtridis: {=bool:?}, cleartx: {=bool:?}, clearrx: {=bool:?} }}" , self . rxen () , self . rxdis () , self . txen () , self . txdis () , self . masteren () , self . masterdis () , self . rxblocken () , self . rxblockdis () , self . txtrien () , self . txtridis () , self . cleartx () , self . clearrx ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "USART Synchronous Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART Synchronous Mode."]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Loopback Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn loopbk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Enable."]
    #[inline(always)]
    pub const fn set_loopbk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Collision Check Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Collision Check Enable."]
    #[inline(always)]
    pub const fn set_ccen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Multi-Processor Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mpm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Mode."]
    #[inline(always)]
    pub const fn set_mpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Multi-Processor Address-Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn mpab(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Address-Bit."]
    #[inline(always)]
    pub const fn set_mpab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Oversampling."]
    #[must_use]
    #[inline(always)]
    pub const fn ovs(&self) -> super::vals::Ovs {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Ovs::from_bits(val as u8)
    }
    #[doc = "Oversampling."]
    #[inline(always)]
    pub const fn set_ovs(&mut self, val: super::vals::Ovs) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Clock Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn clkpol(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Polarity."]
    #[inline(always)]
    pub const fn set_clkpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock Edge for Setup/Sample."]
    #[must_use]
    #[inline(always)]
    pub const fn clkpha(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Edge for Setup/Sample."]
    #[inline(always)]
    pub const fn set_clkpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Most Significant Bit First."]
    #[must_use]
    #[inline(always)]
    pub const fn msbf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Most Significant Bit First."]
    #[inline(always)]
    pub const fn set_msbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Action on Slave-Select in Master Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn csma(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Action on Slave-Select in Master Mode."]
    #[inline(always)]
    pub const fn set_csma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TX Buffer Interrupt Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txbil(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Interrupt Level."]
    #[inline(always)]
    pub const fn set_txbil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Receiver Input Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn rxinv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Input Invert."]
    #[inline(always)]
    pub const fn set_rxinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmitter Output Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn txinv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Output Invert."]
    #[inline(always)]
    pub const fn set_txinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Chip Select Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn csinv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Invert."]
    #[inline(always)]
    pub const fn set_csinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Automatic Chip Select."]
    #[must_use]
    #[inline(always)]
    pub const fn autocs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Chip Select."]
    #[inline(always)]
    pub const fn set_autocs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Automatic TX Tristate."]
    #[must_use]
    #[inline(always)]
    pub const fn autotri(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic TX Tristate."]
    #[inline(always)]
    pub const fn set_autotri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartCard Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn scmode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartCard Mode."]
    #[inline(always)]
    pub const fn set_scmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartCard Retransmit."]
    #[must_use]
    #[inline(always)]
    pub const fn scretrans(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartCard Retransmit."]
    #[inline(always)]
    pub const fn set_scretrans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Skip Parity Error Frames."]
    #[must_use]
    #[inline(always)]
    pub const fn skipperrf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Skip Parity Error Frames."]
    #[inline(always)]
    pub const fn set_skipperrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit 8 Default Value."]
    #[must_use]
    #[inline(always)]
    pub const fn bit8dv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit 8 Default Value."]
    #[inline(always)]
    pub const fn set_bit8dv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Halt DMA on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn errsdma(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Halt DMA on Error."]
    #[inline(always)]
    pub const fn set_errsdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Disable RX on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn errsrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Disable RX on Error."]
    #[inline(always)]
    pub const fn set_errsrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Disable TX on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn errstx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Disable TX on Error."]
    #[inline(always)]
    pub const fn set_errstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Synchronous Slave Setup Early."]
    #[must_use]
    #[inline(always)]
    pub const fn sssearly(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Slave Setup Early."]
    #[inline(always)]
    pub const fn set_sssearly(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Byteswap in Double Accesses."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Byteswap in Double Accesses."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Always Transmit When RX Not Full."]
    #[must_use]
    #[inline(always)]
    pub const fn autotx(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Always Transmit When RX Not Full."]
    #[inline(always)]
    pub const fn set_autotx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Majority Vote Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mvdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Majority Vote Disable."]
    #[inline(always)]
    pub const fn set_mvdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Synchronous Master Sample Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn smsdelay(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Master Sample Delay."]
    #[inline(always)]
    pub const fn set_smsdelay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("sync", &self.sync())
            .field("loopbk", &self.loopbk())
            .field("ccen", &self.ccen())
            .field("mpm", &self.mpm())
            .field("mpab", &self.mpab())
            .field("ovs", &self.ovs())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .field("msbf", &self.msbf())
            .field("csma", &self.csma())
            .field("txbil", &self.txbil())
            .field("rxinv", &self.rxinv())
            .field("txinv", &self.txinv())
            .field("csinv", &self.csinv())
            .field("autocs", &self.autocs())
            .field("autotri", &self.autotri())
            .field("scmode", &self.scmode())
            .field("scretrans", &self.scretrans())
            .field("skipperrf", &self.skipperrf())
            .field("bit8dv", &self.bit8dv())
            .field("errsdma", &self.errsdma())
            .field("errsrx", &self.errsrx())
            .field("errstx", &self.errstx())
            .field("sssearly", &self.sssearly())
            .field("byteswap", &self.byteswap())
            .field("autotx", &self.autotx())
            .field("mvdis", &self.mvdis())
            .field("smsdelay", &self.smsdelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ sync: {=bool:?}, loopbk: {=bool:?}, ccen: {=bool:?}, mpm: {=bool:?}, mpab: {=bool:?}, ovs: {:?}, clkpol: {=bool:?}, clkpha: {=bool:?}, msbf: {=bool:?}, csma: {=bool:?}, txbil: {=bool:?}, rxinv: {=bool:?}, txinv: {=bool:?}, csinv: {=bool:?}, autocs: {=bool:?}, autotri: {=bool:?}, scmode: {=bool:?}, scretrans: {=bool:?}, skipperrf: {=bool:?}, bit8dv: {=bool:?}, errsdma: {=bool:?}, errsrx: {=bool:?}, errstx: {=bool:?}, sssearly: {=bool:?}, byteswap: {=bool:?}, autotx: {=bool:?}, mvdis: {=bool:?}, smsdelay: {=bool:?} }}" , self . sync () , self . loopbk () , self . ccen () , self . mpm () , self . mpab () , self . ovs () , self . clkpol () , self . clkpha () , self . msbf () , self . csma () , self . txbil () , self . rxinv () , self . txinv () , self . csinv () , self . autocs () , self . autotri () , self . scmode () , self . scretrans () , self . skipperrf () , self . bit8dv () , self . errsdma () , self . errsrx () , self . errstx () , self . sssearly () , self . byteswap () , self . autotx () , self . mvdis () , self . smsdelay ())
    }
}
#[doc = "Control Register Extended."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlx(pub u32);
impl Ctrlx {
    #[doc = "Debug Halt."]
    #[must_use]
    #[inline(always)]
    pub const fn dbghalt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Halt."]
    #[inline(always)]
    pub const fn set_dbghalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CTS Pin Inversion."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsinv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Pin Inversion."]
    #[inline(always)]
    pub const fn set_ctsinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTS Function Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Function Enabled."]
    #[inline(always)]
    pub const fn set_ctsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RTS Pin Inversion."]
    #[must_use]
    #[inline(always)]
    pub const fn rtsinv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RTS Pin Inversion."]
    #[inline(always)]
    pub const fn set_rtsinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctrlx {
    #[inline(always)]
    fn default() -> Ctrlx {
        Ctrlx(0)
    }
}
impl core::fmt::Debug for Ctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrlx")
            .field("dbghalt", &self.dbghalt())
            .field("ctsinv", &self.ctsinv())
            .field("ctsen", &self.ctsen())
            .field("rtsinv", &self.rtsinv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrlx {{ dbghalt: {=bool:?}, ctsinv: {=bool:?}, ctsen: {=bool:?}, rtsinv: {=bool:?} }}" , self . dbghalt () , self . ctsinv () , self . ctsen () , self . rtsinv ())
    }
}
#[doc = "USART Frame Format Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frame(pub u32);
impl Frame {
    #[doc = "Data-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn databits(&self) -> super::vals::Databits {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Databits::from_bits(val as u8)
    }
    #[doc = "Data-Bit Mode."]
    #[inline(always)]
    pub const fn set_databits(&mut self, val: super::vals::Databits) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Parity-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn parity(&self) -> super::vals::Parity {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Parity::from_bits(val as u8)
    }
    #[doc = "Parity-Bit Mode."]
    #[inline(always)]
    pub const fn set_parity(&mut self, val: super::vals::Parity) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Stop-Bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stopbits(&self) -> super::vals::Stopbits {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Stopbits::from_bits(val as u8)
    }
    #[doc = "Stop-Bit Mode."]
    #[inline(always)]
    pub const fn set_stopbits(&mut self, val: super::vals::Stopbits) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Frame {
    #[inline(always)]
    fn default() -> Frame {
        Frame(0)
    }
}
impl core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frame")
            .field("databits", &self.databits())
            .field("parity", &self.parity())
            .field("stopbits", &self.stopbits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frame {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frame {{ databits: {:?}, parity: {:?}, stopbits: {:?} }}",
            self.databits(),
            self.parity(),
            self.stopbits()
        )
    }
}
#[doc = "I2S Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2sctrl(pub u32);
impl I2sctrl {
    #[doc = "Enable I2S Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable I2S Mode."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stero or Mono."]
    #[must_use]
    #[inline(always)]
    pub const fn mono(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stero or Mono."]
    #[inline(always)]
    pub const fn set_mono(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Justification of I2S Data."]
    #[must_use]
    #[inline(always)]
    pub const fn justify(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Justification of I2S Data."]
    #[inline(always)]
    pub const fn set_justify(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Separate DMA Request for Left/Right Data."]
    #[must_use]
    #[inline(always)]
    pub const fn dmasplit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Separate DMA Request for Left/Right Data."]
    #[inline(always)]
    pub const fn set_dmasplit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Delay on I2S Data."]
    #[must_use]
    #[inline(always)]
    pub const fn delay(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Delay on I2S Data."]
    #[inline(always)]
    pub const fn set_delay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2S Word Format."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::Format {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Format::from_bits(val as u8)
    }
    #[doc = "I2S Word Format."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::Format) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for I2sctrl {
    #[inline(always)]
    fn default() -> I2sctrl {
        I2sctrl(0)
    }
}
impl core::fmt::Debug for I2sctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2sctrl")
            .field("en", &self.en())
            .field("mono", &self.mono())
            .field("justify", &self.justify())
            .field("dmasplit", &self.dmasplit())
            .field("delay", &self.delay())
            .field("format", &self.format())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2sctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "I2sctrl {{ en: {=bool:?}, mono: {=bool:?}, justify: {=bool:?}, dmasplit: {=bool:?}, delay: {=bool:?}, format: {:?} }}" , self . en () , self . mono () , self . justify () , self . dmasplit () , self . delay () , self . format ())
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
    #[doc = "RXFULL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RXFULL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TXUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txuf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TXUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "MPAF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "MPAF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SSM Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ssm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SSM Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ssm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CCF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CCF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TXIDLE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TXIDLE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TCMP0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TCMP0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TCMP1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TCMP1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TCMP2 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TCMP2 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("rxfull", &self.rxfull())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("txuf", &self.txuf())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("ssm", &self.ssm())
            .field("ccf", &self.ccf())
            .field("txidle", &self.txidle())
            .field("tcmp0", &self.tcmp0())
            .field("tcmp1", &self.tcmp1())
            .field("tcmp2", &self.tcmp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxfull: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, txuf: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, ssm: {=bool:?}, ccf: {=bool:?}, txidle: {=bool:?}, tcmp0: {=bool:?}, tcmp1: {=bool:?}, tcmp2: {=bool:?} }}" , self . txc () , self . txbl () , self . rxdatav () , self . rxfull () , self . rxof () , self . rxuf () , self . txof () , self . txuf () , self . perr () , self . ferr () , self . mpaf () , self . ssm () , self . ccf () , self . txidle () , self . tcmp0 () , self . tcmp1 () , self . tcmp2 ())
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
    #[doc = "RX Buffer Full Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RX Buffer Full Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RX Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RX Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TX Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TX Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TX Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txuf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TX Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Framing Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Multi-Processor Address Frame Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Processor Address Frame Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Slave-Select in Master Mode Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ssm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave-Select in Master Mode Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ssm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Collision Check Fail Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Collision Check Fail Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TX Idle Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TX Idle Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Timer Comparator 0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Comparator 0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Timer Comparator 1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Comparator 1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timer Comparator 2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Comparator 2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("rxfull", &self.rxfull())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("txuf", &self.txuf())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("ssm", &self.ssm())
            .field("ccf", &self.ccf())
            .field("txidle", &self.txidle())
            .field("tcmp0", &self.tcmp0())
            .field("tcmp1", &self.tcmp1())
            .field("tcmp2", &self.tcmp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxfull: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, txuf: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, ssm: {=bool:?}, ccf: {=bool:?}, txidle: {=bool:?}, tcmp0: {=bool:?}, tcmp1: {=bool:?}, tcmp2: {=bool:?} }}" , self . txc () , self . txbl () , self . rxdatav () , self . rxfull () , self . rxof () , self . rxuf () , self . txof () , self . txuf () , self . perr () , self . ferr () , self . mpaf () , self . ssm () , self . ccf () , self . txidle () , self . tcmp0 () , self . tcmp1 () , self . tcmp2 ())
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
    #[doc = "Set RXFULL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXFULL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set RXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set TXUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txuf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set PERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set PERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set FERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set FERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set MPAF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mpaf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set MPAF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mpaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set SSM Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ssm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set SSM Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ssm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set CCF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set CCF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set TXIDLE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXIDLE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set TCMP0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set TCMP0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set TCMP1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set TCMP1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set TCMP2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmp2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set TCMP2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("rxfull", &self.rxfull())
            .field("rxof", &self.rxof())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("txuf", &self.txuf())
            .field("perr", &self.perr())
            .field("ferr", &self.ferr())
            .field("mpaf", &self.mpaf())
            .field("ssm", &self.ssm())
            .field("ccf", &self.ccf())
            .field("txidle", &self.txidle())
            .field("tcmp0", &self.tcmp0())
            .field("tcmp1", &self.tcmp1())
            .field("tcmp2", &self.tcmp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ txc: {=bool:?}, rxfull: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, txuf: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, ssm: {=bool:?}, ccf: {=bool:?}, txidle: {=bool:?}, tcmp0: {=bool:?}, tcmp1: {=bool:?}, tcmp2: {=bool:?} }}" , self . txc () , self . rxfull () , self . rxof () , self . rxuf () , self . txof () , self . txuf () , self . perr () , self . ferr () , self . mpaf () , self . ssm () , self . ccf () , self . txidle () , self . tcmp0 () , self . tcmp1 () , self . tcmp2 ())
    }
}
#[doc = "USART Input Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "RX PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rxprssel(&self) -> super::vals::Rxprssel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Rxprssel::from_bits(val as u8)
    }
    #[doc = "RX PRS Channel Select."]
    #[inline(always)]
    pub const fn set_rxprssel(&mut self, val: super::vals::Rxprssel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PRS RX Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxprs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PRS RX Enable."]
    #[inline(always)]
    pub const fn set_rxprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CLK PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clkprssel(&self) -> super::vals::Clkprssel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Clkprssel::from_bits(val as u8)
    }
    #[doc = "CLK PRS Channel Select."]
    #[inline(always)]
    pub const fn set_clkprssel(&mut self, val: super::vals::Clkprssel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "PRS CLK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkprs(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PRS CLK Enable."]
    #[inline(always)]
    pub const fn set_clkprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("clkprssel", &self.clkprssel())
            .field("clkprs", &self.clkprs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ rxprssel: {:?}, rxprs: {=bool:?}, clkprssel: {:?}, clkprs: {=bool:?} }}",
            self.rxprssel(),
            self.rxprs(),
            self.clkprssel(),
            self.clkprs()
        )
    }
}
#[doc = "IrDA Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irctrl(pub u32);
impl Irctrl {
    #[doc = "Enable IrDA Module."]
    #[must_use]
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IrDA Module."]
    #[inline(always)]
    pub const fn set_iren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IrDA TX Pulse Width."]
    #[must_use]
    #[inline(always)]
    pub const fn irpw(&self) -> super::vals::Irpw {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Irpw::from_bits(val as u8)
    }
    #[doc = "IrDA TX Pulse Width."]
    #[inline(always)]
    pub const fn set_irpw(&mut self, val: super::vals::Irpw) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "IrDA RX Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn irfilt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IrDA RX Filter."]
    #[inline(always)]
    pub const fn set_irfilt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IrDA PRS Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn irprsen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IrDA PRS Channel Enable."]
    #[inline(always)]
    pub const fn set_irprsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IrDA PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn irprssel(&self) -> super::vals::Irprssel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Irprssel::from_bits(val as u8)
    }
    #[doc = "IrDA PRS Channel Select."]
    #[inline(always)]
    pub const fn set_irprssel(&mut self, val: super::vals::Irprssel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Irctrl {
    #[inline(always)]
    fn default() -> Irctrl {
        Irctrl(0)
    }
}
impl core::fmt::Debug for Irctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irctrl")
            .field("iren", &self.iren())
            .field("irpw", &self.irpw())
            .field("irfilt", &self.irfilt())
            .field("irprsen", &self.irprsen())
            .field("irprssel", &self.irprssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irctrl {{ iren: {=bool:?}, irpw: {:?}, irfilt: {=bool:?}, irprsen: {=bool:?}, irprssel: {:?} }}" , self . iren () , self . irpw () , self . irfilt () , self . irprsen () , self . irprssel ())
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
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn csloc(&self) -> super::vals::Csloc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Csloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_csloc(&mut self, val: super::vals::Csloc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn clkloc(&self) -> super::vals::Clkloc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Clkloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_clkloc(&mut self, val: super::vals::Clkloc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
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
            .field("csloc", &self.csloc())
            .field("clkloc", &self.clkloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ rxloc: {:?}, txloc: {:?}, csloc: {:?}, clkloc: {:?} }}",
            self.rxloc(),
            self.txloc(),
            self.csloc(),
            self.clkloc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsloc(&self) -> super::vals::Ctsloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ctsloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ctsloc(&mut self, val: super::vals::Ctsloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn rtsloc(&self) -> super::vals::Rtsloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Rtsloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_rtsloc(&mut self, val: super::vals::Rtsloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
}
impl Default for Routeloc1 {
    #[inline(always)]
    fn default() -> Routeloc1 {
        Routeloc1(0)
    }
}
impl core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc1")
            .field("ctsloc", &self.ctsloc())
            .field("rtsloc", &self.rtsloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc1 {{ ctsloc: {:?}, rtsloc: {:?} }}",
            self.ctsloc(),
            self.rtsloc()
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
    #[doc = "CS Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cspen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CS Pin Enable."]
    #[inline(always)]
    pub const fn set_cspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CLK Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkpen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CLK Pin Enable."]
    #[inline(always)]
    pub const fn set_clkpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTS Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ctspen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTS Pin Enable."]
    #[inline(always)]
    pub const fn set_ctspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RTS Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtspen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RTS Pin Enable."]
    #[inline(always)]
    pub const fn set_rtspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("cspen", &self.cspen())
            .field("clkpen", &self.clkpen())
            .field("ctspen", &self.ctspen())
            .field("rtspen", &self.rtspen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ rxpen: {=bool:?}, txpen: {=bool:?}, cspen: {=bool:?}, clkpen: {=bool:?}, ctspen: {=bool:?}, rtspen: {=bool:?} }}" , self . rxpen () , self . txpen () , self . cspen () , self . clkpen () , self . ctspen () , self . rtspen ())
    }
}
#[doc = "RX Buffer Data Register."]
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
#[doc = "RX Buffer Data Extended Register."]
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
    #[doc = "Data Parity Error."]
    #[must_use]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error."]
    #[inline(always)]
    pub const fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Framing Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error."]
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
#[doc = "RX Buffer Data Extended Peek Register."]
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
    #[doc = "Data Parity Error Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn perrp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error Peek."]
    #[inline(always)]
    pub const fn set_perrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Framing Error Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn ferrp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error Peek."]
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
#[doc = "RX FIFO Double Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdouble(pub u32);
impl Rxdouble {
    #[doc = "RX Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data 0."]
    #[inline(always)]
    pub const fn set_rxdata0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Data 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data 1."]
    #[inline(always)]
    pub const fn set_rxdata1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Rxdouble {
    #[inline(always)]
    fn default() -> Rxdouble {
        Rxdouble(0)
    }
}
impl core::fmt::Debug for Rxdouble {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdouble")
            .field("rxdata0", &self.rxdata0())
            .field("rxdata1", &self.rxdata1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdouble {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxdouble {{ rxdata0: {=u8:?}, rxdata1: {=u8:?} }}",
            self.rxdata0(),
            self.rxdata1()
        )
    }
}
#[doc = "RX Buffer Double Data Extended Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdoublex(pub u32);
impl Rxdoublex {
    #[doc = "RX Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data 0."]
    #[inline(always)]
    pub const fn set_rxdata0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Data Parity Error 0."]
    #[must_use]
    #[inline(always)]
    pub const fn perr0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error 0."]
    #[inline(always)]
    pub const fn set_perr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Framing Error 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error 0."]
    #[inline(always)]
    pub const fn set_ferr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RX Data 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data 1."]
    #[inline(always)]
    pub const fn set_rxdata1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Data Parity Error 1."]
    #[must_use]
    #[inline(always)]
    pub const fn perr1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error 1."]
    #[inline(always)]
    pub const fn set_perr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Data Framing Error 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ferr1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error 1."]
    #[inline(always)]
    pub const fn set_ferr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxdoublex {
    #[inline(always)]
    fn default() -> Rxdoublex {
        Rxdoublex(0)
    }
}
impl core::fmt::Debug for Rxdoublex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdoublex")
            .field("rxdata0", &self.rxdata0())
            .field("perr0", &self.perr0())
            .field("ferr0", &self.ferr0())
            .field("rxdata1", &self.rxdata1())
            .field("perr1", &self.perr1())
            .field("ferr1", &self.ferr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdoublex {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rxdoublex {{ rxdata0: {=u16:?}, perr0: {=bool:?}, ferr0: {=bool:?}, rxdata1: {=u16:?}, perr1: {=bool:?}, ferr1: {=bool:?} }}" , self . rxdata0 () , self . perr0 () , self . ferr0 () , self . rxdata1 () , self . perr1 () , self . ferr1 ())
    }
}
#[doc = "RX Buffer Double Data Extended Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdoublexp(pub u32);
impl Rxdoublexp {
    #[doc = "RX Data 0 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data 0 Peek."]
    #[inline(always)]
    pub const fn set_rxdatap0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Data Parity Error 0 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn perrp0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error 0 Peek."]
    #[inline(always)]
    pub const fn set_perrp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Framing Error 0 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn ferrp0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error 0 Peek."]
    #[inline(always)]
    pub const fn set_ferrp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RX Data 1 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "RX Data 1 Peek."]
    #[inline(always)]
    pub const fn set_rxdatap1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Data Parity Error 1 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn perrp1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Data Parity Error 1 Peek."]
    #[inline(always)]
    pub const fn set_perrp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Data Framing Error 1 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn ferrp1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Data Framing Error 1 Peek."]
    #[inline(always)]
    pub const fn set_ferrp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxdoublexp {
    #[inline(always)]
    fn default() -> Rxdoublexp {
        Rxdoublexp(0)
    }
}
impl core::fmt::Debug for Rxdoublexp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdoublexp")
            .field("rxdatap0", &self.rxdatap0())
            .field("perrp0", &self.perrp0())
            .field("ferrp0", &self.ferrp0())
            .field("rxdatap1", &self.rxdatap1())
            .field("perrp1", &self.perrp1())
            .field("ferrp1", &self.ferrp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdoublexp {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rxdoublexp {{ rxdatap0: {=u16:?}, perrp0: {=bool:?}, ferrp0: {=bool:?}, rxdatap1: {=u16:?}, perrp1: {=bool:?}, ferrp1: {=bool:?} }}" , self . rxdatap0 () , self . perrp0 () , self . ferrp0 () , self . rxdatap1 () , self . perrp1 () , self . ferrp1 ())
    }
}
#[doc = "USART Status Register."]
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
    #[doc = "SPI Master Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Master Mode."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Block Incoming Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxblock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Block Incoming Data."]
    #[inline(always)]
    pub const fn set_rxblock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmitter Tristated."]
    #[must_use]
    #[inline(always)]
    pub const fn txtri(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Tristated."]
    #[inline(always)]
    pub const fn set_txtri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TX Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TX Complete."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TX Buffer Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Level."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Valid."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RX FIFO Full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Full."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TX Buffer Expects Double Right Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txbdright(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Expects Double Right Data."]
    #[inline(always)]
    pub const fn set_txbdright(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TX Buffer Expects Single Right Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txbsright(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Expects Single Right Data."]
    #[inline(always)]
    pub const fn set_txbsright(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RX Data Right."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatavright(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Right."]
    #[inline(always)]
    pub const fn set_rxdatavright(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RX Full of Right Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfullright(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RX Full of Right Data."]
    #[inline(always)]
    pub const fn set_rxfullright(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TX Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn txidle(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TX Idle."]
    #[inline(always)]
    pub const fn set_txidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The USART Timer Restarted Itself."]
    #[must_use]
    #[inline(always)]
    pub const fn timerrestarted(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The USART Timer Restarted Itself."]
    #[inline(always)]
    pub const fn set_timerrestarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TX Buffer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn txbufcnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "TX Buffer Count."]
    #[inline(always)]
    pub const fn set_txbufcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
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
            .field("master", &self.master())
            .field("rxblock", &self.rxblock())
            .field("txtri", &self.txtri())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxfull", &self.rxfull())
            .field("txbdright", &self.txbdright())
            .field("txbsright", &self.txbsright())
            .field("rxdatavright", &self.rxdatavright())
            .field("rxfullright", &self.rxfullright())
            .field("txidle", &self.txidle())
            .field("timerrestarted", &self.timerrestarted())
            .field("txbufcnt", &self.txbufcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ rxens: {=bool:?}, txens: {=bool:?}, master: {=bool:?}, rxblock: {=bool:?}, txtri: {=bool:?}, txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxfull: {=bool:?}, txbdright: {=bool:?}, txbsright: {=bool:?}, rxdatavright: {=bool:?}, rxfullright: {=bool:?}, txidle: {=bool:?}, timerrestarted: {=bool:?}, txbufcnt: {=u8:?} }}" , self . rxens () , self . txens () , self . master () , self . rxblock () , self . txtri () , self . txc () , self . txbl () , self . rxdatav () , self . rxfull () , self . txbdright () , self . txbsright () , self . rxdatavright () , self . rxfullright () , self . txidle () , self . timerrestarted () , self . txbufcnt ())
    }
}
#[doc = "Used to Generate Interrupts and Various Delays."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timecmp0(pub u32);
impl Timecmp0 {
    #[doc = "Timer Comparator 0."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmpval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Comparator 0."]
    #[inline(always)]
    pub const fn set_tcmpval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Start Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> super::vals::Timecmp0Tstart {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timecmp0Tstart::from_bits(val as u8)
    }
    #[doc = "Timer Start Source."]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: super::vals::Timecmp0Tstart) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Source Used to Disable Comparator 0."]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Timecmp0Tstop {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Timecmp0Tstop::from_bits(val as u8)
    }
    #[doc = "Source Used to Disable Comparator 0."]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Timecmp0Tstop) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Restart Timer on TCMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn restarten(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Timer on TCMP0."]
    #[inline(always)]
    pub const fn set_restarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Timecmp0 {
    #[inline(always)]
    fn default() -> Timecmp0 {
        Timecmp0(0)
    }
}
impl core::fmt::Debug for Timecmp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timecmp0")
            .field("tcmpval", &self.tcmpval())
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("restarten", &self.restarten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timecmp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timecmp0 {{ tcmpval: {=u8:?}, tstart: {:?}, tstop: {:?}, restarten: {=bool:?} }}",
            self.tcmpval(),
            self.tstart(),
            self.tstop(),
            self.restarten()
        )
    }
}
#[doc = "Used to Generate Interrupts and Various Delays."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timecmp1(pub u32);
impl Timecmp1 {
    #[doc = "Timer Comparator 1."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmpval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Comparator 1."]
    #[inline(always)]
    pub const fn set_tcmpval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Start Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> super::vals::Timecmp1Tstart {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timecmp1Tstart::from_bits(val as u8)
    }
    #[doc = "Timer Start Source."]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: super::vals::Timecmp1Tstart) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Source Used to Disable Comparator 1."]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Timecmp1Tstop {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Timecmp1Tstop::from_bits(val as u8)
    }
    #[doc = "Source Used to Disable Comparator 1."]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Timecmp1Tstop) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Restart Timer on TCMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn restarten(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Timer on TCMP1."]
    #[inline(always)]
    pub const fn set_restarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Timecmp1 {
    #[inline(always)]
    fn default() -> Timecmp1 {
        Timecmp1(0)
    }
}
impl core::fmt::Debug for Timecmp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timecmp1")
            .field("tcmpval", &self.tcmpval())
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("restarten", &self.restarten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timecmp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timecmp1 {{ tcmpval: {=u8:?}, tstart: {:?}, tstop: {:?}, restarten: {=bool:?} }}",
            self.tcmpval(),
            self.tstart(),
            self.tstop(),
            self.restarten()
        )
    }
}
#[doc = "Used to Generate Interrupts and Various Delays."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timecmp2(pub u32);
impl Timecmp2 {
    #[doc = "Timer Comparator 2."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmpval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Comparator 2."]
    #[inline(always)]
    pub const fn set_tcmpval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Start Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> super::vals::Timecmp2Tstart {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timecmp2Tstart::from_bits(val as u8)
    }
    #[doc = "Timer Start Source."]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: super::vals::Timecmp2Tstart) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Source Used to Disable Comparator 2."]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Timecmp2Tstop {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Timecmp2Tstop::from_bits(val as u8)
    }
    #[doc = "Source Used to Disable Comparator 2."]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Timecmp2Tstop) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Restart Timer on TCMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn restarten(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Timer on TCMP2."]
    #[inline(always)]
    pub const fn set_restarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Timecmp2 {
    #[inline(always)]
    fn default() -> Timecmp2 {
        Timecmp2(0)
    }
}
impl core::fmt::Debug for Timecmp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timecmp2")
            .field("tcmpval", &self.tcmpval())
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("restarten", &self.restarten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timecmp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timecmp2 {{ tcmpval: {=u8:?}, tstart: {:?}, tstop: {:?}, restarten: {=bool:?} }}",
            self.tcmpval(),
            self.tstart(),
            self.tstop(),
            self.restarten()
        )
    }
}
#[doc = "Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing(pub u32);
impl Timing {
    #[doc = "TX Frame Start Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn txdelay(&self) -> super::vals::Txdelay {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Txdelay::from_bits(val as u8)
    }
    #[doc = "TX Frame Start Delay."]
    #[inline(always)]
    pub const fn set_txdelay(&mut self, val: super::vals::Txdelay) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Chip Select Setup."]
    #[must_use]
    #[inline(always)]
    pub const fn cssetup(&self) -> super::vals::Cssetup {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Cssetup::from_bits(val as u8)
    }
    #[doc = "Chip Select Setup."]
    #[inline(always)]
    pub const fn set_cssetup(&mut self, val: super::vals::Cssetup) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Inter-character Spacing."]
    #[must_use]
    #[inline(always)]
    pub const fn ics(&self) -> super::vals::Ics {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Ics::from_bits(val as u8)
    }
    #[doc = "Inter-character Spacing."]
    #[inline(always)]
    pub const fn set_ics(&mut self, val: super::vals::Ics) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Chip Select Hold."]
    #[must_use]
    #[inline(always)]
    pub const fn cshold(&self) -> super::vals::Cshold {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Cshold::from_bits(val as u8)
    }
    #[doc = "Chip Select Hold."]
    #[inline(always)]
    pub const fn set_cshold(&mut self, val: super::vals::Cshold) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Timing {
    #[inline(always)]
    fn default() -> Timing {
        Timing(0)
    }
}
impl core::fmt::Debug for Timing {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing")
            .field("txdelay", &self.txdelay())
            .field("cssetup", &self.cssetup())
            .field("ics", &self.ics())
            .field("cshold", &self.cshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timing {{ txdelay: {:?}, cssetup: {:?}, ics: {:?}, cshold: {:?} }}",
            self.txdelay(),
            self.cssetup(),
            self.ics(),
            self.cshold()
        )
    }
}
#[doc = "USART Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigctrl(pub u32);
impl Trigctrl {
    #[doc = "Receive Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Trigger Enable."]
    #[inline(always)]
    pub const fn set_rxten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txten(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Trigger Enable."]
    #[inline(always)]
    pub const fn set_txten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AUTOTX Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autotxten(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AUTOTX Trigger Enable."]
    #[inline(always)]
    pub const fn set_autotxten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL."]
    #[must_use]
    #[inline(always)]
    pub const fn txarx0en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL."]
    #[inline(always)]
    pub const fn set_txarx0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL."]
    #[must_use]
    #[inline(always)]
    pub const fn txarx1en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL."]
    #[inline(always)]
    pub const fn set_txarx1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL."]
    #[must_use]
    #[inline(always)]
    pub const fn txarx2en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL."]
    #[inline(always)]
    pub const fn set_txarx2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times."]
    #[must_use]
    #[inline(always)]
    pub const fn rxatx0en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times."]
    #[inline(always)]
    pub const fn set_rxatx0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times."]
    #[must_use]
    #[inline(always)]
    pub const fn rxatx1en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times."]
    #[inline(always)]
    pub const fn set_rxatx1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times."]
    #[must_use]
    #[inline(always)]
    pub const fn rxatx2en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times."]
    #[inline(always)]
    pub const fn set_rxatx2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Trigger PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tsel(&self) -> super::vals::Tsel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Tsel::from_bits(val as u8)
    }
    #[doc = "Trigger PRS Channel Select."]
    #[inline(always)]
    pub const fn set_tsel(&mut self, val: super::vals::Tsel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Trigctrl {
    #[inline(always)]
    fn default() -> Trigctrl {
        Trigctrl(0)
    }
}
impl core::fmt::Debug for Trigctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trigctrl")
            .field("rxten", &self.rxten())
            .field("txten", &self.txten())
            .field("autotxten", &self.autotxten())
            .field("txarx0en", &self.txarx0en())
            .field("txarx1en", &self.txarx1en())
            .field("txarx2en", &self.txarx2en())
            .field("rxatx0en", &self.rxatx0en())
            .field("rxatx1en", &self.rxatx1en())
            .field("rxatx2en", &self.rxatx2en())
            .field("tsel", &self.tsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trigctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trigctrl {{ rxten: {=bool:?}, txten: {=bool:?}, autotxten: {=bool:?}, txarx0en: {=bool:?}, txarx1en: {=bool:?}, txarx2en: {=bool:?}, rxatx0en: {=bool:?}, rxatx1en: {=bool:?}, rxatx2en: {=bool:?}, tsel: {:?} }}" , self . rxten () , self . txten () , self . autotxten () , self . txarx0en () , self . txarx1en () , self . txarx2en () , self . rxatx0en () , self . rxatx1en () , self . rxatx2en () , self . tsel ())
    }
}
#[doc = "TX Buffer Data Register."]
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
#[doc = "TX Buffer Data Extended Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdatax(pub u32);
impl Txdatax {
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdatax(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdatax(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Unblock RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn ubrxat(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Unblock RX After Transmission."]
    #[inline(always)]
    pub const fn set_ubrxat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set TXTRI After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txtriat(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXTRI After Transmission."]
    #[inline(always)]
    pub const fn set_txtriat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    #[doc = "Clear TXEN After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisat(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Clear TXEN After Transmission."]
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
            .field("txdatax", &self.txdatax())
            .field("ubrxat", &self.ubrxat())
            .field("txtriat", &self.txtriat())
            .field("txbreak", &self.txbreak())
            .field("txdisat", &self.txdisat())
            .field("rxenat", &self.rxenat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdatax {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Txdatax {{ txdatax: {=u16:?}, ubrxat: {=bool:?}, txtriat: {=bool:?}, txbreak: {=bool:?}, txdisat: {=bool:?}, rxenat: {=bool:?} }}" , self . txdatax () , self . ubrxat () , self . txtriat () , self . txbreak () , self . txdisat () , self . rxenat ())
    }
}
#[doc = "TX Buffer Double Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdouble(pub u32);
impl Txdouble {
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Txdouble {
    #[inline(always)]
    fn default() -> Txdouble {
        Txdouble(0)
    }
}
impl core::fmt::Debug for Txdouble {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdouble")
            .field("txdata0", &self.txdata0())
            .field("txdata1", &self.txdata1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdouble {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txdouble {{ txdata0: {=u8:?}, txdata1: {=u8:?} }}",
            self.txdata0(),
            self.txdata1()
        )
    }
}
#[doc = "TX Buffer Double Data Extended Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdoublex(pub u32);
impl Txdoublex {
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Unblock RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn ubrxat0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Unblock RX After Transmission."]
    #[inline(always)]
    pub const fn set_ubrxat0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set TXTRI After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txtriat0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXTRI After Transmission."]
    #[inline(always)]
    pub const fn set_txtriat0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Transmit Data as Break."]
    #[must_use]
    #[inline(always)]
    pub const fn txbreak0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data as Break."]
    #[inline(always)]
    pub const fn set_txbreak0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Clear TXEN After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisat0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Clear TXEN After Transmission."]
    #[inline(always)]
    pub const fn set_txdisat0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn rxenat0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX After Transmission."]
    #[inline(always)]
    pub const fn set_rxenat0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TX Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "TX Data."]
    #[inline(always)]
    pub const fn set_txdata1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Unblock RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn ubrxat1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Unblock RX After Transmission."]
    #[inline(always)]
    pub const fn set_ubrxat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set TXTRI After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txtriat1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXTRI After Transmission."]
    #[inline(always)]
    pub const fn set_txtriat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit Data as Break."]
    #[must_use]
    #[inline(always)]
    pub const fn txbreak1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data as Break."]
    #[inline(always)]
    pub const fn set_txbreak1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Clear TXEN After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txdisat1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Clear TXEN After Transmission."]
    #[inline(always)]
    pub const fn set_txdisat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable RX After Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn rxenat1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX After Transmission."]
    #[inline(always)]
    pub const fn set_rxenat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Txdoublex {
    #[inline(always)]
    fn default() -> Txdoublex {
        Txdoublex(0)
    }
}
impl core::fmt::Debug for Txdoublex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdoublex")
            .field("txdata0", &self.txdata0())
            .field("ubrxat0", &self.ubrxat0())
            .field("txtriat0", &self.txtriat0())
            .field("txbreak0", &self.txbreak0())
            .field("txdisat0", &self.txdisat0())
            .field("rxenat0", &self.rxenat0())
            .field("txdata1", &self.txdata1())
            .field("ubrxat1", &self.ubrxat1())
            .field("txtriat1", &self.txtriat1())
            .field("txbreak1", &self.txbreak1())
            .field("txdisat1", &self.txdisat1())
            .field("rxenat1", &self.rxenat1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdoublex {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Txdoublex {{ txdata0: {=u16:?}, ubrxat0: {=bool:?}, txtriat0: {=bool:?}, txbreak0: {=bool:?}, txdisat0: {=bool:?}, rxenat0: {=bool:?}, txdata1: {=u16:?}, ubrxat1: {=bool:?}, txtriat1: {=bool:?}, txbreak1: {=bool:?}, txdisat1: {=bool:?}, rxenat1: {=bool:?} }}" , self . txdata0 () , self . ubrxat0 () , self . txtriat0 () , self . txbreak0 () , self . txdisat0 () , self . rxenat0 () , self . txdata1 () , self . ubrxat1 () , self . txtriat1 () , self . txbreak1 () , self . txdisat1 () , self . rxenat1 ())
    }
}
