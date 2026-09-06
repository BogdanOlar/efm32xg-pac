#[doc = "Bit Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bittiming(pub u32);
impl Bittiming {
    #[doc = "Baud Rate Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn brp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Baud Rate Prescaler."]
    #[inline(always)]
    pub const fn set_brp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Synchronization Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn sjw(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization Jump Width."]
    #[inline(always)]
    pub const fn set_sjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Time Segment Before the Sample Point."]
    #[must_use]
    #[inline(always)]
    pub const fn tseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Time Segment Before the Sample Point."]
    #[inline(always)]
    pub const fn set_tseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Time Segment After the Sample Point."]
    #[must_use]
    #[inline(always)]
    pub const fn tseg2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Time Segment After the Sample Point."]
    #[inline(always)]
    pub const fn set_tseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
}
impl Default for Bittiming {
    #[inline(always)]
    fn default() -> Bittiming {
        Bittiming(0)
    }
}
impl core::fmt::Debug for Bittiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bittiming")
            .field("brp", &self.brp())
            .field("sjw", &self.sjw())
            .field("tseg1", &self.tseg1())
            .field("tseg2", &self.tseg2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bittiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bittiming {{ brp: {=u8:?}, sjw: {=u8:?}, tseg1: {=u8:?}, tseg2: {=u8:?} }}",
            self.brp(),
            self.sjw(),
            self.tseg1(),
            self.tseg2()
        )
    }
}
#[doc = "BRP Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brpe(pub u32);
impl Brpe {
    #[doc = "Baud Rate Prescaler Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn brpe(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Baud Rate Prescaler Extension."]
    #[inline(always)]
    pub const fn set_brpe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Brpe {
    #[inline(always)]
    fn default() -> Brpe {
        Brpe(0)
    }
}
impl core::fmt::Debug for Brpe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brpe").field("brpe", &self.brpe()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brpe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Brpe {{ brpe: {=u8:?} }}", self.brpe())
    }
}
#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Debug Halt."]
    #[must_use]
    #[inline(always)]
    pub const fn dbghalt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Halt."]
    #[inline(always)]
    pub const fn set_dbghalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("dbghalt", &self.dbghalt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config {{ dbghalt: {=bool:?} }}", self.dbghalt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Initialize."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialize."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Status Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Status Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_eie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable Automatic Retransmission."]
    #[must_use]
    #[inline(always)]
    pub const fn dar(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Automatic Retransmission."]
    #[inline(always)]
    pub const fn set_dar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Configuration Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Change Enable."]
    #[inline(always)]
    pub const fn set_cce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test Mode Enable Write."]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test Mode Enable Write."]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("init", &self.init())
            .field("ie", &self.ie())
            .field("sie", &self.sie())
            .field("eie", &self.eie())
            .field("dar", &self.dar())
            .field("cce", &self.cce())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ init: {=bool:?}, ie: {=bool:?}, sie: {=bool:?}, eie: {=bool:?}, dar: {=bool:?}, cce: {=bool:?}, test: {=bool:?} }}" , self . init () , self . ie () , self . sie () , self . eie () , self . dar () , self . cce () , self . test ())
    }
}
#[doc = "Error Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errcnt(pub u32);
impl Errcnt {
    #[doc = "Transmit Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter."]
    #[inline(always)]
    pub const fn set_tec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive Error Counter."]
    #[inline(always)]
    pub const fn set_rec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive Error Passive."]
    #[must_use]
    #[inline(always)]
    pub const fn recerrp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Passive."]
    #[inline(always)]
    pub const fn set_recerrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Errcnt {
    #[inline(always)]
    fn default() -> Errcnt {
        Errcnt(0)
    }
}
impl core::fmt::Debug for Errcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Errcnt")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("recerrp", &self.recerrp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Errcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Errcnt {{ tec: {=u8:?}, rec: {=u8:?}, recerrp: {=bool:?} }}",
            self.tec(),
            self.rec(),
            self.recerrp()
        )
    }
}
#[doc = "Status Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If1ien(pub u32);
impl If1ien {
    #[doc = "STATUS Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "STATUS Interrupt Enable."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for If1ien {
    #[inline(always)]
    fn default() -> If1ien {
        If1ien(0)
    }
}
impl core::fmt::Debug for If1ien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If1ien")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If1ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If1ien {{ status: {=bool:?} }}", self.status())
    }
}
#[doc = "Status Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If1if(pub u32);
impl If1if {
    #[doc = "Status Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Status Interrupt Flag."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for If1if {
    #[inline(always)]
    fn default() -> If1if {
        If1if(0)
    }
}
impl core::fmt::Debug for If1if {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If1if")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If1if {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If1if {{ status: {=bool:?} }}", self.status())
    }
}
#[doc = "Message Object Interrupt Flag Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If1ifc(pub u32);
impl If1ifc {
    #[doc = "Clear STATUS Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear STATUS Interrupt Flag."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for If1ifc {
    #[inline(always)]
    fn default() -> If1ifc {
        If1ifc(0)
    }
}
impl core::fmt::Debug for If1ifc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If1ifc")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If1ifc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If1ifc {{ status: {=bool:?} }}", self.status())
    }
}
#[doc = "Message Object Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If1ifs(pub u32);
impl If1ifs {
    #[doc = "Set STATUS Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set STATUS Interrupt Flag."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for If1ifs {
    #[inline(always)]
    fn default() -> If1ifs {
        If1ifs(0)
    }
}
impl core::fmt::Debug for If1ifs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If1ifs")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If1ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If1ifs {{ status: {=bool:?} }}", self.status())
    }
}
#[doc = "Interrupt Identification Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intid(pub u32);
impl Intid {
    #[doc = "Interrupt Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn intid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Interrupt Identifier."]
    #[inline(always)]
    pub const fn set_intid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Status Interupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intstat(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status Interupt."]
    #[inline(always)]
    pub const fn set_intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Intid {
    #[inline(always)]
    fn default() -> Intid {
        Intid(0)
    }
}
impl core::fmt::Debug for Intid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intid")
            .field("intid", &self.intid())
            .field("intstat", &self.intstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intid {{ intid: {=u8:?}, intstat: {=bool:?} }}",
            self.intid(),
            self.intstat()
        )
    }
}
#[doc = "Interface Arbitration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Arb(pub u32);
impl Mir0Arb {
    #[doc = "Message Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Message Identifier."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Message Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Message Direction."]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Extended Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn xtd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Identifier."]
    #[inline(always)]
    pub const fn set_xtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Message Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn msgval(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Message Valid."]
    #[inline(always)]
    pub const fn set_msgval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mir0Arb {
    #[inline(always)]
    fn default() -> Mir0Arb {
        Mir0Arb(0)
    }
}
impl core::fmt::Debug for Mir0Arb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Arb")
            .field("id", &self.id())
            .field("dir", &self.dir())
            .field("xtd", &self.xtd())
            .field("msgval", &self.msgval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Arb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mir0Arb {{ id: {=u32:?}, dir: {=bool:?}, xtd: {=bool:?}, msgval: {=bool:?} }}",
            self.id(),
            self.dir(),
            self.xtd(),
            self.msgval()
        )
    }
}
#[doc = "Interface Command Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Cmdmask(pub u32);
impl Mir0Cmdmask {
    #[doc = "CC Channel Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn datab(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CC Channel Mode."]
    #[inline(always)]
    pub const fn set_datab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Access Data Bytes 0-3."]
    #[must_use]
    #[inline(always)]
    pub const fn dataa(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Access Data Bytes 0-3."]
    #[inline(always)]
    pub const fn set_dataa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmission Request Bit/ New Data Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn txrqstnewdat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Request Bit/ New Data Bit."]
    #[inline(always)]
    pub const fn set_txrqstnewdat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Interrupt Pending Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn clrintpnd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Interrupt Pending Bit."]
    #[inline(always)]
    pub const fn set_clrintpnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Access Control Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn control(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Access Control Bits."]
    #[inline(always)]
    pub const fn set_control(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Access Arbitration Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn arbacc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Access Arbitration Bits."]
    #[inline(always)]
    pub const fn set_arbacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Access Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn maskacc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Access Mask Bits."]
    #[inline(always)]
    pub const fn set_maskacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write/Read RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn wrrd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write/Read RAM."]
    #[inline(always)]
    pub const fn set_wrrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Mir0Cmdmask {
    #[inline(always)]
    fn default() -> Mir0Cmdmask {
        Mir0Cmdmask(0)
    }
}
impl core::fmt::Debug for Mir0Cmdmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Cmdmask")
            .field("datab", &self.datab())
            .field("dataa", &self.dataa())
            .field("txrqstnewdat", &self.txrqstnewdat())
            .field("clrintpnd", &self.clrintpnd())
            .field("control", &self.control())
            .field("arbacc", &self.arbacc())
            .field("maskacc", &self.maskacc())
            .field("wrrd", &self.wrrd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Cmdmask {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mir0Cmdmask {{ datab: {=bool:?}, dataa: {=bool:?}, txrqstnewdat: {=bool:?}, clrintpnd: {=bool:?}, control: {=bool:?}, arbacc: {=bool:?}, maskacc: {=bool:?}, wrrd: {=bool:?} }}" , self . datab () , self . dataa () , self . txrqstnewdat () , self . clrintpnd () , self . control () , self . arbacc () , self . maskacc () , self . wrrd ())
    }
}
#[doc = "Interface Command Request Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Cmdreq(pub u32);
impl Mir0Cmdreq {
    #[doc = "Message Number."]
    #[must_use]
    #[inline(always)]
    pub const fn msgnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Message Number."]
    #[inline(always)]
    pub const fn set_msgnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Busy Flag."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Mir0Cmdreq {
    #[inline(always)]
    fn default() -> Mir0Cmdreq {
        Mir0Cmdreq(0)
    }
}
impl core::fmt::Debug for Mir0Cmdreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Cmdreq")
            .field("msgnum", &self.msgnum())
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Cmdreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mir0Cmdreq {{ msgnum: {=u8:?}, busy: {=bool:?} }}",
            self.msgnum(),
            self.busy()
        )
    }
}
#[doc = "Interface Message Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Ctrl(pub u32);
impl Mir0Ctrl {
    #[doc = "Data Length Code."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Length Code."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "End of Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn eob(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End of Buffer."]
    #[inline(always)]
    pub const fn set_eob(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmit Request."]
    #[must_use]
    #[inline(always)]
    pub const fn txrqst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Request."]
    #[inline(always)]
    pub const fn set_txrqst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Remote Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rmten(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Enable."]
    #[inline(always)]
    pub const fn set_rmten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Use Acceptance Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn umask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Use Acceptance Mask."]
    #[inline(always)]
    pub const fn set_umask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn intpnd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Pending."]
    #[inline(always)]
    pub const fn set_intpnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Message Lost (only Valid for Message Objects With Direction = Receive)."]
    #[must_use]
    #[inline(always)]
    pub const fn messageof(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Message Lost (only Valid for Message Objects With Direction = Receive)."]
    #[inline(always)]
    pub const fn set_messageof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "New Data."]
    #[must_use]
    #[inline(always)]
    pub const fn datavalid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "New Data."]
    #[inline(always)]
    pub const fn set_datavalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Mir0Ctrl {
    #[inline(always)]
    fn default() -> Mir0Ctrl {
        Mir0Ctrl(0)
    }
}
impl core::fmt::Debug for Mir0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Ctrl")
            .field("dlc", &self.dlc())
            .field("eob", &self.eob())
            .field("txrqst", &self.txrqst())
            .field("rmten", &self.rmten())
            .field("rxie", &self.rxie())
            .field("txie", &self.txie())
            .field("umask", &self.umask())
            .field("intpnd", &self.intpnd())
            .field("messageof", &self.messageof())
            .field("datavalid", &self.datavalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mir0Ctrl {{ dlc: {=u8:?}, eob: {=bool:?}, txrqst: {=bool:?}, rmten: {=bool:?}, rxie: {=bool:?}, txie: {=bool:?}, umask: {=bool:?}, intpnd: {=bool:?}, messageof: {=bool:?}, datavalid: {=bool:?} }}" , self . dlc () , self . eob () , self . txrqst () , self . rmten () , self . rxie () , self . txie () , self . umask () , self . intpnd () , self . messageof () , self . datavalid ())
    }
}
#[doc = "Interface Data B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Datah(pub u32);
impl Mir0Datah {
    #[doc = "Fifth Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fifth Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sixth Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sixth Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Seventh Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Seventh Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Eight Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Eight Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mir0Datah {
    #[inline(always)]
    fn default() -> Mir0Datah {
        Mir0Datah(0)
    }
}
impl core::fmt::Debug for Mir0Datah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Datah")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Datah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mir0Datah {{ data4: {=u8:?}, data5: {=u8:?}, data6: {=u8:?}, data7: {=u8:?} }}",
            self.data4(),
            self.data5(),
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "Interface Data a Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Datal(pub u32);
impl Mir0Datal {
    #[doc = "First Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "First Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Second Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Second Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Third Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Third Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Fourth Byte of CAN Data Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fourth Byte of CAN Data Frame."]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mir0Datal {
    #[inline(always)]
    fn default() -> Mir0Datal {
        Mir0Datal(0)
    }
}
impl core::fmt::Debug for Mir0Datal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Datal")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Datal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mir0Datal {{ data0: {=u8:?}, data1: {=u8:?}, data2: {=u8:?}, data3: {=u8:?} }}",
            self.data0(),
            self.data1(),
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "Interface Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mir0Mask(pub u32);
impl Mir0Mask {
    #[doc = "Identifier Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Identifier Mask."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Mask Message Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn mdir(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Mask Message Direction."]
    #[inline(always)]
    pub const fn set_mdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Mask Extended Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn mxtd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Mask Extended Identifier."]
    #[inline(always)]
    pub const fn set_mxtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mir0Mask {
    #[inline(always)]
    fn default() -> Mir0Mask {
        Mir0Mask(0)
    }
}
impl core::fmt::Debug for Mir0Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mir0Mask")
            .field("mask", &self.mask())
            .field("mdir", &self.mdir())
            .field("mxtd", &self.mxtd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mir0Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mir0Mask {{ mask: {=u32:?}, mdir: {=bool:?}, mxtd: {=bool:?} }}",
            self.mask(),
            self.mdir(),
            self.mxtd()
        )
    }
}
#[doc = "I/O Routing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Route(pub u32);
impl Route {
    #[doc = "TX Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX Pin Enable."]
    #[inline(always)]
    pub const fn set_txpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Pin Location."]
    #[must_use]
    #[inline(always)]
    pub const fn rxloc(&self) -> super::vals::Rxloc {
        let val = (self.0 >> 2usize) & 0x3f;
        super::vals::Rxloc::from_bits(val as u8)
    }
    #[doc = "RX Pin Location."]
    #[inline(always)]
    pub const fn set_rxloc(&mut self, val: super::vals::Rxloc) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val.to_bits() as u32) & 0x3f) << 2usize);
    }
    #[doc = "TX Pin Location."]
    #[must_use]
    #[inline(always)]
    pub const fn txloc(&self) -> super::vals::Txloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Txloc::from_bits(val as u8)
    }
    #[doc = "TX Pin Location."]
    #[inline(always)]
    pub const fn set_txloc(&mut self, val: super::vals::Txloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
}
impl Default for Route {
    #[inline(always)]
    fn default() -> Route {
        Route(0)
    }
}
impl core::fmt::Debug for Route {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Route")
            .field("txpen", &self.txpen())
            .field("rxloc", &self.rxloc())
            .field("txloc", &self.txloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Route {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Route {{ txpen: {=bool:?}, rxloc: {:?}, txloc: {:?} }}",
            self.txpen(),
            self.rxloc(),
            self.txloc()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Last Error Code."]
    #[must_use]
    #[inline(always)]
    pub const fn lec(&self) -> super::vals::Lec {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lec::from_bits(val as u8)
    }
    #[doc = "Last Error Code."]
    #[inline(always)]
    pub const fn set_lec(&mut self, val: super::vals::Lec) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Transmitted a Message Successfully."]
    #[must_use]
    #[inline(always)]
    pub const fn txok(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitted a Message Successfully."]
    #[inline(always)]
    pub const fn set_txok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Received a Message Successfully."]
    #[must_use]
    #[inline(always)]
    pub const fn rxok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Received a Message Successfully."]
    #[inline(always)]
    pub const fn set_rxok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Error Passive."]
    #[must_use]
    #[inline(always)]
    pub const fn epass(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive."]
    #[inline(always)]
    pub const fn set_epass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ewarn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status."]
    #[inline(always)]
    pub const fn set_ewarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus Off Status."]
    #[must_use]
    #[inline(always)]
    pub const fn boff(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Status."]
    #[inline(always)]
    pub const fn set_boff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("lec", &self.lec())
            .field("txok", &self.txok())
            .field("rxok", &self.rxok())
            .field("epass", &self.epass())
            .field("ewarn", &self.ewarn())
            .field("boff", &self.boff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ lec: {:?}, txok: {=bool:?}, rxok: {=bool:?}, epass: {=bool:?}, ewarn: {=bool:?}, boff: {=bool:?} }}" , self . lec () , self . txok () , self . rxok () , self . epass () , self . ewarn () , self . boff ())
    }
}
#[doc = "Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test(pub u32);
impl Test {
    #[doc = "Basic Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn basic(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Basic Mode."]
    #[inline(always)]
    pub const fn set_basic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Silent Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn silent(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Silent Mode."]
    #[inline(always)]
    pub const fn set_silent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Loopback Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lback(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Mode."]
    #[inline(always)]
    pub const fn set_lback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of CAN_TX Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Tx::from_bits(val as u8)
    }
    #[doc = "Control of CAN_TX Pin."]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Monitors the Actual Value of CAN_RX Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the Actual Value of CAN_RX Pin."]
    #[inline(always)]
    pub const fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Test {
    #[inline(always)]
    fn default() -> Test {
        Test(0)
    }
}
impl core::fmt::Debug for Test {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Test")
            .field("basic", &self.basic())
            .field("silent", &self.silent())
            .field("lback", &self.lback())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Test {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Test {{ basic: {=bool:?}, silent: {=bool:?}, lback: {=bool:?}, tx: {:?}, rx: {=bool:?} }}" , self . basic () , self . silent () , self . lback () , self . tx () , self . rx ())
    }
}
#[doc = "Transmission Request Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Transreq(pub u32);
impl Transreq {
    #[doc = "Transmission Request Bits (Of All Message Objects)."]
    #[must_use]
    #[inline(always)]
    pub const fn txrqstout(&self) -> super::vals::Txrqstout {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Txrqstout::from_bits(val as u32)
    }
    #[doc = "Transmission Request Bits (Of All Message Objects)."]
    #[inline(always)]
    pub const fn set_txrqstout(&mut self, val: super::vals::Txrqstout) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Transreq {
    #[inline(always)]
    fn default() -> Transreq {
        Transreq(0)
    }
}
impl core::fmt::Debug for Transreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Transreq")
            .field("txrqstout", &self.txrqstout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Transreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Transreq {{ txrqstout: {:?} }}", self.txrqstout())
    }
}
