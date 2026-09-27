#[doc = "CC Channel Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcCcv(pub u32);
impl CcCcv {
    #[doc = "CC Channel Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ccv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CC Channel Value."]
    #[inline(always)]
    pub const fn set_ccv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CcCcv {
    #[inline(always)]
    fn default() -> CcCcv {
        CcCcv(0)
    }
}
impl core::fmt::Debug for CcCcv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcCcv").field("ccv", &self.ccv()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CcCcv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CcCcv {{ ccv: {=u16:?} }}", self.ccv())
    }
}
#[doc = "CC Channel Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcCcvb(pub u32);
impl CcCcvb {
    #[doc = "CC Channel Value Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvb(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CC Channel Value Buffer."]
    #[inline(always)]
    pub const fn set_ccvb(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CcCcvb {
    #[inline(always)]
    fn default() -> CcCcvb {
        CcCcvb(0)
    }
}
impl core::fmt::Debug for CcCcvb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcCcvb")
            .field("ccvb", &self.ccvb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CcCcvb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CcCcvb {{ ccvb: {=u16:?} }}", self.ccvb())
    }
}
#[doc = "CC Channel Value Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcCcvp(pub u32);
impl CcCcvp {
    #[doc = "CC Channel Value Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CC Channel Value Peek."]
    #[inline(always)]
    pub const fn set_ccvp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CcCcvp {
    #[inline(always)]
    fn default() -> CcCcvp {
        CcCcvp(0)
    }
}
impl core::fmt::Debug for CcCcvp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcCcvp")
            .field("ccvp", &self.ccvp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CcCcvp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CcCcvp {{ ccvp: {=u16:?} }}", self.ccvp())
    }
}
#[doc = "CC Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcCtrl(pub u32);
impl CcCtrl {
    #[doc = "CC Channel Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cc0CtrlMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cc0CtrlMode::from_bits(val as u8)
    }
    #[doc = "CC Channel Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cc0CtrlMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Output Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn outinv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output Invert."]
    #[inline(always)]
    pub const fn set_outinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare Output Initial State."]
    #[must_use]
    #[inline(always)]
    pub const fn coist(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Output Initial State."]
    #[inline(always)]
    pub const fn set_coist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Compare Match Output Action."]
    #[must_use]
    #[inline(always)]
    pub const fn cmoa(&self) -> super::vals::Cc0CtrlCmoa {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Cc0CtrlCmoa::from_bits(val as u8)
    }
    #[doc = "Compare Match Output Action."]
    #[inline(always)]
    pub const fn set_cmoa(&mut self, val: super::vals::Cc0CtrlCmoa) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Counter Overflow Output Action."]
    #[must_use]
    #[inline(always)]
    pub const fn cofoa(&self) -> super::vals::Cc0CtrlCofoa {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Cc0CtrlCofoa::from_bits(val as u8)
    }
    #[doc = "Counter Overflow Output Action."]
    #[inline(always)]
    pub const fn set_cofoa(&mut self, val: super::vals::Cc0CtrlCofoa) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Counter Underflow Output Action."]
    #[must_use]
    #[inline(always)]
    pub const fn cufoa(&self) -> super::vals::Cc0CtrlCufoa {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Cc0CtrlCufoa::from_bits(val as u8)
    }
    #[doc = "Counter Underflow Output Action."]
    #[inline(always)]
    pub const fn set_cufoa(&mut self, val: super::vals::Cc0CtrlCufoa) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Compare/Capture Channel PRS Input Channel Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Compare/Capture Channel PRS Input Channel Selection."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Input Capture Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn icedge(&self) -> super::vals::Cc0CtrlIcedge {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Cc0CtrlIcedge::from_bits(val as u8)
    }
    #[doc = "Input Capture Edge Select."]
    #[inline(always)]
    pub const fn set_icedge(&mut self, val: super::vals::Cc0CtrlIcedge) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Input Capture Event Control."]
    #[must_use]
    #[inline(always)]
    pub const fn icevctrl(&self) -> super::vals::Cc0CtrlIcevctrl {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Cc0CtrlIcevctrl::from_bits(val as u8)
    }
    #[doc = "Input Capture Event Control."]
    #[inline(always)]
    pub const fn set_icevctrl(&mut self, val: super::vals::Cc0CtrlIcevctrl) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "PRS Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn prsconf(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Configuration."]
    #[inline(always)]
    pub const fn set_prsconf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Input Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn insel(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Input Selection."]
    #[inline(always)]
    pub const fn set_insel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Digital Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Filter."]
    #[inline(always)]
    pub const fn set_filt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CcCtrl {
    #[inline(always)]
    fn default() -> CcCtrl {
        CcCtrl(0)
    }
}
impl core::fmt::Debug for CcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcCtrl")
            .field("mode", &self.mode())
            .field("outinv", &self.outinv())
            .field("coist", &self.coist())
            .field("cmoa", &self.cmoa())
            .field("cofoa", &self.cofoa())
            .field("cufoa", &self.cufoa())
            .field("prssel", &self.prssel())
            .field("icedge", &self.icedge())
            .field("icevctrl", &self.icevctrl())
            .field("prsconf", &self.prsconf())
            .field("insel", &self.insel())
            .field("filt", &self.filt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CcCtrl {{ mode: {:?}, outinv: {=bool:?}, coist: {=bool:?}, cmoa: {:?}, cofoa: {:?}, cufoa: {:?}, prssel: {:?}, icedge: {:?}, icevctrl: {:?}, prsconf: {=bool:?}, insel: {=bool:?}, filt: {=bool:?} }}" , self . mode () , self . outinv () , self . coist () , self . cmoa () , self . cofoa () , self . cufoa () , self . prssel () , self . icedge () , self . icevctrl () , self . prsconf () , self . insel () , self . filt ())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Start Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Timer."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Timer."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ start: {=bool:?}, stop: {=bool:?} }}",
            self.start(),
            self.stop()
        )
    }
}
#[doc = "Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Value."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::CtrlMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CtrlMode::from_bits(val as u8)
    }
    #[doc = "Timer Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::CtrlMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timer Start/Stop/Reload Synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Start/Stop/Reload Synchronization."]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "One-shot Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osmen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "One-shot Mode Enable."]
    #[inline(always)]
    pub const fn set_osmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Quadrature Decoder Mode Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn qdm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Quadrature Decoder Mode Selection."]
    #[inline(always)]
    pub const fn set_qdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Request Clear on Active."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaclract(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Clear on Active."]
    #[inline(always)]
    pub const fn set_dmaclract(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Rising Input Edge Action."]
    #[must_use]
    #[inline(always)]
    pub const fn risea(&self) -> super::vals::Risea {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Risea::from_bits(val as u8)
    }
    #[doc = "Timer Rising Input Edge Action."]
    #[inline(always)]
    pub const fn set_risea(&mut self, val: super::vals::Risea) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Timer Falling Input Edge Action."]
    #[must_use]
    #[inline(always)]
    pub const fn falla(&self) -> super::vals::Falla {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Falla::from_bits(val as u8)
    }
    #[doc = "Timer Falling Input Edge Action."]
    #[inline(always)]
    pub const fn set_falla(&mut self, val: super::vals::Falla) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "2x Count Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn x2cnt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "2x Count Mode."]
    #[inline(always)]
    pub const fn set_x2cnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Prescaler Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::Presc {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Presc::from_bits(val as u8)
    }
    #[doc = "Prescaler Setting."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::Presc) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Always Track Inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn ati(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Always Track Inputs."]
    #[inline(always)]
    pub const fn set_ati(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Reload-Start Sets Compare Output Initial State."]
    #[must_use]
    #[inline(always)]
    pub const fn rsscoist(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Reload-Start Sets Compare Output Initial State."]
    #[inline(always)]
    pub const fn set_rsscoist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("mode", &self.mode())
            .field("sync", &self.sync())
            .field("osmen", &self.osmen())
            .field("qdm", &self.qdm())
            .field("debugrun", &self.debugrun())
            .field("dmaclract", &self.dmaclract())
            .field("risea", &self.risea())
            .field("falla", &self.falla())
            .field("x2cnt", &self.x2cnt())
            .field("clksel", &self.clksel())
            .field("presc", &self.presc())
            .field("ati", &self.ati())
            .field("rsscoist", &self.rsscoist())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ mode: {:?}, sync: {=bool:?}, osmen: {=bool:?}, qdm: {=bool:?}, debugrun: {=bool:?}, dmaclract: {=bool:?}, risea: {:?}, falla: {:?}, x2cnt: {=bool:?}, clksel: {:?}, presc: {:?}, ati: {=bool:?}, rsscoist: {=bool:?} }}" , self . mode () , self . sync () , self . osmen () , self . qdm () , self . debugrun () , self . dmaclract () , self . risea () , self . falla () , self . x2cnt () , self . clksel () , self . presc () , self . ati () , self . rsscoist ())
    }
}
#[doc = "DTI Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtctrl(pub u32);
impl Dtctrl {
    #[doc = "DTI Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Enable."]
    #[inline(always)]
    pub const fn set_dten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTI Automatic Start-up Functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdas(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Automatic Start-up Functionality."]
    #[inline(always)]
    pub const fn set_dtdas(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DTI Inactive Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn dtipol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Inactive Polarity."]
    #[inline(always)]
    pub const fn set_dtipol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DTI Complementary Output Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcinv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Complementary Output Invert."]
    #[inline(always)]
    pub const fn set_dtcinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DTI PRS Source Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 4usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DTI PRS Source Channel Select."]
    #[inline(always)]
    pub const fn set_dtprssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "DTI Always Run."]
    #[must_use]
    #[inline(always)]
    pub const fn dtar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Always Run."]
    #[inline(always)]
    pub const fn set_dtar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DTI Fault Action on Timer Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn dtfats(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Fault Action on Timer Stop."]
    #[inline(always)]
    pub const fn set_dtfats(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DTI PRS Source Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprsen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS Source Enable."]
    #[inline(always)]
    pub const fn set_dtprsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Dtctrl {
    #[inline(always)]
    fn default() -> Dtctrl {
        Dtctrl(0)
    }
}
impl core::fmt::Debug for Dtctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtctrl")
            .field("dten", &self.dten())
            .field("dtdas", &self.dtdas())
            .field("dtipol", &self.dtipol())
            .field("dtcinv", &self.dtcinv())
            .field("dtprssel", &self.dtprssel())
            .field("dtar", &self.dtar())
            .field("dtfats", &self.dtfats())
            .field("dtprsen", &self.dtprsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dtctrl {{ dten: {=bool:?}, dtdas: {=bool:?}, dtipol: {=bool:?}, dtcinv: {=bool:?}, dtprssel: {:?}, dtar: {=bool:?}, dtfats: {=bool:?}, dtprsen: {=bool:?} }}" , self . dten () , self . dtdas () , self . dtipol () , self . dtcinv () , self . dtprssel () , self . dtar () , self . dtfats () , self . dtprsen ())
    }
}
#[doc = "DTI Fault Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtfault(pub u32);
impl Dtfault {
    #[doc = "DTI PRS 0 Fault."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS 0 Fault."]
    #[inline(always)]
    pub const fn set_dtprs0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTI PRS 1 Fault."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS 1 Fault."]
    #[inline(always)]
    pub const fn set_dtprs1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DTI Debugger Fault."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdbgf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Debugger Fault."]
    #[inline(always)]
    pub const fn set_dtdbgf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DTI Lockup Fault."]
    #[must_use]
    #[inline(always)]
    pub const fn dtlockupf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Lockup Fault."]
    #[inline(always)]
    pub const fn set_dtlockupf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Dtfault {
    #[inline(always)]
    fn default() -> Dtfault {
        Dtfault(0)
    }
}
impl core::fmt::Debug for Dtfault {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtfault")
            .field("dtprs0f", &self.dtprs0f())
            .field("dtprs1f", &self.dtprs1f())
            .field("dtdbgf", &self.dtdbgf())
            .field("dtlockupf", &self.dtlockupf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtfault {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dtfault {{ dtprs0f: {=bool:?}, dtprs1f: {=bool:?}, dtdbgf: {=bool:?}, dtlockupf: {=bool:?} }}" , self . dtprs0f () , self . dtprs1f () , self . dtdbgf () , self . dtlockupf ())
    }
}
#[doc = "DTI Fault Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtfaultc(pub u32);
impl Dtfaultc {
    #[doc = "DTI PRS0 Fault Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs0fc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS0 Fault Clear."]
    #[inline(always)]
    pub const fn set_dtprs0fc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTI PRS1 Fault Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs1fc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS1 Fault Clear."]
    #[inline(always)]
    pub const fn set_dtprs1fc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DTI Debugger Fault Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdbgfc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Debugger Fault Clear."]
    #[inline(always)]
    pub const fn set_dtdbgfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DTI Lockup Fault Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn tlockupfc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Lockup Fault Clear."]
    #[inline(always)]
    pub const fn set_tlockupfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Dtfaultc {
    #[inline(always)]
    fn default() -> Dtfaultc {
        Dtfaultc(0)
    }
}
impl core::fmt::Debug for Dtfaultc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtfaultc")
            .field("dtprs0fc", &self.dtprs0fc())
            .field("dtprs1fc", &self.dtprs1fc())
            .field("dtdbgfc", &self.dtdbgfc())
            .field("tlockupfc", &self.tlockupfc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtfaultc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dtfaultc {{ dtprs0fc: {=bool:?}, dtprs1fc: {=bool:?}, dtdbgfc: {=bool:?}, tlockupfc: {=bool:?} }}" , self . dtprs0fc () , self . dtprs1fc () , self . dtdbgfc () , self . tlockupfc ())
    }
}
#[doc = "DTI Fault Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtfc(pub u32);
impl Dtfc {
    #[doc = "DTI PRS Fault Source 0 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs0fsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DTI PRS Fault Source 0 Select."]
    #[inline(always)]
    pub const fn set_dtprs0fsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DTI PRS Fault Source 1 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs1fsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DTI PRS Fault Source 1 Select."]
    #[inline(always)]
    pub const fn set_dtprs1fsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "DTI Fault Action."]
    #[must_use]
    #[inline(always)]
    pub const fn dtfa(&self) -> super::vals::Dtfa {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dtfa::from_bits(val as u8)
    }
    #[doc = "DTI Fault Action."]
    #[inline(always)]
    pub const fn set_dtfa(&mut self, val: super::vals::Dtfa) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DTI PRS 0 Fault Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs0fen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS 0 Fault Enable."]
    #[inline(always)]
    pub const fn set_dtprs0fen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DTI PRS 1 Fault Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtprs1fen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DTI PRS 1 Fault Enable."]
    #[inline(always)]
    pub const fn set_dtprs1fen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DTI Debugger Fault Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdbgfen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Debugger Fault Enable."]
    #[inline(always)]
    pub const fn set_dtdbgfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DTI Lockup Fault Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtlockupfen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DTI Lockup Fault Enable."]
    #[inline(always)]
    pub const fn set_dtlockupfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Dtfc {
    #[inline(always)]
    fn default() -> Dtfc {
        Dtfc(0)
    }
}
impl core::fmt::Debug for Dtfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtfc")
            .field("dtprs0fsel", &self.dtprs0fsel())
            .field("dtprs1fsel", &self.dtprs1fsel())
            .field("dtfa", &self.dtfa())
            .field("dtprs0fen", &self.dtprs0fen())
            .field("dtprs1fen", &self.dtprs1fen())
            .field("dtdbgfen", &self.dtdbgfen())
            .field("dtlockupfen", &self.dtlockupfen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtfc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dtfc {{ dtprs0fsel: {:?}, dtprs1fsel: {:?}, dtfa: {:?}, dtprs0fen: {=bool:?}, dtprs1fen: {=bool:?}, dtdbgfen: {=bool:?}, dtlockupfen: {=bool:?} }}" , self . dtprs0fsel () , self . dtprs1fsel () , self . dtfa () , self . dtprs0fen () , self . dtprs1fen () , self . dtdbgfen () , self . dtlockupfen ())
    }
}
#[doc = "DTI Configuration Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtlock(pub u32);
impl Dtlock {
    #[doc = "DTI Lock Key."]
    #[must_use]
    #[inline(always)]
    pub const fn lockkey(&self) -> super::super::msc::vals::Lockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::super::msc::vals::Lockkey::from_bits(val as u16)
    }
    #[doc = "DTI Lock Key."]
    #[inline(always)]
    pub const fn set_lockkey(&mut self, val: super::super::msc::vals::Lockkey) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dtlock {
    #[inline(always)]
    fn default() -> Dtlock {
        Dtlock(0)
    }
}
impl core::fmt::Debug for Dtlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtlock")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dtlock {{ lockkey: {:?} }}", self.lockkey())
    }
}
#[doc = "DTI Output Generation Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtogen(pub u32);
impl Dtogen {
    #[doc = "DTI CC0 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcc0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CC0 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcc0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTI CC1 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcc1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CC1 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcc1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DTI CC2 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcc2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CC2 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcc2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DTI CDTI0 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcdti0en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CDTI0 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcdti0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DTI CDTI1 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcdti1en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CDTI1 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcdti1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DTI CDTI2 Output Generation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtogcdti2en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DTI CDTI2 Output Generation Enable."]
    #[inline(always)]
    pub const fn set_dtogcdti2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Dtogen {
    #[inline(always)]
    fn default() -> Dtogen {
        Dtogen(0)
    }
}
impl core::fmt::Debug for Dtogen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtogen")
            .field("dtogcc0en", &self.dtogcc0en())
            .field("dtogcc1en", &self.dtogcc1en())
            .field("dtogcc2en", &self.dtogcc2en())
            .field("dtogcdti0en", &self.dtogcdti0en())
            .field("dtogcdti1en", &self.dtogcdti1en())
            .field("dtogcdti2en", &self.dtogcdti2en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtogen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dtogen {{ dtogcc0en: {=bool:?}, dtogcc1en: {=bool:?}, dtogcc2en: {=bool:?}, dtogcdti0en: {=bool:?}, dtogcdti1en: {=bool:?}, dtogcdti2en: {=bool:?} }}" , self . dtogcc0en () , self . dtogcc1en () , self . dtogcc2en () , self . dtogcdti0en () , self . dtogcdti1en () , self . dtogcdti2en ())
    }
}
#[doc = "DTI Time Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dttime(pub u32);
impl Dttime {
    #[doc = "DTI Prescaler Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn dtpresc(&self) -> super::vals::Dtpresc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Dtpresc::from_bits(val as u8)
    }
    #[doc = "DTI Prescaler Setting."]
    #[inline(always)]
    pub const fn set_dtpresc(&mut self, val: super::vals::Dtpresc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DTI Rise-time."]
    #[must_use]
    #[inline(always)]
    pub const fn dtriset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "DTI Rise-time."]
    #[inline(always)]
    pub const fn set_dtriset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "DTI Fall-time."]
    #[must_use]
    #[inline(always)]
    pub const fn dtfallt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "DTI Fall-time."]
    #[inline(always)]
    pub const fn set_dtfallt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Dttime {
    #[inline(always)]
    fn default() -> Dttime {
        Dttime(0)
    }
}
impl core::fmt::Debug for Dttime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dttime")
            .field("dtpresc", &self.dtpresc())
            .field("dtriset", &self.dtriset())
            .field("dtfallt", &self.dtfallt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dttime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dttime {{ dtpresc: {:?}, dtriset: {=u8:?}, dtfallt: {=u8:?} }}",
            self.dtpresc(),
            self.dtriset(),
            self.dtfallt()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "OF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DIRCHG Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dirchg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DIRCHG Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dirchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CC0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CC0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CC1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CC1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CC2 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CC2 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CC3 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CC3 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ICBOF0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ICBOF0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_icbof0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ICBOF1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ICBOF1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_icbof1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ICBOF2 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ICBOF2 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_icbof2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ICBOF3 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ICBOF3 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_icbof3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("dirchg", &self.dirchg())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("cc3", &self.cc3())
            .field("icbof0", &self.icbof0())
            .field("icbof1", &self.icbof1())
            .field("icbof2", &self.icbof2())
            .field("icbof3", &self.icbof3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ of: {=bool:?}, uf: {=bool:?}, dirchg: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, icbof0: {=bool:?}, icbof1: {=bool:?}, icbof2: {=bool:?}, icbof3: {=bool:?} }}" , self . of () , self . uf () , self . dirchg () , self . cc0 () , self . cc1 () , self . cc2 () , self . cc3 () , self . icbof0 () , self . icbof1 () , self . icbof2 () , self . icbof3 ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Direction Change Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dirchg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Direction Change Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dirchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CC Channel 0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CC Channel 1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CC Channel 2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CC Channel 3 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 3 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CC Channel 0 Input Capture Buffer Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 0 Input Capture Buffer Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CC Channel 1 Input Capture Buffer Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 1 Input Capture Buffer Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CC Channel 2 Input Capture Buffer Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 2 Input Capture Buffer Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CC Channel 3 Input Capture Buffer Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 3 Input Capture Buffer Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("dirchg", &self.dirchg())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("cc3", &self.cc3())
            .field("icbof0", &self.icbof0())
            .field("icbof1", &self.icbof1())
            .field("icbof2", &self.icbof2())
            .field("icbof3", &self.icbof3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ of: {=bool:?}, uf: {=bool:?}, dirchg: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, icbof0: {=bool:?}, icbof1: {=bool:?}, icbof2: {=bool:?}, icbof3: {=bool:?} }}" , self . of () , self . uf () , self . dirchg () , self . cc0 () , self . cc1 () , self . cc2 () , self . cc3 () , self . icbof0 () , self . icbof1 () , self . icbof2 () , self . icbof3 ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set OF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set OF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set UF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set UF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set DIRCHG Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dirchg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set DIRCHG Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dirchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set CC0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set CC1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set CC2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set CC3 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC3 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set ICBOF0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set ICBOF0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set ICBOF1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set ICBOF1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set ICBOF2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set ICBOF2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set ICBOF3 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icbof3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set ICBOF3 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icbof3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("dirchg", &self.dirchg())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("cc3", &self.cc3())
            .field("icbof0", &self.icbof0())
            .field("icbof1", &self.icbof1())
            .field("icbof2", &self.icbof2())
            .field("icbof3", &self.icbof3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ of: {=bool:?}, uf: {=bool:?}, dirchg: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, icbof0: {=bool:?}, icbof1: {=bool:?}, icbof2: {=bool:?}, icbof3: {=bool:?} }}" , self . of () , self . uf () , self . dirchg () , self . cc0 () , self . cc1 () , self . cc2 () , self . cc3 () , self . icbof0 () , self . icbof1 () , self . icbof2 () , self . icbof3 ())
    }
}
#[doc = "TIMER Configuration Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Timer Lock Key."]
    #[must_use]
    #[inline(always)]
    pub const fn timerlockkey(&self) -> super::vals::Timerlockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Timerlockkey::from_bits(val as u16)
    }
    #[doc = "Timer Lock Key."]
    #[inline(always)]
    pub const fn set_timerlockkey(&mut self, val: super::vals::Timerlockkey) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("timerlockkey", &self.timerlockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lock {{ timerlockkey: {:?} }}", self.timerlockkey())
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
    pub const fn cc0loc(&self) -> super::vals::Cc0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cc0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cc0loc(&mut self, val: super::vals::Cc0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1loc(&self) -> super::vals::Cc1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Cc1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cc1loc(&mut self, val: super::vals::Cc1loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2loc(&self) -> super::vals::Cc2loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Cc2loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cc2loc(&mut self, val: super::vals::Cc2loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn cc3loc(&self) -> super::vals::Cc3loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Cc3loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cc3loc(&mut self, val: super::vals::Cc3loc) {
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
            .field("cc0loc", &self.cc0loc())
            .field("cc1loc", &self.cc1loc())
            .field("cc2loc", &self.cc2loc())
            .field("cc3loc", &self.cc3loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ cc0loc: {:?}, cc1loc: {:?}, cc2loc: {:?}, cc3loc: {:?} }}",
            self.cc0loc(),
            self.cc1loc(),
            self.cc2loc(),
            self.cc3loc()
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
    pub const fn cdti0loc(&self) -> super::vals::Cdti0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cdti0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cdti0loc(&mut self, val: super::vals::Cdti0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn cdti1loc(&self) -> super::vals::Cdti1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Cdti1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cdti1loc(&mut self, val: super::vals::Cdti1loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn cdti2loc(&self) -> super::vals::Cdti2loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Cdti2loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_cdti2loc(&mut self, val: super::vals::Cdti2loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
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
            .field("cdti0loc", &self.cdti0loc())
            .field("cdti1loc", &self.cdti1loc())
            .field("cdti2loc", &self.cdti2loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc2 {{ cdti0loc: {:?}, cdti1loc: {:?}, cdti2loc: {:?} }}",
            self.cdti0loc(),
            self.cdti1loc(),
            self.cdti2loc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CC Channel 0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 0 Pin Enable."]
    #[inline(always)]
    pub const fn set_cc0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CC Channel 1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 1 Pin Enable."]
    #[inline(always)]
    pub const fn set_cc1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CC Channel 2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 2 Pin Enable."]
    #[inline(always)]
    pub const fn set_cc2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CC Channel 3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc3pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 3 Pin Enable."]
    #[inline(always)]
    pub const fn set_cc3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CC Channel 0 Complementary Dead-Time Insertion Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cdti0pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 0 Complementary Dead-Time Insertion Pin Enable."]
    #[inline(always)]
    pub const fn set_cdti0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CC Channel 1 Complementary Dead-Time Insertion Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cdti1pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 1 Complementary Dead-Time Insertion Pin Enable."]
    #[inline(always)]
    pub const fn set_cdti1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CC Channel 2 Complementary Dead-Time Insertion Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cdti2pen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel 2 Complementary Dead-Time Insertion Pin Enable."]
    #[inline(always)]
    pub const fn set_cdti2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("cc0pen", &self.cc0pen())
            .field("cc1pen", &self.cc1pen())
            .field("cc2pen", &self.cc2pen())
            .field("cc3pen", &self.cc3pen())
            .field("cdti0pen", &self.cdti0pen())
            .field("cdti1pen", &self.cdti1pen())
            .field("cdti2pen", &self.cdti2pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, cc3pen: {=bool:?}, cdti0pen: {=bool:?}, cdti1pen: {=bool:?}, cdti2pen: {=bool:?} }}" , self . cc0pen () , self . cc1pen () , self . cc2pen () , self . cc3pen () , self . cdti0pen () , self . cdti1pen () , self . cdti2pen ())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Running."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Running."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TOPB Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn topbv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TOPB Valid."]
    #[inline(always)]
    pub const fn set_topbv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CC0 CCVB Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvbv0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CC0 CCVB Valid."]
    #[inline(always)]
    pub const fn set_ccvbv0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CC1 CCVB Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvbv1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CC1 CCVB Valid."]
    #[inline(always)]
    pub const fn set_ccvbv1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CC2 CCVB Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvbv2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CC2 CCVB Valid."]
    #[inline(always)]
    pub const fn set_ccvbv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CC3 CCVB Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ccvbv3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CC3 CCVB Valid."]
    #[inline(always)]
    pub const fn set_ccvbv3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CC0 Input Capture Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn icv0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CC0 Input Capture Valid."]
    #[inline(always)]
    pub const fn set_icv0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CC1 Input Capture Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn icv1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CC1 Input Capture Valid."]
    #[inline(always)]
    pub const fn set_icv1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CC2 Input Capture Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn icv2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CC2 Input Capture Valid."]
    #[inline(always)]
    pub const fn set_icv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CC3 Input Capture Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn icv3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CC3 Input Capture Valid."]
    #[inline(always)]
    pub const fn set_icv3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CC0 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ccpol0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "CC0 Polarity."]
    #[inline(always)]
    pub const fn set_ccpol0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "CC1 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ccpol1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CC1 Polarity."]
    #[inline(always)]
    pub const fn set_ccpol1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CC2 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ccpol2(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "CC2 Polarity."]
    #[inline(always)]
    pub const fn set_ccpol2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "CC3 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ccpol3(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "CC3 Polarity."]
    #[inline(always)]
    pub const fn set_ccpol3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
            .field("running", &self.running())
            .field("dir", &self.dir())
            .field("topbv", &self.topbv())
            .field("ccvbv0", &self.ccvbv0())
            .field("ccvbv1", &self.ccvbv1())
            .field("ccvbv2", &self.ccvbv2())
            .field("ccvbv3", &self.ccvbv3())
            .field("icv0", &self.icv0())
            .field("icv1", &self.icv1())
            .field("icv2", &self.icv2())
            .field("icv3", &self.icv3())
            .field("ccpol0", &self.ccpol0())
            .field("ccpol1", &self.ccpol1())
            .field("ccpol2", &self.ccpol2())
            .field("ccpol3", &self.ccpol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ running: {=bool:?}, dir: {=bool:?}, topbv: {=bool:?}, ccvbv0: {=bool:?}, ccvbv1: {=bool:?}, ccvbv2: {=bool:?}, ccvbv3: {=bool:?}, icv0: {=bool:?}, icv1: {=bool:?}, icv2: {=bool:?}, icv3: {=bool:?}, ccpol0: {=bool:?}, ccpol1: {=bool:?}, ccpol2: {=bool:?}, ccpol3: {=bool:?} }}" , self . running () , self . dir () , self . topbv () , self . ccvbv0 () , self . ccvbv1 () , self . ccvbv2 () , self . ccvbv3 () , self . icv0 () , self . icv1 () , self . icv2 () , self . icv3 () , self . ccpol0 () , self . ccpol1 () , self . ccpol2 () , self . ccpol3 ())
    }
}
#[doc = "Counter Top Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Top(pub u32);
impl Top {
    #[doc = "Counter Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Top Value."]
    #[inline(always)]
    pub const fn set_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Top {
    #[inline(always)]
    fn default() -> Top {
        Top(0)
    }
}
impl core::fmt::Debug for Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Top").field("top", &self.top()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Top {{ top: {=u16:?} }}", self.top())
    }
}
#[doc = "Counter Top Value Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topb(pub u32);
impl Topb {
    #[doc = "Counter Top Value Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn topb(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Top Value Buffer."]
    #[inline(always)]
    pub const fn set_topb(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Topb {
    #[inline(always)]
    fn default() -> Topb {
        Topb(0)
    }
}
impl core::fmt::Debug for Topb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Topb").field("topb", &self.topb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Topb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Topb {{ topb: {=u16:?} }}", self.topb())
    }
}
