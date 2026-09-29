#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Watchdog Timer Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Clear."]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Cmd").field("clear", &self.clear()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ clear: {=bool:?} }}", self.clear())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Watchdog Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Enable."]
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
    #[doc = "Energy Mode 2 Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em2run(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Mode 2 Run Enable."]
    #[inline(always)]
    pub const fn set_em2run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Energy Mode 3 Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em3run(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Mode 3 Run Enable."]
    #[inline(always)]
    pub const fn set_em3run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Configuration Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Energy Mode 4 Block."]
    #[must_use]
    #[inline(always)]
    pub const fn em4block(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Mode 4 Block."]
    #[inline(always)]
    pub const fn set_em4block(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Oscillator Disable Block."]
    #[must_use]
    #[inline(always)]
    pub const fn swoscblock(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Software Oscillator Disable Block."]
    #[inline(always)]
    pub const fn set_swoscblock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Watchdog Timeout Period Select."]
    #[must_use]
    #[inline(always)]
    pub const fn persel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Watchdog Timeout Period Select."]
    #[inline(always)]
    pub const fn set_persel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Watchdog Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "Watchdog Clock Select."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Watchdog Timeout Period Select."]
    #[must_use]
    #[inline(always)]
    pub const fn warnsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Watchdog Timeout Period Select."]
    #[inline(always)]
    pub const fn set_warnsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Watchdog Illegal Window Select."]
    #[must_use]
    #[inline(always)]
    pub const fn winsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Watchdog Illegal Window Select."]
    #[inline(always)]
    pub const fn set_winsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Watchdog Clear Source."]
    #[must_use]
    #[inline(always)]
    pub const fn clrsrc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Clear Source."]
    #[inline(always)]
    pub const fn set_clrsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Watchdog Reset Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wdogrstdis(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Reset Disable."]
    #[inline(always)]
    pub const fn set_wdogrstdis(&mut self, val: bool) {
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
            .field("en", &self.en())
            .field("debugrun", &self.debugrun())
            .field("em2run", &self.em2run())
            .field("em3run", &self.em3run())
            .field("lock", &self.lock())
            .field("em4block", &self.em4block())
            .field("swoscblock", &self.swoscblock())
            .field("persel", &self.persel())
            .field("clksel", &self.clksel())
            .field("warnsel", &self.warnsel())
            .field("winsel", &self.winsel())
            .field("clrsrc", &self.clrsrc())
            .field("wdogrstdis", &self.wdogrstdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, debugrun: {=bool:?}, em2run: {=bool:?}, em3run: {=bool:?}, lock: {=bool:?}, em4block: {=bool:?}, swoscblock: {=bool:?}, persel: {=u8:?}, clksel: {:?}, warnsel: {=u8:?}, winsel: {=u8:?}, clrsrc: {=bool:?}, wdogrstdis: {=bool:?} }}" , self . en () , self . debugrun () , self . em2run () , self . em3run () , self . lock () , self . em4block () , self . swoscblock () , self . persel () , self . clksel () , self . warnsel () , self . winsel () , self . clrsrc () , self . wdogrstdis ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "TOUT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TOUT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WARN Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn warn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WARN Interrupt Enable."]
    #[inline(always)]
    pub const fn set_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WIN Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WIN Interrupt Enable."]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PEM0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pem0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PEM0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pem0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PEM1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pem1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PEM1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pem1(&mut self, val: bool) {
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
            .field("tout", &self.tout())
            .field("warn", &self.warn())
            .field("win", &self.win())
            .field("pem0", &self.pem0())
            .field("pem1", &self.pem1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ tout: {=bool:?}, warn: {=bool:?}, win: {=bool:?}, pem0: {=bool:?}, pem1: {=bool:?} }}" , self . tout () , self . warn () , self . win () , self . pem0 () , self . pem1 ())
    }
}
#[doc = "Watchdog Interrupt Flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "WDOG Timeout Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Timeout Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WDOG Warning Timeout Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn warn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Warning Timeout Interrupt Flag."]
    #[inline(always)]
    pub const fn set_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WDOG Window Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Window Interrupt Flag."]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PRS Channel Zero Event Missing Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pem0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Channel Zero Event Missing Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pem0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PRS Channel One Event Missing Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pem1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Channel One Event Missing Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pem1(&mut self, val: bool) {
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
            .field("tout", &self.tout())
            .field("warn", &self.warn())
            .field("win", &self.win())
            .field("pem0", &self.pem0())
            .field("pem1", &self.pem1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ tout: {=bool:?}, warn: {=bool:?}, win: {=bool:?}, pem0: {=bool:?}, pem1: {=bool:?} }}" , self . tout () , self . warn () , self . win () , self . pem0 () , self . pem1 ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set TOUT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set TOUT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set WARN Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn warn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set WARN Interrupt Flag."]
    #[inline(always)]
    pub const fn set_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set WIN Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set WIN Interrupt Flag."]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set PEM0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pem0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set PEM0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pem0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set PEM1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pem1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set PEM1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pem1(&mut self, val: bool) {
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
            .field("tout", &self.tout())
            .field("warn", &self.warn())
            .field("win", &self.win())
            .field("pem0", &self.pem0())
            .field("pem1", &self.pem1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ tout: {=bool:?}, warn: {=bool:?}, win: {=bool:?}, pem0: {=bool:?}, pem1: {=bool:?} }}" , self . tout () , self . warn () , self . win () , self . pem0 () , self . pem1 ())
    }
}
#[doc = "PRS Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pch0Prsctrl(pub u32);
impl Pch0Prsctrl {
    #[doc = "PRS Channel PRS Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Pch0PrsctrlPrssel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pch0PrsctrlPrssel::from_bits(val as u8)
    }
    #[doc = "PRS Channel PRS Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Pch0PrsctrlPrssel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PRS Missing Event Will Trigger a Watchdog Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn prsmissrsten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Missing Event Will Trigger a Watchdog Reset."]
    #[inline(always)]
    pub const fn set_prsmissrsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Pch0Prsctrl {
    #[inline(always)]
    fn default() -> Pch0Prsctrl {
        Pch0Prsctrl(0)
    }
}
impl core::fmt::Debug for Pch0Prsctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pch0Prsctrl")
            .field("prssel", &self.prssel())
            .field("prsmissrsten", &self.prsmissrsten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pch0Prsctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pch0Prsctrl {{ prssel: {:?}, prsmissrsten: {=bool:?} }}",
            self.prssel(),
            self.prsmissrsten()
        )
    }
}
#[doc = "PRS Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pch1Prsctrl(pub u32);
impl Pch1Prsctrl {
    #[doc = "PRS Channel PRS Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Pch1PrsctrlPrssel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pch1PrsctrlPrssel::from_bits(val as u8)
    }
    #[doc = "PRS Channel PRS Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Pch1PrsctrlPrssel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PRS Missing Event Will Trigger a Watchdog Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn prsmissrsten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Missing Event Will Trigger a Watchdog Reset."]
    #[inline(always)]
    pub const fn set_prsmissrsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Pch1Prsctrl {
    #[inline(always)]
    fn default() -> Pch1Prsctrl {
        Pch1Prsctrl(0)
    }
}
impl core::fmt::Debug for Pch1Prsctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pch1Prsctrl")
            .field("prssel", &self.prssel())
            .field("prsmissrsten", &self.prsmissrsten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pch1Prsctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pch1Prsctrl {{ prssel: {:?}, prsmissrsten: {=bool:?} }}",
            self.prssel(),
            self.prsmissrsten()
        )
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
    #[doc = "PCH0_PRSCTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn pch0_prsctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PCH0_PRSCTRL Register Busy."]
    #[inline(always)]
    pub const fn set_pch0_prsctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PCH1_PRSCTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn pch1_prsctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PCH1_PRSCTRL Register Busy."]
    #[inline(always)]
    pub const fn set_pch1_prsctrl(&mut self, val: bool) {
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
            .field("pch0_prsctrl", &self.pch0_prsctrl())
            .field("pch1_prsctrl", &self.pch1_prsctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Syncbusy {{ ctrl: {=bool:?}, cmd: {=bool:?}, pch0_prsctrl: {=bool:?}, pch1_prsctrl: {=bool:?} }}" , self . ctrl () , self . cmd () , self . pch0_prsctrl () , self . pch1_prsctrl ())
    }
}
