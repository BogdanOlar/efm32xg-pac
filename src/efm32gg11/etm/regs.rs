#[doc = "ETM Authentication Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmauthstatus(pub u32);
impl Etmauthstatus {
    #[doc = "Non-secure invasive Debug Status."]
    #[must_use]
    #[inline(always)]
    pub const fn nonsecinvdbg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Non-secure invasive Debug Status."]
    #[inline(always)]
    pub const fn set_nonsecinvdbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure non-invasive Debug Status."]
    #[must_use]
    #[inline(always)]
    pub const fn nonsecnoninvdbg(&self) -> super::vals::Nonsecnoninvdbg {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Nonsecnoninvdbg::from_bits(val as u8)
    }
    #[doc = "Non-secure non-invasive Debug Status."]
    #[inline(always)]
    pub const fn set_nonsecnoninvdbg(&mut self, val: super::vals::Nonsecnoninvdbg) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Secure invasive Debug Status."]
    #[must_use]
    #[inline(always)]
    pub const fn secinvdbg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Secure invasive Debug Status."]
    #[inline(always)]
    pub const fn set_secinvdbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Secure non-invasive Debug Status."]
    #[must_use]
    #[inline(always)]
    pub const fn secnoninvdbg(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Secure non-invasive Debug Status."]
    #[inline(always)]
    pub const fn set_secnoninvdbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Etmauthstatus {
    #[inline(always)]
    fn default() -> Etmauthstatus {
        Etmauthstatus(0)
    }
}
impl core::fmt::Debug for Etmauthstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmauthstatus")
            .field("nonsecinvdbg", &self.nonsecinvdbg())
            .field("nonsecnoninvdbg", &self.nonsecnoninvdbg())
            .field("secinvdbg", &self.secinvdbg())
            .field("secnoninvdbg", &self.secnoninvdbg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmauthstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmauthstatus {{ nonsecinvdbg: {=u8:?}, nonsecnoninvdbg: {:?}, secinvdbg: {=u8:?}, secnoninvdbg: {=u8:?} }}" , self . nonsecinvdbg () , self . nonsecnoninvdbg () , self . secinvdbg () , self . secnoninvdbg ())
    }
}
#[doc = "Configuration Code Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmccer(pub u32);
impl Etmccer {
    #[doc = "Extended External Input Selectors."]
    #[must_use]
    #[inline(always)]
    pub const fn extinpsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Extended External Input Selectors."]
    #[inline(always)]
    pub const fn set_extinpsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Extended External Input Bus."]
    #[must_use]
    #[inline(always)]
    pub const fn extinpbus(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0xff;
        val as u8
    }
    #[doc = "Extended External Input Bus."]
    #[inline(always)]
    pub const fn set_extinpbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
    }
    #[doc = "Readable Registers."]
    #[must_use]
    #[inline(always)]
    pub const fn readregs(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Readable Registers."]
    #[inline(always)]
    pub const fn set_readregs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Data Address comparisons."]
    #[must_use]
    #[inline(always)]
    pub const fn daddrcmp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Data Address comparisons."]
    #[inline(always)]
    pub const fn set_daddrcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Instrumentation Resources."]
    #[must_use]
    #[inline(always)]
    pub const fn instres(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Instrumentation Resources."]
    #[inline(always)]
    pub const fn set_instres(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "EmbeddedICE watchpoint inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn eicewpnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "EmbeddedICE watchpoint inputs."]
    #[inline(always)]
    pub const fn set_eicewpnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn teicewpnt(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs."]
    #[inline(always)]
    pub const fn set_teicewpnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "EmbeddedICE Behavior control Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn eiceimp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "EmbeddedICE Behavior control Implemented."]
    #[inline(always)]
    pub const fn set_eiceimp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Timestamping Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn timp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamping Implemented."]
    #[inline(always)]
    pub const fn set_timp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reduced Function Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rfcnt(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Reduced Function Counter."]
    #[inline(always)]
    pub const fn set_rfcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Timestamp Encoding."]
    #[must_use]
    #[inline(always)]
    pub const fn tenc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Encoding."]
    #[inline(always)]
    pub const fn set_tenc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Timestamp Size."]
    #[must_use]
    #[inline(always)]
    pub const fn tsize(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Size."]
    #[inline(always)]
    pub const fn set_tsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Etmccer {
    #[inline(always)]
    fn default() -> Etmccer {
        Etmccer(0)
    }
}
impl core::fmt::Debug for Etmccer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmccer")
            .field("extinpsel", &self.extinpsel())
            .field("extinpbus", &self.extinpbus())
            .field("readregs", &self.readregs())
            .field("daddrcmp", &self.daddrcmp())
            .field("instres", &self.instres())
            .field("eicewpnt", &self.eicewpnt())
            .field("teicewpnt", &self.teicewpnt())
            .field("eiceimp", &self.eiceimp())
            .field("timp", &self.timp())
            .field("rfcnt", &self.rfcnt())
            .field("tenc", &self.tenc())
            .field("tsize", &self.tsize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmccer {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmccer {{ extinpsel: {=u8:?}, extinpbus: {=u8:?}, readregs: {=bool:?}, daddrcmp: {=bool:?}, instres: {=u8:?}, eicewpnt: {=u8:?}, teicewpnt: {=bool:?}, eiceimp: {=bool:?}, timp: {=bool:?}, rfcnt: {=bool:?}, tenc: {=bool:?}, tsize: {=bool:?} }}" , self . extinpsel () , self . extinpbus () , self . readregs () , self . daddrcmp () , self . instres () , self . eicewpnt () , self . teicewpnt () , self . eiceimp () , self . timp () , self . rfcnt () , self . tenc () , self . tsize ())
    }
}
#[doc = "Configuration Code Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmccr(pub u32);
impl Etmccr {
    #[doc = "Number of Address Comparator Pairs."]
    #[must_use]
    #[inline(always)]
    pub const fn adrcmppair(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Address Comparator Pairs."]
    #[inline(always)]
    pub const fn set_adrcmppair(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of Data Value Comparators."]
    #[must_use]
    #[inline(always)]
    pub const fn datacmpnum(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Data Value Comparators."]
    #[inline(always)]
    pub const fn set_datacmpnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of Memeory Map Decoders."]
    #[must_use]
    #[inline(always)]
    pub const fn mmdeccnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Memeory Map Decoders."]
    #[inline(always)]
    pub const fn set_mmdeccnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Number of Counters."]
    #[must_use]
    #[inline(always)]
    pub const fn countnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Counters."]
    #[inline(always)]
    pub const fn set_countnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Sequencer Present."]
    #[must_use]
    #[inline(always)]
    pub const fn seqpres(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Sequencer Present."]
    #[inline(always)]
    pub const fn set_seqpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Number of External Inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn extinpnum(&self) -> super::vals::Extinpnum {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Extinpnum::from_bits(val as u8)
    }
    #[doc = "Number of External Inputs."]
    #[inline(always)]
    pub const fn set_extinpnum(&mut self, val: super::vals::Extinpnum) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Number of External Output."]
    #[must_use]
    #[inline(always)]
    pub const fn extoutnum(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Number of External Output."]
    #[inline(always)]
    pub const fn set_extoutnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "FIFIO FULL present."]
    #[must_use]
    #[inline(always)]
    pub const fn fifofullpres(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "FIFIO FULL present."]
    #[inline(always)]
    pub const fn set_fifofullpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of context ID Comparators."]
    #[must_use]
    #[inline(always)]
    pub const fn idcompnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Number of context ID Comparators."]
    #[inline(always)]
    pub const fn set_idcompnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Trace Start/Stop Block Present."]
    #[must_use]
    #[inline(always)]
    pub const fn tracess(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Start/Stop Block Present."]
    #[inline(always)]
    pub const fn set_tracess(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor and Memeory Access."]
    #[must_use]
    #[inline(always)]
    pub const fn mmaccess(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor and Memeory Access."]
    #[inline(always)]
    pub const fn set_mmaccess(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ETM ID Register Present."]
    #[must_use]
    #[inline(always)]
    pub const fn etmid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ETM ID Register Present."]
    #[inline(always)]
    pub const fn set_etmid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Etmccr {
    #[inline(always)]
    fn default() -> Etmccr {
        Etmccr(0)
    }
}
impl core::fmt::Debug for Etmccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmccr")
            .field("adrcmppair", &self.adrcmppair())
            .field("datacmpnum", &self.datacmpnum())
            .field("mmdeccnt", &self.mmdeccnt())
            .field("countnum", &self.countnum())
            .field("seqpres", &self.seqpres())
            .field("extinpnum", &self.extinpnum())
            .field("extoutnum", &self.extoutnum())
            .field("fifofullpres", &self.fifofullpres())
            .field("idcompnum", &self.idcompnum())
            .field("tracess", &self.tracess())
            .field("mmaccess", &self.mmaccess())
            .field("etmid", &self.etmid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmccr {{ adrcmppair: {=u8:?}, datacmpnum: {=u8:?}, mmdeccnt: {=u8:?}, countnum: {=u8:?}, seqpres: {=bool:?}, extinpnum: {:?}, extoutnum: {=u8:?}, fifofullpres: {=bool:?}, idcompnum: {=u8:?}, tracess: {=bool:?}, mmaccess: {=bool:?}, etmid: {=bool:?} }}" , self . adrcmppair () , self . datacmpnum () , self . mmdeccnt () , self . countnum () , self . seqpres () , self . extinpnum () , self . extoutnum () , self . fifofullpres () , self . idcompnum () , self . tracess () , self . mmaccess () , self . etmid ())
    }
}
#[doc = "Component ID0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcidr0(pub u32);
impl Etmcidr0 {
    #[doc = "CoreSight Preamble."]
    #[must_use]
    #[inline(always)]
    pub const fn preamb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CoreSight Preamble."]
    #[inline(always)]
    pub const fn set_preamb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmcidr0 {
    #[inline(always)]
    fn default() -> Etmcidr0 {
        Etmcidr0(0)
    }
}
impl core::fmt::Debug for Etmcidr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcidr0")
            .field("preamb", &self.preamb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcidr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmcidr0 {{ preamb: {=u8:?} }}", self.preamb())
    }
}
#[doc = "Component ID1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcidr1(pub u32);
impl Etmcidr1 {
    #[doc = "CoreSight Preamble."]
    #[must_use]
    #[inline(always)]
    pub const fn preamb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CoreSight Preamble."]
    #[inline(always)]
    pub const fn set_preamb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmcidr1 {
    #[inline(always)]
    fn default() -> Etmcidr1 {
        Etmcidr1(0)
    }
}
impl core::fmt::Debug for Etmcidr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcidr1")
            .field("preamb", &self.preamb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcidr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmcidr1 {{ preamb: {=u8:?} }}", self.preamb())
    }
}
#[doc = "Component ID2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcidr2(pub u32);
impl Etmcidr2 {
    #[doc = "CoreSight Preamble."]
    #[must_use]
    #[inline(always)]
    pub const fn preamb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CoreSight Preamble."]
    #[inline(always)]
    pub const fn set_preamb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmcidr2 {
    #[inline(always)]
    fn default() -> Etmcidr2 {
        Etmcidr2(0)
    }
}
impl core::fmt::Debug for Etmcidr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcidr2")
            .field("preamb", &self.preamb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcidr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmcidr2 {{ preamb: {=u8:?} }}", self.preamb())
    }
}
#[doc = "Component ID3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcidr3(pub u32);
impl Etmcidr3 {
    #[doc = "CoreSight Preamble."]
    #[must_use]
    #[inline(always)]
    pub const fn preamb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CoreSight Preamble."]
    #[inline(always)]
    pub const fn set_preamb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmcidr3 {
    #[inline(always)]
    fn default() -> Etmcidr3 {
        Etmcidr3(0)
    }
}
impl core::fmt::Debug for Etmcidr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcidr3")
            .field("preamb", &self.preamb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcidr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmcidr3 {{ preamb: {=u8:?} }}", self.preamb())
    }
}
#[doc = "ETM Claim Tag Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmclaimclr(pub u32);
impl Etmclaimclr {
    #[doc = "Tag Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrtag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tag Bits."]
    #[inline(always)]
    pub const fn set_clrtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmclaimclr {
    #[inline(always)]
    fn default() -> Etmclaimclr {
        Etmclaimclr(0)
    }
}
impl core::fmt::Debug for Etmclaimclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmclaimclr")
            .field("clrtag", &self.clrtag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmclaimclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmclaimclr {{ clrtag: {=bool:?} }}", self.clrtag())
    }
}
#[doc = "ETM Claim Tag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmclaimset(pub u32);
impl Etmclaimset {
    #[doc = "Tag Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn settag(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Tag Bits."]
    #[inline(always)]
    pub const fn set_settag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmclaimset {
    #[inline(always)]
    fn default() -> Etmclaimset {
        Etmclaimset(0)
    }
}
impl core::fmt::Debug for Etmclaimset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmclaimset")
            .field("settag", &self.settag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmclaimset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmclaimset {{ settag: {=u8:?} }}", self.settag())
    }
}
#[doc = "Counter Reload Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcntrldvr1(pub u32);
impl Etmcntrldvr1 {
    #[doc = "Free running counter reload value."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free running counter reload value."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Etmcntrldvr1 {
    #[inline(always)]
    fn default() -> Etmcntrldvr1 {
        Etmcntrldvr1(0)
    }
}
impl core::fmt::Debug for Etmcntrldvr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcntrldvr1")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcntrldvr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmcntrldvr1 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Main Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmcr(pub u32);
impl Etmcr {
    #[doc = "ETM Control in low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdwn(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Control in low power mode."]
    #[inline(always)]
    pub const fn set_powerdwn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ETM Port Size."]
    #[must_use]
    #[inline(always)]
    pub const fn portsize(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "ETM Port Size."]
    #[inline(always)]
    pub const fn set_portsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Stall Processor."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Stall Processor."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Branch Output."]
    #[must_use]
    #[inline(always)]
    pub const fn branchoutput(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Branch Output."]
    #[inline(always)]
    pub const fn set_branchoutput(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Debug Request Control."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgreqctrl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Request Control."]
    #[inline(always)]
    pub const fn set_dbgreqctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ETM Programming."]
    #[must_use]
    #[inline(always)]
    pub const fn etmprog(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Programming."]
    #[inline(always)]
    pub const fn set_etmprog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ETM Port Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn etmportsel(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Port Selection."]
    #[inline(always)]
    pub const fn set_etmportsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Mode\\[2\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn portmode2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode\\[2\\]."]
    #[inline(always)]
    pub const fn set_portmode2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Mode Control."]
    #[must_use]
    #[inline(always)]
    pub const fn portmode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port Mode Control."]
    #[inline(always)]
    pub const fn set_portmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port Size\\[3\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn eportsize(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Port Size\\[3\\]."]
    #[inline(always)]
    pub const fn set_eportsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "Time Stamp Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tstampen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Stamp Enable."]
    #[inline(always)]
    pub const fn set_tstampen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Etmcr {
    #[inline(always)]
    fn default() -> Etmcr {
        Etmcr(0)
    }
}
impl core::fmt::Debug for Etmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmcr")
            .field("powerdwn", &self.powerdwn())
            .field("portsize", &self.portsize())
            .field("stall", &self.stall())
            .field("branchoutput", &self.branchoutput())
            .field("dbgreqctrl", &self.dbgreqctrl())
            .field("etmprog", &self.etmprog())
            .field("etmportsel", &self.etmportsel())
            .field("portmode2", &self.portmode2())
            .field("portmode", &self.portmode())
            .field("eportsize", &self.eportsize())
            .field("tstampen", &self.tstampen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmcr {{ powerdwn: {=bool:?}, portsize: {=u8:?}, stall: {=bool:?}, branchoutput: {=bool:?}, dbgreqctrl: {=bool:?}, etmprog: {=bool:?}, etmportsel: {=bool:?}, portmode2: {=bool:?}, portmode: {=u8:?}, eportsize: {=u8:?}, tstampen: {=bool:?} }}" , self . powerdwn () , self . portsize () , self . stall () , self . branchoutput () , self . dbgreqctrl () , self . etmprog () , self . etmportsel () , self . portmode2 () , self . portmode () , self . eportsize () , self . tstampen ())
    }
}
#[doc = "CoreSight Device Type Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmdevtype(pub u32);
impl Etmdevtype {
    #[doc = "Trace Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tracesrc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Trace Source."]
    #[inline(always)]
    pub const fn set_tracesrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Processor Trace."]
    #[must_use]
    #[inline(always)]
    pub const fn proctrace(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Processor Trace."]
    #[inline(always)]
    pub const fn set_proctrace(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Etmdevtype {
    #[inline(always)]
    fn default() -> Etmdevtype {
        Etmdevtype(0)
    }
}
impl core::fmt::Debug for Etmdevtype {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmdevtype")
            .field("tracesrc", &self.tracesrc())
            .field("proctrace", &self.proctrace())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmdevtype {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmdevtype {{ tracesrc: {=u8:?}, proctrace: {=u8:?} }}",
            self.tracesrc(),
            self.proctrace()
        )
    }
}
#[doc = "ETM Fifo Full Level Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmfflr(pub u32);
impl Etmfflr {
    #[doc = "Bytes left in FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn bytenum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bytes left in FIFO."]
    #[inline(always)]
    pub const fn set_bytenum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmfflr {
    #[inline(always)]
    fn default() -> Etmfflr {
        Etmfflr(0)
    }
}
impl core::fmt::Debug for Etmfflr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmfflr")
            .field("bytenum", &self.bytenum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmfflr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmfflr {{ bytenum: {=u8:?} }}", self.bytenum())
    }
}
#[doc = "ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmidr(pub u32);
impl Etmidr {
    #[doc = "Implementation Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn impver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Implementation Revision."]
    #[inline(always)]
    pub const fn set_impver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Minor ETM Architecture Version."]
    #[must_use]
    #[inline(always)]
    pub const fn etmminver(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor ETM Architecture Version."]
    #[inline(always)]
    pub const fn set_etmminver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Major ETM Architecture Version."]
    #[must_use]
    #[inline(always)]
    pub const fn etmmajver(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major ETM Architecture Version."]
    #[inline(always)]
    pub const fn set_etmmajver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Implementer Code."]
    #[must_use]
    #[inline(always)]
    pub const fn procfam(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Implementer Code."]
    #[inline(always)]
    pub const fn set_procfam(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Load PC First."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Load PC First."]
    #[inline(always)]
    pub const fn set_lpcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "32-bit Thumb Instruction Tracing."]
    #[must_use]
    #[inline(always)]
    pub const fn thumbt(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "32-bit Thumb Instruction Tracing."]
    #[inline(always)]
    pub const fn set_thumbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Security Extension Support."]
    #[must_use]
    #[inline(always)]
    pub const fn secext(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Security Extension Support."]
    #[inline(always)]
    pub const fn set_secext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Branch Packet Encoding."]
    #[must_use]
    #[inline(always)]
    pub const fn bpe(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Branch Packet Encoding."]
    #[inline(always)]
    pub const fn set_bpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Implementer Code."]
    #[must_use]
    #[inline(always)]
    pub const fn impcode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Implementer Code."]
    #[inline(always)]
    pub const fn set_impcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Etmidr {
    #[inline(always)]
    fn default() -> Etmidr {
        Etmidr(0)
    }
}
impl core::fmt::Debug for Etmidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmidr")
            .field("impver", &self.impver())
            .field("etmminver", &self.etmminver())
            .field("etmmajver", &self.etmmajver())
            .field("procfam", &self.procfam())
            .field("lpcf", &self.lpcf())
            .field("thumbt", &self.thumbt())
            .field("secext", &self.secext())
            .field("bpe", &self.bpe())
            .field("impcode", &self.impcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmidr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmidr {{ impver: {=u8:?}, etmminver: {=u8:?}, etmmajver: {=u8:?}, procfam: {=u8:?}, lpcf: {=bool:?}, thumbt: {=bool:?}, secext: {=bool:?}, bpe: {=bool:?}, impcode: {=u8:?} }}" , self . impver () , self . etmminver () , self . etmmajver () , self . procfam () , self . lpcf () , self . thumbt () , self . secext () , self . bpe () , self . impcode ())
    }
}
#[doc = "ETM ID Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmidr2(pub u32);
impl Etmidr2 {
    #[doc = "RFE Transfer Order."]
    #[must_use]
    #[inline(always)]
    pub const fn rfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RFE Transfer Order."]
    #[inline(always)]
    pub const fn set_rfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SWP Transfer Order."]
    #[must_use]
    #[inline(always)]
    pub const fn swp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SWP Transfer Order."]
    #[inline(always)]
    pub const fn set_swp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Etmidr2 {
    #[inline(always)]
    fn default() -> Etmidr2 {
        Etmidr2(0)
    }
}
impl core::fmt::Debug for Etmidr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmidr2")
            .field("rfe", &self.rfe())
            .field("swp", &self.swp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmidr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmidr2 {{ rfe: {=bool:?}, swp: {=bool:?} }}",
            self.rfe(),
            self.swp()
        )
    }
}
#[doc = "Integration Test Miscellaneous Inputs Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmiscin(pub u32);
impl Etmiscin {
    #[doc = "EXTIN Value."]
    #[must_use]
    #[inline(always)]
    pub const fn extin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "EXTIN Value."]
    #[inline(always)]
    pub const fn set_extin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Core Halt."]
    #[must_use]
    #[inline(always)]
    pub const fn corehalt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Core Halt."]
    #[inline(always)]
    pub const fn set_corehalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Etmiscin {
    #[inline(always)]
    fn default() -> Etmiscin {
        Etmiscin(0)
    }
}
impl core::fmt::Debug for Etmiscin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmiscin")
            .field("extin", &self.extin())
            .field("corehalt", &self.corehalt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmiscin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmiscin {{ extin: {=u8:?}, corehalt: {=bool:?} }}",
            self.extin(),
            self.corehalt()
        )
    }
}
#[doc = "ETM Integration Test ATB Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmitatbctr0(pub u32);
impl Etmitatbctr0 {
    #[doc = "ATVALID Output Value."]
    #[must_use]
    #[inline(always)]
    pub const fn atvalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ATVALID Output Value."]
    #[inline(always)]
    pub const fn set_atvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmitatbctr0 {
    #[inline(always)]
    fn default() -> Etmitatbctr0 {
        Etmitatbctr0(0)
    }
}
impl core::fmt::Debug for Etmitatbctr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmitatbctr0")
            .field("atvalid", &self.atvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmitatbctr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmitatbctr0 {{ atvalid: {=bool:?} }}", self.atvalid())
    }
}
#[doc = "ETM Integration Test ATB Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmitatbctr2(pub u32);
impl Etmitatbctr2 {
    #[doc = "ATREADY Input Value."]
    #[must_use]
    #[inline(always)]
    pub const fn atready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ATREADY Input Value."]
    #[inline(always)]
    pub const fn set_atready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmitatbctr2 {
    #[inline(always)]
    fn default() -> Etmitatbctr2 {
        Etmitatbctr2(0)
    }
}
impl core::fmt::Debug for Etmitatbctr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmitatbctr2")
            .field("atready", &self.atready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmitatbctr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmitatbctr2 {{ atready: {=bool:?} }}", self.atready())
    }
}
#[doc = "ETM Integration Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmitctrl(pub u32);
impl Etmitctrl {
    #[doc = "Integration Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Integration Mode Enable."]
    #[inline(always)]
    pub const fn set_iten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmitctrl {
    #[inline(always)]
    fn default() -> Etmitctrl {
        Etmitctrl(0)
    }
}
impl core::fmt::Debug for Etmitctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmitctrl")
            .field("iten", &self.iten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmitctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmitctrl {{ iten: {=bool:?} }}", self.iten())
    }
}
#[doc = "ETM Lock Access Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmlar(pub u32);
impl Etmlar {
    #[doc = "Key Value."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Key Value."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmlar {
    #[inline(always)]
    fn default() -> Etmlar {
        Etmlar(0)
    }
}
impl core::fmt::Debug for Etmlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmlar").field("key", &self.key()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmlar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmlar {{ key: {=bool:?} }}", self.key())
    }
}
#[doc = "Lock Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmlsr(pub u32);
impl Etmlsr {
    #[doc = "ETM Locking Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn lockimp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Locking Implemented."]
    #[inline(always)]
    pub const fn set_lockimp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ETM locked."]
    #[must_use]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETM locked."]
    #[inline(always)]
    pub const fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Etmlsr {
    #[inline(always)]
    fn default() -> Etmlsr {
        Etmlsr(0)
    }
}
impl core::fmt::Debug for Etmlsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmlsr")
            .field("lockimp", &self.lockimp())
            .field("locked", &self.locked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmlsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmlsr {{ lockimp: {=bool:?}, locked: {=bool:?} }}",
            self.lockimp(),
            self.locked()
        )
    }
}
#[doc = "Device Power-down Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpdsr(pub u32);
impl Etmpdsr {
    #[doc = "ETM Powered Up."]
    #[must_use]
    #[inline(always)]
    pub const fn etmup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Powered Up."]
    #[inline(always)]
    pub const fn set_etmup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etmpdsr {
    #[inline(always)]
    fn default() -> Etmpdsr {
        Etmpdsr(0)
    }
}
impl core::fmt::Debug for Etmpdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpdsr")
            .field("etmup", &self.etmup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmpdsr {{ etmup: {=bool:?} }}", self.etmup())
    }
}
#[doc = "Peripheral ID0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpidr0(pub u32);
impl Etmpidr0 {
    #[doc = "Part Number."]
    #[must_use]
    #[inline(always)]
    pub const fn partnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Part Number."]
    #[inline(always)]
    pub const fn set_partnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Etmpidr0 {
    #[inline(always)]
    fn default() -> Etmpidr0 {
        Etmpidr0(0)
    }
}
impl core::fmt::Debug for Etmpidr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpidr0")
            .field("partnum", &self.partnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpidr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmpidr0 {{ partnum: {=u8:?} }}", self.partnum())
    }
}
#[doc = "Peripheral ID1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpidr1(pub u32);
impl Etmpidr1 {
    #[doc = "Part Number."]
    #[must_use]
    #[inline(always)]
    pub const fn partnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Part Number."]
    #[inline(always)]
    pub const fn set_partnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "JEP106 Identity Code."]
    #[must_use]
    #[inline(always)]
    pub const fn idcode(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "JEP106 Identity Code."]
    #[inline(always)]
    pub const fn set_idcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Etmpidr1 {
    #[inline(always)]
    fn default() -> Etmpidr1 {
        Etmpidr1(0)
    }
}
impl core::fmt::Debug for Etmpidr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpidr1")
            .field("partnum", &self.partnum())
            .field("idcode", &self.idcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpidr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmpidr1 {{ partnum: {=u8:?}, idcode: {=u8:?} }}",
            self.partnum(),
            self.idcode()
        )
    }
}
#[doc = "Peripheral ID2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpidr2(pub u32);
impl Etmpidr2 {
    #[doc = "JEP106 Identity Code."]
    #[must_use]
    #[inline(always)]
    pub const fn idcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "JEP106 Identity Code."]
    #[inline(always)]
    pub const fn set_idcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Always 1."]
    #[must_use]
    #[inline(always)]
    pub const fn always1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Always 1."]
    #[inline(always)]
    pub const fn set_always1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Revision."]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Etmpidr2 {
    #[inline(always)]
    fn default() -> Etmpidr2 {
        Etmpidr2(0)
    }
}
impl core::fmt::Debug for Etmpidr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpidr2")
            .field("idcode", &self.idcode())
            .field("always1", &self.always1())
            .field("rev", &self.rev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpidr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmpidr2 {{ idcode: {=u8:?}, always1: {=bool:?}, rev: {=u8:?} }}",
            self.idcode(),
            self.always1(),
            self.rev()
        )
    }
}
#[doc = "Peripheral ID3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpidr3(pub u32);
impl Etmpidr3 {
    #[doc = "Customer Modified."]
    #[must_use]
    #[inline(always)]
    pub const fn custmod(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Customer Modified."]
    #[inline(always)]
    pub const fn set_custmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RevAnd."]
    #[must_use]
    #[inline(always)]
    pub const fn revand(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "RevAnd."]
    #[inline(always)]
    pub const fn set_revand(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Etmpidr3 {
    #[inline(always)]
    fn default() -> Etmpidr3 {
        Etmpidr3(0)
    }
}
impl core::fmt::Debug for Etmpidr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpidr3")
            .field("custmod", &self.custmod())
            .field("revand", &self.revand())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpidr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmpidr3 {{ custmod: {=u8:?}, revand: {=u8:?} }}",
            self.custmod(),
            self.revand()
        )
    }
}
#[doc = "Peripheral ID4 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmpidr4(pub u32);
impl Etmpidr4 {
    #[doc = "JEP106 Continuation Code."]
    #[must_use]
    #[inline(always)]
    pub const fn contcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "JEP106 Continuation Code."]
    #[inline(always)]
    pub const fn set_contcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4KB Count."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "4KB Count."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Etmpidr4 {
    #[inline(always)]
    fn default() -> Etmpidr4 {
        Etmpidr4(0)
    }
}
impl core::fmt::Debug for Etmpidr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmpidr4")
            .field("contcode", &self.contcode())
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmpidr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmpidr4 {{ contcode: {=u8:?}, count: {=u8:?} }}",
            self.contcode(),
            self.count()
        )
    }
}
#[doc = "ETM System Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmscr(pub u32);
impl Etmscr {
    #[doc = "Maximum Port Size."]
    #[must_use]
    #[inline(always)]
    pub const fn maxportsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Maximum Port Size."]
    #[inline(always)]
    pub const fn set_maxportsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "FIFO FULL Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn fifofull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO FULL Supported."]
    #[inline(always)]
    pub const fn set_fifofull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Max Port Size\\[3\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn maxportsize3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Max Port Size\\[3\\]."]
    #[inline(always)]
    pub const fn set_maxportsize3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Size Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn portsize(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Size Supported."]
    #[inline(always)]
    pub const fn set_portsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Mode Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn portmode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port Mode Supported."]
    #[inline(always)]
    pub const fn set_portmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Number of Supported Processros."]
    #[must_use]
    #[inline(always)]
    pub const fn procnum(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Supported Processros."]
    #[inline(always)]
    pub const fn set_procnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "No Fetch Comparison."]
    #[must_use]
    #[inline(always)]
    pub const fn nofetchcomp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "No Fetch Comparison."]
    #[inline(always)]
    pub const fn set_nofetchcomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Etmscr {
    #[inline(always)]
    fn default() -> Etmscr {
        Etmscr(0)
    }
}
impl core::fmt::Debug for Etmscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmscr")
            .field("maxportsize", &self.maxportsize())
            .field("fifofull", &self.fifofull())
            .field("maxportsize3", &self.maxportsize3())
            .field("portsize", &self.portsize())
            .field("portmode", &self.portmode())
            .field("procnum", &self.procnum())
            .field("nofetchcomp", &self.nofetchcomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmscr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmscr {{ maxportsize: {=u8:?}, fifofull: {=bool:?}, maxportsize3: {=bool:?}, portsize: {=bool:?}, portmode: {=bool:?}, procnum: {=u8:?}, nofetchcomp: {=bool:?} }}" , self . maxportsize () , self . fifofull () , self . maxportsize3 () , self . portsize () , self . portmode () , self . procnum () , self . nofetchcomp ())
    }
}
#[doc = "ETM Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmsr(pub u32);
impl Etmsr {
    #[doc = "ETM Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn ethof(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Overflow."]
    #[inline(always)]
    pub const fn set_ethof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ETM Programming Bit Status."]
    #[must_use]
    #[inline(always)]
    pub const fn etmprogbit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETM Programming Bit Status."]
    #[inline(always)]
    pub const fn set_etmprogbit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Trace Start/Stop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tracestat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Start/Stop Status."]
    #[inline(always)]
    pub const fn set_tracestat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Trigger Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn trigbit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Bit."]
    #[inline(always)]
    pub const fn set_trigbit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Etmsr {
    #[inline(always)]
    fn default() -> Etmsr {
        Etmsr(0)
    }
}
impl core::fmt::Debug for Etmsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmsr")
            .field("ethof", &self.ethof())
            .field("etmprogbit", &self.etmprogbit())
            .field("tracestat", &self.tracestat())
            .field("trigbit", &self.trigbit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmsr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Etmsr {{ ethof: {=bool:?}, etmprogbit: {=bool:?}, tracestat: {=bool:?}, trigbit: {=bool:?} }}" , self . ethof () , self . etmprogbit () , self . tracestat () , self . trigbit ())
    }
}
#[doc = "Synchronisation Frequency Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmsyncfr(pub u32);
impl Etmsyncfr {
    #[doc = "Synchronisation Frequency Value."]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Synchronisation Frequency Value."]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Etmsyncfr {
    #[inline(always)]
    fn default() -> Etmsyncfr {
        Etmsyncfr(0)
    }
}
impl core::fmt::Debug for Etmsyncfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmsyncfr")
            .field("freq", &self.freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmsyncfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmsyncfr {{ freq: {=u16:?} }}", self.freq())
    }
}
#[doc = "ETM Trace control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmtecr1(pub u32);
impl Etmtecr1 {
    #[doc = "Address Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn adrcmp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Comparator."]
    #[inline(always)]
    pub const fn set_adrcmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memmap."]
    #[must_use]
    #[inline(always)]
    pub const fn memmap(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Memmap."]
    #[inline(always)]
    pub const fn set_memmap(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "Trace Include/Exclude Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn incexctl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Include/Exclude Flag."]
    #[inline(always)]
    pub const fn set_incexctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Trace Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Trace Control Enable."]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Etmtecr1 {
    #[inline(always)]
    fn default() -> Etmtecr1 {
        Etmtecr1(0)
    }
}
impl core::fmt::Debug for Etmtecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmtecr1")
            .field("adrcmp", &self.adrcmp())
            .field("memmap", &self.memmap())
            .field("incexctl", &self.incexctl())
            .field("tce", &self.tce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmtecr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmtecr1 {{ adrcmp: {=u8:?}, memmap: {=u16:?}, incexctl: {=bool:?}, tce: {=bool:?} }}",
            self.adrcmp(),
            self.memmap(),
            self.incexctl(),
            self.tce()
        )
    }
}
#[doc = "ETM TraceEnable Event Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmteevr(pub u32);
impl Etmteevr {
    #[doc = "ETM Resource A Trace Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn resa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource A Trace Enable."]
    #[inline(always)]
    pub const fn set_resa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ETM Resource B Trace Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn resb(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource B Trace Enable."]
    #[inline(always)]
    pub const fn set_resb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "ETM Function Trace Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn etmfcnen(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "ETM Function Trace Enable."]
    #[inline(always)]
    pub const fn set_etmfcnen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
}
impl Default for Etmteevr {
    #[inline(always)]
    fn default() -> Etmteevr {
        Etmteevr(0)
    }
}
impl core::fmt::Debug for Etmteevr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmteevr")
            .field("resa", &self.resa())
            .field("resb", &self.resb())
            .field("etmfcnen", &self.etmfcnen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmteevr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmteevr {{ resa: {=u8:?}, resb: {=u8:?}, etmfcnen: {=u8:?} }}",
            self.resa(),
            self.resb(),
            self.etmfcnen()
        )
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmtesseicr(pub u32);
impl Etmtesseicr {
    #[doc = "Stop Resource Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn startrsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Stop Resource Selection."]
    #[inline(always)]
    pub const fn set_startrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Stop Resource Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn stoprsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Stop Resource Selection."]
    #[inline(always)]
    pub const fn set_stoprsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Etmtesseicr {
    #[inline(always)]
    fn default() -> Etmtesseicr {
        Etmtesseicr(0)
    }
}
impl core::fmt::Debug for Etmtesseicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmtesseicr")
            .field("startrsel", &self.startrsel())
            .field("stoprsel", &self.stoprsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmtesseicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmtesseicr {{ startrsel: {=u8:?}, stoprsel: {=u8:?} }}",
            self.startrsel(),
            self.stoprsel()
        )
    }
}
#[doc = "CoreSight Trace ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmtraceidr(pub u32);
impl Etmtraceidr {
    #[doc = "Trace ID."]
    #[must_use]
    #[inline(always)]
    pub const fn traceid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Trace ID."]
    #[inline(always)]
    pub const fn set_traceid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Etmtraceidr {
    #[inline(always)]
    fn default() -> Etmtraceidr {
        Etmtraceidr(0)
    }
}
impl core::fmt::Debug for Etmtraceidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmtraceidr")
            .field("traceid", &self.traceid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmtraceidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etmtraceidr {{ traceid: {=u8:?} }}", self.traceid())
    }
}
#[doc = "ETM Trigger Event Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmtrigger(pub u32);
impl Etmtrigger {
    #[doc = "ETM Resource A."]
    #[must_use]
    #[inline(always)]
    pub const fn resa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource A."]
    #[inline(always)]
    pub const fn set_resa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ETM Resource B."]
    #[must_use]
    #[inline(always)]
    pub const fn resb(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource B."]
    #[inline(always)]
    pub const fn set_resb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "ETM Function."]
    #[must_use]
    #[inline(always)]
    pub const fn etmfcn(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "ETM Function."]
    #[inline(always)]
    pub const fn set_etmfcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
}
impl Default for Etmtrigger {
    #[inline(always)]
    fn default() -> Etmtrigger {
        Etmtrigger(0)
    }
}
impl core::fmt::Debug for Etmtrigger {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmtrigger")
            .field("resa", &self.resa())
            .field("resb", &self.resb())
            .field("etmfcn", &self.etmfcn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmtrigger {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmtrigger {{ resa: {=u8:?}, resb: {=u8:?}, etmfcn: {=u8:?} }}",
            self.resa(),
            self.resb(),
            self.etmfcn()
        )
    }
}
#[doc = "Timestamp Event Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etmtsevr(pub u32);
impl Etmtsevr {
    #[doc = "ETM Resource A Event."]
    #[must_use]
    #[inline(always)]
    pub const fn resaevt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource A Event."]
    #[inline(always)]
    pub const fn set_resaevt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ETM Resource B Event."]
    #[must_use]
    #[inline(always)]
    pub const fn resbevt(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "ETM Resource B Event."]
    #[inline(always)]
    pub const fn set_resbevt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "ETM Function Event."]
    #[must_use]
    #[inline(always)]
    pub const fn etmfcnevt(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "ETM Function Event."]
    #[inline(always)]
    pub const fn set_etmfcnevt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
}
impl Default for Etmtsevr {
    #[inline(always)]
    fn default() -> Etmtsevr {
        Etmtsevr(0)
    }
}
impl core::fmt::Debug for Etmtsevr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etmtsevr")
            .field("resaevt", &self.resaevt())
            .field("resbevt", &self.resbevt())
            .field("etmfcnevt", &self.etmfcnevt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etmtsevr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etmtsevr {{ resaevt: {=u8:?}, resbevt: {=u8:?}, etmfcnevt: {=u8:?} }}",
            self.resaevt(),
            self.resbevt(),
            self.etmfcnevt()
        )
    }
}
#[doc = "Integration Test Trigger Out Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ittrigout(pub u32);
impl Ittrigout {
    #[doc = "Trigger output value."]
    #[must_use]
    #[inline(always)]
    pub const fn triggerout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger output value."]
    #[inline(always)]
    pub const fn set_triggerout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ittrigout {
    #[inline(always)]
    fn default() -> Ittrigout {
        Ittrigout(0)
    }
}
impl core::fmt::Debug for Ittrigout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ittrigout")
            .field("triggerout", &self.triggerout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ittrigout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ittrigout {{ triggerout: {=bool:?} }}",
            self.triggerout()
        )
    }
}
