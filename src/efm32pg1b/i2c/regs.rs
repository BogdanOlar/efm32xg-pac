#[doc = "Clock Division Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
    #[doc = "Send Start Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Send Start Condition."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Send Stop Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Send Stop Condition."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Send ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Send ACK."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Send NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Send NACK."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Continue Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Continue Transmission."]
    #[inline(always)]
    pub const fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Abort Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Abort Transmission."]
    #[inline(always)]
    pub const fn set_abort(&mut self, val: bool) {
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
    #[doc = "Clear Pending Commands."]
    #[must_use]
    #[inline(always)]
    pub const fn clearpc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Pending Commands."]
    #[inline(always)]
    pub const fn set_clearpc(&mut self, val: bool) {
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
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("ack", &self.ack())
            .field("nack", &self.nack())
            .field("cont", &self.cont())
            .field("abort", &self.abort())
            .field("cleartx", &self.cleartx())
            .field("clearpc", &self.clearpc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ start: {=bool:?}, stop: {=bool:?}, ack: {=bool:?}, nack: {=bool:?}, cont: {=bool:?}, abort: {=bool:?}, cleartx: {=bool:?}, clearpc: {=bool:?} }}" , self . start () , self . stop () , self . ack () , self . nack () , self . cont () , self . abort () , self . cleartx () , self . clearpc ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "I2C Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Addressable as Slave."]
    #[must_use]
    #[inline(always)]
    pub const fn slave(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Addressable as Slave."]
    #[inline(always)]
    pub const fn set_slave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Automatic Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn autoack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Acknowledge."]
    #[inline(always)]
    pub const fn set_autoack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Automatic STOP When Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn autose(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic STOP When Empty."]
    #[inline(always)]
    pub const fn set_autose(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatic STOP on NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn autosn(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic STOP on NACK."]
    #[inline(always)]
    pub const fn set_autosn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbitration Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn arbdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Disable."]
    #[inline(always)]
    pub const fn set_arbdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "General Call Address Match Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gcamen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Address Match Enable."]
    #[inline(always)]
    pub const fn set_gcamen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TX Buffer Interrupt Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txbil(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Interrupt Level."]
    #[inline(always)]
    pub const fn set_txbil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock Low High Ratio."]
    #[must_use]
    #[inline(always)]
    pub const fn clhr(&self) -> super::vals::Clhr {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Clhr::from_bits(val as u8)
    }
    #[doc = "Clock Low High Ratio."]
    #[inline(always)]
    pub const fn set_clhr(&mut self, val: super::vals::Clhr) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Bus Idle Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn bito(&self) -> super::vals::Bito {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bito::from_bits(val as u8)
    }
    #[doc = "Bus Idle Timeout."]
    #[inline(always)]
    pub const fn set_bito(&mut self, val: super::vals::Bito) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Go Idle on Bus Idle Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn gibito(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Go Idle on Bus Idle Timeout."]
    #[inline(always)]
    pub const fn set_gibito(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Clock Low Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn clto(&self) -> super::vals::Clto {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Clto::from_bits(val as u8)
    }
    #[doc = "Clock Low Timeout."]
    #[inline(always)]
    pub const fn set_clto(&mut self, val: super::vals::Clto) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
            .field("en", &self.en())
            .field("slave", &self.slave())
            .field("autoack", &self.autoack())
            .field("autose", &self.autose())
            .field("autosn", &self.autosn())
            .field("arbdis", &self.arbdis())
            .field("gcamen", &self.gcamen())
            .field("txbil", &self.txbil())
            .field("clhr", &self.clhr())
            .field("bito", &self.bito())
            .field("gibito", &self.gibito())
            .field("clto", &self.clto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, slave: {=bool:?}, autoack: {=bool:?}, autose: {=bool:?}, autosn: {=bool:?}, arbdis: {=bool:?}, gcamen: {=bool:?}, txbil: {=bool:?}, clhr: {:?}, bito: {:?}, gibito: {=bool:?}, clto: {:?} }}" , self . en () , self . slave () , self . autoack () , self . autose () , self . autosn () , self . arbdis () , self . gcamen () , self . txbil () , self . clhr () , self . bito () , self . gibito () , self . clto ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "START Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Enable."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RSTART Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RSTART Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADDR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADDR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TXC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TXC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TXBL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TXBL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXDATAV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXDATAV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ACK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "NACK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "MSTOP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mstop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MSTOP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ARBLOST Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn arblost(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ARBLOST Interrupt Enable."]
    #[inline(always)]
    pub const fn set_arblost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "BUSERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn buserr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BUSERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_buserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BUSHOLD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bushold(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BUSHOLD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bushold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "RXUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "RXUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BITO Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bito(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "BITO Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bito(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CLTO Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clto(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CLTO Interrupt Enable."]
    #[inline(always)]
    pub const fn set_clto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SSTOP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SSTOP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RXFULL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RXFULL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CLERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CLERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_clerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
            .field("start", &self.start())
            .field("rstart", &self.rstart())
            .field("addr", &self.addr())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("ack", &self.ack())
            .field("nack", &self.nack())
            .field("mstop", &self.mstop())
            .field("arblost", &self.arblost())
            .field("buserr", &self.buserr())
            .field("bushold", &self.bushold())
            .field("txof", &self.txof())
            .field("rxuf", &self.rxuf())
            .field("bito", &self.bito())
            .field("clto", &self.clto())
            .field("sstop", &self.sstop())
            .field("rxfull", &self.rxfull())
            .field("clerr", &self.clerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ start: {=bool:?}, rstart: {=bool:?}, addr: {=bool:?}, txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, ack: {=bool:?}, nack: {=bool:?}, mstop: {=bool:?}, arblost: {=bool:?}, buserr: {=bool:?}, bushold: {=bool:?}, txof: {=bool:?}, rxuf: {=bool:?}, bito: {=bool:?}, clto: {=bool:?}, sstop: {=bool:?}, rxfull: {=bool:?}, clerr: {=bool:?} }}" , self . start () , self . rstart () , self . addr () , self . txc () , self . txbl () , self . rxdatav () , self . ack () , self . nack () , self . mstop () , self . arblost () , self . buserr () , self . bushold () , self . txof () , self . rxuf () , self . bito () , self . clto () , self . sstop () , self . rxfull () , self . clerr ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "START Condition Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "START Condition Interrupt Flag."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Repeated START Condition Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated START Condition Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Interrupt Flag."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer Completed Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit Buffer Level Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Level Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Data Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Acknowledge Received Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge Received Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Not Acknowledge Received Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge Received Interrupt Flag."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Master STOP Condition Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mstop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master STOP Condition Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration Lost Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn arblost(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt Flag."]
    #[inline(always)]
    pub const fn set_arblost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn buserr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_buserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus Held Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bushold(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Held Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bushold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Buffer Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Receive Buffer Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bus Idle Timeout Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bito(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Idle Timeout Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bito(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Clock Low Timeout Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clto(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Low Timeout Interrupt Flag."]
    #[inline(always)]
    pub const fn set_clto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave STOP Condition Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave STOP Condition Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Buffer Full Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Full Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Clock Low Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Low Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_clerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
            .field("start", &self.start())
            .field("rstart", &self.rstart())
            .field("addr", &self.addr())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("ack", &self.ack())
            .field("nack", &self.nack())
            .field("mstop", &self.mstop())
            .field("arblost", &self.arblost())
            .field("buserr", &self.buserr())
            .field("bushold", &self.bushold())
            .field("txof", &self.txof())
            .field("rxuf", &self.rxuf())
            .field("bito", &self.bito())
            .field("clto", &self.clto())
            .field("sstop", &self.sstop())
            .field("rxfull", &self.rxfull())
            .field("clerr", &self.clerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ start: {=bool:?}, rstart: {=bool:?}, addr: {=bool:?}, txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, ack: {=bool:?}, nack: {=bool:?}, mstop: {=bool:?}, arblost: {=bool:?}, buserr: {=bool:?}, bushold: {=bool:?}, txof: {=bool:?}, rxuf: {=bool:?}, bito: {=bool:?}, clto: {=bool:?}, sstop: {=bool:?}, rxfull: {=bool:?}, clerr: {=bool:?} }}" , self . start () , self . rstart () , self . addr () , self . txc () , self . txbl () , self . rxdatav () , self . ack () , self . nack () , self . mstop () , self . arblost () , self . buserr () , self . bushold () , self . txof () , self . rxuf () , self . bito () , self . clto () , self . sstop () , self . rxfull () , self . clerr ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set START Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set START Interrupt Flag."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set RSTART Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set RSTART Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set ADDR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set ADDR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set TXC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set ACK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set ACK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set NACK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set NACK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set MSTOP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mstop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set MSTOP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set ARBLOST Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn arblost(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set ARBLOST Interrupt Flag."]
    #[inline(always)]
    pub const fn set_arblost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set BUSERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn buserr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set BUSERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_buserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set BUSHOLD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bushold(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set BUSHOLD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bushold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set TXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set BITO Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bito(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set BITO Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bito(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set CLTO Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clto(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set CLTO Interrupt Flag."]
    #[inline(always)]
    pub const fn set_clto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set SSTOP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set SSTOP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set RXFULL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set RXFULL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set CLERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set CLERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_clerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
            .field("start", &self.start())
            .field("rstart", &self.rstart())
            .field("addr", &self.addr())
            .field("txc", &self.txc())
            .field("ack", &self.ack())
            .field("nack", &self.nack())
            .field("mstop", &self.mstop())
            .field("arblost", &self.arblost())
            .field("buserr", &self.buserr())
            .field("bushold", &self.bushold())
            .field("txof", &self.txof())
            .field("rxuf", &self.rxuf())
            .field("bito", &self.bito())
            .field("clto", &self.clto())
            .field("sstop", &self.sstop())
            .field("rxfull", &self.rxfull())
            .field("clerr", &self.clerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ start: {=bool:?}, rstart: {=bool:?}, addr: {=bool:?}, txc: {=bool:?}, ack: {=bool:?}, nack: {=bool:?}, mstop: {=bool:?}, arblost: {=bool:?}, buserr: {=bool:?}, bushold: {=bool:?}, txof: {=bool:?}, rxuf: {=bool:?}, bito: {=bool:?}, clto: {=bool:?}, sstop: {=bool:?}, rxfull: {=bool:?}, clerr: {=bool:?} }}" , self . start () , self . rstart () , self . addr () , self . txc () , self . ack () , self . nack () , self . mstop () , self . arblost () , self . buserr () , self . bushold () , self . txof () , self . rxuf () , self . bito () , self . clto () , self . sstop () , self . rxfull () , self . clerr ())
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
    pub const fn sdaloc(&self) -> super::vals::Sdaloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sdaloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_sdaloc(&mut self, val: super::vals::Sdaloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn sclloc(&self) -> super::vals::Sclloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Sclloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_sclloc(&mut self, val: super::vals::Sclloc) {
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
            .field("sdaloc", &self.sdaloc())
            .field("sclloc", &self.sclloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ sdaloc: {:?}, sclloc: {:?} }}",
            self.sdaloc(),
            self.sclloc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "SDA Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdapen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDA Pin Enable."]
    #[inline(always)]
    pub const fn set_sdapen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SCL Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sclpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Pin Enable."]
    #[inline(always)]
    pub const fn set_sclpen(&mut self, val: bool) {
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
            .field("sdapen", &self.sdapen())
            .field("sclpen", &self.sclpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routepen {{ sdapen: {=bool:?}, sclpen: {=bool:?} }}",
            self.sdapen(),
            self.sclpen()
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
#[doc = "Receive Buffer Data Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdatap(pub u32);
impl Rxdatap {
    #[doc = "RX Data Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data Peek."]
    #[inline(always)]
    pub const fn set_rxdatap(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxdatap {
    #[inline(always)]
    fn default() -> Rxdatap {
        Rxdatap(0)
    }
}
impl core::fmt::Debug for Rxdatap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdatap")
            .field("rxdatap", &self.rxdatap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdatap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxdatap {{ rxdatap: {=u8:?} }}", self.rxdatap())
    }
}
#[doc = "Receive Buffer Double Data Register."]
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
#[doc = "Receive Buffer Double Data Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdoublep(pub u32);
impl Rxdoublep {
    #[doc = "RX Data 0 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data 0 Peek."]
    #[inline(always)]
    pub const fn set_rxdatap0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Data 1 Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatap1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX Data 1 Peek."]
    #[inline(always)]
    pub const fn set_rxdatap1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Rxdoublep {
    #[inline(always)]
    fn default() -> Rxdoublep {
        Rxdoublep(0)
    }
}
impl core::fmt::Debug for Rxdoublep {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdoublep")
            .field("rxdatap0", &self.rxdatap0())
            .field("rxdatap1", &self.rxdatap1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdoublep {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxdoublep {{ rxdatap0: {=u8:?}, rxdatap1: {=u8:?} }}",
            self.rxdatap0(),
            self.rxdatap1()
        )
    }
}
#[doc = "Slave Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saddr(pub u32);
impl Saddr {
    #[doc = "Slave Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Saddr {
    #[inline(always)]
    fn default() -> Saddr {
        Saddr(0)
    }
}
impl core::fmt::Debug for Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Saddr").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Saddr {{ addr: {=u8:?} }}", self.addr())
    }
}
#[doc = "Slave Address Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saddrmask(pub u32);
impl Saddrmask {
    #[doc = "Slave Address Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address Mask."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Saddrmask {
    #[inline(always)]
    fn default() -> Saddrmask {
        Saddrmask(0)
    }
}
impl core::fmt::Debug for Saddrmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Saddrmask")
            .field("mask", &self.mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Saddrmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Saddrmask {{ mask: {=u8:?} }}", self.mask())
    }
}
#[doc = "State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "Bus Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Master."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter."]
    #[must_use]
    #[inline(always)]
    pub const fn transmitter(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter."]
    #[inline(always)]
    pub const fn set_transmitter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Nack Received."]
    #[must_use]
    #[inline(always)]
    pub const fn nacked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Nack Received."]
    #[inline(always)]
    pub const fn set_nacked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bus Held."]
    #[must_use]
    #[inline(always)]
    pub const fn bushold(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Held."]
    #[inline(always)]
    pub const fn set_bushold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmission State."]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "Transmission State."]
    #[inline(always)]
    pub const fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("State")
            .field("busy", &self.busy())
            .field("master", &self.master())
            .field("transmitter", &self.transmitter())
            .field("nacked", &self.nacked())
            .field("bushold", &self.bushold())
            .field("state", &self.state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for State {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "State {{ busy: {=bool:?}, master: {=bool:?}, transmitter: {=bool:?}, nacked: {=bool:?}, bushold: {=bool:?}, state: {:?} }}" , self . busy () , self . master () , self . transmitter () , self . nacked () , self . bushold () , self . state ())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Pending START."]
    #[must_use]
    #[inline(always)]
    pub const fn pstart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pending START."]
    #[inline(always)]
    pub const fn set_pstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pending STOP."]
    #[must_use]
    #[inline(always)]
    pub const fn pstop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pending STOP."]
    #[inline(always)]
    pub const fn set_pstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pending ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn pack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pending ACK."]
    #[inline(always)]
    pub const fn set_pack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pending NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn pnack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pending NACK."]
    #[inline(always)]
    pub const fn set_pnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pending Continue."]
    #[must_use]
    #[inline(always)]
    pub const fn pcont(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pending Continue."]
    #[inline(always)]
    pub const fn set_pcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pending Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn pabort(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pending Abort."]
    #[inline(always)]
    pub const fn set_pabort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TX Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TX Complete."]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TX Buffer Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txbl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer Level."]
    #[inline(always)]
    pub const fn set_txbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RX Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdatav(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Valid."]
    #[inline(always)]
    pub const fn set_rxdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RX FIFO Full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Full."]
    #[inline(always)]
    pub const fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("pstart", &self.pstart())
            .field("pstop", &self.pstop())
            .field("pack", &self.pack())
            .field("pnack", &self.pnack())
            .field("pcont", &self.pcont())
            .field("pabort", &self.pabort())
            .field("txc", &self.txc())
            .field("txbl", &self.txbl())
            .field("rxdatav", &self.rxdatav())
            .field("rxfull", &self.rxfull())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ pstart: {=bool:?}, pstop: {=bool:?}, pack: {=bool:?}, pnack: {=bool:?}, pcont: {=bool:?}, pabort: {=bool:?}, txc: {=bool:?}, txbl: {=bool:?}, rxdatav: {=bool:?}, rxfull: {=bool:?} }}" , self . pstart () , self . pstop () , self . pack () , self . pnack () , self . pcont () , self . pabort () , self . txc () , self . txbl () , self . rxdatav () , self . rxfull ())
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
#[doc = "Transmit Buffer Double Data Register."]
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
