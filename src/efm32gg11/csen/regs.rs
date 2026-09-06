#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl(pub u32);
impl Anactrl {
    #[doc = "Reference Current Control."]
    #[must_use]
    #[inline(always)]
    pub const fn irefprog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Reference Current Control."]
    #[inline(always)]
    pub const fn set_irefprog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Current DAC and Reference Current Scale."]
    #[must_use]
    #[inline(always)]
    pub const fn idacirefs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Current DAC and Reference Current Scale."]
    #[inline(always)]
    pub const fn set_idacirefs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Reset Timing."]
    #[must_use]
    #[inline(always)]
    pub const fn trstprog(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Reset Timing."]
    #[inline(always)]
    pub const fn set_trstprog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Anactrl {
    #[inline(always)]
    fn default() -> Anactrl {
        Anactrl(0)
    }
}
impl core::fmt::Debug for Anactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anactrl")
            .field("irefprog", &self.irefprog())
            .field("idacirefs", &self.idacirefs())
            .field("trstprog", &self.trstprog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Anactrl {{ irefprog: {=u8:?}, idacirefs: {=u8:?}, trstprog: {=u8:?} }}",
            self.irefprog(),
            self.idacirefs(),
            self.trstprog()
        )
    }
}
#[doc = "APORT Request Conflict."]
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
    #[doc = "1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2xconflict(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport2xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2yconflict(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport2yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3xconflict(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport3xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3yconflict(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport3yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4xconflict(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport4xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4yconflict(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport4yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("aport2xconflict", &self.aport2xconflict())
            .field("aport2yconflict", &self.aport2yconflict())
            .field("aport3xconflict", &self.aport3xconflict())
            .field("aport3yconflict", &self.aport3yconflict())
            .field("aport4xconflict", &self.aport4xconflict())
            .field("aport4yconflict", &self.aport4yconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportconflict {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aportconflict {{ aport1xconflict: {=bool:?}, aport1yconflict: {=bool:?}, aport2xconflict: {=bool:?}, aport2yconflict: {=bool:?}, aport3xconflict: {=bool:?}, aport3yconflict: {=bool:?}, aport4xconflict: {=bool:?}, aport4yconflict: {=bool:?} }}" , self . aport1xconflict () , self . aport1yconflict () , self . aport2xconflict () , self . aport2yconflict () , self . aport3xconflict () , self . aport3yconflict () , self . aport4xconflict () , self . aport4yconflict ())
    }
}
#[doc = "APORT Request Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportreq(pub u32);
impl Aportreq {
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[inline(always)]
    pub const fn set_aport1xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
    #[inline(always)]
    pub const fn set_aport1yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2xreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[inline(always)]
    pub const fn set_aport2xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1 If the Bus Connected to APORT2Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2yreq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2Y is Requested."]
    #[inline(always)]
    pub const fn set_aport2yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1 If the Bus Connected to APORT3X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3xreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3X is Requested."]
    #[inline(always)]
    pub const fn set_aport3xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 If the Bus Connected to APORT3Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3yreq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3Y is Requested."]
    #[inline(always)]
    pub const fn set_aport3yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 If the Bus Connected to APORT4X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4xreq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4X is Requested."]
    #[inline(always)]
    pub const fn set_aport4xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1 If the Bus Connected to APORT4Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4yreq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4Y is Requested."]
    #[inline(always)]
    pub const fn set_aport4yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("aport2xreq", &self.aport2xreq())
            .field("aport2yreq", &self.aport2yreq())
            .field("aport3xreq", &self.aport3xreq())
            .field("aport3yreq", &self.aport3yreq())
            .field("aport4xreq", &self.aport4xreq())
            .field("aport4yreq", &self.aport4yreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportreq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aportreq {{ aport1xreq: {=bool:?}, aport1yreq: {=bool:?}, aport2xreq: {=bool:?}, aport2yreq: {=bool:?}, aport3xreq: {=bool:?}, aport3yreq: {=bool:?}, aport4xreq: {=bool:?}, aport4yreq: {=bool:?} }}" , self . aport1xreq () , self . aport1yreq () , self . aport2xreq () , self . aport2yreq () , self . aport3xreq () , self . aport3yreq () , self . aport4xreq () , self . aport4yreq ())
    }
}
#[doc = "Command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Start Software-Triggered Conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Software-Triggered Conversions."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
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
        f.debug_struct("Cmd").field("start", &self.start()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ start: {=bool:?} }}", self.start())
    }
}
#[doc = "Comparator Threshold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpthr(pub u32);
impl Cmpthr {
    #[doc = "Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpthr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparator Threshold."]
    #[inline(always)]
    pub const fn set_cmpthr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cmpthr {
    #[inline(always)]
    fn default() -> Cmpthr {
        Cmpthr(0)
    }
}
impl core::fmt::Debug for Cmpthr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpthr")
            .field("cmpthr", &self.cmpthr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpthr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmpthr {{ cmpthr: {=u16:?} }}", self.cmpthr())
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "CSEN Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CSEN Digital Comparator Polarity Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cmppol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Digital Comparator Polarity Select."]
    #[inline(always)]
    pub const fn set_cmppol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CSEN Conversion Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Cm {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cm::from_bits(val as u8)
    }
    #[doc = "CSEN Conversion Mode Select."]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Cm) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SAR Conversion Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn sarcr(&self) -> super::vals::Sarcr {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sarcr::from_bits(val as u8)
    }
    #[doc = "SAR Conversion Resolution."]
    #[inline(always)]
    pub const fn set_sarcr(&mut self, val: super::vals::Sarcr) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CSEN Accumulator Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn acu(&self) -> super::vals::Acu {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Acu::from_bits(val as u8)
    }
    #[doc = "CSEN Accumulator Mode Select."]
    #[inline(always)]
    pub const fn set_acu(&mut self, val: super::vals::Acu) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "CSEN Multiple Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mcen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Multiple Channel Enable."]
    #[inline(always)]
    pub const fn set_mcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Start Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stm(&self) -> super::vals::Stm {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Stm::from_bits(val as u8)
    }
    #[doc = "Start Trigger Select."]
    #[inline(always)]
    pub const fn set_stm(&mut self, val: super::vals::Stm) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CSEN Digital Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Digital Comparator Enable."]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CSEN Disable Right-Shift."]
    #[must_use]
    #[inline(always)]
    pub const fn drsf(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Disable Right-Shift."]
    #[inline(always)]
    pub const fn set_drsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CSEN DMA Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN DMA Enable Bit."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CSEN Converter Select."]
    #[must_use]
    #[inline(always)]
    pub const fn convsel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Converter Select."]
    #[inline(always)]
    pub const fn set_convsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "CSEN Chop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chopen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Chop Enable."]
    #[inline(always)]
    pub const fn set_chopen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CSEN Automatic Ground Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autognd(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Automatic Ground Enable."]
    #[inline(always)]
    pub const fn set_autognd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "CSEN Mux Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn mxuc(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "CSEN Mux Disconnect."]
    #[inline(always)]
    pub const fn set_mxuc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn emacmpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled."]
    #[inline(always)]
    pub const fn set_emacmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Select Warmup Mode for CSEN."]
    #[must_use]
    #[inline(always)]
    pub const fn warmupmode(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Select Warmup Mode for CSEN."]
    #[inline(always)]
    pub const fn set_warmupmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Local Sensing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn localsens(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Local Sensing Enable."]
    #[inline(always)]
    pub const fn set_localsens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Charge Pump Accuracy."]
    #[must_use]
    #[inline(always)]
    pub const fn cpaccuracy(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Charge Pump Accuracy."]
    #[inline(always)]
    pub const fn set_cpaccuracy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("cmppol", &self.cmppol())
            .field("cm", &self.cm())
            .field("sarcr", &self.sarcr())
            .field("acu", &self.acu())
            .field("mcen", &self.mcen())
            .field("stm", &self.stm())
            .field("cmpen", &self.cmpen())
            .field("drsf", &self.drsf())
            .field("dmaen", &self.dmaen())
            .field("convsel", &self.convsel())
            .field("chopen", &self.chopen())
            .field("autognd", &self.autognd())
            .field("mxuc", &self.mxuc())
            .field("emacmpen", &self.emacmpen())
            .field("warmupmode", &self.warmupmode())
            .field("localsens", &self.localsens())
            .field("cpaccuracy", &self.cpaccuracy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, cmppol: {=bool:?}, cm: {:?}, sarcr: {:?}, acu: {:?}, mcen: {=bool:?}, stm: {:?}, cmpen: {=bool:?}, drsf: {=bool:?}, dmaen: {=bool:?}, convsel: {=bool:?}, chopen: {=bool:?}, autognd: {=bool:?}, mxuc: {=bool:?}, emacmpen: {=bool:?}, warmupmode: {=bool:?}, localsens: {=bool:?}, cpaccuracy: {=bool:?} }}" , self . en () , self . cmppol () , self . cm () , self . sarcr () , self . acu () , self . mcen () , self . stm () , self . cmpen () , self . drsf () , self . dmaen () , self . convsel () , self . chopen () , self . autognd () , self . mxuc () , self . emacmpen () , self . warmupmode () , self . localsens () , self . cpaccuracy ())
    }
}
#[doc = "Delta Modulation Baseline."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmbaseline(pub u32);
impl Dmbaseline {
    #[doc = "Delta Modulator Integrator Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn baselineup(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delta Modulator Integrator Initial Value."]
    #[inline(always)]
    pub const fn set_baselineup(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delta Modulator Integrator Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn baselinedn(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delta Modulator Integrator Initial Value."]
    #[inline(always)]
    pub const fn set_baselinedn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dmbaseline {
    #[inline(always)]
    fn default() -> Dmbaseline {
        Dmbaseline(0)
    }
}
impl core::fmt::Debug for Dmbaseline {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmbaseline")
            .field("baselineup", &self.baselineup())
            .field("baselinedn", &self.baselinedn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmbaseline {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmbaseline {{ baselineup: {=u16:?}, baselinedn: {=u16:?} }}",
            self.baselineup(),
            self.baselinedn()
        )
    }
}
#[doc = "Delta Modulation Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcfg(pub u32);
impl Dmcfg {
    #[doc = "Delta Modulator Gain Step."]
    #[must_use]
    #[inline(always)]
    pub const fn dmg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Delta Modulator Gain Step."]
    #[inline(always)]
    pub const fn set_dmg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Delta Modulator Gain Reduction Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn dmr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delta Modulator Gain Reduction Interval."]
    #[inline(always)]
    pub const fn set_dmr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Delta Modulator Conversion Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn dmcr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Delta Modulator Conversion Rate."]
    #[inline(always)]
    pub const fn set_dmcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Delta Modulator Conversion Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn crmode(&self) -> super::vals::Crmode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Crmode::from_bits(val as u8)
    }
    #[doc = "Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub const fn set_crmode(&mut self, val: super::vals::Crmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Delta Modulation Gain Step Reduction Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmgrdis(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Delta Modulation Gain Step Reduction Disable."]
    #[inline(always)]
    pub const fn set_dmgrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Dmcfg {
    #[inline(always)]
    fn default() -> Dmcfg {
        Dmcfg(0)
    }
}
impl core::fmt::Debug for Dmcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmcfg")
            .field("dmg", &self.dmg())
            .field("dmr", &self.dmr())
            .field("dmcr", &self.dmcr())
            .field("crmode", &self.crmode())
            .field("dmgrdis", &self.dmgrdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmcfg {{ dmg: {=u8:?}, dmr: {=u8:?}, dmcr: {=u8:?}, crmode: {:?}, dmgrdis: {=bool:?} }}" , self . dmg () , self . dmr () , self . dmcr () , self . crmode () , self . dmgrdis ())
    }
}
#[doc = "Exponential Moving Average."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ema(pub u32);
impl Ema {
    #[doc = "Calculated Exponential Moving Average."]
    #[must_use]
    #[inline(always)]
    pub const fn ema(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Calculated Exponential Moving Average."]
    #[inline(always)]
    pub const fn set_ema(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Ema {
    #[inline(always)]
    fn default() -> Ema {
        Ema(0)
    }
}
impl core::fmt::Debug for Ema {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ema").field("ema", &self.ema()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ema {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ema {{ ema: {=u32:?} }}", self.ema())
    }
}
#[doc = "Exponential Moving Average Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emactrl(pub u32);
impl Emactrl {
    #[doc = "EMA Sample Weight."]
    #[must_use]
    #[inline(always)]
    pub const fn emasample(&self) -> super::vals::Emasample {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Emasample::from_bits(val as u8)
    }
    #[doc = "EMA Sample Weight."]
    #[inline(always)]
    pub const fn set_emasample(&mut self, val: super::vals::Emasample) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Emactrl {
    #[inline(always)]
    fn default() -> Emactrl {
        Emactrl(0)
    }
}
impl core::fmt::Debug for Emactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emactrl")
            .field("emasample", &self.emasample())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Emactrl {{ emasample: {:?} }}", self.emasample())
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "CMP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CONV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn conv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CONV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_conv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EOS Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eos(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EOS Interrupt Enable."]
    #[inline(always)]
    pub const fn set_eos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMAOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dmaof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
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
            .field("cmp", &self.cmp())
            .field("conv", &self.conv())
            .field("eos", &self.eos())
            .field("dmaof", &self.dmaof())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ cmp: {=bool:?}, conv: {=bool:?}, eos: {=bool:?}, dmaof: {=bool:?}, aportconflict: {=bool:?} }}" , self . cmp () , self . conv () , self . eos () , self . dmaof () , self . aportconflict ())
    }
}
#[doc = "Interrupt Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Digital Comparator Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Comparator Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Conversion Done Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn conv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Done Interrupt Flag."]
    #[inline(always)]
    pub const fn set_conv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End of Scan Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn eos(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "End of Scan Interrupt Flag."]
    #[inline(always)]
    pub const fn set_eos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dmaof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APORT Conflict Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Conflict Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
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
            .field("cmp", &self.cmp())
            .field("conv", &self.conv())
            .field("eos", &self.eos())
            .field("dmaof", &self.dmaof())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ cmp: {=bool:?}, conv: {=bool:?}, eos: {=bool:?}, dmaof: {=bool:?}, aportconflict: {=bool:?} }}" , self . cmp () , self . conv () , self . eos () , self . dmaof () , self . aportconflict ())
    }
}
#[doc = "Interrupt Flag Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set CMP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set CMP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set CONV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn conv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set CONV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_conv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set EOS Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn eos(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set EOS Interrupt Flag."]
    #[inline(always)]
    pub const fn set_eos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set DMAOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set DMAOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dmaof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
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
            .field("cmp", &self.cmp())
            .field("conv", &self.conv())
            .field("eos", &self.eos())
            .field("dmaof", &self.dmaof())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ cmp: {=bool:?}, conv: {=bool:?}, eos: {=bool:?}, dmaof: {=bool:?}, aportconflict: {=bool:?} }}" , self . cmp () , self . conv () , self . eos () , self . dmaof () , self . aportconflict ())
    }
}
#[doc = "PRS Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prssel(pub u32);
impl Prssel {
    #[doc = "PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
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
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prssel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Prssel {{ prssel: {:?} }}", self.prssel())
    }
}
#[doc = "Scan Input Selection 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scaninputsel0(pub u32);
impl Scaninputsel0 {
    #[doc = "CSEN_INPUT0-7 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input0to7sel(&self) -> super::vals::Input0to7sel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Input0to7sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT0-7 Select."]
    #[inline(always)]
    pub const fn set_input0to7sel(&mut self, val: super::vals::Input0to7sel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "CSEN_INPUT8-15 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input8to15sel(&self) -> super::vals::Input8to15sel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Input8to15sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT8-15 Select."]
    #[inline(always)]
    pub const fn set_input8to15sel(&mut self, val: super::vals::Input8to15sel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "CSEN_INPUT16-23 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input16to23sel(&self) -> super::vals::Input16to23sel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Input16to23sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT16-23 Select."]
    #[inline(always)]
    pub const fn set_input16to23sel(&mut self, val: super::vals::Input16to23sel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CSEN_INPUT24-31 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input24to31sel(&self) -> super::vals::Input24to31sel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Input24to31sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT24-31 Select."]
    #[inline(always)]
    pub const fn set_input24to31sel(&mut self, val: super::vals::Input24to31sel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Scaninputsel0 {
    #[inline(always)]
    fn default() -> Scaninputsel0 {
        Scaninputsel0(0)
    }
}
impl core::fmt::Debug for Scaninputsel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scaninputsel0")
            .field("input0to7sel", &self.input0to7sel())
            .field("input8to15sel", &self.input8to15sel())
            .field("input16to23sel", &self.input16to23sel())
            .field("input24to31sel", &self.input24to31sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scaninputsel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scaninputsel0 {{ input0to7sel: {:?}, input8to15sel: {:?}, input16to23sel: {:?}, input24to31sel: {:?} }}" , self . input0to7sel () , self . input8to15sel () , self . input16to23sel () , self . input24to31sel ())
    }
}
#[doc = "Scan Input Selection 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scaninputsel1(pub u32);
impl Scaninputsel1 {
    #[doc = "CSEN_INPUT32-39 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input32to39sel(&self) -> super::vals::Input32to39sel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Input32to39sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT32-39 Select."]
    #[inline(always)]
    pub const fn set_input32to39sel(&mut self, val: super::vals::Input32to39sel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "CSEN_INPUT40-47 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input40to47sel(&self) -> super::vals::Input40to47sel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Input40to47sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT40-47 Select."]
    #[inline(always)]
    pub const fn set_input40to47sel(&mut self, val: super::vals::Input40to47sel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "CSEN_INPUT48-55 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input48to55sel(&self) -> super::vals::Input48to55sel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Input48to55sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT48-55 Select."]
    #[inline(always)]
    pub const fn set_input48to55sel(&mut self, val: super::vals::Input48to55sel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CSEN_INPUT56-63 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn input56to63sel(&self) -> super::vals::Input56to63sel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Input56to63sel::from_bits(val as u8)
    }
    #[doc = "CSEN_INPUT56-63 Select."]
    #[inline(always)]
    pub const fn set_input56to63sel(&mut self, val: super::vals::Input56to63sel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Scaninputsel1 {
    #[inline(always)]
    fn default() -> Scaninputsel1 {
        Scaninputsel1(0)
    }
}
impl core::fmt::Debug for Scaninputsel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scaninputsel1")
            .field("input32to39sel", &self.input32to39sel())
            .field("input40to47sel", &self.input40to47sel())
            .field("input48to55sel", &self.input48to55sel())
            .field("input56to63sel", &self.input56to63sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scaninputsel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scaninputsel1 {{ input32to39sel: {:?}, input40to47sel: {:?}, input48to55sel: {:?}, input56to63sel: {:?} }}" , self . input32to39sel () , self . input40to47sel () , self . input48to55sel () , self . input56to63sel ())
    }
}
#[doc = "Single Conversion Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlectrl(pub u32);
impl Singlectrl {
    #[doc = "Single Channel Input Select."]
    #[must_use]
    #[inline(always)]
    pub const fn singlesel(&self) -> super::vals::Singlesel {
        let val = (self.0 >> 4usize) & 0x7f;
        super::vals::Singlesel::from_bits(val as u8)
    }
    #[doc = "Single Channel Input Select."]
    #[inline(always)]
    pub const fn set_singlesel(&mut self, val: super::vals::Singlesel) {
        self.0 = (self.0 & !(0x7f << 4usize)) | (((val.to_bits() as u32) & 0x7f) << 4usize);
    }
}
impl Default for Singlectrl {
    #[inline(always)]
    fn default() -> Singlectrl {
        Singlectrl(0)
    }
}
impl core::fmt::Debug for Singlectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Singlectrl")
            .field("singlesel", &self.singlesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Singlectrl {{ singlesel: {:?} }}", self.singlesel())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn csenbusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy Flag."]
    #[inline(always)]
    pub const fn set_csenbusy(&mut self, val: bool) {
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
            .field("csenbusy", &self.csenbusy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ csenbusy: {=bool:?} }}", self.csenbusy())
    }
}
#[doc = "Timing Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timctrl(pub u32);
impl Timctrl {
    #[doc = "Period Counter Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn pcpresc(&self) -> super::vals::Pcpresc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pcpresc::from_bits(val as u8)
    }
    #[doc = "Period Counter Prescaler."]
    #[inline(always)]
    pub const fn set_pcpresc(&mut self, val: super::vals::Pcpresc) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Period Counter Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pctop(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Period Counter Top Value."]
    #[inline(always)]
    pub const fn set_pctop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Warmup Period Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn warmupcnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Warmup Period Counter."]
    #[inline(always)]
    pub const fn set_warmupcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Timctrl {
    #[inline(always)]
    fn default() -> Timctrl {
        Timctrl(0)
    }
}
impl core::fmt::Debug for Timctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timctrl")
            .field("pcpresc", &self.pcpresc())
            .field("pctop", &self.pctop())
            .field("warmupcnt", &self.warmupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timctrl {{ pcpresc: {:?}, pctop: {=u8:?}, warmupcnt: {=u8:?} }}",
            self.pcpresc(),
            self.pctop(),
            self.warmupcnt()
        )
    }
}
