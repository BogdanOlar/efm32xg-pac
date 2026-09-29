#[doc = "APORT Request Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportconflict(pub u32);
impl Aportconflict {
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport1xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yconflict(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport1yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Aportconflict {
    #[inline(always)]
    fn default() -> Aportconflict {
        Aportconflict(0)
    }
}
impl core::fmt::Debug for Aportconflict {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aportconflict")
            .field("aport1xconflict", &self.aport1xconflict())
            .field("aport1yconflict", &self.aport1yconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportconflict {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aportconflict {{ aport1xconflict: {=bool:?}, aport1yconflict: {=bool:?} }}",
            self.aport1xconflict(),
            self.aport1yconflict()
        )
    }
}
#[doc = "APORT Request Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportreq(pub u32);
impl Aportreq {
    #[doc = "1 If the APORT Bus Connected to APORT1X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the APORT Bus Connected to APORT1X is Requested."]
    #[inline(always)]
    pub const fn set_aport1xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 If the Bus Connected to APORT1Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1Y is Requested."]
    #[inline(always)]
    pub const fn set_aport1yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Aportreq {
    #[inline(always)]
    fn default() -> Aportreq {
        Aportreq(0)
    }
}
impl core::fmt::Debug for Aportreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aportreq")
            .field("aport1xreq", &self.aport1xreq())
            .field("aport1yreq", &self.aport1yreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aportreq {{ aport1xreq: {=bool:?}, aport1yreq: {=bool:?} }}",
            self.aport1xreq(),
            self.aport1yreq()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Current DAC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current DAC Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Current Sink Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cursink(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Current Sink Enable."]
    #[inline(always)]
    pub const fn set_cursink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Minimum Output Transition Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn minouttrans(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum Output Transition Enable."]
    #[inline(always)]
    pub const fn set_minouttrans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "APORT Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportouten(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Output Enable."]
    #[inline(always)]
    pub const fn set_aportouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APORT Output Select."]
    #[must_use]
    #[inline(always)]
    pub const fn aportoutsel(&self) -> super::vals::Aportoutsel {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Aportoutsel::from_bits(val as u8)
    }
    #[doc = "APORT Output Select."]
    #[inline(always)]
    pub const fn set_aportoutsel(&mut self, val: super::vals::Aportoutsel) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Power Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Select."]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EM2 Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn em2delay(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EM2 Delay."]
    #[inline(always)]
    pub const fn set_em2delay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "APORT Bus Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportmasterdis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus Master Disable."]
    #[inline(always)]
    pub const fn set_aportmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PRS Controlled APORT Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportoutenprs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Controlled APORT Output Enable."]
    #[inline(always)]
    pub const fn set_aportoutenprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "IDAC Output Enable PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "IDAC Output Enable PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
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
            .field("cursink", &self.cursink())
            .field("minouttrans", &self.minouttrans())
            .field("aportouten", &self.aportouten())
            .field("aportoutsel", &self.aportoutsel())
            .field("pwrsel", &self.pwrsel())
            .field("em2delay", &self.em2delay())
            .field("aportmasterdis", &self.aportmasterdis())
            .field("aportoutenprs", &self.aportoutenprs())
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, cursink: {=bool:?}, minouttrans: {=bool:?}, aportouten: {=bool:?}, aportoutsel: {:?}, pwrsel: {=bool:?}, em2delay: {=bool:?}, aportmasterdis: {=bool:?}, aportoutenprs: {=bool:?}, prssel: {:?} }}" , self . en () , self . cursink () , self . minouttrans () , self . aportouten () , self . aportoutsel () , self . pwrsel () , self . em2delay () , self . aportmasterdis () , self . aportoutenprs () , self . prssel ())
    }
}
#[doc = "Current Programming Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Curprog(pub u32);
impl Curprog {
    #[doc = "Current Range Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rangesel(&self) -> super::vals::Rangesel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rangesel::from_bits(val as u8)
    }
    #[doc = "Current Range Select."]
    #[inline(always)]
    pub const fn set_rangesel(&mut self, val: super::vals::Rangesel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Current Step Size Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stepsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Current Step Size Select."]
    #[inline(always)]
    pub const fn set_stepsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Tune the Current to Given Accuracy."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Tune the Current to Given Accuracy."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Curprog {
    #[inline(always)]
    fn default() -> Curprog {
        Curprog(0)
    }
}
impl core::fmt::Debug for Curprog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Curprog")
            .field("rangesel", &self.rangesel())
            .field("stepsel", &self.stepsel())
            .field("tuning", &self.tuning())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Curprog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Curprog {{ rangesel: {:?}, stepsel: {=u8:?}, tuning: {=u8:?} }}",
            self.rangesel(),
            self.stepsel(),
            self.tuning()
        )
    }
}
#[doc = "Duty Cycle Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dutyconfig(pub u32);
impl Dutyconfig {
    #[doc = "Duty Cycle Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em2dutycycledis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Duty Cycle Enable."]
    #[inline(always)]
    pub const fn set_em2dutycycledis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dutyconfig {
    #[inline(always)]
    fn default() -> Dutyconfig {
        Dutyconfig(0)
    }
}
impl core::fmt::Debug for Dutyconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dutyconfig")
            .field("em2dutycycledis", &self.em2dutycycledis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dutyconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dutyconfig {{ em2dutycycledis: {=bool:?} }}",
            self.em2dutycycledis()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ aportconflict: {=bool:?} }}",
            self.aportconflict()
        )
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "APORT Conflict Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Conflict Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If {{ aportconflict: {=bool:?} }}", self.aportconflict())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ aportconflict: {=bool:?} }}",
            self.aportconflict()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "APORT Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Conflict Output."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ aportconflict: {=bool:?} }}",
            self.aportconflict()
        )
    }
}
