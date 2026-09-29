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
    pub const fn sevonprssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 1usize) & 0x0f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "SEVONPRS PRS Channel Select."]
    #[inline(always)]
    pub const fn set_sevonprssel(&mut self, val: super::vals::Prssel) {
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
    pub const fn prssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x0f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DMA Request 0 PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Prssel) {
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
    pub const fn ch0val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Current Value."]
    #[inline(always)]
    pub const fn set_ch0val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1val(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Current Value."]
    #[inline(always)]
    pub const fn set_ch1val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2val(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Current Value."]
    #[inline(always)]
    pub const fn set_ch2val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Current Value."]
    #[inline(always)]
    pub const fn set_ch3val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4val(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Current Value."]
    #[inline(always)]
    pub const fn set_ch4val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5val(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Current Value."]
    #[inline(always)]
    pub const fn set_ch5val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6val(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Current Value."]
    #[inline(always)]
    pub const fn set_ch6val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7val(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Current Value."]
    #[inline(always)]
    pub const fn set_ch7val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8val(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Current Value."]
    #[inline(always)]
    pub const fn set_ch8val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Current Value."]
    #[inline(always)]
    pub const fn set_ch9val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10val(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Current Value."]
    #[inline(always)]
    pub const fn set_ch10val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11val(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Current Value."]
    #[inline(always)]
    pub const fn set_ch11val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("ch0val", &self.ch0val())
            .field("ch1val", &self.ch1val())
            .field("ch2val", &self.ch2val())
            .field("ch3val", &self.ch3val())
            .field("ch4val", &self.ch4val())
            .field("ch5val", &self.ch5val())
            .field("ch6val", &self.ch6val())
            .field("ch7val", &self.ch7val())
            .field("ch8val", &self.ch8val())
            .field("ch9val", &self.ch9val())
            .field("ch10val", &self.ch10val())
            .field("ch11val", &self.ch11val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Peek {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Peek {{ ch0val: {=bool:?}, ch1val: {=bool:?}, ch2val: {=bool:?}, ch3val: {=bool:?}, ch4val: {=bool:?}, ch5val: {=bool:?}, ch6val: {=bool:?}, ch7val: {=bool:?}, ch8val: {=bool:?}, ch9val: {=bool:?}, ch10val: {=bool:?}, ch11val: {=bool:?} }}" , self . ch0val () , self . ch1val () , self . ch2val () , self . ch3val () , self . ch4val () , self . ch5val () , self . ch6val () , self . ch7val () , self . ch8val () , self . ch9val () , self . ch10val () , self . ch11val ())
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
    pub const fn ch0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH2 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH3 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH4 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4pen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH4 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH5 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH5 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH6 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6pen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH6 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH7 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7pen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH7 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CH8 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CH8 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch8pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CH9 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CH9 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch9pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CH10 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10pen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CH10 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch10pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CH11 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11pen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CH11 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch11pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("ch0pen", &self.ch0pen())
            .field("ch1pen", &self.ch1pen())
            .field("ch2pen", &self.ch2pen())
            .field("ch3pen", &self.ch3pen())
            .field("ch4pen", &self.ch4pen())
            .field("ch5pen", &self.ch5pen())
            .field("ch6pen", &self.ch6pen())
            .field("ch7pen", &self.ch7pen())
            .field("ch8pen", &self.ch8pen())
            .field("ch9pen", &self.ch9pen())
            .field("ch10pen", &self.ch10pen())
            .field("ch11pen", &self.ch11pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ ch0pen: {=bool:?}, ch1pen: {=bool:?}, ch2pen: {=bool:?}, ch3pen: {=bool:?}, ch4pen: {=bool:?}, ch5pen: {=bool:?}, ch6pen: {=bool:?}, ch7pen: {=bool:?}, ch8pen: {=bool:?}, ch9pen: {=bool:?}, ch10pen: {=bool:?}, ch11pen: {=bool:?} }}" , self . ch0pen () , self . ch1pen () , self . ch2pen () , self . ch3pen () , self . ch4pen () , self . ch5pen () , self . ch6pen () , self . ch7pen () , self . ch8pen () , self . ch9pen () , self . ch10pen () , self . ch11pen ())
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
    pub const fn ch0level(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Software Level."]
    #[inline(always)]
    pub const fn set_ch0level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1level(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Software Level."]
    #[inline(always)]
    pub const fn set_ch1level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2level(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Software Level."]
    #[inline(always)]
    pub const fn set_ch2level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3level(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Software Level."]
    #[inline(always)]
    pub const fn set_ch3level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4level(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Software Level."]
    #[inline(always)]
    pub const fn set_ch4level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5level(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Software Level."]
    #[inline(always)]
    pub const fn set_ch5level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6level(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Software Level."]
    #[inline(always)]
    pub const fn set_ch6level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7level(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Software Level."]
    #[inline(always)]
    pub const fn set_ch7level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8level(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Software Level."]
    #[inline(always)]
    pub const fn set_ch8level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9level(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Software Level."]
    #[inline(always)]
    pub const fn set_ch9level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10level(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Software Level."]
    #[inline(always)]
    pub const fn set_ch10level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11level(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Software Level."]
    #[inline(always)]
    pub const fn set_ch11level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("ch0level", &self.ch0level())
            .field("ch1level", &self.ch1level())
            .field("ch2level", &self.ch2level())
            .field("ch3level", &self.ch3level())
            .field("ch4level", &self.ch4level())
            .field("ch5level", &self.ch5level())
            .field("ch6level", &self.ch6level())
            .field("ch7level", &self.ch7level())
            .field("ch8level", &self.ch8level())
            .field("ch9level", &self.ch9level())
            .field("ch10level", &self.ch10level())
            .field("ch11level", &self.ch11level())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swlevel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swlevel {{ ch0level: {=bool:?}, ch1level: {=bool:?}, ch2level: {=bool:?}, ch3level: {=bool:?}, ch4level: {=bool:?}, ch5level: {=bool:?}, ch6level: {=bool:?}, ch7level: {=bool:?}, ch8level: {=bool:?}, ch9level: {=bool:?}, ch10level: {=bool:?}, ch11level: {=bool:?} }}" , self . ch0level () , self . ch1level () , self . ch2level () , self . ch3level () , self . ch4level () , self . ch5level () , self . ch6level () , self . ch7level () , self . ch8level () , self . ch9level () , self . ch10level () , self . ch11level ())
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
    pub const fn ch0pulse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch0pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1pulse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch1pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2pulse(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch2pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3pulse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch3pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4pulse(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch4pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5pulse(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch5pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6pulse(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch6pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7pulse(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch7pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8pulse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch8pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9pulse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch9pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10pulse(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch10pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11pulse(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch11pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("ch0pulse", &self.ch0pulse())
            .field("ch1pulse", &self.ch1pulse())
            .field("ch2pulse", &self.ch2pulse())
            .field("ch3pulse", &self.ch3pulse())
            .field("ch4pulse", &self.ch4pulse())
            .field("ch5pulse", &self.ch5pulse())
            .field("ch6pulse", &self.ch6pulse())
            .field("ch7pulse", &self.ch7pulse())
            .field("ch8pulse", &self.ch8pulse())
            .field("ch9pulse", &self.ch9pulse())
            .field("ch10pulse", &self.ch10pulse())
            .field("ch11pulse", &self.ch11pulse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpulse {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swpulse {{ ch0pulse: {=bool:?}, ch1pulse: {=bool:?}, ch2pulse: {=bool:?}, ch3pulse: {=bool:?}, ch4pulse: {=bool:?}, ch5pulse: {=bool:?}, ch6pulse: {=bool:?}, ch7pulse: {=bool:?}, ch8pulse: {=bool:?}, ch9pulse: {=bool:?}, ch10pulse: {=bool:?}, ch11pulse: {=bool:?} }}" , self . ch0pulse () , self . ch1pulse () , self . ch2pulse () , self . ch3pulse () , self . ch4pulse () , self . ch5pulse () , self . ch6pulse () , self . ch7pulse () , self . ch8pulse () , self . ch9pulse () , self . ch10pulse () , self . ch11pulse ())
    }
}
