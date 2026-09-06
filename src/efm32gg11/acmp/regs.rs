#[doc = "APORT Conflict Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportconflict(pub u32);
impl Aportconflict {
    #[doc = "1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport0xconflict(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport0xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport0yconflict(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport0yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
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
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yconflict(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
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
            .field("aport0xconflict", &self.aport0xconflict())
            .field("aport0yconflict", &self.aport0yconflict())
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
        defmt :: write ! (f , "Aportconflict {{ aport0xconflict: {=bool:?}, aport0yconflict: {=bool:?}, aport1xconflict: {=bool:?}, aport1yconflict: {=bool:?}, aport2xconflict: {=bool:?}, aport2yconflict: {=bool:?}, aport3xconflict: {=bool:?}, aport3yconflict: {=bool:?}, aport4xconflict: {=bool:?}, aport4yconflict: {=bool:?} }}" , self . aport0xconflict () , self . aport0yconflict () , self . aport1xconflict () , self . aport1yconflict () , self . aport2xconflict () , self . aport2yconflict () , self . aport3xconflict () , self . aport3yconflict () , self . aport4xconflict () , self . aport4yconflict ())
    }
}
#[doc = "APORT Request Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportreq(pub u32);
impl Aportreq {
    #[doc = "1 If the Bus Connected to APORT0X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport0xreq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT0X is Requested."]
    #[inline(always)]
    pub const fn set_aport0xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 If the Bus Connected to APORT0Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport0yreq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT0Y is Requested."]
    #[inline(always)]
    pub const fn set_aport0yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
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
            .field("aport0xreq", &self.aport0xreq())
            .field("aport0yreq", &self.aport0yreq())
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
        defmt :: write ! (f , "Aportreq {{ aport0xreq: {=bool:?}, aport0yreq: {=bool:?}, aport1xreq: {=bool:?}, aport1yreq: {=bool:?}, aport2xreq: {=bool:?}, aport2yreq: {=bool:?}, aport3xreq: {=bool:?}, aport3yreq: {=bool:?}, aport4xreq: {=bool:?}, aport4yreq: {=bool:?} }}" , self . aport0xreq () , self . aport0yreq () , self . aport1xreq () , self . aport1yreq () , self . aport2xreq () , self . aport2yreq () , self . aport3xreq () , self . aport3yreq () , self . aport4xreq () , self . aport4yreq ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Analog Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Inactive Value."]
    #[must_use]
    #[inline(always)]
    pub const fn inactval(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Inactive Value."]
    #[inline(always)]
    pub const fn set_inactval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator GPIO Output Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn gpioinv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator GPIO Output Invert."]
    #[inline(always)]
    pub const fn set_gpioinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APORT Bus X Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportxmasterdis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus X Master Disable."]
    #[inline(always)]
    pub const fn set_aportxmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "APORT Bus Y Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportymasterdis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus Y Master Disable."]
    #[inline(always)]
    pub const fn set_aportymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "APORT Bus Master Disable for Bus Selected By VASEL."]
    #[must_use]
    #[inline(always)]
    pub const fn aportvmasterdis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus Master Disable for Bus Selected By VASEL."]
    #[inline(always)]
    pub const fn set_aportvmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsel(&self) -> super::vals::Pwrsel {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Select."]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: super::vals::Pwrsel) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "ACMP Accuracy Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn accuracy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ACMP Accuracy Mode."]
    #[inline(always)]
    pub const fn set_accuracy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Input Range."]
    #[must_use]
    #[inline(always)]
    pub const fn inputrange(&self) -> super::vals::Inputrange {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Inputrange::from_bits(val as u8)
    }
    #[doc = "Input Range."]
    #[inline(always)]
    pub const fn set_inputrange(&mut self, val: super::vals::Inputrange) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Rising Edge Interrupt Sense."]
    #[must_use]
    #[inline(always)]
    pub const fn irise(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Rising Edge Interrupt Sense."]
    #[inline(always)]
    pub const fn set_irise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Falling Edge Interrupt Sense."]
    #[must_use]
    #[inline(always)]
    pub const fn ifall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Falling Edge Interrupt Sense."]
    #[inline(always)]
    pub const fn set_ifall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Bias Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn biasprog(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Bias Configuration."]
    #[inline(always)]
    pub const fn set_biasprog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Full Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn fullbias(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Full Bias Current."]
    #[inline(always)]
    pub const fn set_fullbias(&mut self, val: bool) {
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
            .field("inactval", &self.inactval())
            .field("gpioinv", &self.gpioinv())
            .field("aportxmasterdis", &self.aportxmasterdis())
            .field("aportymasterdis", &self.aportymasterdis())
            .field("aportvmasterdis", &self.aportvmasterdis())
            .field("pwrsel", &self.pwrsel())
            .field("accuracy", &self.accuracy())
            .field("inputrange", &self.inputrange())
            .field("irise", &self.irise())
            .field("ifall", &self.ifall())
            .field("biasprog", &self.biasprog())
            .field("fullbias", &self.fullbias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, inactval: {=bool:?}, gpioinv: {=bool:?}, aportxmasterdis: {=bool:?}, aportymasterdis: {=bool:?}, aportvmasterdis: {=bool:?}, pwrsel: {:?}, accuracy: {=bool:?}, inputrange: {:?}, irise: {=bool:?}, ifall: {=bool:?}, biasprog: {=u8:?}, fullbias: {=bool:?} }}" , self . en () , self . inactval () , self . gpioinv () , self . aportxmasterdis () , self . aportymasterdis () , self . aportvmasterdis () , self . pwrsel () , self . accuracy () , self . inputrange () , self . irise () , self . ifall () , self . biasprog () , self . fullbias ())
    }
}
#[doc = "External Override Interface Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extifctrl(pub u32);
impl Extifctrl {
    #[doc = "Enable External Interface."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External Interface."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APORT Selection for External Interface."]
    #[must_use]
    #[inline(always)]
    pub const fn aportsel(&self) -> super::vals::Aportsel {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Aportsel::from_bits(val as u8)
    }
    #[doc = "APORT Selection for External Interface."]
    #[inline(always)]
    pub const fn set_aportsel(&mut self, val: super::vals::Aportsel) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Extifctrl {
    #[inline(always)]
    fn default() -> Extifctrl {
        Extifctrl(0)
    }
}
impl core::fmt::Debug for Extifctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extifctrl")
            .field("en", &self.en())
            .field("aportsel", &self.aportsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extifctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Extifctrl {{ en: {=bool:?}, aportsel: {:?} }}",
            self.en(),
            self.aportsel()
        )
    }
}
#[doc = "Hysteresis 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hysteresis0(pub u32);
impl Hysteresis0 {
    #[doc = "Hysteresis Select When ACMPOUT=0."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::Hysteresis0Hyst {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Hysteresis0Hyst::from_bits(val as u8)
    }
    #[doc = "Hysteresis Select When ACMPOUT=0."]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: super::vals::Hysteresis0Hyst) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Divider for VA Voltage When ACMPOUT=0."]
    #[must_use]
    #[inline(always)]
    pub const fn divva(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Divider for VA Voltage When ACMPOUT=0."]
    #[inline(always)]
    pub const fn set_divva(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Divider for VB Voltage When ACMPOUT=0."]
    #[must_use]
    #[inline(always)]
    pub const fn divvb(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divider for VB Voltage When ACMPOUT=0."]
    #[inline(always)]
    pub const fn set_divvb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Hysteresis0 {
    #[inline(always)]
    fn default() -> Hysteresis0 {
        Hysteresis0(0)
    }
}
impl core::fmt::Debug for Hysteresis0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hysteresis0")
            .field("hyst", &self.hyst())
            .field("divva", &self.divva())
            .field("divvb", &self.divvb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hysteresis0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hysteresis0 {{ hyst: {:?}, divva: {=u8:?}, divvb: {=u8:?} }}",
            self.hyst(),
            self.divva(),
            self.divvb()
        )
    }
}
#[doc = "Hysteresis 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hysteresis1(pub u32);
impl Hysteresis1 {
    #[doc = "Hysteresis Select When ACMPOUT=1."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::Hysteresis1Hyst {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Hysteresis1Hyst::from_bits(val as u8)
    }
    #[doc = "Hysteresis Select When ACMPOUT=1."]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: super::vals::Hysteresis1Hyst) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Divider for VA Voltage When ACMPOUT=1."]
    #[must_use]
    #[inline(always)]
    pub const fn divva(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Divider for VA Voltage When ACMPOUT=1."]
    #[inline(always)]
    pub const fn set_divva(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Divider for VB Voltage When ACMPOUT=1."]
    #[must_use]
    #[inline(always)]
    pub const fn divvb(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divider for VB Voltage When ACMPOUT=1."]
    #[inline(always)]
    pub const fn set_divvb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Hysteresis1 {
    #[inline(always)]
    fn default() -> Hysteresis1 {
        Hysteresis1(0)
    }
}
impl core::fmt::Debug for Hysteresis1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hysteresis1")
            .field("hyst", &self.hyst())
            .field("divva", &self.divva())
            .field("divvb", &self.divvb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hysteresis1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hysteresis1 {{ hyst: {:?}, divva: {=u8:?}, divvb: {=u8:?} }}",
            self.hyst(),
            self.divva(),
            self.divvb()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "EDGE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edge(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EDGE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WARMUP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn warmup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WARMUP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_warmup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("edge", &self.edge())
            .field("warmup", &self.warmup())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ edge: {=bool:?}, warmup: {=bool:?}, aportconflict: {=bool:?} }}",
            self.edge(),
            self.warmup(),
            self.aportconflict()
        )
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Edge Triggered Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn edge(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Triggered Interrupt Flag."]
    #[inline(always)]
    pub const fn set_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Warm-up Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn warmup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Warm-up Interrupt Flag."]
    #[inline(always)]
    pub const fn set_warmup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APORT Conflict Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Conflict Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("edge", &self.edge())
            .field("warmup", &self.warmup())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "If {{ edge: {=bool:?}, warmup: {=bool:?}, aportconflict: {=bool:?} }}",
            self.edge(),
            self.warmup(),
            self.aportconflict()
        )
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set EDGE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn edge(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set EDGE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set WARMUP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn warmup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set WARMUP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_warmup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("edge", &self.edge())
            .field("warmup", &self.warmup())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ edge: {=bool:?}, warmup: {=bool:?}, aportconflict: {=bool:?} }}",
            self.edge(),
            self.warmup(),
            self.aportconflict()
        )
    }
}
#[doc = "Input Selection Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputsel(pub u32);
impl Inputsel {
    #[doc = "Positive Input Select."]
    #[must_use]
    #[inline(always)]
    pub const fn possel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Positive Input Select."]
    #[inline(always)]
    pub const fn set_possel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Negative Input Select."]
    #[must_use]
    #[inline(always)]
    pub const fn negsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Negative Input Select."]
    #[inline(always)]
    pub const fn set_negsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "VA Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vasel(&self) -> super::vals::Vasel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Vasel::from_bits(val as u8)
    }
    #[doc = "VA Selection."]
    #[inline(always)]
    pub const fn set_vasel(&mut self, val: super::vals::Vasel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "VB Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbsel(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "VB Selection."]
    #[inline(always)]
    pub const fn set_vbsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Low-Power Sampled Voltage Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vlpsel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Sampled Voltage Selection."]
    #[inline(always)]
    pub const fn set_vlpsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Capacitive Sense Mode Internal Resistor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn csresen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Capacitive Sense Mode Internal Resistor Enable."]
    #[inline(always)]
    pub const fn set_csresen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Capacitive Sense Mode Internal Resistor Select."]
    #[must_use]
    #[inline(always)]
    pub const fn csressel(&self) -> super::vals::Csressel {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Csressel::from_bits(val as u8)
    }
    #[doc = "Capacitive Sense Mode Internal Resistor Select."]
    #[inline(always)]
    pub const fn set_csressel(&mut self, val: super::vals::Csressel) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Inputsel {
    #[inline(always)]
    fn default() -> Inputsel {
        Inputsel(0)
    }
}
impl core::fmt::Debug for Inputsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inputsel")
            .field("possel", &self.possel())
            .field("negsel", &self.negsel())
            .field("vasel", &self.vasel())
            .field("vbsel", &self.vbsel())
            .field("vlpsel", &self.vlpsel())
            .field("csresen", &self.csresen())
            .field("csressel", &self.csressel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inputsel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inputsel {{ possel: {=u8:?}, negsel: {=u8:?}, vasel: {:?}, vbsel: {=bool:?}, vlpsel: {=bool:?}, csresen: {=bool:?}, csressel: {:?} }}" , self . possel () , self . negsel () , self . vasel () , self . vbsel () , self . vlpsel () , self . csresen () , self . csressel ())
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
    pub const fn outloc(&self) -> super::vals::Outloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Outloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_outloc(&mut self, val: super::vals::Outloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
            .field("outloc", &self.outloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routeloc0 {{ outloc: {:?} }}", self.outloc())
    }
}
#[doc = "I/O Routing Pine Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "ACMP Output Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn outpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ACMP Output Pin Enable."]
    #[inline(always)]
    pub const fn set_outpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("outpen", &self.outpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routepen {{ outpen: {=bool:?} }}", self.outpen())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Analog Comparator Active."]
    #[must_use]
    #[inline(always)]
    pub const fn acmpact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Active."]
    #[inline(always)]
    pub const fn set_acmpact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator Output."]
    #[must_use]
    #[inline(always)]
    pub const fn acmpout(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Output."]
    #[inline(always)]
    pub const fn set_acmpout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APORT Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn aportconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Conflict Output."]
    #[inline(always)]
    pub const fn set_aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External Override Interface Active."]
    #[must_use]
    #[inline(always)]
    pub const fn extifact(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External Override Interface Active."]
    #[inline(always)]
    pub const fn set_extifact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("acmpact", &self.acmpact())
            .field("acmpout", &self.acmpout())
            .field("aportconflict", &self.aportconflict())
            .field("extifact", &self.extifact())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ acmpact: {=bool:?}, acmpout: {=bool:?}, aportconflict: {=bool:?}, extifact: {=bool:?} }}" , self . acmpact () , self . acmpout () , self . aportconflict () , self . extifact ())
    }
}
