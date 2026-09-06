#[doc = "Software Unlock AAP Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aapunlockcmd(pub u32);
impl Aapunlockcmd {
    #[doc = "Software Unlock AAP Command."]
    #[must_use]
    #[inline(always)]
    pub const fn unlockaap(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Unlock AAP Command."]
    #[inline(always)]
    pub const fn set_unlockaap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Aapunlockcmd {
    #[inline(always)]
    fn default() -> Aapunlockcmd {
        Aapunlockcmd(0)
    }
}
impl core::fmt::Debug for Aapunlockcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aapunlockcmd")
            .field("unlockaap", &self.unlockaap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aapunlockcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aapunlockcmd {{ unlockaap: {=bool:?} }}",
            self.unlockaap()
        )
    }
}
#[doc = "Bank Switching Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bankswitchlock(pub u32);
impl Bankswitchlock {
    #[doc = "Bank Switching Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn bankswitchlockkey(&self) -> super::vals::Bankswitchlockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Bankswitchlockkey::from_bits(val as u16)
    }
    #[doc = "Bank Switching Lock."]
    #[inline(always)]
    pub const fn set_bankswitchlockkey(&mut self, val: super::vals::Bankswitchlockkey) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bankswitchlock {
    #[inline(always)]
    fn default() -> Bankswitchlock {
        Bankswitchlock(0)
    }
}
impl core::fmt::Debug for Bankswitchlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bankswitchlock")
            .field("bankswitchlockkey", &self.bankswitchlockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bankswitchlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bankswitchlock {{ bankswitchlockkey: {:?} }}",
            self.bankswitchlockkey()
        )
    }
}
#[doc = "Bootloader Read and Write Enable, Write Once Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootloaderctrl(pub u32);
impl Bootloaderctrl {
    #[doc = "Flash Bootloader Read Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn blrdis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Bootloader Read Disable."]
    #[inline(always)]
    pub const fn set_blrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash Bootloader Write/Erase Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn blwdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Bootloader Write/Erase Disable."]
    #[inline(always)]
    pub const fn set_blwdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Bootloaderctrl {
    #[inline(always)]
    fn default() -> Bootloaderctrl {
        Bootloaderctrl(0)
    }
}
impl core::fmt::Debug for Bootloaderctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootloaderctrl")
            .field("blrdis", &self.blrdis())
            .field("blwdis", &self.blwdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootloaderctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bootloaderctrl {{ blrdis: {=bool:?}, blwdis: {=bool:?} }}",
            self.blrdis(),
            self.blwdis()
        )
    }
}
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
#[doc = "Cache Configuration Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacheconfig0(pub u32);
impl Cacheconfig0 {
    #[doc = "Instruction Cache Low-Power Level."]
    #[must_use]
    #[inline(always)]
    pub const fn cachelplevel(&self) -> super::vals::Cachelplevel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cachelplevel::from_bits(val as u8)
    }
    #[doc = "Instruction Cache Low-Power Level."]
    #[inline(always)]
    pub const fn set_cachelplevel(&mut self, val: super::vals::Cachelplevel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Cacheconfig0 {
    #[inline(always)]
    fn default() -> Cacheconfig0 {
        Cacheconfig0(0)
    }
}
impl core::fmt::Debug for Cacheconfig0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacheconfig0")
            .field("cachelplevel", &self.cachelplevel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacheconfig0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacheconfig0 {{ cachelplevel: {:?} }}",
            self.cachelplevel()
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
    #[doc = "BANK SWITCHING COMMAND."]
    #[must_use]
    #[inline(always)]
    pub const fn switchingbank(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BANK SWITCHING COMMAND."]
    #[inline(always)]
    pub const fn set_switchingbank(&mut self, val: bool) {
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
            .field("pwrup", &self.pwrup())
            .field("switchingbank", &self.switchingbank())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ pwrup: {=bool:?}, switchingbank: {=bool:?} }}",
            self.pwrup(),
            self.switchingbank()
        )
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
    #[doc = "Timeout Bus Fault Response Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timeoutfaulten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Bus Fault Response Enable."]
    #[inline(always)]
    pub const fn set_timeoutfaulten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Two Bit ECC Error Bus Fault Response Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rameccerrfaulten(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Two Bit ECC Error Bus Fault Response Enable."]
    #[inline(always)]
    pub const fn set_rameccerrfaulten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EBI Bus Fault Response Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ebifaulten(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EBI Bus Fault Response Enable."]
    #[inline(always)]
    pub const fn set_ebifaulten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Peripheral Access Wait Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn waitmode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral Access Wait Mode."]
    #[inline(always)]
    pub const fn set_waitmode(&mut self, val: bool) {
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
            .field("addrfaulten", &self.addrfaulten())
            .field("clkdisfaulten", &self.clkdisfaulten())
            .field("pwrupondemand", &self.pwrupondemand())
            .field("ifcreadclear", &self.ifcreadclear())
            .field("timeoutfaulten", &self.timeoutfaulten())
            .field("rameccerrfaulten", &self.rameccerrfaulten())
            .field("ebifaulten", &self.ebifaulten())
            .field("waitmode", &self.waitmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ addrfaulten: {=bool:?}, clkdisfaulten: {=bool:?}, pwrupondemand: {=bool:?}, ifcreadclear: {=bool:?}, timeoutfaulten: {=bool:?}, rameccerrfaulten: {=bool:?}, ebifaulten: {=bool:?}, waitmode: {=bool:?} }}" , self . addrfaulten () , self . clkdisfaulten () , self . pwrupondemand () , self . ifcreadclear () , self . timeoutfaulten () , self . rameccerrfaulten () , self . ebifaulten () , self . waitmode ())
    }
}
#[doc = "RAM ECC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccctrl(pub u32);
impl Eccctrl {
    #[doc = "RAM ECC Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rameccewen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAM ECC Write Enable."]
    #[inline(always)]
    pub const fn set_rameccewen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAM ECC Check Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rameccchken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAM ECC Check Enable."]
    #[inline(always)]
    pub const fn set_rameccchken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAM1 ECC Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1eccewen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 ECC Write Enable."]
    #[inline(always)]
    pub const fn set_ram1eccewen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAM1 ECC Check Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1eccchken(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 ECC Check Enable."]
    #[inline(always)]
    pub const fn set_ram1eccchken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Eccctrl {
    #[inline(always)]
    fn default() -> Eccctrl {
        Eccctrl(0)
    }
}
impl core::fmt::Debug for Eccctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eccctrl")
            .field("rameccewen", &self.rameccewen())
            .field("rameccchken", &self.rameccchken())
            .field("ram1eccewen", &self.ram1eccewen())
            .field("ram1eccchken", &self.ram1eccchken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eccctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Eccctrl {{ rameccewen: {=bool:?}, rameccchken: {=bool:?}, ram1eccewen: {=bool:?}, ram1eccchken: {=bool:?} }}" , self . rameccewen () , self . rameccchken () , self . ram1eccewen () , self . ram1eccchken ())
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
    #[doc = "WDATAOV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wdataov(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "WDATAOV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wdataov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LVEWRITE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvewrite(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LVEWRITE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lvewrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RAMERR1B Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr1b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMERR1B Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ramerr1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMERR2B Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr2b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMERR2B Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ramerr2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAM1ERR1B Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err1b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1ERR1B Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ram1err1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAM1ERR2B Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err2b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1ERR2B Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ram1err2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("wdataov", &self.wdataov())
            .field("lvewrite", &self.lvewrite())
            .field("ramerr1b", &self.ramerr1b())
            .field("ramerr2b", &self.ramerr2b())
            .field("ram1err1b", &self.ram1err1b())
            .field("ram1err2b", &self.ram1err2b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?}, wdataov: {=bool:?}, lvewrite: {=bool:?}, ramerr1b: {=bool:?}, ramerr2b: {=bool:?}, ram1err1b: {=bool:?}, ram1err2b: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr () , self . wdataov () , self . lvewrite () , self . ramerr1b () , self . ramerr2b () , self . ram1err1b () , self . ram1err2b ())
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
    #[doc = "Flash Controller Write Buffer Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn wdataov(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Controller Write Buffer Overflow."]
    #[inline(always)]
    pub const fn set_wdataov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Flash LVE Write Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lvewrite(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Flash LVE Write Error Flag."]
    #[inline(always)]
    pub const fn set_lvewrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RAM 1-bit ECC Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr1b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAM 1-bit ECC Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ramerr1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAM 2-bit ECC Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr2b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAM 2-bit ECC Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ramerr2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAM1 1-bit ECC Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err1b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 1-bit ECC Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ram1err1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAM1 2-bit ECC Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err2b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 2-bit ECC Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ram1err2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("wdataov", &self.wdataov())
            .field("lvewrite", &self.lvewrite())
            .field("ramerr1b", &self.ramerr1b())
            .field("ramerr2b", &self.ramerr2b())
            .field("ram1err1b", &self.ram1err1b())
            .field("ram1err2b", &self.ram1err2b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?}, wdataov: {=bool:?}, lvewrite: {=bool:?}, ramerr1b: {=bool:?}, ramerr2b: {=bool:?}, ram1err1b: {=bool:?}, ram1err2b: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr () , self . wdataov () , self . lvewrite () , self . ramerr1b () , self . ramerr2b () , self . ram1err1b () , self . ram1err2b ())
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
    #[doc = "Set WDATAOV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wdataov(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set WDATAOV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_wdataov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set LVEWRITE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lvewrite(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set LVEWRITE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lvewrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set RAMERR1B Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr1b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set RAMERR1B Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ramerr1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set RAMERR2B Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ramerr2b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set RAMERR2B Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ramerr2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set RAM1ERR1B Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err1b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set RAM1ERR1B Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ram1err1b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set RAM1ERR2B Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1err2b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set RAM1ERR2B Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ram1err2b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("wdataov", &self.wdataov())
            .field("lvewrite", &self.lvewrite())
            .field("ramerr1b", &self.ramerr1b())
            .field("ramerr2b", &self.ramerr2b())
            .field("ram1err1b", &self.ram1err1b())
            .field("ram1err2b", &self.ram1err2b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ erase: {=bool:?}, write: {=bool:?}, chof: {=bool:?}, cmof: {=bool:?}, pwrupf: {=bool:?}, icacherr: {=bool:?}, wdataov: {=bool:?}, lvewrite: {=bool:?}, ramerr1b: {=bool:?}, ramerr2b: {=bool:?}, ram1err1b: {=bool:?}, ram1err2b: {=bool:?} }}" , self . erase () , self . write () , self . chof () , self . cmof () , self . pwrupf () , self . icacherr () , self . wdataov () , self . lvewrite () , self . ramerr1b () , self . ramerr2b () , self . ram1err1b () , self . ram1err2b ())
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
#[doc = "RAM Control Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramctrl(pub u32);
impl Ramctrl {
    #[doc = "RAM WAIT STATE Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramwsen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAM WAIT STATE Enable."]
    #[inline(always)]
    pub const fn set_ramwsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAM Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramprefetchen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAM Prefetch Enable."]
    #[inline(always)]
    pub const fn set_ramprefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAM1 WAIT STATE Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1wsen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 WAIT STATE Enable."]
    #[inline(always)]
    pub const fn set_ram1wsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RAM1 Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1prefetchen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RAM1 Prefetch Enable."]
    #[inline(always)]
    pub const fn set_ram1prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RAM2 CACHE Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram2cacheen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAM2 CACHE Enable."]
    #[inline(always)]
    pub const fn set_ram2cacheen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAM2 WAIT STATE Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram2wsen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAM2 WAIT STATE Enable."]
    #[inline(always)]
    pub const fn set_ram2wsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAM2 Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ram2prefetchen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAM2 Prefetch Enable."]
    #[inline(always)]
    pub const fn set_ram2prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Ramctrl {
    #[inline(always)]
    fn default() -> Ramctrl {
        Ramctrl(0)
    }
}
impl core::fmt::Debug for Ramctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ramctrl")
            .field("ramwsen", &self.ramwsen())
            .field("ramprefetchen", &self.ramprefetchen())
            .field("ram1wsen", &self.ram1wsen())
            .field("ram1prefetchen", &self.ram1prefetchen())
            .field("ram2cacheen", &self.ram2cacheen())
            .field("ram2wsen", &self.ram2wsen())
            .field("ram2prefetchen", &self.ram2prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ramctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ramctrl {{ ramwsen: {=bool:?}, ramprefetchen: {=bool:?}, ram1wsen: {=bool:?}, ram1prefetchen: {=bool:?}, ram2cacheen: {=bool:?}, ram2wsen: {=bool:?}, ram2prefetchen: {=bool:?} }}" , self . ramwsen () , self . ramprefetchen () , self . ram1wsen () , self . ram1prefetchen () , self . ram2cacheen () , self . ram2wsen () , self . ram2prefetchen ())
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
    #[doc = "External Bus Interface Cache Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ebicdis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "External Bus Interface Cache Disable."]
    #[inline(always)]
    pub const fn set_ebicdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    #[doc = "QSPI Cache Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn qspicdis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "QSPI Cache Disable."]
    #[inline(always)]
    pub const fn set_qspicdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("ebicdis", &self.ebicdis())
            .field("prefetch", &self.prefetch())
            .field("usehprot", &self.usehprot())
            .field("qspicdis", &self.qspicdis())
            .field("mode", &self.mode())
            .field("scbtp", &self.scbtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Readctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Readctrl {{ ifcdis: {=bool:?}, aidis: {=bool:?}, iccdis: {=bool:?}, ebicdis: {=bool:?}, prefetch: {=bool:?}, usehprot: {=bool:?}, qspicdis: {=bool:?}, mode: {:?}, scbtp: {=bool:?} }}" , self . ifcdis () , self . aidis () , self . iccdis () , self . ebicdis () , self . prefetch () , self . usehprot () , self . qspicdis () , self . mode () , self . scbtp ())
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
    #[doc = "BANK SWITCHING STATUS."]
    #[must_use]
    #[inline(always)]
    pub const fn bankswitched(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BANK SWITCHING STATUS."]
    #[inline(always)]
    pub const fn set_bankswitched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write Data Buffer Valid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wdatavalid(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data Buffer Valid Flag."]
    #[inline(always)]
    pub const fn set_wdatavalid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Flash Power Up Checkerboard Pattern Check Fail Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrupckbdfailcount(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Flash Power Up Checkerboard Pattern Check Fail Count."]
    #[inline(always)]
    pub const fn set_pwrupckbdfailcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
            .field("bankswitched", &self.bankswitched())
            .field("wdatavalid", &self.wdatavalid())
            .field("pwrupckbdfailcount", &self.pwrupckbdfailcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ busy: {=bool:?}, locked: {=bool:?}, invaddr: {=bool:?}, wdataready: {=bool:?}, wordtimeout: {=bool:?}, eraseaborted: {=bool:?}, pcrunning: {=bool:?}, bankswitched: {=bool:?}, wdatavalid: {=u8:?}, pwrupckbdfailcount: {=u8:?} }}" , self . busy () , self . locked () , self . invaddr () , self . wdataready () , self . wordtimeout () , self . eraseaborted () , self . pcrunning () , self . bankswitched () , self . wdatavalid () , self . pwrupckbdfailcount ())
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
    #[doc = "Mass Erase Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn erasemain1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mass Erase Region 1."]
    #[inline(always)]
    pub const fn set_erasemain1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("erasemain1", &self.erasemain1())
            .field("clearwdata", &self.clearwdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Writecmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Writecmd {{ laddrim: {=bool:?}, erasepage: {=bool:?}, writeend: {=bool:?}, writeonce: {=bool:?}, writetrig: {=bool:?}, eraseabort: {=bool:?}, erasemain0: {=bool:?}, erasemain1: {=bool:?}, clearwdata: {=bool:?} }}" , self . laddrim () , self . erasepage () , self . writeend () , self . writeonce () , self . writetrig () , self . eraseabort () , self . erasemain0 () , self . erasemain1 () , self . clearwdata ())
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
    #[doc = "Read-While-Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwwen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Read-While-Write Enable."]
    #[inline(always)]
    pub const fn set_rwwen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("rwwen", &self.rwwen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Writectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Writectrl {{ wren: {=bool:?}, irqeraseabort: {=bool:?}, rwwen: {=bool:?} }}",
            self.wren(),
            self.irqeraseabort(),
            self.rwwen()
        )
    }
}
