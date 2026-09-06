#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Reset Cause Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rcclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Cause Clear."]
    #[inline(always)]
    pub const fn set_rcclr(&mut self, val: bool) {
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
        f.debug_struct("Cmd").field("rcclr", &self.rcclr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ rcclr: {=bool:?} }}", self.rcclr())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "WDOG Reset Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wdogrmode(&self) -> super::vals::Wdogrmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wdogrmode::from_bits(val as u8)
    }
    #[doc = "WDOG Reset Mode."]
    #[inline(always)]
    pub const fn set_wdogrmode(&mut self, val: super::vals::Wdogrmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Core LOCKUP Reset Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lockuprmode(&self) -> super::vals::Lockuprmode {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Lockuprmode::from_bits(val as u8)
    }
    #[doc = "Core LOCKUP Reset Mode."]
    #[inline(always)]
    pub const fn set_lockuprmode(&mut self, val: super::vals::Lockuprmode) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Core Sysreset Reset Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sysrmode(&self) -> super::vals::Sysrmode {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Sysrmode::from_bits(val as u8)
    }
    #[doc = "Core Sysreset Reset Mode."]
    #[inline(always)]
    pub const fn set_sysrmode(&mut self, val: super::vals::Sysrmode) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "PIN Reset Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pinrmode(&self) -> super::vals::Pinrmode {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pinrmode::from_bits(val as u8)
    }
    #[doc = "PIN Reset Mode."]
    #[inline(always)]
    pub const fn set_pinrmode(&mut self, val: super::vals::Pinrmode) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "System Software Reset State."]
    #[must_use]
    #[inline(always)]
    pub const fn resetstate(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "System Software Reset State."]
    #[inline(always)]
    pub const fn set_resetstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
            .field("wdogrmode", &self.wdogrmode())
            .field("lockuprmode", &self.lockuprmode())
            .field("sysrmode", &self.sysrmode())
            .field("pinrmode", &self.pinrmode())
            .field("resetstate", &self.resetstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ wdogrmode: {:?}, lockuprmode: {:?}, sysrmode: {:?}, pinrmode: {:?}, resetstate: {=u8:?} }}" , self . wdogrmode () , self . lockuprmode () , self . sysrmode () , self . pinrmode () , self . resetstate ())
    }
}
#[doc = "Configuration Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Configuration Lock Key."]
    #[must_use]
    #[inline(always)]
    pub const fn lockkey(&self) -> super::super::msc::vals::Lockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::super::msc::vals::Lockkey::from_bits(val as u16)
    }
    #[doc = "Configuration Lock Key."]
    #[inline(always)]
    pub const fn set_lockkey(&mut self, val: super::super::msc::vals::Lockkey) {
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
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lock {{ lockkey: {:?} }}", self.lockkey())
    }
}
#[doc = "Reset Cause Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcause(pub u32);
impl Rstcause {
    #[doc = "Power on Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn porst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power on Reset."]
    #[inline(always)]
    pub const fn set_porst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Brown Out Detector AVDD Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn avddbod(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Brown Out Detector AVDD Reset."]
    #[inline(always)]
    pub const fn set_avddbod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Brown Out Detector DVDD Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dvddbod(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Brown Out Detector DVDD Reset."]
    #[inline(always)]
    pub const fn set_dvddbod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Brown Out Detector Decouple Domain Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn decbod(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Brown Out Detector Decouple Domain Reset."]
    #[inline(always)]
    pub const fn set_decbod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "External Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn extrst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Pin Reset."]
    #[inline(always)]
    pub const fn set_extrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LOCKUP Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lockuprst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LOCKUP Reset."]
    #[inline(always)]
    pub const fn set_lockuprst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "System Request Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sysreqrst(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "System Request Reset."]
    #[inline(always)]
    pub const fn set_sysreqrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Watchdog Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn wdogrst(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Reset."]
    #[inline(always)]
    pub const fn set_wdogrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Backup Mode Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn bumoderst(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Backup Mode Reset."]
    #[inline(always)]
    pub const fn set_bumoderst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EM4 Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn em4rst(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Reset."]
    #[inline(always)]
    pub const fn set_em4rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Rstcause {
    #[inline(always)]
    fn default() -> Rstcause {
        Rstcause(0)
    }
}
impl core::fmt::Debug for Rstcause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstcause")
            .field("porst", &self.porst())
            .field("avddbod", &self.avddbod())
            .field("dvddbod", &self.dvddbod())
            .field("decbod", &self.decbod())
            .field("extrst", &self.extrst())
            .field("lockuprst", &self.lockuprst())
            .field("sysreqrst", &self.sysreqrst())
            .field("wdogrst", &self.wdogrst())
            .field("bumoderst", &self.bumoderst())
            .field("em4rst", &self.em4rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstcause {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rstcause {{ porst: {=bool:?}, avddbod: {=bool:?}, dvddbod: {=bool:?}, decbod: {=bool:?}, extrst: {=bool:?}, lockuprst: {=bool:?}, sysreqrst: {=bool:?}, wdogrst: {=bool:?}, bumoderst: {=bool:?}, em4rst: {=bool:?} }}" , self . porst () , self . avddbod () , self . dvddbod () , self . decbod () , self . extrst () , self . lockuprst () , self . sysreqrst () , self . wdogrst () , self . bumoderst () , self . em4rst ())
    }
}
