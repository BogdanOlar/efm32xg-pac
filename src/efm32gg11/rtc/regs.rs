#[doc = "Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter Value."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
        defmt::write!(f, "Cnt {{ cnt: {=u32:?} }}", self.cnt())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompaComp(pub u32);
impl CompaComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompaComp {
    #[inline(always)]
    fn default() -> CompaComp {
        CompaComp(0)
    }
}
impl core::fmt::Debug for CompaComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompaComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompaComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompaComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompbComp(pub u32);
impl CompbComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompbComp {
    #[inline(always)]
    fn default() -> CompbComp {
        CompbComp(0)
    }
}
impl core::fmt::Debug for CompbComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompbComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompbComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompbComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompcComp(pub u32);
impl CompcComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompcComp {
    #[inline(always)]
    fn default() -> CompcComp {
        CompcComp(0)
    }
}
impl core::fmt::Debug for CompcComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompcComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompcComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompcComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompdComp(pub u32);
impl CompdComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompdComp {
    #[inline(always)]
    fn default() -> CompdComp {
        CompdComp(0)
    }
}
impl core::fmt::Debug for CompdComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompdComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompdComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompdComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompeComp(pub u32);
impl CompeComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompeComp {
    #[inline(always)]
    fn default() -> CompeComp {
        CompeComp(0)
    }
}
impl core::fmt::Debug for CompeComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompeComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompeComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompeComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Compare Value Register X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompfComp(pub u32);
impl CompfComp {
    #[doc = "Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for CompfComp {
    #[inline(always)]
    fn default() -> CompfComp {
        CompfComp(0)
    }
}
impl core::fmt::Debug for CompfComp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompfComp")
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompfComp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CompfComp {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "RTC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Compare Channel 0 is Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp0top(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Channel 0 is Top Value."]
    #[inline(always)]
    pub const fn set_comp0top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("debugrun", &self.debugrun())
            .field("comp0top", &self.comp0top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ en: {=bool:?}, debugrun: {=bool:?}, comp0top: {=bool:?} }}",
            self.en(),
            self.debugrun(),
            self.comp0top()
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
    #[doc = "COMP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "COMP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
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
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ of: {=bool:?}, comp: {=u8:?} }}",
            self.of(),
            self.comp()
        )
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
    #[doc = "Compare Match X Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Compare Match X Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
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
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "If {{ of: {=bool:?}, comp: {=u8:?} }}",
            self.of(),
            self.comp()
        )
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
    #[doc = "Set COMP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Set COMP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
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
            .field("comp", &self.comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ of: {=bool:?}, comp: {=u8:?} }}",
            self.of(),
            self.comp()
        )
    }
}
