#[doc = "Auxiliary Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Auxcnt(pub u32);
impl Auxcnt {
    #[doc = "Auxiliary Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn auxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auxiliary Counter Value."]
    #[inline(always)]
    pub const fn set_auxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Auxcnt {
    #[inline(always)]
    fn default() -> Auxcnt {
        Auxcnt(0)
    }
}
impl core::fmt::Debug for Auxcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Auxcnt")
            .field("auxcnt", &self.auxcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Auxcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Auxcnt {{ auxcnt: {=u16:?} }}", self.auxcnt())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Load CNT Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn lcntim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Load CNT Immediately."]
    #[inline(always)]
    pub const fn set_lcntim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Load TOPB Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn ltopbim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Load TOPB Immediately."]
    #[inline(always)]
    pub const fn set_ltopbim(&mut self, val: bool) {
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
            .field("lcntim", &self.lcntim())
            .field("ltopbim", &self.ltopbim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ lcntim: {=bool:?}, ltopbim: {=bool:?} }}",
            self.lcntim(),
            self.ltopbim()
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
    #[doc = "Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode Select."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Enable Digital Pulse Width Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital Pulse Width Filter."]
    #[inline(always)]
    pub const fn set_filt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable PCNT Clock Domain Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rsten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PCNT Clock Domain Reset."]
    #[inline(always)]
    pub const fn set_rsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable CNT Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cntrsten(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable CNT Reset."]
    #[inline(always)]
    pub const fn set_cntrsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable AUXCNT Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn auxcntrsten(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable AUXCNT Reset."]
    #[inline(always)]
    pub const fn set_auxcntrsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Debug Mode Halt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debughalt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Halt Enable."]
    #[inline(always)]
    pub const fn set_debughalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable Hysteresis."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Hysteresis."]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Count Direction Determined By S1."]
    #[must_use]
    #[inline(always)]
    pub const fn s1cdir(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction Determined By S1."]
    #[inline(always)]
    pub const fn set_s1cdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Controls When the Counter Counts."]
    #[must_use]
    #[inline(always)]
    pub const fn cntev(&self) -> super::vals::Cntev {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Cntev::from_bits(val as u8)
    }
    #[doc = "Controls When the Counter Counts."]
    #[inline(always)]
    pub const fn set_cntev(&mut self, val: super::vals::Cntev) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Controls When the Auxiliary Counter Counts."]
    #[must_use]
    #[inline(always)]
    pub const fn auxcntev(&self) -> super::vals::Auxcntev {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Auxcntev::from_bits(val as u8)
    }
    #[doc = "Controls When the Auxiliary Counter Counts."]
    #[inline(always)]
    pub const fn set_auxcntev(&mut self, val: super::vals::Auxcntev) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Non-Quadrature Mode Counter Direction Control."]
    #[must_use]
    #[inline(always)]
    pub const fn cntdir(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Quadrature Mode Counter Direction Control."]
    #[inline(always)]
    pub const fn set_cntdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edge(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Select."]
    #[inline(always)]
    pub const fn set_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Sets the Mode for Triggered Compare and Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn tccmode(&self) -> super::vals::Tccmode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Tccmode::from_bits(val as u8)
    }
    #[doc = "Sets the Mode for Triggered Compare and Clear."]
    #[inline(always)]
    pub const fn set_tccmode(&mut self, val: super::vals::Tccmode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Set the LFA Prescaler for Triggered Compare and Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn tccpresc(&self) -> super::vals::Tccpresc {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::Tccpresc::from_bits(val as u8)
    }
    #[doc = "Set the LFA Prescaler for Triggered Compare and Clear."]
    #[inline(always)]
    pub const fn set_tccpresc(&mut self, val: super::vals::Tccpresc) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "Triggered Compare and Clear Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tcccomp(&self) -> super::vals::Tcccomp {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Tcccomp::from_bits(val as u8)
    }
    #[doc = "Triggered Compare and Clear Compare Mode."]
    #[inline(always)]
    pub const fn set_tcccomp(&mut self, val: super::vals::Tcccomp) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "PRS Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsgateen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Gate Enable."]
    #[inline(always)]
    pub const fn set_prsgateen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TCC PRS Polarity Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tccprspol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TCC PRS Polarity Select."]
    #[inline(always)]
    pub const fn set_tccprspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TCC PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tccprssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 26usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "TCC PRS Channel Select."]
    #[inline(always)]
    pub const fn set_tccprssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val.to_bits() as u32) & 0x1f) << 26usize);
    }
    #[doc = "TOPB High Frequency Value Select."]
    #[must_use]
    #[inline(always)]
    pub const fn topbhfsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TOPB High Frequency Value Select."]
    #[inline(always)]
    pub const fn set_topbhfsel(&mut self, val: bool) {
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
            .field("mode", &self.mode())
            .field("filt", &self.filt())
            .field("rsten", &self.rsten())
            .field("cntrsten", &self.cntrsten())
            .field("auxcntrsten", &self.auxcntrsten())
            .field("debughalt", &self.debughalt())
            .field("hyst", &self.hyst())
            .field("s1cdir", &self.s1cdir())
            .field("cntev", &self.cntev())
            .field("auxcntev", &self.auxcntev())
            .field("cntdir", &self.cntdir())
            .field("edge", &self.edge())
            .field("tccmode", &self.tccmode())
            .field("tccpresc", &self.tccpresc())
            .field("tcccomp", &self.tcccomp())
            .field("prsgateen", &self.prsgateen())
            .field("tccprspol", &self.tccprspol())
            .field("tccprssel", &self.tccprssel())
            .field("topbhfsel", &self.topbhfsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ mode: {:?}, filt: {=bool:?}, rsten: {=bool:?}, cntrsten: {=bool:?}, auxcntrsten: {=bool:?}, debughalt: {=bool:?}, hyst: {=bool:?}, s1cdir: {=bool:?}, cntev: {:?}, auxcntev: {:?}, cntdir: {=bool:?}, edge: {=bool:?}, tccmode: {:?}, tccpresc: {:?}, tcccomp: {:?}, prsgateen: {=bool:?}, tccprspol: {=bool:?}, tccprssel: {:?}, topbhfsel: {=bool:?} }}" , self . mode () , self . filt () , self . rsten () , self . cntrsten () , self . auxcntrsten () , self . debughalt () , self . hyst () , self . s1cdir () , self . cntev () , self . auxcntev () , self . cntdir () , self . edge () , self . tccmode () , self . tccpresc () , self . tcccomp () , self . prsgateen () , self . tccprspol () , self . tccprssel () , self . topbhfsel ())
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
    #[doc = "UF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DIRCNG Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dircng(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DIRCNG Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dircng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AUXOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn auxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AUXOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_auxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TCC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TCC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OQSTERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn oqsterr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OQSTERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_oqsterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dircng", &self.dircng())
            .field("auxof", &self.auxof())
            .field("tcc", &self.tcc())
            .field("oqsterr", &self.oqsterr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ uf: {=bool:?}, of: {=bool:?}, dircng: {=bool:?}, auxof: {=bool:?}, tcc: {=bool:?}, oqsterr: {=bool:?} }}" , self . uf () , self . of () , self . dircng () , self . auxof () , self . tcc () , self . oqsterr ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Underflow Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Direction Change Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dircng(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Direction Change Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dircng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Auxiliary Overflow Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Overflow Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_auxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Triggered Compare Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Compare Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Oversampling Quadrature State Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn oqsterr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Oversampling Quadrature State Error Interrupt."]
    #[inline(always)]
    pub const fn set_oqsterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dircng", &self.dircng())
            .field("auxof", &self.auxof())
            .field("tcc", &self.tcc())
            .field("oqsterr", &self.oqsterr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ uf: {=bool:?}, of: {=bool:?}, dircng: {=bool:?}, auxof: {=bool:?}, tcc: {=bool:?}, oqsterr: {=bool:?} }}" , self . uf () , self . of () , self . dircng () , self . auxof () , self . tcc () , self . oqsterr ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set UF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set UF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set OF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set OF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set DIRCNG Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dircng(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set DIRCNG Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dircng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set AUXOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auxof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set AUXOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_auxof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set TCC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set TCC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set OQSTERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oqsterr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set OQSTERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_oqsterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dircng", &self.dircng())
            .field("auxof", &self.auxof())
            .field("tcc", &self.tcc())
            .field("oqsterr", &self.oqsterr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ uf: {=bool:?}, of: {=bool:?}, dircng: {=bool:?}, auxof: {=bool:?}, tcc: {=bool:?}, oqsterr: {=bool:?} }}" , self . uf () , self . of () , self . dircng () , self . auxof () , self . tcc () , self . oqsterr ())
    }
}
#[doc = "PCNT Input Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "S0IN PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn s0prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "S0IN PRS Channel Select."]
    #[inline(always)]
    pub const fn set_s0prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "S0IN PRS Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn s0prsen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "S0IN PRS Enable."]
    #[inline(always)]
    pub const fn set_s0prsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "S1IN PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn s1prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "S1IN PRS Channel Select."]
    #[inline(always)]
    pub const fn set_s1prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
    #[doc = "S1IN PRS Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn s1prsen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "S1IN PRS Enable."]
    #[inline(always)]
    pub const fn set_s1prsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("s0prssel", &self.s0prssel())
            .field("s0prsen", &self.s0prsen())
            .field("s1prssel", &self.s1prssel())
            .field("s1prsen", &self.s1prsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ s0prssel: {:?}, s0prsen: {=bool:?}, s1prssel: {:?}, s1prsen: {=bool:?} }}",
            self.s0prssel(),
            self.s0prsen(),
            self.s1prssel(),
            self.s1prsen()
        )
    }
}
#[doc = "Oversampling Config Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ovscfg(pub u32);
impl Ovscfg {
    #[doc = "Configure Filter Length for Inputs S0IN and S1IN."]
    #[must_use]
    #[inline(always)]
    pub const fn filtlen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Configure Filter Length for Inputs S0IN and S1IN."]
    #[inline(always)]
    pub const fn set_filtlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Flutter Remove."]
    #[must_use]
    #[inline(always)]
    pub const fn flutterrm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Flutter Remove."]
    #[inline(always)]
    pub const fn set_flutterrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Ovscfg {
    #[inline(always)]
    fn default() -> Ovscfg {
        Ovscfg(0)
    }
}
impl core::fmt::Debug for Ovscfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ovscfg")
            .field("filtlen", &self.filtlen())
            .field("flutterrm", &self.flutterrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ovscfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ovscfg {{ filtlen: {=u8:?}, flutterrm: {=bool:?} }}",
            self.filtlen(),
            self.flutterrm()
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
    pub const fn s0inloc(&self) -> super::vals::S0inloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::S0inloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_s0inloc(&mut self, val: super::vals::S0inloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn s1inloc(&self) -> super::vals::S1inloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::S1inloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_s1inloc(&mut self, val: super::vals::S1inloc) {
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
            .field("s0inloc", &self.s0inloc())
            .field("s1inloc", &self.s1inloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ s0inloc: {:?}, s1inloc: {:?} }}",
            self.s0inloc(),
            self.s1inloc()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Current Counter Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Counter Direction."]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Status").field("dir", &self.dir()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ dir: {=bool:?} }}", self.dir())
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
    #[doc = "TOPB Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn topb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TOPB Register Busy."]
    #[inline(always)]
    pub const fn set_topb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OVSCFG Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn ovscfg(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OVSCFG Register Busy."]
    #[inline(always)]
    pub const fn set_ovscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("topb", &self.topb())
            .field("ovscfg", &self.ovscfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syncbusy {{ ctrl: {=bool:?}, cmd: {=bool:?}, topb: {=bool:?}, ovscfg: {=bool:?} }}",
            self.ctrl(),
            self.cmd(),
            self.topb(),
            self.ovscfg()
        )
    }
}
#[doc = "Top Value Register."]
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
#[doc = "Top Value Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topb(pub u32);
impl Topb {
    #[doc = "Counter Top Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn topb(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Top Buffer."]
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
