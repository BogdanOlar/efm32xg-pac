#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Start LETIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start LETIMER."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop LETIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop LETIMER."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear LETIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear LETIMER."]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Toggle Output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cto0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Toggle Output 0."]
    #[inline(always)]
    pub const fn set_cto0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clear Toggle Output 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cto1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Toggle Output 1."]
    #[inline(always)]
    pub const fn set_cto1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("clear", &self.clear())
            .field("cto0", &self.cto0())
            .field("cto1", &self.cto1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ start: {=bool:?}, stop: {=bool:?}, clear: {=bool:?}, cto0: {=bool:?}, cto1: {=bool:?} }}" , self . start () , self . stop () , self . clear () , self . cto0 () , self . cto1 ())
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
#[doc = "Compare Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp0(pub u32);
impl Comp0 {
    #[doc = "Compare Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value 0."]
    #[inline(always)]
    pub const fn set_comp0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Comp0 {
    #[inline(always)]
    fn default() -> Comp0 {
        Comp0(0)
    }
}
impl core::fmt::Debug for Comp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp0")
            .field("comp0", &self.comp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Comp0 {{ comp0: {=u16:?} }}", self.comp0())
    }
}
#[doc = "Compare Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp1(pub u32);
impl Comp1 {
    #[doc = "Compare Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn comp1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value 1."]
    #[inline(always)]
    pub const fn set_comp1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Comp1 {
    #[inline(always)]
    fn default() -> Comp1 {
        Comp1(0)
    }
}
impl core::fmt::Debug for Comp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp1")
            .field("comp1", &self.comp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Comp1 {{ comp1: {=u16:?} }}", self.comp1())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Repeat Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn repmode(&self) -> super::vals::Repmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Repmode::from_bits(val as u8)
    }
    #[doc = "Repeat Mode."]
    #[inline(always)]
    pub const fn set_repmode(&mut self, val: super::vals::Repmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Underflow Output Action 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ufoa0(&self) -> super::vals::Ufoa0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Ufoa0::from_bits(val as u8)
    }
    #[doc = "Underflow Output Action 0."]
    #[inline(always)]
    pub const fn set_ufoa0(&mut self, val: super::vals::Ufoa0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Underflow Output Action 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ufoa1(&self) -> super::vals::Ufoa1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ufoa1::from_bits(val as u8)
    }
    #[doc = "Underflow Output Action 1."]
    #[inline(always)]
    pub const fn set_ufoa1(&mut self, val: super::vals::Ufoa1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Output 0 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn opol0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Output 0 Polarity."]
    #[inline(always)]
    pub const fn set_opol0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Output 1 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn opol1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output 1 Polarity."]
    #[inline(always)]
    pub const fn set_opol1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Buffered Top."]
    #[must_use]
    #[inline(always)]
    pub const fn buftop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Buffered Top."]
    #[inline(always)]
    pub const fn set_buftop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Compare Value 0 is Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0top(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Value 0 is Top Value."]
    #[inline(always)]
    pub const fn set_comp0top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("repmode", &self.repmode())
            .field("ufoa0", &self.ufoa0())
            .field("ufoa1", &self.ufoa1())
            .field("opol0", &self.opol0())
            .field("opol1", &self.opol1())
            .field("buftop", &self.buftop())
            .field("comp0top", &self.comp0top())
            .field("debugrun", &self.debugrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ repmode: {:?}, ufoa0: {:?}, ufoa1: {:?}, opol0: {=bool:?}, opol1: {=bool:?}, buftop: {=bool:?}, comp0top: {=bool:?}, debugrun: {=bool:?} }}" , self . repmode () , self . ufoa0 () , self . ufoa1 () , self . opol0 () , self . opol1 () , self . buftop () , self . comp0top () , self . debugrun ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "COMP0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "COMP0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMP1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMP1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "REP0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rep0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "REP0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rep0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "REP1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rep1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "REP1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("comp0", &self.comp0())
            .field("comp1", &self.comp1())
            .field("uf", &self.uf())
            .field("rep0", &self.rep0())
            .field("rep1", &self.rep1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ comp0: {=bool:?}, comp1: {=bool:?}, uf: {=bool:?}, rep0: {=bool:?}, rep1: {=bool:?} }}" , self . comp0 () , self . comp1 () , self . uf () , self . rep0 () , self . rep1 ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Compare Match 0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Match 0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Compare Match 1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Match 1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Repeat Counter 0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rep0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Counter 0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rep0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeat Counter 1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rep1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Counter 1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("comp0", &self.comp0())
            .field("comp1", &self.comp1())
            .field("uf", &self.uf())
            .field("rep0", &self.rep0())
            .field("rep1", &self.rep1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ comp0: {=bool:?}, comp1: {=bool:?}, uf: {=bool:?}, rep0: {=bool:?}, rep1: {=bool:?} }}" , self . comp0 () , self . comp1 () , self . uf () , self . rep0 () , self . rep1 ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set COMP0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set COMP0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set COMP1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set COMP1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set UF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set UF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set REP0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rep0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set REP0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rep0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set REP1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rep1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set REP1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("comp0", &self.comp0())
            .field("comp1", &self.comp1())
            .field("uf", &self.uf())
            .field("rep0", &self.rep0())
            .field("rep1", &self.rep1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ comp0: {=bool:?}, comp1: {=bool:?}, uf: {=bool:?}, rep0: {=bool:?}, rep1: {=bool:?} }}" , self . comp0 () , self . comp1 () , self . uf () , self . rep0 () , self . rep1 ())
    }
}
#[doc = "PRS Input Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prssel(pub u32);
impl Prssel {
    #[doc = "PRS Start Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prsstartsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Start Select."]
    #[inline(always)]
    pub const fn set_prsstartsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PRS Stop Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prsstopsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Stop Select."]
    #[inline(always)]
    pub const fn set_prsstopsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val.to_bits() as u32) & 0x0f) << 6usize);
    }
    #[doc = "PRS Clear Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prsclearsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Clear Select."]
    #[inline(always)]
    pub const fn set_prsclearsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "PRS Start Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsstartmode(&self) -> super::vals::Prsstartmode {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Prsstartmode::from_bits(val as u8)
    }
    #[doc = "PRS Start Mode."]
    #[inline(always)]
    pub const fn set_prsstartmode(&mut self, val: super::vals::Prsstartmode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "PRS Stop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsstopmode(&self) -> super::vals::Prsstopmode {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Prsstopmode::from_bits(val as u8)
    }
    #[doc = "PRS Stop Mode."]
    #[inline(always)]
    pub const fn set_prsstopmode(&mut self, val: super::vals::Prsstopmode) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "PRS Clear Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsclearmode(&self) -> super::vals::Prsclearmode {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Prsclearmode::from_bits(val as u8)
    }
    #[doc = "PRS Clear Mode."]
    #[inline(always)]
    pub const fn set_prsclearmode(&mut self, val: super::vals::Prsclearmode) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for Prssel {
    #[inline(always)]
    fn default() -> Prssel {
        Prssel(0)
    }
}
impl core::fmt::Debug for Prssel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prssel")
            .field("prsstartsel", &self.prsstartsel())
            .field("prsstopsel", &self.prsstopsel())
            .field("prsclearsel", &self.prsclearsel())
            .field("prsstartmode", &self.prsstartmode())
            .field("prsstopmode", &self.prsstopmode())
            .field("prsclearmode", &self.prsclearmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prssel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prssel {{ prsstartsel: {:?}, prsstopsel: {:?}, prsclearsel: {:?}, prsstartmode: {:?}, prsstopmode: {:?}, prsclearmode: {:?} }}" , self . prsstartsel () , self . prsstopsel () , self . prsclearsel () , self . prsstartmode () , self . prsstopmode () , self . prsclearmode ())
    }
}
#[doc = "Repeat Counter Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rep0(pub u32);
impl Rep0 {
    #[doc = "Repeat Counter 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rep0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repeat Counter 0."]
    #[inline(always)]
    pub const fn set_rep0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rep0 {
    #[inline(always)]
    fn default() -> Rep0 {
        Rep0(0)
    }
}
impl core::fmt::Debug for Rep0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rep0").field("rep0", &self.rep0()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rep0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rep0 {{ rep0: {=u8:?} }}", self.rep0())
    }
}
#[doc = "Repeat Counter Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rep1(pub u32);
impl Rep1 {
    #[doc = "Repeat Counter 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rep1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repeat Counter 1."]
    #[inline(always)]
    pub const fn set_rep1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rep1 {
    #[inline(always)]
    fn default() -> Rep1 {
        Rep1(0)
    }
}
impl core::fmt::Debug for Rep1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rep1").field("rep1", &self.rep1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rep1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rep1 {{ rep1: {=u8:?} }}", self.rep1())
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
    pub const fn out0loc(&self) -> super::vals::Out0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Out0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_out0loc(&mut self, val: super::vals::Out0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn out1loc(&self) -> super::vals::Out1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Out1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_out1loc(&mut self, val: super::vals::Out1loc) {
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
            .field("out0loc", &self.out0loc())
            .field("out1loc", &self.out1loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ out0loc: {:?}, out1loc: {:?} }}",
            self.out0loc(),
            self.out1loc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "Output 0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn out0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output 0 Pin Enable."]
    #[inline(always)]
    pub const fn set_out0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Output 1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn out1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output 1 Pin Enable."]
    #[inline(always)]
    pub const fn set_out1pen(&mut self, val: bool) {
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
            .field("out0pen", &self.out0pen())
            .field("out1pen", &self.out1pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routepen {{ out0pen: {=bool:?}, out1pen: {=bool:?} }}",
            self.out0pen(),
            self.out1pen()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "LETIMER Running."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LETIMER Running."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
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
        f.debug_struct("Status")
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ running: {=bool:?} }}", self.running())
    }
}
#[doc = "Synchronization Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
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
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syncbusy {{ cmd: {=bool:?} }}", self.cmd())
    }
}
