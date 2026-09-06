#[doc = "Alignment Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alignerrs(pub u32);
impl Alignerrs {
    #[doc = "Alignment errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Alignment errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Alignerrs {
    #[inline(always)]
    fn default() -> Alignerrs {
        Alignerrs(0)
    }
}
impl core::fmt::Debug for Alignerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Alignerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Alignerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Alignerrs {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Receive DMA Flushed Packets."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoflushedpkts(pub u32);
impl Autoflushedpkts {
    #[doc = "Flushed RX pkts counter."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flushed RX pkts counter."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Autoflushedpkts {
    #[inline(always)]
    fn default() -> Autoflushedpkts {
        Autoflushedpkts(0)
    }
}
impl core::fmt::Debug for Autoflushedpkts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoflushedpkts")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoflushedpkts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Autoflushedpkts {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Carrier Sense Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crserrs(pub u32);
impl Crserrs {
    #[doc = "Carrier sense errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Carrier sense errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Crserrs {
    #[inline(always)]
    fn default() -> Crserrs {
        Crserrs(0)
    }
}
impl core::fmt::Debug for Crserrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crserrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crserrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Crserrs {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Ethernet control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TSU Clock selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn tsuclksel(&self) -> super::vals::Tsuclksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Tsuclksel::from_bits(val as u8)
    }
    #[doc = "TSU Clock selection value."]
    #[inline(always)]
    pub const fn set_tsuclksel(&mut self, val: super::vals::Tsuclksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Clock division factor of TSUPRESC+1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsupresc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock division factor of TSUPRESC+1."]
    #[inline(always)]
    pub const fn set_tsupresc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "MII select signal."]
    #[must_use]
    #[inline(always)]
    pub const fn miisel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MII select signal."]
    #[inline(always)]
    pub const fn set_miisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk."]
    #[must_use]
    #[inline(always)]
    pub const fn gblclken(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk."]
    #[inline(always)]
    pub const fn set_gblclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "REFCLK source select for RMII_TXD and RMII_TX_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn txrefclksel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "REFCLK source select for RMII_TXD and RMII_TX_EN."]
    #[inline(always)]
    pub const fn set_txrefclksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("tsuclksel", &self.tsuclksel())
            .field("tsupresc", &self.tsupresc())
            .field("miisel", &self.miisel())
            .field("gblclken", &self.gblclken())
            .field("txrefclksel", &self.txrefclksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ tsuclksel: {:?}, tsupresc: {=u8:?}, miisel: {=bool:?}, gblclken: {=bool:?}, txrefclksel: {=bool:?} }}" , self . tsuclksel () , self . tsupresc () , self . miisel () , self . gblclken () , self . txrefclksel ())
    }
}
#[doc = "Deferred Transmission Frames."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deferredframes(pub u32);
impl Deferredframes {
    #[doc = "Deferred transmission frames."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Deferred transmission frames."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Deferredframes {
    #[inline(always)]
    fn default() -> Deferredframes {
        Deferredframes(0)
    }
}
impl core::fmt::Debug for Deferredframes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Deferredframes")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Deferredframes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Deferredframes {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "DMA Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacfg(pub u32);
impl Dmacfg {
    #[doc = "Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[must_use]
    #[inline(always)]
    pub const fn ambabrstlen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub const fn set_ambabrstlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Enable header data Splitting."]
    #[must_use]
    #[inline(always)]
    pub const fn hdrdataspliten(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable header data Splitting."]
    #[inline(always)]
    pub const fn set_hdrdataspliten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receiver packet buffer memory size select."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpbufsize(&self) -> super::vals::Rxpbufsize {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Rxpbufsize::from_bits(val as u8)
    }
    #[doc = "Receiver packet buffer memory size select."]
    #[inline(always)]
    pub const fn set_rxpbufsize(&mut self, val: super::vals::Rxpbufsize) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Transmitter packet buffer memory size select."]
    #[must_use]
    #[inline(always)]
    pub const fn txpbufsize(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub const fn set_txpbufsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmitter IP, TCP and UDP checksum generation offload enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txpbuftcpen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter IP, TCP and UDP checksum generation offload enable."]
    #[inline(always)]
    pub const fn set_txpbuftcpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Forces the DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn inflastdbufsizeen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the DMA."]
    #[inline(always)]
    pub const fn set_inflastdbufsizeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA receive buffer size in external AMBA (AHB) system memory."]
    #[must_use]
    #[inline(always)]
    pub const fn rxbufsize(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub const fn set_rxbufsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Auto Discard RX pkts during lack of resource."]
    #[must_use]
    #[inline(always)]
    pub const fn frcdiscardonerr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub const fn set_frcdiscardonerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Force max length bursts on RX."]
    #[must_use]
    #[inline(always)]
    pub const fn frcmaxambabrstrx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Force max length bursts on RX."]
    #[inline(always)]
    pub const fn set_frcmaxambabrstrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Force max length bursts on TX."]
    #[must_use]
    #[inline(always)]
    pub const fn frcmaxambabrsttx(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Force max length bursts on TX."]
    #[inline(always)]
    pub const fn set_frcmaxambabrsttx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable RX extended BD mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rxbdextndmodeen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX extended BD mode."]
    #[inline(always)]
    pub const fn set_rxbdextndmodeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable TX extended BD mode."]
    #[must_use]
    #[inline(always)]
    pub const fn txbdextendmodeen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX extended BD mode."]
    #[inline(always)]
    pub const fn set_txbdextendmodeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Dmacfg {
    #[inline(always)]
    fn default() -> Dmacfg {
        Dmacfg(0)
    }
}
impl core::fmt::Debug for Dmacfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmacfg")
            .field("ambabrstlen", &self.ambabrstlen())
            .field("hdrdataspliten", &self.hdrdataspliten())
            .field("rxpbufsize", &self.rxpbufsize())
            .field("txpbufsize", &self.txpbufsize())
            .field("txpbuftcpen", &self.txpbuftcpen())
            .field("inflastdbufsizeen", &self.inflastdbufsizeen())
            .field("rxbufsize", &self.rxbufsize())
            .field("frcdiscardonerr", &self.frcdiscardonerr())
            .field("frcmaxambabrstrx", &self.frcmaxambabrstrx())
            .field("frcmaxambabrsttx", &self.frcmaxambabrsttx())
            .field("rxbdextndmodeen", &self.rxbdextndmodeen())
            .field("txbdextendmodeen", &self.txbdextendmodeen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmacfg {{ ambabrstlen: {=u8:?}, hdrdataspliten: {=bool:?}, rxpbufsize: {:?}, txpbufsize: {=bool:?}, txpbuftcpen: {=bool:?}, inflastdbufsizeen: {=bool:?}, rxbufsize: {=u8:?}, frcdiscardonerr: {=bool:?}, frcmaxambabrstrx: {=bool:?}, frcmaxambabrsttx: {=bool:?}, rxbdextndmodeen: {=bool:?}, txbdextendmodeen: {=bool:?} }}" , self . ambabrstlen () , self . hdrdataspliten () , self . rxpbufsize () , self . txpbufsize () , self . txpbuftcpen () , self . inflastdbufsizeen () , self . rxbufsize () , self . frcdiscardonerr () , self . frcmaxambabrstrx () , self . frcmaxambabrsttx () , self . rxbdextndmodeen () , self . txbdextendmodeen ())
    }
}
#[doc = "Excessive Collisions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Excesscols(pub u32);
impl Excesscols {
    #[doc = "Excessive collisions."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Excessive collisions."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Excesscols {
    #[inline(always)]
    fn default() -> Excesscols {
        Excesscols(0)
    }
}
impl core::fmt::Debug for Excesscols {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Excesscols")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Excesscols {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Excesscols {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Oversize Frames Received."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Excessiverxlen(pub u32);
impl Excessiverxlen {
    #[doc = "Oversize frames received."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Oversize frames received."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Excessiverxlen {
    #[inline(always)]
    fn default() -> Excessiverxlen {
        Excessiverxlen(0)
    }
}
impl core::fmt::Debug for Excessiverxlen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Excessiverxlen")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Excessiverxlen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Excessiverxlen {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frame Check Sequence Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcserrs(pub u32);
impl Fcserrs {
    #[doc = "Frame check sequence errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Frame check sequence errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Fcserrs {
    #[inline(always)]
    fn default() -> Fcserrs {
        Fcserrs(0)
    }
}
impl core::fmt::Debug for Fcserrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcserrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcserrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcserrs {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Interrupt Disable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienc(pub u32);
impl Ienc {
    #[doc = "Disable management done interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mngmntdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Disable management done interrupt."]
    #[inline(always)]
    pub const fn set_mngmntdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable receive complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcmplt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disable receive complete interrupt."]
    #[inline(always)]
    pub const fn set_rxcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable receive used bit read interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxusedbitread(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disable receive used bit read interrupt."]
    #[inline(always)]
    pub const fn set_rxusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable transmit used bit read interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txusedbitread(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable transmit used bit read interrupt."]
    #[inline(always)]
    pub const fn set_txusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable transmit buffer under run interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txunderrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable transmit buffer under run interrupt."]
    #[inline(always)]
    pub const fn set_txunderrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Disable retry limit exceeded or late collision interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrylmtorlatecol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Disable retry limit exceeded or late collision interrupt."]
    #[inline(always)]
    pub const fn set_rtrylmtorlatecol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable transmit frame corruption due to AMBA (AHB) error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ambaerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable transmit frame corruption due to AMBA (AHB) error interrupt."]
    #[inline(always)]
    pub const fn set_ambaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Disable transmit complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmplt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disable transmit complete interrupt."]
    #[inline(always)]
    pub const fn set_txcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Disable receive overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoverrun(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Disable receive overrun interrupt."]
    #[inline(always)]
    pub const fn set_rxoverrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Disable bresp/hresp not OK interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Disable bresp/hresp not OK interrupt."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Disable pause frame with non-zero pause quantum interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nonzeropfrmquant(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Disable pause frame with non-zero pause quantum interrupt."]
    #[inline(always)]
    pub const fn set_nonzeropfrmquant(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Disable pause time zero interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pausetimezero(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Disable pause time zero interrupt."]
    #[inline(always)]
    pub const fn set_pausetimezero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Disable pause frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pfrmtx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Disable pause frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_pfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Disable PTP delay_req frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP delay_req frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Disable PTP sync frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmrx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP sync frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Disable PTP delay_req frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP delay_req frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Disable PTP sync frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmtx(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP sync frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Disable PTP pdelay_req frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP pdelay_req frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Disable PTP pdelay_resp frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP pdelay_resp frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Disable PTP pdelay_req frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP pdelay_req frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Disable PTP pdelay_resp frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmtx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Disable PTP pdelay_resp frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Disable TSU seconds register increment interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tsusecregincr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Disable TSU seconds register increment interrupt."]
    #[inline(always)]
    pub const fn set_tsusecregincr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Disable RX LPI indication interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlpiindc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Disable RX LPI indication interrupt."]
    #[inline(always)]
    pub const fn set_rxlpiindc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Disable WOL event received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wolevntrx(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Disable WOL event received interrupt."]
    #[inline(always)]
    pub const fn set_wolevntrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Disable TSU timer comparison interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutimercomp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Disable TSU timer comparison interrupt."]
    #[inline(always)]
    pub const fn set_tsutimercomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ienc {
    #[inline(always)]
    fn default() -> Ienc {
        Ienc(0)
    }
}
impl core::fmt::Debug for Ienc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienc")
            .field("mngmntdone", &self.mngmntdone())
            .field("rxcmplt", &self.rxcmplt())
            .field("rxusedbitread", &self.rxusedbitread())
            .field("txusedbitread", &self.txusedbitread())
            .field("txunderrun", &self.txunderrun())
            .field("rtrylmtorlatecol", &self.rtrylmtorlatecol())
            .field("ambaerr", &self.ambaerr())
            .field("txcmplt", &self.txcmplt())
            .field("rxoverrun", &self.rxoverrun())
            .field("respnotok", &self.respnotok())
            .field("nonzeropfrmquant", &self.nonzeropfrmquant())
            .field("pausetimezero", &self.pausetimezero())
            .field("pfrmtx", &self.pfrmtx())
            .field("ptpdlyreqfrmrx", &self.ptpdlyreqfrmrx())
            .field("ptpsyncfrmrx", &self.ptpsyncfrmrx())
            .field("ptpdlyreqfrmtx", &self.ptpdlyreqfrmtx())
            .field("ptpsyncfrmtx", &self.ptpsyncfrmtx())
            .field("ptppdlyreqfrmrx", &self.ptppdlyreqfrmrx())
            .field("ptppdlyrespfrmrx", &self.ptppdlyrespfrmrx())
            .field("ptppdlyreqfrmtx", &self.ptppdlyreqfrmtx())
            .field("ptppdlyrespfrmtx", &self.ptppdlyrespfrmtx())
            .field("tsusecregincr", &self.tsusecregincr())
            .field("rxlpiindc", &self.rxlpiindc())
            .field("wolevntrx", &self.wolevntrx())
            .field("tsutimercomp", &self.tsutimercomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ienc {{ mngmntdone: {=bool:?}, rxcmplt: {=bool:?}, rxusedbitread: {=bool:?}, txusedbitread: {=bool:?}, txunderrun: {=bool:?}, rtrylmtorlatecol: {=bool:?}, ambaerr: {=bool:?}, txcmplt: {=bool:?}, rxoverrun: {=bool:?}, respnotok: {=bool:?}, nonzeropfrmquant: {=bool:?}, pausetimezero: {=bool:?}, pfrmtx: {=bool:?}, ptpdlyreqfrmrx: {=bool:?}, ptpsyncfrmrx: {=bool:?}, ptpdlyreqfrmtx: {=bool:?}, ptpsyncfrmtx: {=bool:?}, ptppdlyreqfrmrx: {=bool:?}, ptppdlyrespfrmrx: {=bool:?}, ptppdlyreqfrmtx: {=bool:?}, ptppdlyrespfrmtx: {=bool:?}, tsusecregincr: {=bool:?}, rxlpiindc: {=bool:?}, wolevntrx: {=bool:?}, tsutimercomp: {=bool:?} }}" , self . mngmntdone () , self . rxcmplt () , self . rxusedbitread () , self . txusedbitread () , self . txunderrun () , self . rtrylmtorlatecol () , self . ambaerr () , self . txcmplt () , self . rxoverrun () , self . respnotok () , self . nonzeropfrmquant () , self . pausetimezero () , self . pfrmtx () , self . ptpdlyreqfrmrx () , self . ptpsyncfrmrx () , self . ptpdlyreqfrmtx () , self . ptpsyncfrmtx () , self . ptppdlyreqfrmrx () , self . ptppdlyrespfrmrx () , self . ptppdlyreqfrmtx () , self . ptppdlyrespfrmtx () , self . tsusecregincr () , self . rxlpiindc () , self . wolevntrx () , self . tsutimercomp ())
    }
}
#[doc = "Interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienro(pub u32);
impl Ienro {
    #[doc = "management done interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mngmntdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "management done interrupt mask."]
    #[inline(always)]
    pub const fn set_mngmntdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "receive complete interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcmplt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "receive complete interrupt mask."]
    #[inline(always)]
    pub const fn set_rxcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "receive used bit read interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxusedbitread(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "receive used bit read interrupt mask."]
    #[inline(always)]
    pub const fn set_rxusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "transmit used bit read interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txusedbitread(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "transmit used bit read interrupt mask."]
    #[inline(always)]
    pub const fn set_txusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "transmit buffer under run interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txunderrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "transmit buffer under run interrupt mask."]
    #[inline(always)]
    pub const fn set_txunderrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Retry limit exceeded or late collision (gigabit mode only) interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrylmtorlatecol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Retry limit exceeded or late collision (gigabit mode only) interrupt mask."]
    #[inline(always)]
    pub const fn set_rtrylmtorlatecol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) error interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ambaerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) error interrupt mask."]
    #[inline(always)]
    pub const fn set_ambaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit complete interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmplt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete interrupt mask."]
    #[inline(always)]
    pub const fn set_txcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Unused."]
    #[must_use]
    #[inline(always)]
    pub const fn unused(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Unused."]
    #[inline(always)]
    pub const fn set_unused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive overrun interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoverrun(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun interrupt mask."]
    #[inline(always)]
    pub const fn set_rxoverrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "bresp/hresp not OK interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "bresp/hresp not OK interrupt mask."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pause frame with non-zero pause quantum interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn nonzeropfrmquant(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pause frame with non-zero pause quantum interrupt mask."]
    #[inline(always)]
    pub const fn set_nonzeropfrmquant(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "pause time zero interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn pausetimezero(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "pause time zero interrupt mask."]
    #[inline(always)]
    pub const fn set_pausetimezero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "pause frame transmitted interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn pfrmtx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "pause frame transmitted interrupt mask."]
    #[inline(always)]
    pub const fn set_pfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PTP delay_req frame received mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PTP delay_req frame received mask."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PTP sync frame received mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmrx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PTP sync frame received mask."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PTP delay_req frame transmitted mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PTP delay_req frame transmitted mask."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "PTP sync frame transmitted mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmtx(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "PTP sync frame transmitted mask."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PTP pdelay_req frame received mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_req frame received mask."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PTP pdelay_resp frame received mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_resp frame received mask."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PTP pdelay_req frame transmitted mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_req frame transmitted mask."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PTP pdelay_resp frame transmitted mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmtx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_resp frame transmitted mask."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TSU seconds register increment mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tsusecregincr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TSU seconds register increment mask."]
    #[inline(always)]
    pub const fn set_tsusecregincr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "RX LPI indication mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlpiindc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "RX LPI indication mask."]
    #[inline(always)]
    pub const fn set_rxlpiindc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "WOL event received mask."]
    #[must_use]
    #[inline(always)]
    pub const fn wolevntrx(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "WOL event received mask."]
    #[inline(always)]
    pub const fn set_wolevntrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TSU timer comparison interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutimercomp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TSU timer comparison interrupt mask."]
    #[inline(always)]
    pub const fn set_tsutimercomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ienro {
    #[inline(always)]
    fn default() -> Ienro {
        Ienro(0)
    }
}
impl core::fmt::Debug for Ienro {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienro")
            .field("mngmntdone", &self.mngmntdone())
            .field("rxcmplt", &self.rxcmplt())
            .field("rxusedbitread", &self.rxusedbitread())
            .field("txusedbitread", &self.txusedbitread())
            .field("txunderrun", &self.txunderrun())
            .field("rtrylmtorlatecol", &self.rtrylmtorlatecol())
            .field("ambaerr", &self.ambaerr())
            .field("txcmplt", &self.txcmplt())
            .field("unused", &self.unused())
            .field("rxoverrun", &self.rxoverrun())
            .field("respnotok", &self.respnotok())
            .field("nonzeropfrmquant", &self.nonzeropfrmquant())
            .field("pausetimezero", &self.pausetimezero())
            .field("pfrmtx", &self.pfrmtx())
            .field("ptpdlyreqfrmrx", &self.ptpdlyreqfrmrx())
            .field("ptpsyncfrmrx", &self.ptpsyncfrmrx())
            .field("ptpdlyreqfrmtx", &self.ptpdlyreqfrmtx())
            .field("ptpsyncfrmtx", &self.ptpsyncfrmtx())
            .field("ptppdlyreqfrmrx", &self.ptppdlyreqfrmrx())
            .field("ptppdlyrespfrmrx", &self.ptppdlyrespfrmrx())
            .field("ptppdlyreqfrmtx", &self.ptppdlyreqfrmtx())
            .field("ptppdlyrespfrmtx", &self.ptppdlyrespfrmtx())
            .field("tsusecregincr", &self.tsusecregincr())
            .field("rxlpiindc", &self.rxlpiindc())
            .field("wolevntrx", &self.wolevntrx())
            .field("tsutimercomp", &self.tsutimercomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienro {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ienro {{ mngmntdone: {=bool:?}, rxcmplt: {=bool:?}, rxusedbitread: {=bool:?}, txusedbitread: {=bool:?}, txunderrun: {=bool:?}, rtrylmtorlatecol: {=bool:?}, ambaerr: {=bool:?}, txcmplt: {=bool:?}, unused: {=bool:?}, rxoverrun: {=bool:?}, respnotok: {=bool:?}, nonzeropfrmquant: {=bool:?}, pausetimezero: {=bool:?}, pfrmtx: {=bool:?}, ptpdlyreqfrmrx: {=bool:?}, ptpsyncfrmrx: {=bool:?}, ptpdlyreqfrmtx: {=bool:?}, ptpsyncfrmtx: {=bool:?}, ptppdlyreqfrmrx: {=bool:?}, ptppdlyrespfrmrx: {=bool:?}, ptppdlyreqfrmtx: {=bool:?}, ptppdlyrespfrmtx: {=bool:?}, tsusecregincr: {=bool:?}, rxlpiindc: {=bool:?}, wolevntrx: {=bool:?}, tsutimercomp: {=bool:?} }}" , self . mngmntdone () , self . rxcmplt () , self . rxusedbitread () , self . txusedbitread () , self . txunderrun () , self . rtrylmtorlatecol () , self . ambaerr () , self . txcmplt () , self . unused () , self . rxoverrun () , self . respnotok () , self . nonzeropfrmquant () , self . pausetimezero () , self . pfrmtx () , self . ptpdlyreqfrmrx () , self . ptpsyncfrmrx () , self . ptpdlyreqfrmtx () , self . ptpsyncfrmtx () , self . ptppdlyreqfrmrx () , self . ptppdlyrespfrmrx () , self . ptppdlyreqfrmtx () , self . ptppdlyrespfrmtx () , self . tsusecregincr () , self . rxlpiindc () , self . wolevntrx () , self . tsutimercomp ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iens(pub u32);
impl Iens {
    #[doc = "Enable management done interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mngmntdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable management done interrupt."]
    #[inline(always)]
    pub const fn set_mngmntdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable receive complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcmplt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable receive complete interrupt."]
    #[inline(always)]
    pub const fn set_rxcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable receive used bit read interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxusedbitread(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable receive used bit read interrupt."]
    #[inline(always)]
    pub const fn set_rxusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable transmit used bit read interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txusedbitread(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit used bit read interrupt."]
    #[inline(always)]
    pub const fn set_txusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable transmit buffer under run interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txunderrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit buffer under run interrupt."]
    #[inline(always)]
    pub const fn set_txunderrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable retry limit exceeded or late collision interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrylmtorlatecol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable retry limit exceeded or late collision interrupt."]
    #[inline(always)]
    pub const fn set_rtrylmtorlatecol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable transmit frame corruption due to AMBA (AHB) error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ambaerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit frame corruption due to AMBA (AHB) error interrupt."]
    #[inline(always)]
    pub const fn set_ambaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable transmit complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmplt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit complete interrupt."]
    #[inline(always)]
    pub const fn set_txcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable receive overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoverrun(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable receive overrun interrupt."]
    #[inline(always)]
    pub const fn set_rxoverrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable bresp/hresp not OK interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable bresp/hresp not OK interrupt."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable pause frame with non-zero pause quantum interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nonzeropfrmquant(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pause frame with non-zero pause quantum interrupt."]
    #[inline(always)]
    pub const fn set_nonzeropfrmquant(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable pause time zero interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pausetimezero(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pause time zero interrupt."]
    #[inline(always)]
    pub const fn set_pausetimezero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable pause frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pfrmtx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pause frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_pfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable PTP delay_req frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP delay_req frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable PTP sync frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmrx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP sync frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable PTP delay_req frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP delay_req frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PTP sync frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmtx(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP sync frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable PTP pdelay_req frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP pdelay_req frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable PTP pdelay_resp frame received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP pdelay_resp frame received interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable PTP pdelay_req frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP pdelay_req frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable PTP pdelay_resp frame transmitted interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmtx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP pdelay_resp frame transmitted interrupt."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable TSU seconds register increment interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tsusecregincr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TSU seconds register increment interrupt."]
    #[inline(always)]
    pub const fn set_tsusecregincr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable RX LPI indication interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlpiindc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX LPI indication interrupt."]
    #[inline(always)]
    pub const fn set_rxlpiindc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable WOL event received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wolevntrx(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WOL event received interrupt."]
    #[inline(always)]
    pub const fn set_wolevntrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable TSU timer comparison interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutimercomp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TSU timer comparison interrupt."]
    #[inline(always)]
    pub const fn set_tsutimercomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Iens {
    #[inline(always)]
    fn default() -> Iens {
        Iens(0)
    }
}
impl core::fmt::Debug for Iens {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iens")
            .field("mngmntdone", &self.mngmntdone())
            .field("rxcmplt", &self.rxcmplt())
            .field("rxusedbitread", &self.rxusedbitread())
            .field("txusedbitread", &self.txusedbitread())
            .field("txunderrun", &self.txunderrun())
            .field("rtrylmtorlatecol", &self.rtrylmtorlatecol())
            .field("ambaerr", &self.ambaerr())
            .field("txcmplt", &self.txcmplt())
            .field("rxoverrun", &self.rxoverrun())
            .field("respnotok", &self.respnotok())
            .field("nonzeropfrmquant", &self.nonzeropfrmquant())
            .field("pausetimezero", &self.pausetimezero())
            .field("pfrmtx", &self.pfrmtx())
            .field("ptpdlyreqfrmrx", &self.ptpdlyreqfrmrx())
            .field("ptpsyncfrmrx", &self.ptpsyncfrmrx())
            .field("ptpdlyreqfrmtx", &self.ptpdlyreqfrmtx())
            .field("ptpsyncfrmtx", &self.ptpsyncfrmtx())
            .field("ptppdlyreqfrmrx", &self.ptppdlyreqfrmrx())
            .field("ptppdlyrespfrmrx", &self.ptppdlyrespfrmrx())
            .field("ptppdlyreqfrmtx", &self.ptppdlyreqfrmtx())
            .field("ptppdlyrespfrmtx", &self.ptppdlyrespfrmtx())
            .field("tsusecregincr", &self.tsusecregincr())
            .field("rxlpiindc", &self.rxlpiindc())
            .field("wolevntrx", &self.wolevntrx())
            .field("tsutimercomp", &self.tsutimercomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iens {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Iens {{ mngmntdone: {=bool:?}, rxcmplt: {=bool:?}, rxusedbitread: {=bool:?}, txusedbitread: {=bool:?}, txunderrun: {=bool:?}, rtrylmtorlatecol: {=bool:?}, ambaerr: {=bool:?}, txcmplt: {=bool:?}, rxoverrun: {=bool:?}, respnotok: {=bool:?}, nonzeropfrmquant: {=bool:?}, pausetimezero: {=bool:?}, pfrmtx: {=bool:?}, ptpdlyreqfrmrx: {=bool:?}, ptpsyncfrmrx: {=bool:?}, ptpdlyreqfrmtx: {=bool:?}, ptpsyncfrmtx: {=bool:?}, ptppdlyreqfrmrx: {=bool:?}, ptppdlyrespfrmrx: {=bool:?}, ptppdlyreqfrmtx: {=bool:?}, ptppdlyrespfrmtx: {=bool:?}, tsusecregincr: {=bool:?}, rxlpiindc: {=bool:?}, wolevntrx: {=bool:?}, tsutimercomp: {=bool:?} }}" , self . mngmntdone () , self . rxcmplt () , self . rxusedbitread () , self . txusedbitread () , self . txunderrun () , self . rtrylmtorlatecol () , self . ambaerr () , self . txcmplt () , self . rxoverrun () , self . respnotok () , self . nonzeropfrmquant () , self . pausetimezero () , self . pfrmtx () , self . ptpdlyreqfrmrx () , self . ptpsyncfrmrx () , self . ptpdlyreqfrmtx () , self . ptpsyncfrmtx () , self . ptppdlyreqfrmrx () , self . ptppdlyrespfrmrx () , self . ptppdlyreqfrmtx () , self . ptppdlyrespfrmtx () , self . tsusecregincr () , self . rxlpiindc () , self . wolevntrx () , self . tsutimercomp ())
    }
}
#[doc = "Interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "Management frame sent."]
    #[must_use]
    #[inline(always)]
    pub const fn mngmntdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Management frame sent."]
    #[inline(always)]
    pub const fn set_mngmntdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive complete."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcmplt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive complete."]
    #[inline(always)]
    pub const fn set_rxcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX used bit read."]
    #[must_use]
    #[inline(always)]
    pub const fn rxusedbitread(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RX used bit read."]
    #[inline(always)]
    pub const fn set_rxusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TX used bit read."]
    #[must_use]
    #[inline(always)]
    pub const fn txusedbitread(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TX used bit read."]
    #[inline(always)]
    pub const fn set_txusedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit under run."]
    #[must_use]
    #[inline(always)]
    pub const fn txunderrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit under run."]
    #[inline(always)]
    pub const fn set_txunderrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Retry limit exceeded or late collision."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrylmtorlatecol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Retry limit exceeded or late collision."]
    #[inline(always)]
    pub const fn set_rtrylmtorlatecol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) error."]
    #[must_use]
    #[inline(always)]
    pub const fn ambaerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub const fn set_ambaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit complete."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmplt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete."]
    #[inline(always)]
    pub const fn set_txcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive overrun."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoverrun(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun."]
    #[inline(always)]
    pub const fn set_rxoverrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Hresp not OK."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Hresp not OK."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pause frame with non-zero pause quantum received."]
    #[must_use]
    #[inline(always)]
    pub const fn nonzeropfrmquant(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pause frame with non-zero pause quantum received."]
    #[inline(always)]
    pub const fn set_nonzeropfrmquant(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pause Time zero."]
    #[must_use]
    #[inline(always)]
    pub const fn pausetimezero(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pause Time zero."]
    #[inline(always)]
    pub const fn set_pausetimezero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pause frame transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn pfrmtx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Pause frame transmitted."]
    #[inline(always)]
    pub const fn set_pfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PTP delay_req frame received."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PTP delay_req frame received."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PTP sync frame received."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmrx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PTP sync frame received."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PTP delay_req frame transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PTP delay_req frame transmitted."]
    #[inline(always)]
    pub const fn set_ptpdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "PTP sync frame transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpsyncfrmtx(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "PTP sync frame transmitted."]
    #[inline(always)]
    pub const fn set_ptpsyncfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PTP pdelay_req frame received."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmrx(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_req frame received."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PTP pdelay_resp frame received."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_resp frame received."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PTP pdelay_req frame transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyreqfrmtx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_req frame transmitted."]
    #[inline(always)]
    pub const fn set_ptppdlyreqfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PTP pdelay_resp frame transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn ptppdlyrespfrmtx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PTP pdelay_resp frame transmitted."]
    #[inline(always)]
    pub const fn set_ptppdlyrespfrmtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TSU seconds register increment."]
    #[must_use]
    #[inline(always)]
    pub const fn tsusecregincr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TSU seconds register increment."]
    #[inline(always)]
    pub const fn set_tsusecregincr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive LPI indication status bit change."]
    #[must_use]
    #[inline(always)]
    pub const fn rxlpiindc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Receive LPI indication status bit change."]
    #[inline(always)]
    pub const fn set_rxlpiindc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "WOL event received interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wolevntrx(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "WOL event received interrupt."]
    #[inline(always)]
    pub const fn set_wolevntrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TSU timer comparison interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutimercomp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TSU timer comparison interrupt."]
    #[inline(always)]
    pub const fn set_tsutimercomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ifcr {
    #[inline(always)]
    fn default() -> Ifcr {
        Ifcr(0)
    }
}
impl core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifcr")
            .field("mngmntdone", &self.mngmntdone())
            .field("rxcmplt", &self.rxcmplt())
            .field("rxusedbitread", &self.rxusedbitread())
            .field("txusedbitread", &self.txusedbitread())
            .field("txunderrun", &self.txunderrun())
            .field("rtrylmtorlatecol", &self.rtrylmtorlatecol())
            .field("ambaerr", &self.ambaerr())
            .field("txcmplt", &self.txcmplt())
            .field("rxoverrun", &self.rxoverrun())
            .field("respnotok", &self.respnotok())
            .field("nonzeropfrmquant", &self.nonzeropfrmquant())
            .field("pausetimezero", &self.pausetimezero())
            .field("pfrmtx", &self.pfrmtx())
            .field("ptpdlyreqfrmrx", &self.ptpdlyreqfrmrx())
            .field("ptpsyncfrmrx", &self.ptpsyncfrmrx())
            .field("ptpdlyreqfrmtx", &self.ptpdlyreqfrmtx())
            .field("ptpsyncfrmtx", &self.ptpsyncfrmtx())
            .field("ptppdlyreqfrmrx", &self.ptppdlyreqfrmrx())
            .field("ptppdlyrespfrmrx", &self.ptppdlyrespfrmrx())
            .field("ptppdlyreqfrmtx", &self.ptppdlyreqfrmtx())
            .field("ptppdlyrespfrmtx", &self.ptppdlyrespfrmtx())
            .field("tsusecregincr", &self.tsusecregincr())
            .field("rxlpiindc", &self.rxlpiindc())
            .field("wolevntrx", &self.wolevntrx())
            .field("tsutimercomp", &self.tsutimercomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifcr {{ mngmntdone: {=bool:?}, rxcmplt: {=bool:?}, rxusedbitread: {=bool:?}, txusedbitread: {=bool:?}, txunderrun: {=bool:?}, rtrylmtorlatecol: {=bool:?}, ambaerr: {=bool:?}, txcmplt: {=bool:?}, rxoverrun: {=bool:?}, respnotok: {=bool:?}, nonzeropfrmquant: {=bool:?}, pausetimezero: {=bool:?}, pfrmtx: {=bool:?}, ptpdlyreqfrmrx: {=bool:?}, ptpsyncfrmrx: {=bool:?}, ptpdlyreqfrmtx: {=bool:?}, ptpsyncfrmtx: {=bool:?}, ptppdlyreqfrmrx: {=bool:?}, ptppdlyrespfrmrx: {=bool:?}, ptppdlyreqfrmtx: {=bool:?}, ptppdlyrespfrmtx: {=bool:?}, tsusecregincr: {=bool:?}, rxlpiindc: {=bool:?}, wolevntrx: {=bool:?}, tsutimercomp: {=bool:?} }}" , self . mngmntdone () , self . rxcmplt () , self . rxusedbitread () , self . txusedbitread () , self . txunderrun () , self . rtrylmtorlatecol () , self . ambaerr () , self . txcmplt () , self . rxoverrun () , self . respnotok () , self . nonzeropfrmquant () , self . pausetimezero () , self . pfrmtx () , self . ptpdlyreqfrmrx () , self . ptpsyncfrmrx () , self . ptpdlyreqfrmtx () , self . ptpsyncfrmtx () , self . ptppdlyreqfrmrx () , self . ptppdlyrespfrmrx () , self . ptppdlyreqfrmtx () , self . ptppdlyrespfrmtx () , self . tsusecregincr () , self . rxlpiindc () , self . wolevntrx () , self . tsutimercomp ())
    }
}
#[doc = "Interrupt moderation register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imod(pub u32);
impl Imod {
    #[doc = "Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received."]
    #[must_use]
    #[inline(always)]
    pub const fn rxintmod(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received."]
    #[inline(always)]
    pub const fn set_rxintmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn txintmod(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted."]
    #[inline(always)]
    pub const fn set_txintmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Imod {
    #[inline(always)]
    fn default() -> Imod {
        Imod(0)
    }
}
impl core::fmt::Debug for Imod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imod")
            .field("rxintmod", &self.rxintmod())
            .field("txintmod", &self.txintmod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imod {{ rxintmod: {=u8:?}, txintmod: {=u8:?} }}",
            self.rxintmod(),
            self.txintmod()
        )
    }
}
#[doc = "Maximum Jumbo Frame Size."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jumbomaxlen(pub u32);
impl Jumbomaxlen {
    #[doc = "Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[must_use]
    #[inline(always)]
    pub const fn jumbomaxlen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub const fn set_jumbomaxlen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Jumbomaxlen {
    #[inline(always)]
    fn default() -> Jumbomaxlen {
        Jumbomaxlen(0)
    }
}
impl core::fmt::Debug for Jumbomaxlen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Jumbomaxlen")
            .field("jumbomaxlen", &self.jumbomaxlen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Jumbomaxlen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Jumbomaxlen {{ jumbomaxlen: {=u16:?} }}",
            self.jumbomaxlen()
        )
    }
}
#[doc = "Late Collisions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latecols(pub u32);
impl Latecols {
    #[doc = "Late collisions."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Late collisions."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Latecols {
    #[inline(always)]
    fn default() -> Latecols {
        Latecols(0)
    }
}
impl core::fmt::Debug for Latecols {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Latecols")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latecols {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Latecols {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Specific Address Mask 1 Top 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maskadd1top(pub u32);
impl Maskadd1top {
    #[doc = "Specific Address Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn addrmask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specific Address Mask."]
    #[inline(always)]
    pub const fn set_addrmask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Maskadd1top {
    #[inline(always)]
    fn default() -> Maskadd1top {
        Maskadd1top(0)
    }
}
impl core::fmt::Debug for Maskadd1top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Maskadd1top")
            .field("addrmask", &self.addrmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Maskadd1top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Maskadd1top {{ addrmask: {=u16:?} }}", self.addrmask())
    }
}
#[doc = "Multiple Collision Frames."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Multicols(pub u32);
impl Multicols {
    #[doc = "Multiple collision frames."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Multiple collision frames."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Multicols {
    #[inline(always)]
    fn default() -> Multicols {
        Multicols(0)
    }
}
impl core::fmt::Debug for Multicols {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Multicols")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Multicols {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Multicols {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Network configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Networkcfg(pub u32);
impl Networkcfg {
    #[doc = "Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Speed."]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Full duplex."]
    #[must_use]
    #[inline(always)]
    pub const fn fullduplex(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Full duplex."]
    #[inline(always)]
    pub const fn set_fullduplex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Discard non-VLAN frames."]
    #[must_use]
    #[inline(always)]
    pub const fn discrdnonvlanframes(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Discard non-VLAN frames."]
    #[inline(always)]
    pub const fn set_discrdnonvlanframes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Jumbo frames enable."]
    #[must_use]
    #[inline(always)]
    pub const fn jumboframes(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Jumbo frames enable."]
    #[inline(always)]
    pub const fn set_jumboframes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Copy all frames."]
    #[must_use]
    #[inline(always)]
    pub const fn copyallframes(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Copy all frames."]
    #[inline(always)]
    pub const fn set_copyallframes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "No broadcast."]
    #[must_use]
    #[inline(always)]
    pub const fn nobroadcast(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "No broadcast."]
    #[inline(always)]
    pub const fn set_nobroadcast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Multicast hash enable."]
    #[must_use]
    #[inline(always)]
    pub const fn multicasthashen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Multicast hash enable."]
    #[inline(always)]
    pub const fn set_multicasthashen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Unicast hash enable."]
    #[must_use]
    #[inline(always)]
    pub const fn unicasthashen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast hash enable."]
    #[inline(always)]
    pub const fn set_unicasthashen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive 1536 byte frames."]
    #[must_use]
    #[inline(always)]
    pub const fn rx1536byteframes(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive 1536 byte frames."]
    #[inline(always)]
    pub const fn set_rx1536byteframes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Retry test."]
    #[must_use]
    #[inline(always)]
    pub const fn retrytest(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Retry test."]
    #[inline(always)]
    pub const fn set_retrytest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pause enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pauseen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pause enable."]
    #[inline(always)]
    pub const fn set_pauseen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive buffer offset."]
    #[must_use]
    #[inline(always)]
    pub const fn rxbuffoffset(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Receive buffer offset."]
    #[inline(always)]
    pub const fn set_rxbuffoffset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Length field error frame discard."]
    #[must_use]
    #[inline(always)]
    pub const fn lenfielderrfrmdiscrd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Length field error frame discard."]
    #[inline(always)]
    pub const fn set_lenfielderrfrmdiscrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FCS remove."]
    #[must_use]
    #[inline(always)]
    pub const fn fcsremove(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FCS remove."]
    #[inline(always)]
    pub const fn set_fcsremove(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "MDC clock division."]
    #[must_use]
    #[inline(always)]
    pub const fn mdcclkdiv(&self) -> super::vals::Mdcclkdiv {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::Mdcclkdiv::from_bits(val as u8)
    }
    #[doc = "MDC clock division."]
    #[inline(always)]
    pub const fn set_mdcclkdiv(&mut self, val: super::vals::Mdcclkdiv) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Disable copy of pause frames."]
    #[must_use]
    #[inline(always)]
    pub const fn discopyofpframes(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Disable copy of pause frames."]
    #[inline(always)]
    pub const fn set_discopyofpframes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Receive checksum offload enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxchksumoffloaden(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive checksum offload enable."]
    #[inline(always)]
    pub const fn set_rxchksumoffloaden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable frames to be received in half-duplex mode while transmitting."]
    #[must_use]
    #[inline(always)]
    pub const fn enhalfduplexrx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub const fn set_enhalfduplexrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Ignore RX FCS."]
    #[must_use]
    #[inline(always)]
    pub const fn ignorerxfcs(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore RX FCS."]
    #[inline(always)]
    pub const fn set_ignorerxfcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "IPG stretch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgstrtchen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "IPG stretch enable."]
    #[inline(always)]
    pub const fn set_ipgstrtchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Receive bad preamble."]
    #[must_use]
    #[inline(always)]
    pub const fn nspchange(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Receive bad preamble."]
    #[inline(always)]
    pub const fn set_nspchange(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Ignore IPG rx_er."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoreipgrxer(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore IPG rx_er."]
    #[inline(always)]
    pub const fn set_ignoreipgrxer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Networkcfg {
    #[inline(always)]
    fn default() -> Networkcfg {
        Networkcfg(0)
    }
}
impl core::fmt::Debug for Networkcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Networkcfg")
            .field("speed", &self.speed())
            .field("fullduplex", &self.fullduplex())
            .field("discrdnonvlanframes", &self.discrdnonvlanframes())
            .field("jumboframes", &self.jumboframes())
            .field("copyallframes", &self.copyallframes())
            .field("nobroadcast", &self.nobroadcast())
            .field("multicasthashen", &self.multicasthashen())
            .field("unicasthashen", &self.unicasthashen())
            .field("rx1536byteframes", &self.rx1536byteframes())
            .field("retrytest", &self.retrytest())
            .field("pauseen", &self.pauseen())
            .field("rxbuffoffset", &self.rxbuffoffset())
            .field("lenfielderrfrmdiscrd", &self.lenfielderrfrmdiscrd())
            .field("fcsremove", &self.fcsremove())
            .field("mdcclkdiv", &self.mdcclkdiv())
            .field("discopyofpframes", &self.discopyofpframes())
            .field("rxchksumoffloaden", &self.rxchksumoffloaden())
            .field("enhalfduplexrx", &self.enhalfduplexrx())
            .field("ignorerxfcs", &self.ignorerxfcs())
            .field("ipgstrtchen", &self.ipgstrtchen())
            .field("nspchange", &self.nspchange())
            .field("ignoreipgrxer", &self.ignoreipgrxer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Networkcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Networkcfg {{ speed: {=bool:?}, fullduplex: {=bool:?}, discrdnonvlanframes: {=bool:?}, jumboframes: {=bool:?}, copyallframes: {=bool:?}, nobroadcast: {=bool:?}, multicasthashen: {=bool:?}, unicasthashen: {=bool:?}, rx1536byteframes: {=bool:?}, retrytest: {=bool:?}, pauseen: {=bool:?}, rxbuffoffset: {=u8:?}, lenfielderrfrmdiscrd: {=bool:?}, fcsremove: {=bool:?}, mdcclkdiv: {:?}, discopyofpframes: {=bool:?}, rxchksumoffloaden: {=bool:?}, enhalfduplexrx: {=bool:?}, ignorerxfcs: {=bool:?}, ipgstrtchen: {=bool:?}, nspchange: {=bool:?}, ignoreipgrxer: {=bool:?} }}" , self . speed () , self . fullduplex () , self . discrdnonvlanframes () , self . jumboframes () , self . copyallframes () , self . nobroadcast () , self . multicasthashen () , self . unicasthashen () , self . rx1536byteframes () , self . retrytest () , self . pauseen () , self . rxbuffoffset () , self . lenfielderrfrmdiscrd () , self . fcsremove () , self . mdcclkdiv () , self . discopyofpframes () , self . rxchksumoffloaden () , self . enhalfduplexrx () , self . ignorerxfcs () , self . ipgstrtchen () , self . nspchange () , self . ignoreipgrxer ())
    }
}
#[doc = "Network control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Networkctrl(pub u32);
impl Networkctrl {
    #[doc = "Loopback local."]
    #[must_use]
    #[inline(always)]
    pub const fn loopbacklocal(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback local."]
    #[inline(always)]
    pub const fn set_loopbacklocal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbrx(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive enable."]
    #[inline(always)]
    pub const fn set_enbrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbtx(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit enable."]
    #[inline(always)]
    pub const fn set_enbtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Management port enable."]
    #[must_use]
    #[inline(always)]
    pub const fn manporten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Management port enable."]
    #[inline(always)]
    pub const fn set_manporten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear statistics registers."]
    #[must_use]
    #[inline(always)]
    pub const fn clrallstatsregs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clear statistics registers."]
    #[inline(always)]
    pub const fn set_clrallstatsregs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Incremental statistics registers."]
    #[must_use]
    #[inline(always)]
    pub const fn incallstatsregs(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Incremental statistics registers."]
    #[inline(always)]
    pub const fn set_incallstatsregs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write enable for statistics registers."]
    #[must_use]
    #[inline(always)]
    pub const fn statswren(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable for statistics registers."]
    #[inline(always)]
    pub const fn set_statswren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Back pressure will force collisions on all received frames."]
    #[must_use]
    #[inline(always)]
    pub const fn backpressure(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Back pressure will force collisions on all received frames."]
    #[inline(always)]
    pub const fn set_backpressure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Start transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txstrt(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Start transmission."]
    #[inline(always)]
    pub const fn set_txstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit halt."]
    #[must_use]
    #[inline(always)]
    pub const fn txhalt(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit halt."]
    #[inline(always)]
    pub const fn set_txhalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit pause frame."]
    #[must_use]
    #[inline(always)]
    pub const fn txpfrmreq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit pause frame."]
    #[inline(always)]
    pub const fn set_txpfrmreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit zero quantum pause frame."]
    #[must_use]
    #[inline(always)]
    pub const fn txpfrmzero(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit zero quantum pause frame."]
    #[inline(always)]
    pub const fn set_txpfrmzero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Store receive time stamp to memory."]
    #[must_use]
    #[inline(always)]
    pub const fn storerxts(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Store receive time stamp to memory."]
    #[inline(always)]
    pub const fn set_storerxts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable PFC Priority Based Pause Reception capabilities."]
    #[must_use]
    #[inline(always)]
    pub const fn pfcenb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub const fn set_pfcenb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write a one to transmit PFC priority based pause frame."]
    #[must_use]
    #[inline(always)]
    pub const fn txpfcpriorpfrm(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub const fn set_txpfcpriorpfrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flush the next packet from the external RX DPRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn flushrxpkt(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub const fn set_flushrxpkt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn txlpien(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub const fn set_txlpien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable detection of unicast PTP unicast frames."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpunicasten(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub const fn set_ptpunicasten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Store UDP / TCP offset to memory."]
    #[must_use]
    #[inline(always)]
    pub const fn storeudpoffset(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub const fn set_storeudpoffset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "1588 One Step Sync Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn onestepsyncmode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1588 One Step Sync Mode."]
    #[inline(always)]
    pub const fn set_onestepsyncmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable multiple PFC pause quantums, one per pause priority."]
    #[must_use]
    #[inline(always)]
    pub const fn pfcctrl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable multiple PFC pause quantums, one per pause priority."]
    #[inline(always)]
    pub const fn set_pfcctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Networkctrl {
    #[inline(always)]
    fn default() -> Networkctrl {
        Networkctrl(0)
    }
}
impl core::fmt::Debug for Networkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Networkctrl")
            .field("loopbacklocal", &self.loopbacklocal())
            .field("enbrx", &self.enbrx())
            .field("enbtx", &self.enbtx())
            .field("manporten", &self.manporten())
            .field("clrallstatsregs", &self.clrallstatsregs())
            .field("incallstatsregs", &self.incallstatsregs())
            .field("statswren", &self.statswren())
            .field("backpressure", &self.backpressure())
            .field("txstrt", &self.txstrt())
            .field("txhalt", &self.txhalt())
            .field("txpfrmreq", &self.txpfrmreq())
            .field("txpfrmzero", &self.txpfrmzero())
            .field("storerxts", &self.storerxts())
            .field("pfcenb", &self.pfcenb())
            .field("txpfcpriorpfrm", &self.txpfcpriorpfrm())
            .field("flushrxpkt", &self.flushrxpkt())
            .field("txlpien", &self.txlpien())
            .field("ptpunicasten", &self.ptpunicasten())
            .field("storeudpoffset", &self.storeudpoffset())
            .field("onestepsyncmode", &self.onestepsyncmode())
            .field("pfcctrl", &self.pfcctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Networkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Networkctrl {{ loopbacklocal: {=bool:?}, enbrx: {=bool:?}, enbtx: {=bool:?}, manporten: {=bool:?}, clrallstatsregs: {=bool:?}, incallstatsregs: {=bool:?}, statswren: {=bool:?}, backpressure: {=bool:?}, txstrt: {=bool:?}, txhalt: {=bool:?}, txpfrmreq: {=bool:?}, txpfrmzero: {=bool:?}, storerxts: {=bool:?}, pfcenb: {=bool:?}, txpfcpriorpfrm: {=bool:?}, flushrxpkt: {=bool:?}, txlpien: {=bool:?}, ptpunicasten: {=bool:?}, storeudpoffset: {=bool:?}, onestepsyncmode: {=bool:?}, pfcctrl: {=bool:?} }}" , self . loopbacklocal () , self . enbrx () , self . enbtx () , self . manporten () , self . clrallstatsregs () , self . incallstatsregs () , self . statswren () , self . backpressure () , self . txstrt () , self . txhalt () , self . txpfrmreq () , self . txpfrmzero () , self . storerxts () , self . pfcenb () , self . txpfcpriorpfrm () , self . flushrxpkt () , self . txlpien () , self . ptpunicasten () , self . storeudpoffset () , self . onestepsyncmode () , self . pfcctrl ())
    }
}
#[doc = "Network status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Networkstatus(pub u32);
impl Networkstatus {
    #[doc = "Returns status of the mdio_in pin."]
    #[must_use]
    #[inline(always)]
    pub const fn mdioin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Returns status of the mdio_in pin."]
    #[inline(always)]
    pub const fn set_mdioin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "The PHY management logic is idle (i.e. has completed)."]
    #[must_use]
    #[inline(always)]
    pub const fn mandone(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "The PHY management logic is idle (i.e. has completed)."]
    #[inline(always)]
    pub const fn set_mandone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set when PFC Priority Based Pause has been negotiated."]
    #[must_use]
    #[inline(always)]
    pub const fn pfcnegotiate(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set when PFC Priority Based Pause has been negotiated."]
    #[inline(always)]
    pub const fn set_pfcnegotiate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPI Indication."]
    #[must_use]
    #[inline(always)]
    pub const fn lpiindicate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Indication."]
    #[inline(always)]
    pub const fn set_lpiindicate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Networkstatus {
    #[inline(always)]
    fn default() -> Networkstatus {
        Networkstatus(0)
    }
}
impl core::fmt::Debug for Networkstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Networkstatus")
            .field("mdioin", &self.mdioin())
            .field("mandone", &self.mandone())
            .field("pfcnegotiate", &self.pfcnegotiate())
            .field("lpiindicate", &self.lpiindicate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Networkstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Networkstatus {{ mdioin: {=bool:?}, mandone: {=bool:?}, pfcnegotiate: {=bool:?}, lpiindicate: {=bool:?} }}" , self . mdioin () , self . mandone () , self . pfcnegotiate () , self . lpiindicate ())
    }
}
#[doc = "Octets Received 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octetsrxedtop(pub u32);
impl Octetsrxedtop {
    #[doc = "Received octets in frame without errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received octets in frame without errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Octetsrxedtop {
    #[inline(always)]
    fn default() -> Octetsrxedtop {
        Octetsrxedtop(0)
    }
}
impl core::fmt::Debug for Octetsrxedtop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Octetsrxedtop")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Octetsrxedtop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Octetsrxedtop {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Octets Transmitted 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octetstxedtop(pub u32);
impl Octetstxedtop {
    #[doc = "Transmitted octets in frame without errors \\[47:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmitted octets in frame without errors \\[47:32\\]."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Octetstxedtop {
    #[inline(always)]
    fn default() -> Octetstxedtop {
        Octetstxedtop(0)
    }
}
impl core::fmt::Debug for Octetstxedtop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Octetstxedtop")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Octetstxedtop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Octetstxedtop {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "RX Partial Store and Forward."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbufrxcutthru(pub u32);
impl Pbufrxcutthru {
    #[doc = "Watermark value."]
    #[must_use]
    #[inline(always)]
    pub const fn dmarxcutthruthr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Watermark value."]
    #[inline(always)]
    pub const fn set_dmarxcutthruthr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Enable RX partial store and forward operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmarxcutthru(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RX partial store and forward operation."]
    #[inline(always)]
    pub const fn set_dmarxcutthru(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pbufrxcutthru {
    #[inline(always)]
    fn default() -> Pbufrxcutthru {
        Pbufrxcutthru(0)
    }
}
impl core::fmt::Debug for Pbufrxcutthru {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pbufrxcutthru")
            .field("dmarxcutthruthr", &self.dmarxcutthruthr())
            .field("dmarxcutthru", &self.dmarxcutthru())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pbufrxcutthru {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pbufrxcutthru {{ dmarxcutthruthr: {=u16:?}, dmarxcutthru: {=bool:?} }}",
            self.dmarxcutthruthr(),
            self.dmarxcutthru()
        )
    }
}
#[doc = "TX Partial Store and Forward."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbuftxcutthru(pub u32);
impl Pbuftxcutthru {
    #[doc = "Watermark value."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatxcutthruthr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Watermark value."]
    #[inline(always)]
    pub const fn set_dmatxcutthruthr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Enable TX partial store and forward operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatxcutthru(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX partial store and forward operation."]
    #[inline(always)]
    pub const fn set_dmatxcutthru(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pbuftxcutthru {
    #[inline(always)]
    fn default() -> Pbuftxcutthru {
        Pbuftxcutthru(0)
    }
}
impl core::fmt::Debug for Pbuftxcutthru {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pbuftxcutthru")
            .field("dmatxcutthruthr", &self.dmatxcutthruthr())
            .field("dmatxcutthru", &self.dmatxcutthru())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pbuftxcutthru {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pbuftxcutthru {{ dmatxcutthruthr: {=u16:?}, dmatxcutthru: {=bool:?} }}",
            self.dmatxcutthruthr(),
            self.dmatxcutthru()
        )
    }
}
#[doc = "Pause Frames Received."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pframesrxed(pub u32);
impl Pframesrxed {
    #[doc = "Received pause frames."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received pause frames."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pframesrxed {
    #[inline(always)]
    fn default() -> Pframesrxed {
        Pframesrxed(0)
    }
}
impl core::fmt::Debug for Pframesrxed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pframesrxed")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pframesrxed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pframesrxed {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Pause Frames Transmitted."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pframestxed(pub u32);
impl Pframestxed {
    #[doc = "Transmitted pause frames."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmitted pause frames."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pframestxed {
    #[inline(always)]
    fn default() -> Pframestxed {
        Pframestxed(0)
    }
}
impl core::fmt::Debug for Pframestxed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pframestxed")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pframestxed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pframestxed {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "PHY management register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phymngmnt(pub u32);
impl Phymngmnt {
    #[doc = "PHY read write data."]
    #[must_use]
    #[inline(always)]
    pub const fn phyrwdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PHY read write data."]
    #[inline(always)]
    pub const fn set_phyrwdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Must be written with 10."]
    #[must_use]
    #[inline(always)]
    pub const fn write10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Must be written with 10."]
    #[inline(always)]
    pub const fn set_write10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Register address - specifies the register in the PHY to access."]
    #[must_use]
    #[inline(always)]
    pub const fn regaddr(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub const fn set_regaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "PHY address."]
    #[must_use]
    #[inline(always)]
    pub const fn phyaddr(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY address."]
    #[inline(always)]
    pub const fn set_phyaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
    #[doc = "Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[must_use]
    #[inline(always)]
    pub const fn operation(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub const fn set_operation(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[must_use]
    #[inline(always)]
    pub const fn write1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub const fn set_write1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Must be written with 0."]
    #[must_use]
    #[inline(always)]
    pub const fn write0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Must be written with 0."]
    #[inline(always)]
    pub const fn set_write0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Phymngmnt {
    #[inline(always)]
    fn default() -> Phymngmnt {
        Phymngmnt(0)
    }
}
impl core::fmt::Debug for Phymngmnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phymngmnt")
            .field("phyrwdata", &self.phyrwdata())
            .field("write10", &self.write10())
            .field("regaddr", &self.regaddr())
            .field("phyaddr", &self.phyaddr())
            .field("operation", &self.operation())
            .field("write1", &self.write1())
            .field("write0", &self.write0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phymngmnt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Phymngmnt {{ phyrwdata: {=u16:?}, write10: {=u8:?}, regaddr: {=u8:?}, phyaddr: {=u8:?}, operation: {=u8:?}, write1: {=bool:?}, write0: {=bool:?} }}" , self . phyrwdata () , self . write10 () , self . regaddr () , self . phyaddr () , self . operation () , self . write1 () , self . write0 ())
    }
}
#[doc = "I/O Route Location Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn miitxloc(&self) -> super::vals::Miitxloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Miitxloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_miitxloc(&mut self, val: super::vals::Miitxloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn miirxloc(&self) -> super::vals::Miirxloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Miirxloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_miirxloc(&mut self, val: super::vals::Miirxloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn miicrsloc(&self) -> super::vals::Miicrsloc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Miicrsloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_miicrsloc(&mut self, val: super::vals::Miicrsloc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn miicolloc(&self) -> super::vals::Miicolloc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Miicolloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_miicolloc(&mut self, val: super::vals::Miicolloc) {
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
            .field("miitxloc", &self.miitxloc())
            .field("miirxloc", &self.miirxloc())
            .field("miicrsloc", &self.miicrsloc())
            .field("miicolloc", &self.miicolloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ miitxloc: {:?}, miirxloc: {:?}, miicrsloc: {:?}, miicolloc: {:?} }}",
            self.miitxloc(),
            self.miirxloc(),
            self.miicrsloc(),
            self.miicolloc()
        )
    }
}
#[doc = "I/O Route Location Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn tsuextclkloc(&self) -> super::vals::Tsuextclkloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Tsuextclkloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_tsuextclkloc(&mut self, val: super::vals::Tsuextclkloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutmrtogloc(&self) -> super::vals::Tsutmrtogloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Tsutmrtogloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_tsutmrtogloc(&mut self, val: super::vals::Tsutmrtogloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn mdioloc(&self) -> super::vals::Mdioloc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Mdioloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_mdioloc(&mut self, val: super::vals::Mdioloc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn rmiiloc(&self) -> super::vals::Rmiiloc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Rmiiloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_rmiiloc(&mut self, val: super::vals::Rmiiloc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
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
            .field("tsuextclkloc", &self.tsuextclkloc())
            .field("tsutmrtogloc", &self.tsutmrtogloc())
            .field("mdioloc", &self.mdioloc())
            .field("rmiiloc", &self.rmiiloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc1 {{ tsuextclkloc: {:?}, tsutmrtogloc: {:?}, mdioloc: {:?}, rmiiloc: {:?} }}",
            self.tsuextclkloc(),
            self.tsutmrtogloc(),
            self.mdioloc(),
            self.rmiiloc()
        )
    }
}
#[doc = "I/O Route Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "MDIO I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiopen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO I/O Enable."]
    #[inline(always)]
    pub const fn set_mdiopen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MII TX ER I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn miitxerpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MII TX ER I/O Enable."]
    #[inline(always)]
    pub const fn set_miitxerpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MII TX ER I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn miirxerpen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MII TX ER I/O Enable."]
    #[inline(always)]
    pub const fn set_miirxerpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "MII I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn miipen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MII I/O Enable."]
    #[inline(always)]
    pub const fn set_miipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RMII I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rmiipen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RMII I/O Enable."]
    #[inline(always)]
    pub const fn set_rmiipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TSU_TMR_CNT_SEC Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsutmrtogpen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TSU_TMR_CNT_SEC Output Enable."]
    #[inline(always)]
    pub const fn set_tsutmrtogpen(&mut self, val: bool) {
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
            .field("mdiopen", &self.mdiopen())
            .field("miitxerpen", &self.miitxerpen())
            .field("miirxerpen", &self.miirxerpen())
            .field("miipen", &self.miipen())
            .field("rmiipen", &self.rmiipen())
            .field("tsutmrtogpen", &self.tsutmrtogpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ mdiopen: {=bool:?}, miitxerpen: {=bool:?}, miirxerpen: {=bool:?}, miipen: {=bool:?}, rmiipen: {=bool:?}, tsutmrtogpen: {=bool:?} }}" , self . mdiopen () , self . miitxerpen () , self . miirxerpen () , self . miipen () , self . rmiipen () , self . tsutmrtogpen ())
    }
}
#[doc = "RX BD control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxbdctrl(pub u32);
impl Rxbdctrl {
    #[doc = "RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames."]
    #[must_use]
    #[inline(always)]
    pub const fn rxbdtsmode(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames."]
    #[inline(always)]
    pub const fn set_rxbdtsmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Rxbdctrl {
    #[inline(always)]
    fn default() -> Rxbdctrl {
        Rxbdctrl(0)
    }
}
impl core::fmt::Debug for Rxbdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxbdctrl")
            .field("rxbdtsmode", &self.rxbdtsmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxbdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxbdctrl {{ rxbdtsmode: {=u8:?} }}", self.rxbdtsmode())
    }
}
#[doc = "IP Header Checksum Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxipckerrs(pub u32);
impl Rxipckerrs {
    #[doc = "IP header checksum errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IP header checksum errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxipckerrs {
    #[inline(always)]
    fn default() -> Rxipckerrs {
        Rxipckerrs(0)
    }
}
impl core::fmt::Debug for Rxipckerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxipckerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxipckerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxipckerrs {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "Jabbers Received."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxjabbers(pub u32);
impl Rxjabbers {
    #[doc = "Jabbers received."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Jabbers received."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rxjabbers {
    #[inline(always)]
    fn default() -> Rxjabbers {
        Rxjabbers(0)
    }
}
impl core::fmt::Debug for Rxjabbers {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxjabbers")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxjabbers {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxjabbers {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Length Field Frame Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxlenerrs(pub u32);
impl Rxlenerrs {
    #[doc = "Length field frame errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Length field frame errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rxlenerrs {
    #[inline(always)]
    fn default() -> Rxlenerrs {
        Rxlenerrs(0)
    }
}
impl core::fmt::Debug for Rxlenerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxlenerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxlenerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxlenerrs {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Received LPI transitions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxlpi(pub u32);
impl Rxlpi {
    #[doc = "Count of RX LPI transitions."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Count of RX LPI transitions."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxlpi {
    #[inline(always)]
    fn default() -> Rxlpi {
        Rxlpi(0)
    }
}
impl core::fmt::Debug for Rxlpi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxlpi")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxlpi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxlpi {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Received LPI time."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxlpitime(pub u32);
impl Rxlpitime {
    #[doc = "Time in LPI."]
    #[must_use]
    #[inline(always)]
    pub const fn lpitime(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time in LPI."]
    #[inline(always)]
    pub const fn set_lpitime(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Rxlpitime {
    #[inline(always)]
    fn default() -> Rxlpitime {
        Rxlpitime(0)
    }
}
impl core::fmt::Debug for Rxlpitime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxlpitime")
            .field("lpitime", &self.lpitime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxlpitime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxlpitime {{ lpitime: {=u32:?} }}", self.lpitime())
    }
}
#[doc = "Receive Overruns."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxoverruns(pub u32);
impl Rxoverruns {
    #[doc = "Receive overruns."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive overruns."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rxoverruns {
    #[inline(always)]
    fn default() -> Rxoverruns {
        Rxoverruns(0)
    }
}
impl core::fmt::Debug for Rxoverruns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxoverruns")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxoverruns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxoverruns {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Received Pause Quantum Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxpausequant(pub u32);
impl Rxpausequant {
    #[doc = "Received pause quantum."]
    #[must_use]
    #[inline(always)]
    pub const fn quant(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received pause quantum."]
    #[inline(always)]
    pub const fn set_quant(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxpausequant {
    #[inline(always)]
    fn default() -> Rxpausequant {
        Rxpausequant(0)
    }
}
impl core::fmt::Debug for Rxpausequant {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxpausequant")
            .field("quant", &self.quant())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxpausequant {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxpausequant {{ quant: {=u16:?} }}", self.quant())
    }
}
#[doc = "Start address of the receive buffer queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxqptr(pub u32);
impl Rxqptr {
    #[doc = "Receive buffer queue base address."]
    #[must_use]
    #[inline(always)]
    pub const fn dmarxqptr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Receive buffer queue base address."]
    #[inline(always)]
    pub const fn set_dmarxqptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Rxqptr {
    #[inline(always)]
    fn default() -> Rxqptr {
        Rxqptr(0)
    }
}
impl core::fmt::Debug for Rxqptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxqptr")
            .field("dmarxqptr", &self.dmarxqptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxqptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxqptr {{ dmarxqptr: {=u32:?} }}", self.dmarxqptr())
    }
}
#[doc = "Receive Resource Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxresourceerrs(pub u32);
impl Rxresourceerrs {
    #[doc = "Receive resource errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Receive resource errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Rxresourceerrs {
    #[inline(always)]
    fn default() -> Rxresourceerrs {
        Rxresourceerrs(0)
    }
}
impl core::fmt::Debug for Rxresourceerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxresourceerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxresourceerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxresourceerrs {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Receive status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxstatus(pub u32);
impl Rxstatus {
    #[doc = "Buffer not available."]
    #[must_use]
    #[inline(always)]
    pub const fn buffnotavail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer not available."]
    #[inline(always)]
    pub const fn set_buffnotavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame received."]
    #[must_use]
    #[inline(always)]
    pub const fn frmrx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Frame received."]
    #[inline(always)]
    pub const fn set_frmrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive overrun."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoverrun(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun."]
    #[inline(always)]
    pub const fn set_rxoverrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "bresp/hresp not OK."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "bresp/hresp not OK."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Rxstatus {
    #[inline(always)]
    fn default() -> Rxstatus {
        Rxstatus(0)
    }
}
impl core::fmt::Debug for Rxstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxstatus")
            .field("buffnotavail", &self.buffnotavail())
            .field("frmrx", &self.frmrx())
            .field("rxoverrun", &self.rxoverrun())
            .field("respnotok", &self.respnotok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rxstatus {{ buffnotavail: {=bool:?}, frmrx: {=bool:?}, rxoverrun: {=bool:?}, respnotok: {=bool:?} }}" , self . buffnotavail () , self . frmrx () , self . rxoverrun () , self . respnotok ())
    }
}
#[doc = "Receive Symbol Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxsymbolerrs(pub u32);
impl Rxsymbolerrs {
    #[doc = "Receive symbol errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive symbol errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rxsymbolerrs {
    #[inline(always)]
    fn default() -> Rxsymbolerrs {
        Rxsymbolerrs(0)
    }
}
impl core::fmt::Debug for Rxsymbolerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxsymbolerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxsymbolerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxsymbolerrs {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "TCP Checksum Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxtcpckerrs(pub u32);
impl Rxtcpckerrs {
    #[doc = "TCP checksum errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TCP checksum errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxtcpckerrs {
    #[inline(always)]
    fn default() -> Rxtcpckerrs {
        Rxtcpckerrs(0)
    }
}
impl core::fmt::Debug for Rxtcpckerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxtcpckerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxtcpckerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxtcpckerrs {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "UDP Checksum Errors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxudpckerrs(pub u32);
impl Rxudpckerrs {
    #[doc = "UDP checksum errors."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "UDP checksum errors."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxudpckerrs {
    #[inline(always)]
    fn default() -> Rxudpckerrs {
        Rxudpckerrs(0)
    }
}
impl core::fmt::Debug for Rxudpckerrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxudpckerrs")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxudpckerrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxudpckerrs {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "Single Collision Frames."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlecols(pub u32);
impl Singlecols {
    #[doc = "Single collision frames."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Single collision frames."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Singlecols {
    #[inline(always)]
    fn default() -> Singlecols {
        Singlecols(0)
    }
}
impl core::fmt::Debug for Singlecols {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Singlecols")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlecols {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Singlecols {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Specific Address 1 Top."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Specaddr1top(pub u32);
impl Specaddr1top {
    #[doc = "Specific address 1 MSB."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specific address 1 MSB."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "MAC SA or DA selection."]
    #[must_use]
    #[inline(always)]
    pub const fn filtertype(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC SA or DA selection."]
    #[inline(always)]
    pub const fn set_filtertype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Specaddr1top {
    #[inline(always)]
    fn default() -> Specaddr1top {
        Specaddr1top(0)
    }
}
impl core::fmt::Debug for Specaddr1top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Specaddr1top")
            .field("addr", &self.addr())
            .field("filtertype", &self.filtertype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Specaddr1top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Specaddr1top {{ addr: {=u16:?}, filtertype: {=bool:?} }}",
            self.addr(),
            self.filtertype()
        )
    }
}
#[doc = "Specific Address 2 Top."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Specaddr2top(pub u32);
impl Specaddr2top {
    #[doc = "Specific address 2 MSB."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specific address 2 MSB."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "MAC SA or DA selection."]
    #[must_use]
    #[inline(always)]
    pub const fn filtertype(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC SA or DA selection."]
    #[inline(always)]
    pub const fn set_filtertype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Filter byte Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn filterbytemask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Filter byte Mask."]
    #[inline(always)]
    pub const fn set_filterbytemask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Specaddr2top {
    #[inline(always)]
    fn default() -> Specaddr2top {
        Specaddr2top(0)
    }
}
impl core::fmt::Debug for Specaddr2top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Specaddr2top")
            .field("addr", &self.addr())
            .field("filtertype", &self.filtertype())
            .field("filterbytemask", &self.filterbytemask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Specaddr2top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Specaddr2top {{ addr: {=u16:?}, filtertype: {=bool:?}, filterbytemask: {=u8:?} }}",
            self.addr(),
            self.filtertype(),
            self.filterbytemask()
        )
    }
}
#[doc = "Specific Address 3 Top."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Specaddr3top(pub u32);
impl Specaddr3top {
    #[doc = "Specific address 3 MSB."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specific address 3 MSB."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "MAC SA or DA selection."]
    #[must_use]
    #[inline(always)]
    pub const fn filtertype(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC SA or DA selection."]
    #[inline(always)]
    pub const fn set_filtertype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Filter byte Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn filterbytemask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Filter byte Mask."]
    #[inline(always)]
    pub const fn set_filterbytemask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Specaddr3top {
    #[inline(always)]
    fn default() -> Specaddr3top {
        Specaddr3top(0)
    }
}
impl core::fmt::Debug for Specaddr3top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Specaddr3top")
            .field("addr", &self.addr())
            .field("filtertype", &self.filtertype())
            .field("filterbytemask", &self.filterbytemask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Specaddr3top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Specaddr3top {{ addr: {=u16:?}, filtertype: {=bool:?}, filterbytemask: {=u8:?} }}",
            self.addr(),
            self.filtertype(),
            self.filterbytemask()
        )
    }
}
#[doc = "Specific Address 4 Top."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Specaddr4top(pub u32);
impl Specaddr4top {
    #[doc = "Specific address 4 MSB."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specific address 4 MSB."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "MAC SA or DA selection."]
    #[must_use]
    #[inline(always)]
    pub const fn filtertype(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC SA or DA selection."]
    #[inline(always)]
    pub const fn set_filtertype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Filter byte Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn filterbytemask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Filter byte Mask."]
    #[inline(always)]
    pub const fn set_filterbytemask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Specaddr4top {
    #[inline(always)]
    fn default() -> Specaddr4top {
        Specaddr4top(0)
    }
}
impl core::fmt::Debug for Specaddr4top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Specaddr4top")
            .field("addr", &self.addr())
            .field("filtertype", &self.filtertype())
            .field("filterbytemask", &self.filterbytemask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Specaddr4top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Specaddr4top {{ addr: {=u16:?}, filtertype: {=bool:?}, filterbytemask: {=u8:?} }}",
            self.addr(),
            self.filtertype(),
            self.filterbytemask()
        )
    }
}
#[doc = "Type ID Match 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spectype1(pub u32);
impl Spectype1 {
    #[doc = "Type ID match 1."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type ID match 1."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable copying of type ID match 1 matched frames."]
    #[must_use]
    #[inline(always)]
    pub const fn enbcopy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable copying of type ID match 1 matched frames."]
    #[inline(always)]
    pub const fn set_enbcopy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spectype1 {
    #[inline(always)]
    fn default() -> Spectype1 {
        Spectype1(0)
    }
}
impl core::fmt::Debug for Spectype1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spectype1")
            .field("match_", &self.match_())
            .field("enbcopy", &self.enbcopy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spectype1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spectype1 {{ match_: {=u16:?}, enbcopy: {=bool:?} }}",
            self.match_(),
            self.enbcopy()
        )
    }
}
#[doc = "Type ID Match 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spectype2(pub u32);
impl Spectype2 {
    #[doc = "Type ID match 2."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type ID match 2."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable copying of type ID match 2 matched frames."]
    #[must_use]
    #[inline(always)]
    pub const fn enbcopy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable copying of type ID match 2 matched frames."]
    #[inline(always)]
    pub const fn set_enbcopy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spectype2 {
    #[inline(always)]
    fn default() -> Spectype2 {
        Spectype2(0)
    }
}
impl core::fmt::Debug for Spectype2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spectype2")
            .field("match_", &self.match_())
            .field("enbcopy", &self.enbcopy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spectype2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spectype2 {{ match_: {=u16:?}, enbcopy: {=bool:?} }}",
            self.match_(),
            self.enbcopy()
        )
    }
}
#[doc = "Type ID Match 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spectype3(pub u32);
impl Spectype3 {
    #[doc = "Type ID match 3."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type ID match 3."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable copying of type ID match 3 matched frames."]
    #[must_use]
    #[inline(always)]
    pub const fn enbcopy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable copying of type ID match 3 matched frames."]
    #[inline(always)]
    pub const fn set_enbcopy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spectype3 {
    #[inline(always)]
    fn default() -> Spectype3 {
        Spectype3(0)
    }
}
impl core::fmt::Debug for Spectype3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spectype3")
            .field("match_", &self.match_())
            .field("enbcopy", &self.enbcopy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spectype3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spectype3 {{ match_: {=u16:?}, enbcopy: {=bool:?} }}",
            self.match_(),
            self.enbcopy()
        )
    }
}
#[doc = "Type ID Match 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spectype4(pub u32);
impl Spectype4 {
    #[doc = "Type ID match 4."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type ID match 4."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable copying of type ID match 4 matched frames."]
    #[must_use]
    #[inline(always)]
    pub const fn enbcopy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable copying of type ID match 4 matched frames."]
    #[inline(always)]
    pub const fn set_enbcopy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spectype4 {
    #[inline(always)]
    fn default() -> Spectype4 {
        Spectype4(0)
    }
}
impl core::fmt::Debug for Spectype4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spectype4")
            .field("match_", &self.match_())
            .field("enbcopy", &self.enbcopy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spectype4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spectype4 {{ match_: {=u16:?}, enbcopy: {=bool:?} }}",
            self.match_(),
            self.enbcopy()
        )
    }
}
#[doc = "Stacked VLAN Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stackedvlan(pub u32);
impl Stackedvlan {
    #[doc = "User defined VLAN_TYPE field."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "User defined VLAN_TYPE field."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable stacked VLAN processing mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enbprocessing(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable stacked VLAN processing mode."]
    #[inline(always)]
    pub const fn set_enbprocessing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Stackedvlan {
    #[inline(always)]
    fn default() -> Stackedvlan {
        Stackedvlan(0)
    }
}
impl core::fmt::Debug for Stackedvlan {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stackedvlan")
            .field("match_", &self.match_())
            .field("enbprocessing", &self.enbprocessing())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stackedvlan {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stackedvlan {{ match_: {=u16:?}, enbprocessing: {=bool:?} }}",
            self.match_(),
            self.enbprocessing()
        )
    }
}
#[doc = "IPG stretch register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stretchratio(pub u32);
impl Stretchratio {
    #[doc = "IPG Stretch."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgstretch(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IPG Stretch."]
    #[inline(always)]
    pub const fn set_ipgstretch(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Stretchratio {
    #[inline(always)]
    fn default() -> Stretchratio {
        Stretchratio(0)
    }
}
impl core::fmt::Debug for Stretchratio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stretchratio")
            .field("ipgstretch", &self.ipgstretch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stretchratio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stretchratio {{ ipgstretch: {=u16:?} }}",
            self.ipgstretch()
        )
    }
}
#[doc = "System wake time."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syswaketime(pub u32);
impl Syswaketime {
    #[doc = "Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en."]
    #[must_use]
    #[inline(always)]
    pub const fn syswaketime(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en."]
    #[inline(always)]
    pub const fn set_syswaketime(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Syswaketime {
    #[inline(always)]
    fn default() -> Syswaketime {
        Syswaketime(0)
    }
}
impl core::fmt::Debug for Syswaketime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syswaketime")
            .field("syswaketime", &self.syswaketime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syswaketime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syswaketime {{ syswaketime: {=u16:?} }}",
            self.syswaketime()
        )
    }
}
#[doc = "TSU timer comparison value seconds \\[47:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsumsbseccmp(pub u32);
impl Tsumsbseccmp {
    #[doc = "TSU timer comparison value (s)."]
    #[must_use]
    #[inline(always)]
    pub const fn compval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSU timer comparison value (s)."]
    #[inline(always)]
    pub const fn set_compval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsumsbseccmp {
    #[inline(always)]
    fn default() -> Tsumsbseccmp {
        Tsumsbseccmp(0)
    }
}
impl core::fmt::Debug for Tsumsbseccmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsumsbseccmp")
            .field("compval", &self.compval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsumsbseccmp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsumsbseccmp {{ compval: {=u16:?} }}", self.compval())
    }
}
#[doc = "TSU timer comparison value nanoseconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsunseccmp(pub u32);
impl Tsunseccmp {
    #[doc = "TSU timer comparison value (ns)."]
    #[must_use]
    #[inline(always)]
    pub const fn compval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "TSU timer comparison value (ns)."]
    #[inline(always)]
    pub const fn set_compval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Tsunseccmp {
    #[inline(always)]
    fn default() -> Tsunseccmp {
        Tsunseccmp(0)
    }
}
impl core::fmt::Debug for Tsunseccmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsunseccmp")
            .field("compval", &self.compval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsunseccmp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsunseccmp {{ compval: {=u32:?} }}", self.compval())
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsupeerrxmsbsec(pub u32);
impl Tsupeerrxmsbsec {
    #[doc = "PTP Peer Event Frame RX Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timersec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PTP Peer Event Frame RX Seconds."]
    #[inline(always)]
    pub const fn set_timersec(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsupeerrxmsbsec {
    #[inline(always)]
    fn default() -> Tsupeerrxmsbsec {
        Tsupeerrxmsbsec(0)
    }
}
impl core::fmt::Debug for Tsupeerrxmsbsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsupeerrxmsbsec")
            .field("timersec", &self.timersec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsupeerrxmsbsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsupeerrxmsbsec {{ timersec: {=u16:?} }}",
            self.timersec()
        )
    }
}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsupeerrxnsec(pub u32);
impl Tsupeerrxnsec {
    #[doc = "PTP Peer Event Frame Received Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "PTP Peer Event Frame Received Nanoseconds."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Tsupeerrxnsec {
    #[inline(always)]
    fn default() -> Tsupeerrxnsec {
        Tsupeerrxnsec(0)
    }
}
impl core::fmt::Debug for Tsupeerrxnsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsupeerrxnsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsupeerrxnsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsupeerrxnsec {{ timer: {=u32:?} }}", self.timer())
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsupeertxmsbsec(pub u32);
impl Tsupeertxmsbsec {
    #[doc = "PTP Peer Event Frame TX Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timersec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PTP Peer Event Frame TX Seconds."]
    #[inline(always)]
    pub const fn set_timersec(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsupeertxmsbsec {
    #[inline(always)]
    fn default() -> Tsupeertxmsbsec {
        Tsupeertxmsbsec(0)
    }
}
impl core::fmt::Debug for Tsupeertxmsbsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsupeertxmsbsec")
            .field("timersec", &self.timersec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsupeertxmsbsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsupeertxmsbsec {{ timersec: {=u16:?} }}",
            self.timersec()
        )
    }
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsupeertxnsec(pub u32);
impl Tsupeertxnsec {
    #[doc = "PTP Peer Event Frame Transmitted Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "PTP Peer Event Frame Transmitted Nanoseconds."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Tsupeertxnsec {
    #[inline(always)]
    fn default() -> Tsupeertxnsec {
        Tsupeertxnsec(0)
    }
}
impl core::fmt::Debug for Tsupeertxnsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsupeertxnsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsupeertxnsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsupeertxnsec {{ timer: {=u32:?} }}", self.timer())
    }
}
#[doc = "PTP Event Frame Received Seconds Register 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuptprxmsbsec(pub u32);
impl Tsuptprxmsbsec {
    #[doc = "PTP Event Frame TX Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timersec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PTP Event Frame TX Seconds."]
    #[inline(always)]
    pub const fn set_timersec(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsuptprxmsbsec {
    #[inline(always)]
    fn default() -> Tsuptprxmsbsec {
        Tsuptprxmsbsec(0)
    }
}
impl core::fmt::Debug for Tsuptprxmsbsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsuptprxmsbsec")
            .field("timersec", &self.timersec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsuptprxmsbsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsuptprxmsbsec {{ timersec: {=u16:?} }}",
            self.timersec()
        )
    }
}
#[doc = "PTP Event Frame Received Nanoseconds Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuptprxnsec(pub u32);
impl Tsuptprxnsec {
    #[doc = "PTP Event Frame Received Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "PTP Event Frame Received Nanoseconds."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Tsuptprxnsec {
    #[inline(always)]
    fn default() -> Tsuptprxnsec {
        Tsuptprxnsec(0)
    }
}
impl core::fmt::Debug for Tsuptprxnsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsuptprxnsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsuptprxnsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsuptprxnsec {{ timer: {=u32:?} }}", self.timer())
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuptptxmsbsec(pub u32);
impl Tsuptptxmsbsec {
    #[doc = "PTP Event Frame TX Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timersec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PTP Event Frame TX Seconds."]
    #[inline(always)]
    pub const fn set_timersec(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsuptptxmsbsec {
    #[inline(always)]
    fn default() -> Tsuptptxmsbsec {
        Tsuptptxmsbsec(0)
    }
}
impl core::fmt::Debug for Tsuptptxmsbsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsuptptxmsbsec")
            .field("timersec", &self.timersec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsuptptxmsbsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsuptptxmsbsec {{ timersec: {=u16:?} }}",
            self.timersec()
        )
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuptptxnsec(pub u32);
impl Tsuptptxnsec {
    #[doc = "PTP Event Frame Transmitted Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "PTP Event Frame Transmitted Nanoseconds."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Tsuptptxnsec {
    #[inline(always)]
    fn default() -> Tsuptptxnsec {
        Tsuptptxnsec(0)
    }
}
impl core::fmt::Debug for Tsuptptxnsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsuptptxnsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsuptptxnsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsuptptxnsec {{ timer: {=u32:?} }}", self.timer())
    }
}
#[doc = "This register returns all zeroes when read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsutimeradjust(pub u32);
impl Tsutimeradjust {
    #[doc = "Timer increment value."]
    #[must_use]
    #[inline(always)]
    pub const fn incrementval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Timer increment value."]
    #[inline(always)]
    pub const fn set_incrementval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Write as one to subtract from the 1588 timer."]
    #[must_use]
    #[inline(always)]
    pub const fn addsubtract(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write as one to subtract from the 1588 timer."]
    #[inline(always)]
    pub const fn set_addsubtract(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tsutimeradjust {
    #[inline(always)]
    fn default() -> Tsutimeradjust {
        Tsutimeradjust(0)
    }
}
impl core::fmt::Debug for Tsutimeradjust {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsutimeradjust")
            .field("incrementval", &self.incrementval())
            .field("addsubtract", &self.addsubtract())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsutimeradjust {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsutimeradjust {{ incrementval: {=u32:?}, addsubtract: {=bool:?} }}",
            self.incrementval(),
            self.addsubtract()
        )
    }
}
#[doc = "1588 Timer Increment Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsutimerincr(pub u32);
impl Tsutimerincr {
    #[doc = "A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn nsincrement(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
    #[inline(always)]
    pub const fn set_nsincrement(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Alternative nanoseconds count."]
    #[must_use]
    #[inline(always)]
    pub const fn altnsincr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Alternative nanoseconds count."]
    #[inline(always)]
    pub const fn set_altnsincr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of incs before alt inc."]
    #[must_use]
    #[inline(always)]
    pub const fn numincs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of incs before alt inc."]
    #[inline(always)]
    pub const fn set_numincs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Tsutimerincr {
    #[inline(always)]
    fn default() -> Tsutimerincr {
        Tsutimerincr(0)
    }
}
impl core::fmt::Debug for Tsutimerincr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsutimerincr")
            .field("nsincrement", &self.nsincrement())
            .field("altnsincr", &self.altnsincr())
            .field("numincs", &self.numincs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsutimerincr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsutimerincr {{ nsincrement: {=u8:?}, altnsincr: {=u8:?}, numincs: {=u8:?} }}",
            self.nsincrement(),
            self.altnsincr(),
            self.numincs()
        )
    }
}
#[doc = "1588 Timer Increment Register subscript nsec."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsutimerincrsubnsec(pub u32);
impl Tsutimerincrsubnsec {
    #[doc = "MSB \\[23:8\\] of the subscript-ns value."]
    #[must_use]
    #[inline(always)]
    pub const fn subnsincr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MSB \\[23:8\\] of the subscript-ns value."]
    #[inline(always)]
    pub const fn set_subnsincr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "LSB \\[7:0\\] of the subscript-ns value."]
    #[must_use]
    #[inline(always)]
    pub const fn subnsincrlsb(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "LSB \\[7:0\\] of the subscript-ns value."]
    #[inline(always)]
    pub const fn set_subnsincrlsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tsutimerincrsubnsec {
    #[inline(always)]
    fn default() -> Tsutimerincrsubnsec {
        Tsutimerincrsubnsec(0)
    }
}
impl core::fmt::Debug for Tsutimerincrsubnsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsutimerincrsubnsec")
            .field("subnsincr", &self.subnsincr())
            .field("subnsincrlsb", &self.subnsincrlsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsutimerincrsubnsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsutimerincrsubnsec {{ subnsincr: {=u16:?}, subnsincrlsb: {=u8:?} }}",
            self.subnsincr(),
            self.subnsincrlsb()
        )
    }
}
#[doc = "1588 Timer Seconds Register 47:32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsutimermsbsec(pub u32);
impl Tsutimermsbsec {
    #[doc = "MSB 16 bits of seconds timer count."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsutimermsbsec {
    #[inline(always)]
    fn default() -> Tsutimermsbsec {
        Tsutimermsbsec(0)
    }
}
impl core::fmt::Debug for Tsutimermsbsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsutimermsbsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsutimermsbsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsutimermsbsec {{ timer: {=u16:?} }}", self.timer())
    }
}
#[doc = "1588 Timer Nanoseconds Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsutimernsec(pub u32);
impl Tsutimernsec {
    #[doc = "Timer count in nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Timer count in nanoseconds."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Tsutimernsec {
    #[inline(always)]
    fn default() -> Tsutimernsec {
        Tsutimernsec(0)
    }
}
impl core::fmt::Debug for Tsutimernsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsutimernsec")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsutimernsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsutimernsec {{ timer: {=u32:?} }}", self.timer())
    }
}
#[doc = "TX BD control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbdctrl(pub u32);
impl Txbdctrl {
    #[doc = "TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames."]
    #[must_use]
    #[inline(always)]
    pub const fn txbdtsmode(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames."]
    #[inline(always)]
    pub const fn set_txbdtsmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Txbdctrl {
    #[inline(always)]
    fn default() -> Txbdctrl {
        Txbdctrl(0)
    }
}
impl core::fmt::Debug for Txbdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbdctrl")
            .field("txbdtsmode", &self.txbdtsmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbdctrl {{ txbdtsmode: {=u8:?} }}", self.txbdtsmode())
    }
}
#[doc = "Transmit LPI transitions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txlpi(pub u32);
impl Txlpi {
    #[doc = "Count of LPI transmitions."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Count of LPI transmitions."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Txlpi {
    #[inline(always)]
    fn default() -> Txlpi {
        Txlpi(0)
    }
}
impl core::fmt::Debug for Txlpi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txlpi")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txlpi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txlpi {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Transmit LPI time."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txlpitime(pub u32);
impl Txlpitime {
    #[doc = "Time in LPI."]
    #[must_use]
    #[inline(always)]
    pub const fn lpitime(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time in LPI."]
    #[inline(always)]
    pub const fn set_lpitime(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Txlpitime {
    #[inline(always)]
    fn default() -> Txlpitime {
        Txlpitime(0)
    }
}
impl core::fmt::Debug for Txlpitime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txlpitime")
            .field("lpitime", &self.lpitime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txlpitime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txlpitime {{ lpitime: {=u32:?} }}", self.lpitime())
    }
}
#[doc = "Transmit Pause Quantum Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpausequant(pub u32);
impl Txpausequant {
    #[doc = "Transmit pause quantum."]
    #[must_use]
    #[inline(always)]
    pub const fn quant(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum."]
    #[inline(always)]
    pub const fn set_quant(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub const fn set_quantp1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Txpausequant {
    #[inline(always)]
    fn default() -> Txpausequant {
        Txpausequant(0)
    }
}
impl core::fmt::Debug for Txpausequant {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txpausequant")
            .field("quant", &self.quant())
            .field("quantp1", &self.quantp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txpausequant {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txpausequant {{ quant: {=u16:?}, quantp1: {=u16:?} }}",
            self.quant(),
            self.quantp1()
        )
    }
}
#[doc = "Transmit Pause Quantum Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpausequant1(pub u32);
impl Txpausequant1 {
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub const fn set_quantp2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub const fn set_quantp3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Txpausequant1 {
    #[inline(always)]
    fn default() -> Txpausequant1 {
        Txpausequant1(0)
    }
}
impl core::fmt::Debug for Txpausequant1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txpausequant1")
            .field("quantp2", &self.quantp2())
            .field("quantp3", &self.quantp3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txpausequant1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txpausequant1 {{ quantp2: {=u16:?}, quantp3: {=u16:?} }}",
            self.quantp2(),
            self.quantp3()
        )
    }
}
#[doc = "Transmit Pause Quantum Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpausequant2(pub u32);
impl Txpausequant2 {
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub const fn set_quantp4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub const fn set_quantp5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Txpausequant2 {
    #[inline(always)]
    fn default() -> Txpausequant2 {
        Txpausequant2(0)
    }
}
impl core::fmt::Debug for Txpausequant2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txpausequant2")
            .field("quantp4", &self.quantp4())
            .field("quantp5", &self.quantp5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txpausequant2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txpausequant2 {{ quantp4: {=u16:?}, quantp5: {=u16:?} }}",
            self.quantp4(),
            self.quantp5()
        )
    }
}
#[doc = "Transmit Pause Quantum Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpausequant3(pub u32);
impl Txpausequant3 {
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub const fn set_quantp6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[must_use]
    #[inline(always)]
    pub const fn quantp7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub const fn set_quantp7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Txpausequant3 {
    #[inline(always)]
    fn default() -> Txpausequant3 {
        Txpausequant3(0)
    }
}
impl core::fmt::Debug for Txpausequant3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txpausequant3")
            .field("quantp6", &self.quantp6())
            .field("quantp7", &self.quantp7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txpausequant3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txpausequant3 {{ quantp6: {=u16:?}, quantp7: {=u16:?} }}",
            self.quantp6(),
            self.quantp7()
        )
    }
}
#[doc = "Transmit PFC Pause Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpfcpause(pub u32);
impl Txpfcpause {
    #[doc = "Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn vectorenb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub const fn set_vectorenb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Txpfcpause {
    #[inline(always)]
    fn default() -> Txpfcpause {
        Txpfcpause(0)
    }
}
impl core::fmt::Debug for Txpfcpause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txpfcpause")
            .field("vectorenb", &self.vectorenb())
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txpfcpause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txpfcpause {{ vectorenb: {=u8:?}, vector: {=u8:?} }}",
            self.vectorenb(),
            self.vector()
        )
    }
}
#[doc = "Start address of the transmit buffer queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txqptr(pub u32);
impl Txqptr {
    #[doc = "Transmit buffer queue base address."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatxqptr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Transmit buffer queue base address."]
    #[inline(always)]
    pub const fn set_dmatxqptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Txqptr {
    #[inline(always)]
    fn default() -> Txqptr {
        Txqptr(0)
    }
}
impl core::fmt::Debug for Txqptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txqptr")
            .field("dmatxqptr", &self.dmatxqptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txqptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txqptr {{ dmatxqptr: {=u32:?} }}", self.dmatxqptr())
    }
}
#[doc = "Transmit status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txstatus(pub u32);
impl Txstatus {
    #[doc = "Used bit read."]
    #[must_use]
    #[inline(always)]
    pub const fn usedbitread(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Used bit read."]
    #[inline(always)]
    pub const fn set_usedbitread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Collision occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn coloccrd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Collision occurred."]
    #[inline(always)]
    pub const fn set_coloccrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retry limit exceeded."]
    #[must_use]
    #[inline(always)]
    pub const fn retrylmtexcd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Retry limit exceeded."]
    #[inline(always)]
    pub const fn set_retrylmtexcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit go."]
    #[must_use]
    #[inline(always)]
    pub const fn txgo(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit go."]
    #[inline(always)]
    pub const fn set_txgo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) errors."]
    #[must_use]
    #[inline(always)]
    pub const fn ambaerr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub const fn set_ambaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit complete."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmplt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete."]
    #[inline(always)]
    pub const fn set_txcmplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit under run."]
    #[must_use]
    #[inline(always)]
    pub const fn txunderrun(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit under run."]
    #[inline(always)]
    pub const fn set_txunderrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Late collision occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn latecoloccrd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Late collision occurred."]
    #[inline(always)]
    pub const fn set_latecoloccrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "bresp/hresp not OK."]
    #[must_use]
    #[inline(always)]
    pub const fn respnotok(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "bresp/hresp not OK."]
    #[inline(always)]
    pub const fn set_respnotok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Txstatus {
    #[inline(always)]
    fn default() -> Txstatus {
        Txstatus(0)
    }
}
impl core::fmt::Debug for Txstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txstatus")
            .field("usedbitread", &self.usedbitread())
            .field("coloccrd", &self.coloccrd())
            .field("retrylmtexcd", &self.retrylmtexcd())
            .field("txgo", &self.txgo())
            .field("ambaerr", &self.ambaerr())
            .field("txcmplt", &self.txcmplt())
            .field("txunderrun", &self.txunderrun())
            .field("latecoloccrd", &self.latecoloccrd())
            .field("respnotok", &self.respnotok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Txstatus {{ usedbitread: {=bool:?}, coloccrd: {=bool:?}, retrylmtexcd: {=bool:?}, txgo: {=bool:?}, ambaerr: {=bool:?}, txcmplt: {=bool:?}, txunderrun: {=bool:?}, latecoloccrd: {=bool:?}, respnotok: {=bool:?} }}" , self . usedbitread () , self . coloccrd () , self . retrylmtexcd () , self . txgo () , self . ambaerr () , self . txcmplt () , self . txunderrun () , self . latecoloccrd () , self . respnotok ())
    }
}
#[doc = "Transmit Under Runs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txunderruns(pub u32);
impl Txunderruns {
    #[doc = "Transmit under runs."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Transmit under runs."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Txunderruns {
    #[inline(always)]
    fn default() -> Txunderruns {
        Txunderruns(0)
    }
}
impl core::fmt::Debug for Txunderruns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txunderruns")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txunderruns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txunderruns {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Undersized Frames Received."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Undersizeframes(pub u32);
impl Undersizeframes {
    #[doc = "Undersize frames received."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Undersize frames received."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Undersizeframes {
    #[inline(always)]
    fn default() -> Undersizeframes {
        Undersizeframes(0)
    }
}
impl core::fmt::Debug for Undersizeframes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Undersizeframes")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Undersizeframes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Undersizeframes {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Wake on LAN Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wolreg(pub u32);
impl Wolreg {
    #[doc = "Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Wake on LAN magic packet event enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wolmask0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on LAN magic packet event enable."]
    #[inline(always)]
    pub const fn set_wolmask0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake on LAN ARP request event enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wolmask1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on LAN ARP request event enable."]
    #[inline(always)]
    pub const fn set_wolmask1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake on LAN specific address register 1 event enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wolmask2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on LAN specific address register 1 event enable."]
    #[inline(always)]
    pub const fn set_wolmask2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Wake on LAN multicast hash event enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wolmask3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on LAN multicast hash event enable."]
    #[inline(always)]
    pub const fn set_wolmask3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Wolreg {
    #[inline(always)]
    fn default() -> Wolreg {
        Wolreg(0)
    }
}
impl core::fmt::Debug for Wolreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wolreg")
            .field("addr", &self.addr())
            .field("wolmask0", &self.wolmask0())
            .field("wolmask1", &self.wolmask1())
            .field("wolmask2", &self.wolmask2())
            .field("wolmask3", &self.wolmask3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wolreg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wolreg {{ addr: {=u16:?}, wolmask0: {=bool:?}, wolmask1: {=bool:?}, wolmask2: {=bool:?}, wolmask3: {=bool:?} }}" , self . addr () , self . wolmask0 () , self . wolmask1 () , self . wolmask2 () , self . wolmask3 ())
    }
}
