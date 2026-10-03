#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCtrl(pub u32);
impl ChCtrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::ChCtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::ChCtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::ChCtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::ChCtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::ChCtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::ChCtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for ChCtrl {
    #[inline(always)]
    fn default() -> ChCtrl {
        ChCtrl(0)
    }
}
impl core::fmt::Debug for ChCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCtrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChCtrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Set Event on PRS."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonprs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set Event on PRS."]
    #[inline(always)]
    pub const fn set_sevonprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SEVONPRS PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonprssel(&self) -> super::vals::Sevonprssel {
        let val = (self.0 >> 1usize) & 0x0f;
        super::vals::Sevonprssel::from_bits(val as u8)
    }
    #[doc = "SEVONPRS PRS Channel Select."]
    #[inline(always)]
    pub const fn set_sevonprssel(&mut self, val: super::vals::Sevonprssel) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val.to_bits() as u32) & 0x0f) << 1usize);
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
            .field("sevonprs", &self.sevonprs())
            .field("sevonprssel", &self.sevonprssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ sevonprs: {=bool:?}, sevonprssel: {:?} }}",
            self.sevonprs(),
            self.sevonprssel()
        )
    }
}
#[doc = "DMA Request 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaReq(pub u32);
impl DmaReq {
    #[doc = "DMA Request 0 PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Dmareq0Prssel {
        let val = (self.0 >> 6usize) & 0x0f;
        super::vals::Dmareq0Prssel::from_bits(val as u8)
    }
    #[doc = "DMA Request 0 PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Dmareq0Prssel) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val.to_bits() as u32) & 0x0f) << 6usize);
    }
}
impl Default for DmaReq {
    #[inline(always)]
    fn default() -> DmaReq {
        DmaReq(0)
    }
}
impl core::fmt::Debug for DmaReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaReq")
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaReq {{ prssel: {:?} }}", self.prssel())
    }
}
#[doc = "PRS Channel Values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek(pub u32);
impl Peek {
    #[doc = "Channel 0 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch_val(&self, n: usize) -> bool {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Current Value."]
    #[inline(always)]
    pub const fn set_ch_val(&mut self, n: usize, val: bool) {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Peek {
    #[inline(always)]
    fn default() -> Peek {
        Peek(0)
    }
}
impl core::fmt::Debug for Peek {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Peek")
            .field("ch_val[0]", &self.ch_val(0usize))
            .field("ch_val[1]", &self.ch_val(1usize))
            .field("ch_val[2]", &self.ch_val(2usize))
            .field("ch_val[3]", &self.ch_val(3usize))
            .field("ch_val[4]", &self.ch_val(4usize))
            .field("ch_val[5]", &self.ch_val(5usize))
            .field("ch_val[6]", &self.ch_val(6usize))
            .field("ch_val[7]", &self.ch_val(7usize))
            .field("ch_val[8]", &self.ch_val(8usize))
            .field("ch_val[9]", &self.ch_val(9usize))
            .field("ch_val[10]", &self.ch_val(10usize))
            .field("ch_val[11]", &self.ch_val(11usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Peek {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Peek {{ ch_val[0]: {=bool:?}, ch_val[1]: {=bool:?}, ch_val[2]: {=bool:?}, ch_val[3]: {=bool:?}, ch_val[4]: {=bool:?}, ch_val[5]: {=bool:?}, ch_val[6]: {=bool:?}, ch_val[7]: {=bool:?}, ch_val[8]: {=bool:?}, ch_val[9]: {=bool:?}, ch_val[10]: {=bool:?}, ch_val[11]: {=bool:?} }}" , self . ch_val (0usize) , self . ch_val (1usize) , self . ch_val (2usize) , self . ch_val (3usize) , self . ch_val (4usize) , self . ch_val (5usize) , self . ch_val (6usize) , self . ch_val (7usize) , self . ch_val (8usize) , self . ch_val (9usize) , self . ch_val (10usize) , self . ch_val (11usize))
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
    pub const fn ch0loc(&self) -> super::vals::Ch0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch0loc(&mut self, val: super::vals::Ch0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1loc(&self) -> super::vals::Ch1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch1loc(&mut self, val: super::vals::Ch1loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2loc(&self) -> super::vals::Ch2loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch2loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch2loc(&mut self, val: super::vals::Ch2loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3loc(&self) -> super::vals::Ch3loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch3loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch3loc(&mut self, val: super::vals::Ch3loc) {
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
            .field("ch0loc", &self.ch0loc())
            .field("ch1loc", &self.ch1loc())
            .field("ch2loc", &self.ch2loc())
            .field("ch3loc", &self.ch3loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ ch0loc: {:?}, ch1loc: {:?}, ch2loc: {:?}, ch3loc: {:?} }}",
            self.ch0loc(),
            self.ch1loc(),
            self.ch2loc(),
            self.ch3loc()
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
    pub const fn ch4loc(&self) -> super::vals::Ch4loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch4loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch4loc(&mut self, val: super::vals::Ch4loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5loc(&self) -> super::vals::Ch5loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch5loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch5loc(&mut self, val: super::vals::Ch5loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6loc(&self) -> super::vals::Ch6loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch6loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch6loc(&mut self, val: super::vals::Ch6loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7loc(&self) -> super::vals::Ch7loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch7loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch7loc(&mut self, val: super::vals::Ch7loc) {
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
            .field("ch4loc", &self.ch4loc())
            .field("ch5loc", &self.ch5loc())
            .field("ch6loc", &self.ch6loc())
            .field("ch7loc", &self.ch7loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc1 {{ ch4loc: {:?}, ch5loc: {:?}, ch6loc: {:?}, ch7loc: {:?} }}",
            self.ch4loc(),
            self.ch5loc(),
            self.ch6loc(),
            self.ch7loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc2(pub u32);
impl Routeloc2 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8loc(&self) -> super::vals::Ch8loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch8loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch8loc(&mut self, val: super::vals::Ch8loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9loc(&self) -> super::vals::Ch9loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch9loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch9loc(&mut self, val: super::vals::Ch9loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10loc(&self) -> super::vals::Ch10loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch10loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch10loc(&mut self, val: super::vals::Ch10loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11loc(&self) -> super::vals::Ch11loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch11loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch11loc(&mut self, val: super::vals::Ch11loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Routeloc2 {
    #[inline(always)]
    fn default() -> Routeloc2 {
        Routeloc2(0)
    }
}
impl core::fmt::Debug for Routeloc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc2")
            .field("ch8loc", &self.ch8loc())
            .field("ch9loc", &self.ch9loc())
            .field("ch10loc", &self.ch10loc())
            .field("ch11loc", &self.ch11loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc2 {{ ch8loc: {:?}, ch9loc: {:?}, ch10loc: {:?}, ch11loc: {:?} }}",
            self.ch8loc(),
            self.ch9loc(),
            self.ch10loc(),
            self.ch11loc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CH0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch_pen(&self, n: usize) -> bool {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CH0 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch_pen(&mut self, n: usize, val: bool) {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("ch_pen[0]", &self.ch_pen(0usize))
            .field("ch_pen[1]", &self.ch_pen(1usize))
            .field("ch_pen[2]", &self.ch_pen(2usize))
            .field("ch_pen[3]", &self.ch_pen(3usize))
            .field("ch_pen[4]", &self.ch_pen(4usize))
            .field("ch_pen[5]", &self.ch_pen(5usize))
            .field("ch_pen[6]", &self.ch_pen(6usize))
            .field("ch_pen[7]", &self.ch_pen(7usize))
            .field("ch_pen[8]", &self.ch_pen(8usize))
            .field("ch_pen[9]", &self.ch_pen(9usize))
            .field("ch_pen[10]", &self.ch_pen(10usize))
            .field("ch_pen[11]", &self.ch_pen(11usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ ch_pen[0]: {=bool:?}, ch_pen[1]: {=bool:?}, ch_pen[2]: {=bool:?}, ch_pen[3]: {=bool:?}, ch_pen[4]: {=bool:?}, ch_pen[5]: {=bool:?}, ch_pen[6]: {=bool:?}, ch_pen[7]: {=bool:?}, ch_pen[8]: {=bool:?}, ch_pen[9]: {=bool:?}, ch_pen[10]: {=bool:?}, ch_pen[11]: {=bool:?} }}" , self . ch_pen (0usize) , self . ch_pen (1usize) , self . ch_pen (2usize) , self . ch_pen (3usize) , self . ch_pen (4usize) , self . ch_pen (5usize) , self . ch_pen (6usize) , self . ch_pen (7usize) , self . ch_pen (8usize) , self . ch_pen (9usize) , self . ch_pen (10usize) , self . ch_pen (11usize))
    }
}
#[doc = "Software Level Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swlevel(pub u32);
impl Swlevel {
    #[doc = "Channel 0 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch_level(&self, n: usize) -> bool {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Software Level."]
    #[inline(always)]
    pub const fn set_ch_level(&mut self, n: usize, val: bool) {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Swlevel {
    #[inline(always)]
    fn default() -> Swlevel {
        Swlevel(0)
    }
}
impl core::fmt::Debug for Swlevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swlevel")
            .field("ch_level[0]", &self.ch_level(0usize))
            .field("ch_level[1]", &self.ch_level(1usize))
            .field("ch_level[2]", &self.ch_level(2usize))
            .field("ch_level[3]", &self.ch_level(3usize))
            .field("ch_level[4]", &self.ch_level(4usize))
            .field("ch_level[5]", &self.ch_level(5usize))
            .field("ch_level[6]", &self.ch_level(6usize))
            .field("ch_level[7]", &self.ch_level(7usize))
            .field("ch_level[8]", &self.ch_level(8usize))
            .field("ch_level[9]", &self.ch_level(9usize))
            .field("ch_level[10]", &self.ch_level(10usize))
            .field("ch_level[11]", &self.ch_level(11usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swlevel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swlevel {{ ch_level[0]: {=bool:?}, ch_level[1]: {=bool:?}, ch_level[2]: {=bool:?}, ch_level[3]: {=bool:?}, ch_level[4]: {=bool:?}, ch_level[5]: {=bool:?}, ch_level[6]: {=bool:?}, ch_level[7]: {=bool:?}, ch_level[8]: {=bool:?}, ch_level[9]: {=bool:?}, ch_level[10]: {=bool:?}, ch_level[11]: {=bool:?} }}" , self . ch_level (0usize) , self . ch_level (1usize) , self . ch_level (2usize) , self . ch_level (3usize) , self . ch_level (4usize) , self . ch_level (5usize) , self . ch_level (6usize) , self . ch_level (7usize) , self . ch_level (8usize) , self . ch_level (9usize) , self . ch_level (10usize) , self . ch_level (11usize))
    }
}
#[doc = "Software Pulse Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swpulse(pub u32);
impl Swpulse {
    #[doc = "Channel 0 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch_pulse(&self, n: usize) -> bool {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch_pulse(&mut self, n: usize, val: bool) {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Swpulse {
    #[inline(always)]
    fn default() -> Swpulse {
        Swpulse(0)
    }
}
impl core::fmt::Debug for Swpulse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swpulse")
            .field("ch_pulse[0]", &self.ch_pulse(0usize))
            .field("ch_pulse[1]", &self.ch_pulse(1usize))
            .field("ch_pulse[2]", &self.ch_pulse(2usize))
            .field("ch_pulse[3]", &self.ch_pulse(3usize))
            .field("ch_pulse[4]", &self.ch_pulse(4usize))
            .field("ch_pulse[5]", &self.ch_pulse(5usize))
            .field("ch_pulse[6]", &self.ch_pulse(6usize))
            .field("ch_pulse[7]", &self.ch_pulse(7usize))
            .field("ch_pulse[8]", &self.ch_pulse(8usize))
            .field("ch_pulse[9]", &self.ch_pulse(9usize))
            .field("ch_pulse[10]", &self.ch_pulse(10usize))
            .field("ch_pulse[11]", &self.ch_pulse(11usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpulse {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swpulse {{ ch_pulse[0]: {=bool:?}, ch_pulse[1]: {=bool:?}, ch_pulse[2]: {=bool:?}, ch_pulse[3]: {=bool:?}, ch_pulse[4]: {=bool:?}, ch_pulse[5]: {=bool:?}, ch_pulse[6]: {=bool:?}, ch_pulse[7]: {=bool:?}, ch_pulse[8]: {=bool:?}, ch_pulse[9]: {=bool:?}, ch_pulse[10]: {=bool:?}, ch_pulse[11]: {=bool:?} }}" , self . ch_pulse (0usize) , self . ch_pulse (1usize) , self . ch_pulse (2usize) , self . ch_pulse (3usize) , self . ch_pulse (4usize) , self . ch_pulse (5usize) , self . ch_pulse (6usize) , self . ch_pulse (7usize) , self . ch_pulse (8usize) , self . ch_pulse (9usize) , self . ch_pulse (10usize) , self . ch_pulse (11usize))
    }
}
