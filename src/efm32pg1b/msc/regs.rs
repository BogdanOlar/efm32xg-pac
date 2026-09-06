#[doc = "Flash Cache Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachecmd(pub u32);
impl Cachecmd {
    #[doc = "Invalidate Instruction Cache."]
    #[must_use]
    #[inline(always)]
    pub const fn invcache(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalidate Instruction Cache."]
    #[inline(always)]
    pub const fn set_invcache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start Performance Counters."]
    #[must_use]
    #[inline(always)]
    pub const fn startpc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start Performance Counters."]
    #[inline(always)]
    pub const fn set_startpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop Performance Counters."]
    #[must_use]
    #[inline(always)]
    pub const fn stoppc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Performance Counters."]
    #[inline(always)]
    pub const fn set_stoppc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cachecmd {
    #[inline(always)]
    fn default() -> Cachecmd {
        Cachecmd(0)
    }
}
impl core::fmt::Debug for Cachecmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cachecmd")
            .field("invcache", &self.invcache())
            .field("startpc", &self.startpc())
            .field("stoppc", &self.stoppc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cachecmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cachecmd {{ invcache: {=bool:?}, startpc: {=bool:?}, stoppc: {=bool:?} }}",
            self.invcache(),
            self.startpc(),
            self.stoppc()
        )
    }
}
#[doc = "Cache Hits Performance Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachehits(pub u32);
impl Cachehits {
    #[doc = "Cache Hits Since Last Performance Counter Start Command."]
    #[must_use]
    #[inline(always)]
    pub const fn cachehits(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Cache Hits Since Last Performance Counter Start Command."]
    #[inline(always)]
    pub const fn set_cachehits(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Cachehits {
    #[inline(always)]
    fn default() -> Cachehits {
        Cachehits(0)
    }
}
impl core::fmt::Debug for Cachehits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cachehits")
            .field("cachehits", &self.cachehits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cachehits {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cachehits {{ cachehits: {=u32:?} }}", self.cachehits())
    }
}
#[doc = "Cache Misses Performance Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachemisses(pub u32);
impl Cachemisses {
    #[doc = "Cache Misses Since Last Performance Counter Start Command."]
    #[must_use]
    #[inline(always)]
    pub const fn cachemisses(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Cache Misses Since Last Performance Counter Start Command."]
    #[inline(always)]
    pub const fn set_cachemisses(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Cachemisses {
    #[inline(always)]
    fn default() -> Cachemisses {
        Cachemisses(0)
    }
}
impl core::fmt::Debug for Cachemisses {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cachemisses")
            .field("cachemisses", &self.cachemisses())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cachemisses {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cachemisses {{ cachemisses: {=u32:?} }}",
            self.cachemisses()
        )
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Flash Power Up Command."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Power Up Command."]
    #[inline(always)]
    pub const fn set_pwrup(&mut self, val: bool) {
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
        f.debug_struct("Cmd").field("pwrup", &self.pwrup()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ pwrup: {=bool:?} }}", self.pwrup())
    }
}
#[doc = "Memory System Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Invalid Address Bus Fault Response Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn addrfaulten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Address Bus Fault Response Enable."]
    #[inline(always)]
    pub const fn set_addrfaulten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock-disabled Bus Fault Response Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdisfaulten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock-disabled Bus Fault Response Enable."]
    #[inline(always)]
    pub const fn set_clkdisfaulten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power Up on Demand During Wake Up."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrupondemand(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power Up on Demand During Wake Up."]
    #[inline(always)]
    pub const fn set_pwrupondemand(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IFC Read Clears IF."]
    #[must_use]
    #[inline(always)]
    pub const fn ifcreadclear(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IFC Read Clears IF."]
    #[inline(always)]
    pub const fn set_ifcreadclear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("addrfaulten", &self.addrfaulten())
            .field("clkdisfaulten", &self.clkdisfaulten())
            .field("pwrupondemand", &self.pwrupondemand())
            .field("ifcreadclear", &self.ifcreadclear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ addrfaulten: {=bool:?}, clkdisfaulten: {=bool:?}, pwrupondemand: {=bool:?}, ifcreadclear: {=bool:?} }}" , self . addrfaulten () , self . clkdisfaulten () , self . pwrupondemand () , self . ifcreadclear ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "ERASE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erase(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ERASE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WRITE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn write(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WRITE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CHOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CHOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_chof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cmof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PWRUPF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrupf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PWRUPF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pwrupf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ICACHERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icacherr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ICACHERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_icacherr(&mut self, val: bool) {
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
            .field("erase", &self.erase())
            .field("write", &self.write())
            .field("chof", &self.chof())
            .field("cmof", &self.cmof())
            .field("pwrupf", &self.pwrupf())
            .field("icacherr", &self.icacherr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Erase Done Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erase(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Done Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_erase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write Done Interrupt Read Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn write(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write Done Interrupt Read Flag."]
    #[inline(always)]
    pub const fn set_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Cache Hits Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Cache Hits Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_chof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Cache Misses Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Cache Misses Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Flash Power Up Sequence Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrupf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Power Up Sequence Complete Flag."]
    #[inline(always)]
    pub const fn set_pwrupf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ICache RAM Parity Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icacherr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ICache RAM Parity Error Flag."]
    #[inline(always)]
    pub const fn set_icacherr(&mut self, val: bool) {
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
            .field("erase", &self.erase())
            .field("write", &self.write())
            .field("chof", &self.chof())
            .field("cmof", &self.cmof())
            .field("pwrupf", &self.pwrupf())
            .field("icacherr", &self.icacherr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set ERASE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erase(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set ERASE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_erase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set WRITE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn write(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set WRITE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set CHOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set CHOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_chof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set CMOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set CMOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set PWRUPF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrupf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set PWRUPF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pwrupf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set ICACHERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn icacherr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set ICACHERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_icacherr(&mut self, val: bool) {
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
            .field("erase", &self.erase())
            .field("write", &self.write())
            .field("chof", &self.chof())
            .field("cmof", &self.cmof())
            .field("pwrupf", &self.pwrupf())
            .field("icacherr", &self.icacherr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr ())
    }
}
#[doc = "Configuration Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Configuration Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lockkey(&self) -> super::vals::Lockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Lockkey::from_bits(val as u16)
    }
    #[doc = "Configuration Lock."]
    #[inline(always)]
    pub const fn set_lockkey(&mut self, val: super::vals::Lockkey) {
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
#[doc = "Mass Erase Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Masslock(pub u32);
impl Masslock {
    #[doc = "Mass Erase Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lockkey(&self) -> super::vals::Lockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Lockkey::from_bits(val as u16)
    }
    #[doc = "Mass Erase Lock."]
    #[inline(always)]
    pub const fn set_lockkey(&mut self, val: super::vals::Lockkey) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Masslock {
    #[inline(always)]
    fn default() -> Masslock {
        Masslock(0)
    }
}
impl core::fmt::Debug for Masslock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Masslock")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Masslock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Masslock {{ lockkey: {:?} }}", self.lockkey())
    }
}
#[doc = "Read Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Readctrl(pub u32);
impl Readctrl {
    #[doc = "Internal Flash Cache Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ifcdis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Flash Cache Disable."]
    #[inline(always)]
    pub const fn set_ifcdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatic Invalidate Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aidis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Invalidate Disable."]
    #[inline(always)]
    pub const fn set_aidis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Context Cache Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn iccdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Context Cache Disable."]
    #[inline(always)]
    pub const fn set_iccdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Prefetch Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch Mode."]
    #[inline(always)]
    pub const fn set_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "AHB_HPROT Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn usehprot(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AHB_HPROT Mode."]
    #[inline(always)]
    pub const fn set_usehprot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Read Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Read Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Suppress Conditional Branch Target Perfetch."]
    #[must_use]
    #[inline(always)]
    pub const fn scbtp(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Suppress Conditional Branch Target Perfetch."]
    #[inline(always)]
    pub const fn set_scbtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Readctrl {
    #[inline(always)]
    fn default() -> Readctrl {
        Readctrl(0)
    }
}
impl core::fmt::Debug for Readctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Readctrl")
            .field("ifcdis", &self.ifcdis())
            .field("aidis", &self.aidis())
            .field("iccdis", &self.iccdis())
            .field("prefetch", &self.prefetch())
            .field("usehprot", &self.usehprot())
            .field("mode", &self.mode())
            .field("scbtp", &self.scbtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Readctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Readctrl {{ ifcdis: {=bool:?}, aidis: {=bool:?}, iccdis: {=bool:?}, prefetch: {=bool:?}, usehprot: {=bool:?}, mode: {:?}, scbtp: {=bool:?} }}" , self . ifcdis () , self . aidis () , self . iccdis () , self . prefetch () , self . usehprot () , self . mode () , self . scbtp ())
    }
}
#[doc = "Startup Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startup(pub u32);
impl Startup {
    #[doc = "Startup Delay 0."]
    #[must_use]
    #[inline(always)]
    pub const fn stdly0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Startup Delay 0."]
    #[inline(always)]
    pub const fn set_stdly0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Startup Delay 0."]
    #[must_use]
    #[inline(always)]
    pub const fn stdly1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x03ff;
        val as u16
    }
    #[doc = "Startup Delay 0."]
    #[inline(always)]
    pub const fn set_stdly1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 12usize)) | (((val as u32) & 0x03ff) << 12usize);
    }
    #[doc = "Active Startup Wait."]
    #[must_use]
    #[inline(always)]
    pub const fn astwait(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Active Startup Wait."]
    #[inline(always)]
    pub const fn set_astwait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Startup Waitstates Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stwsen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Startup Waitstates Enable."]
    #[inline(always)]
    pub const fn set_stwsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Startup Waitstates Always Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stwsaen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Startup Waitstates Always Enable."]
    #[inline(always)]
    pub const fn set_stwsaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Startup Waitstates."]
    #[must_use]
    #[inline(always)]
    pub const fn stws(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Startup Waitstates."]
    #[inline(always)]
    pub const fn set_stws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Startup {
    #[inline(always)]
    fn default() -> Startup {
        Startup(0)
    }
}
impl core::fmt::Debug for Startup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startup")
            .field("stdly0", &self.stdly0())
            .field("stdly1", &self.stdly1())
            .field("astwait", &self.astwait())
            .field("stwsen", &self.stwsen())
            .field("stwsaen", &self.stwsaen())
            .field("stws", &self.stws())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startup {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Startup {{ stdly0: {=u16:?}, stdly1: {=u16:?}, astwait: {=bool:?}, stwsen: {=bool:?}, stwsaen: {=bool:?}, stws: {=u8:?} }}" , self . stdly0 () , self . stdly1 () , self . astwait () , self . stwsen () , self . stwsaen () , self . stws ())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Erase/Write Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Erase/Write Busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Access Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Access Locked."]
    #[inline(always)]
    pub const fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invalid Write Address or Erase Page."]
    #[must_use]
    #[inline(always)]
    pub const fn invaddr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Write Address or Erase Page."]
    #[inline(always)]
    pub const fn set_invaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WDATA Write Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn wdataready(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WDATA Write Ready."]
    #[inline(always)]
    pub const fn set_wdataready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Flash Write Word Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn wordtimeout(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Write Word Timeout."]
    #[inline(always)]
    pub const fn set_wordtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The Current Flash Erase Operation Aborted."]
    #[must_use]
    #[inline(always)]
    pub const fn eraseaborted(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "The Current Flash Erase Operation Aborted."]
    #[inline(always)]
    pub const fn set_eraseaborted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Performance Counters Running."]
    #[must_use]
    #[inline(always)]
    pub const fn pcrunning(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Performance Counters Running."]
    #[inline(always)]
    pub const fn set_pcrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            .field("busy", &self.busy())
            .field("locked", &self.locked())
            .field("invaddr", &self.invaddr())
            .field("wdataready", &self.wdataready())
            .field("wordtimeout", &self.wordtimeout())
            .field("eraseaborted", &self.eraseaborted())
            .field("pcrunning", &self.pcrunning())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ busy: {=bool:?}, locked: {=bool:?}, invaddr: {=bool:?}, wdataready: {=bool:?}, wordtimeout: {=bool:?}, eraseaborted: {=bool:?}, pcrunning: {=bool:?} }}" , self . busy () , self . locked () , self . invaddr () , self . wdataready () , self . wordtimeout () , self . eraseaborted () , self . pcrunning ())
    }
}
#[doc = "Write Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Writecmd(pub u32);
impl Writecmd {
    #[doc = "Load MSC_ADDRB Into ADDR."]
    #[must_use]
    #[inline(always)]
    pub const fn laddrim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Load MSC_ADDRB Into ADDR."]
    #[inline(always)]
    pub const fn set_laddrim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Erase Page."]
    #[must_use]
    #[inline(always)]
    pub const fn erasepage(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Page."]
    #[inline(always)]
    pub const fn set_erasepage(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Write Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn writeend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "End Write Mode."]
    #[inline(always)]
    pub const fn set_writeend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Word Write-Once Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn writeonce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Word Write-Once Trigger."]
    #[inline(always)]
    pub const fn set_writeonce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Word Write Sequence Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn writetrig(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Word Write Sequence Trigger."]
    #[inline(always)]
    pub const fn set_writetrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Abort Erase Sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn eraseabort(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Abort Erase Sequence."]
    #[inline(always)]
    pub const fn set_eraseabort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mass Erase Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn erasemain0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mass Erase Region 0."]
    #[inline(always)]
    pub const fn set_erasemain0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clear WDATA State."]
    #[must_use]
    #[inline(always)]
    pub const fn clearwdata(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Clear WDATA State."]
    #[inline(always)]
    pub const fn set_clearwdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Writecmd {
    #[inline(always)]
    fn default() -> Writecmd {
        Writecmd(0)
    }
}
impl core::fmt::Debug for Writecmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Writecmd")
            .field("laddrim", &self.laddrim())
            .field("erasepage", &self.erasepage())
            .field("writeend", &self.writeend())
            .field("writeonce", &self.writeonce())
            .field("writetrig", &self.writetrig())
            .field("eraseabort", &self.eraseabort())
            .field("erasemain0", &self.erasemain0())
            .field("clearwdata", &self.clearwdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Writecmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Writecmd {{ laddrim: {=bool:?}, erasepage: {=bool:?}, writeend: {=bool:?}, writeonce: {=bool:?}, writetrig: {=bool:?}, eraseabort: {=bool:?}, erasemain0: {=bool:?}, clearwdata: {=bool:?} }}" , self . laddrim () , self . erasepage () , self . writeend () , self . writeonce () , self . writetrig () , self . eraseabort () , self . erasemain0 () , self . clearwdata ())
    }
}
#[doc = "Write Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Writectrl(pub u32);
impl Writectrl {
    #[doc = "Enable Write/Erase Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn wren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Write/Erase Controller."]
    #[inline(always)]
    pub const fn set_wren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Abort Page Erase on Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn irqeraseabort(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Abort Page Erase on Interrupt."]
    #[inline(always)]
    pub const fn set_irqeraseabort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Writectrl {
    #[inline(always)]
    fn default() -> Writectrl {
        Writectrl(0)
    }
}
impl core::fmt::Debug for Writectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Writectrl")
            .field("wren", &self.wren())
            .field("irqeraseabort", &self.irqeraseabort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Writectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Writectrl {{ wren: {=bool:?}, irqeraseabort: {=bool:?} }}",
            self.wren(),
            self.irqeraseabort()
        )
    }
}
