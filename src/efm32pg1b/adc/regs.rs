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
#[doc = "APORT Bus Master Disable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aportmasterdis(pub u32);
impl Aportmasterdis {
    #[doc = "APORT1X Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xmasterdis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "APORT1X Master Disable."]
    #[inline(always)]
    pub const fn set_aport1xmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "APORT1Y Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1ymasterdis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "APORT1Y Master Disable."]
    #[inline(always)]
    pub const fn set_aport1ymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APORT2X Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2xmasterdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "APORT2X Master Disable."]
    #[inline(always)]
    pub const fn set_aport2xmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "APORT2Y Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2ymasterdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "APORT2Y Master Disable."]
    #[inline(always)]
    pub const fn set_aport2ymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "APORT3X Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3xmasterdis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "APORT3X Master Disable."]
    #[inline(always)]
    pub const fn set_aport3xmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "APORT3Y Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3ymasterdis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "APORT3Y Master Disable."]
    #[inline(always)]
    pub const fn set_aport3ymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "APORT4X Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4xmasterdis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "APORT4X Master Disable."]
    #[inline(always)]
    pub const fn set_aport4xmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "APORT4Y Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4ymasterdis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "APORT4Y Master Disable."]
    #[inline(always)]
    pub const fn set_aport4ymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Aportmasterdis {
    #[inline(always)]
    fn default() -> Aportmasterdis {
        Aportmasterdis(0)
    }
}
impl core::fmt::Debug for Aportmasterdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aportmasterdis")
            .field("aport1xmasterdis", &self.aport1xmasterdis())
            .field("aport1ymasterdis", &self.aport1ymasterdis())
            .field("aport2xmasterdis", &self.aport2xmasterdis())
            .field("aport2ymasterdis", &self.aport2ymasterdis())
            .field("aport3xmasterdis", &self.aport3xmasterdis())
            .field("aport3ymasterdis", &self.aport3ymasterdis())
            .field("aport4xmasterdis", &self.aport4xmasterdis())
            .field("aport4ymasterdis", &self.aport4ymasterdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aportmasterdis {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aportmasterdis {{ aport1xmasterdis: {=bool:?}, aport1ymasterdis: {=bool:?}, aport2xmasterdis: {=bool:?}, aport2ymasterdis: {=bool:?}, aport3xmasterdis: {=bool:?}, aport3ymasterdis: {=bool:?}, aport4xmasterdis: {=bool:?}, aport4ymasterdis: {=bool:?} }}" , self . aport1xmasterdis () , self . aport1ymasterdis () , self . aport2xmasterdis () , self . aport2ymasterdis () , self . aport3xmasterdis () , self . aport3ymasterdis () , self . aport4xmasterdis () , self . aport4ymasterdis ())
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
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
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
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biasprog(pub u32);
impl Biasprog {
    #[doc = "Bias Programming Value of Analog ADC Block."]
    #[must_use]
    #[inline(always)]
    pub const fn adcbiasprog(&self) -> super::vals::Adcbiasprog {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Adcbiasprog::from_bits(val as u8)
    }
    #[doc = "Bias Programming Value of Analog ADC Block."]
    #[inline(always)]
    pub const fn set_adcbiasprog(&mut self, val: super::vals::Adcbiasprog) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Clear VREFOF Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vfaultclr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Clear VREFOF Flag."]
    #[inline(always)]
    pub const fn set_vfaultclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Accuracy Setting for the System Bias During ADC Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn gpbiasacc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Accuracy Setting for the System Bias During ADC Operation."]
    #[inline(always)]
    pub const fn set_gpbiasacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Biasprog {
    #[inline(always)]
    fn default() -> Biasprog {
        Biasprog(0)
    }
}
impl core::fmt::Debug for Biasprog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Biasprog")
            .field("adcbiasprog", &self.adcbiasprog())
            .field("vfaultclr", &self.vfaultclr())
            .field("gpbiasacc", &self.gpbiasacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Biasprog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Biasprog {{ adcbiasprog: {:?}, vfaultclr: {=bool:?}, gpbiasacc: {=bool:?} }}",
            self.adcbiasprog(),
            self.vfaultclr(),
            self.gpbiasacc()
        )
    }
}
#[doc = "Calibration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cal(pub u32);
impl Cal {
    #[doc = "Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn singleoffset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode."]
    #[inline(always)]
    pub const fn set_singleoffset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Single Mode Offset Calibration Value for Negative Single-ended Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn singleoffsetinv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Single Mode Offset Calibration Value for Negative Single-ended Mode."]
    #[inline(always)]
    pub const fn set_singleoffsetinv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Single Mode Gain Calibration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn singlegain(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Single Mode Gain Calibration Value."]
    #[inline(always)]
    pub const fn set_singlegain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Negative Single-ended Offset Calibration is Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetinvmode(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Negative Single-ended Offset Calibration is Enabled."]
    #[inline(always)]
    pub const fn set_offsetinvmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn scanoffset(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode."]
    #[inline(always)]
    pub const fn set_scanoffset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Scan Mode Offset Calibration Value for Negative Single-ended Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn scanoffsetinv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Scan Mode Offset Calibration Value for Negative Single-ended Mode."]
    #[inline(always)]
    pub const fn set_scanoffsetinv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Scan Mode Gain Calibration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn scangain(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Scan Mode Gain Calibration Value."]
    #[inline(always)]
    pub const fn set_scangain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "Calibration Mode is Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn calen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Mode is Enabled."]
    #[inline(always)]
    pub const fn set_calen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cal {
    #[inline(always)]
    fn default() -> Cal {
        Cal(0)
    }
}
impl core::fmt::Debug for Cal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cal")
            .field("singleoffset", &self.singleoffset())
            .field("singleoffsetinv", &self.singleoffsetinv())
            .field("singlegain", &self.singlegain())
            .field("offsetinvmode", &self.offsetinvmode())
            .field("scanoffset", &self.scanoffset())
            .field("scanoffsetinv", &self.scanoffsetinv())
            .field("scangain", &self.scangain())
            .field("calen", &self.calen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cal {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cal {{ singleoffset: {=u8:?}, singleoffsetinv: {=u8:?}, singlegain: {=u8:?}, offsetinvmode: {=bool:?}, scanoffset: {=u8:?}, scanoffsetinv: {=u8:?}, scangain: {=u8:?}, calen: {=bool:?} }}" , self . singleoffset () , self . singleoffsetinv () , self . singlegain () , self . offsetinvmode () , self . scanoffset () , self . scanoffsetinv () , self . scangain () , self . calen ())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Single Channel Conversion Start."]
    #[must_use]
    #[inline(always)]
    pub const fn singlestart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Conversion Start."]
    #[inline(always)]
    pub const fn set_singlestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Channel Conversion Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn singlestop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Conversion Stop."]
    #[inline(always)]
    pub const fn set_singlestop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scan Sequence Start."]
    #[must_use]
    #[inline(always)]
    pub const fn scanstart(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence Start."]
    #[inline(always)]
    pub const fn set_scanstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Scan Sequence Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn scanstop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence Stop."]
    #[inline(always)]
    pub const fn set_scanstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("singlestart", &self.singlestart())
            .field("singlestop", &self.singlestop())
            .field("scanstart", &self.scanstart())
            .field("scanstop", &self.scanstop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ singlestart: {=bool:?}, singlestop: {=bool:?}, scanstart: {=bool:?}, scanstop: {=bool:?} }}" , self . singlestart () , self . singlestop () , self . scanstart () , self . scanstop ())
    }
}
#[doc = "Compare Threshold Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpthr(pub u32);
impl Cmpthr {
    #[doc = "Less Than Compare Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn adlt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Less Than Compare Threshold."]
    #[inline(always)]
    pub const fn set_adlt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Greater Than Compare Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn adgt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Greater Than Compare Threshold."]
    #[inline(always)]
    pub const fn set_adgt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("adlt", &self.adlt())
            .field("adgt", &self.adgt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpthr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpthr {{ adlt: {=u16:?}, adgt: {=u16:?} }}",
            self.adlt(),
            self.adgt()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Warm-up Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn warmupmode(&self) -> super::vals::Warmupmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Warmupmode::from_bits(val as u8)
    }
    #[doc = "Warm-up Mode."]
    #[inline(always)]
    pub const fn set_warmupmode(&mut self, val: super::vals::Warmupmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SINGLEFIFO DMA Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn singledmawu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SINGLEFIFO DMA Wakeup."]
    #[inline(always)]
    pub const fn set_singledmawu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SCANFIFO DMA Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn scandmawu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SCANFIFO DMA Wakeup."]
    #[inline(always)]
    pub const fn set_scandmawu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Conversion Tailgating."]
    #[must_use]
    #[inline(always)]
    pub const fn tailgate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Tailgating."]
    #[inline(always)]
    pub const fn set_tailgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Selects ASYNC CLK Enable Mode When ADCCLKMODE=1."]
    #[must_use]
    #[inline(always)]
    pub const fn asyncclken(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Selects ASYNC CLK Enable Mode When ADCCLKMODE=1."]
    #[inline(always)]
    pub const fn set_asyncclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ADC Clock Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn adcclkmode(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Clock Mode."]
    #[inline(always)]
    pub const fn set_adcclkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Prescalar Setting for ADC Sample and Conversion Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::Presc {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Presc::from_bits(val as u8)
    }
    #[doc = "Prescalar Setting for ADC Sample and Conversion Clock."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::Presc) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "1us Time Base."]
    #[must_use]
    #[inline(always)]
    pub const fn timebase(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "1us Time Base."]
    #[inline(always)]
    pub const fn set_timebase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Oversample Rate Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ovsrsel(&self) -> super::vals::Ovsrsel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ovsrsel::from_bits(val as u8)
    }
    #[doc = "Oversample Rate Select."]
    #[inline(always)]
    pub const fn set_ovsrsel(&mut self, val: super::vals::Ovsrsel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Channel Connect."]
    #[must_use]
    #[inline(always)]
    pub const fn chconmode(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Connect."]
    #[inline(always)]
    pub const fn set_chconmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("warmupmode", &self.warmupmode())
            .field("singledmawu", &self.singledmawu())
            .field("scandmawu", &self.scandmawu())
            .field("tailgate", &self.tailgate())
            .field("asyncclken", &self.asyncclken())
            .field("adcclkmode", &self.adcclkmode())
            .field("presc", &self.presc())
            .field("timebase", &self.timebase())
            .field("ovsrsel", &self.ovsrsel())
            .field("chconmode", &self.chconmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ warmupmode: {:?}, singledmawu: {=bool:?}, scandmawu: {=bool:?}, tailgate: {=bool:?}, asyncclken: {=bool:?}, adcclkmode: {=bool:?}, presc: {:?}, timebase: {=u8:?}, ovsrsel: {:?}, chconmode: {=bool:?} }}" , self . warmupmode () , self . singledmawu () , self . scandmawu () , self . tailgate () , self . asyncclken () , self . adcclkmode () , self . presc () , self . timebase () , self . ovsrsel () , self . chconmode ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "SINGLE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn single(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SINGLE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_single(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SCAN Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SCAN Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SINGLEOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn singleof(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SINGLEOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_singleof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCANOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scanof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCANOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scanof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SINGLEUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn singleuf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SINGLEUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_singleuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SCANUF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scanuf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SCANUF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scanuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SINGLECMP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn singlecmp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SINGLECMP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_singlecmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SCANCMP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scancmp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SCANCMP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scancmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "VREFOV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefov(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VREFOV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vrefov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PROGERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn progerr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PROGERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_progerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("single", &self.single())
            .field("scan", &self.scan())
            .field("singleof", &self.singleof())
            .field("scanof", &self.scanof())
            .field("singleuf", &self.singleuf())
            .field("scanuf", &self.scanuf())
            .field("singlecmp", &self.singlecmp())
            .field("scancmp", &self.scancmp())
            .field("vrefov", &self.vrefov())
            .field("progerr", &self.progerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ single: {=bool:?}, scan: {=bool:?}, singleof: {=bool:?}, scanof: {=bool:?}, singleuf: {=bool:?}, scanuf: {=bool:?}, singlecmp: {=bool:?}, scancmp: {=bool:?}, vrefov: {=bool:?}, progerr: {=bool:?} }}" , self . single () , self . scan () , self . singleof () , self . scanof () , self . singleuf () , self . scanuf () , self . singlecmp () , self . scancmp () , self . vrefov () , self . progerr ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Single Conversion Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn single(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Single Conversion Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_single(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Scan Conversion Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Conversion Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Single FIFO Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singleof(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Single FIFO Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singleof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Scan FIFO Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scanof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Scan FIFO Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scanof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Single FIFO Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singleuf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Single FIFO Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singleuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Scan FIFO Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scanuf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Scan FIFO Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scanuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Single Result Compare Match Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singlecmp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Single Result Compare Match Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singlecmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Scan Result Compare Match Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scancmp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Result Compare Match Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scancmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "VREF Over Voltage Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefov(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VREF Over Voltage Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vrefov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Programming Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn progerr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Programming Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_progerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("single", &self.single())
            .field("scan", &self.scan())
            .field("singleof", &self.singleof())
            .field("scanof", &self.scanof())
            .field("singleuf", &self.singleuf())
            .field("scanuf", &self.scanuf())
            .field("singlecmp", &self.singlecmp())
            .field("scancmp", &self.scancmp())
            .field("vrefov", &self.vrefov())
            .field("progerr", &self.progerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ single: {=bool:?}, scan: {=bool:?}, singleof: {=bool:?}, scanof: {=bool:?}, singleuf: {=bool:?}, scanuf: {=bool:?}, singlecmp: {=bool:?}, scancmp: {=bool:?}, vrefov: {=bool:?}, progerr: {=bool:?} }}" , self . single () , self . scan () , self . singleof () , self . scanof () , self . singleuf () , self . scanuf () , self . singlecmp () , self . scancmp () , self . vrefov () , self . progerr ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set SINGLEOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singleof(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set SINGLEOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singleof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set SCANOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scanof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set SCANOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scanof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set SINGLEUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singleuf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set SINGLEUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singleuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set SCANUF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scanuf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set SCANUF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scanuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set SINGLECMP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn singlecmp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set SINGLECMP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_singlecmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set SCANCMP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scancmp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set SCANCMP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scancmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set VREFOV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefov(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set VREFOV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vrefov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set PROGERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn progerr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Set PROGERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_progerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("singleof", &self.singleof())
            .field("scanof", &self.scanof())
            .field("singleuf", &self.singleuf())
            .field("scanuf", &self.scanuf())
            .field("singlecmp", &self.singlecmp())
            .field("scancmp", &self.scancmp())
            .field("vrefov", &self.vrefov())
            .field("progerr", &self.progerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ singleof: {=bool:?}, scanof: {=bool:?}, singleuf: {=bool:?}, scanuf: {=bool:?}, singlecmp: {=bool:?}, scancmp: {=bool:?}, vrefov: {=bool:?}, progerr: {=bool:?} }}" , self . singleof () , self . scanof () , self . singleuf () , self . scanuf () , self . singlecmp () , self . scancmp () , self . vrefov () , self . progerr ())
    }
}
#[doc = "Scan Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scanctrl(pub u32);
impl Scanctrl {
    #[doc = "Scan Sequence Repetitive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence Repetitive Mode."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Scan Sequence Differential Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn diff(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence Differential Mode."]
    #[inline(always)]
    pub const fn set_diff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scan Sequence Result Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn adj(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence Result Adjustment."]
    #[inline(always)]
    pub const fn set_adj(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Scan Sequence Resolution Select."]
    #[must_use]
    #[inline(always)]
    pub const fn res(&self) -> super::vals::ScanctrlRes {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::ScanctrlRes::from_bits(val as u8)
    }
    #[doc = "Scan Sequence Resolution Select."]
    #[inline(always)]
    pub const fn set_res(&mut self, val: super::vals::ScanctrlRes) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Scan Sequence Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> super::vals::ScanctrlRef {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ScanctrlRef::from_bits(val as u8)
    }
    #[doc = "Scan Sequence Reference Selection."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: super::vals::ScanctrlRef) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Scan Acquisition Time."]
    #[must_use]
    #[inline(always)]
    pub const fn at(&self) -> super::vals::ScanctrlAt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::ScanctrlAt::from_bits(val as u8)
    }
    #[doc = "Scan Acquisition Time."]
    #[inline(always)]
    pub const fn set_at(&mut self, val: super::vals::ScanctrlAt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Scan Sequence PRS Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Sequence PRS Trigger Enable."]
    #[inline(always)]
    pub const fn set_prsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Compare Logic Enable for Scan."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Logic Enable for Scan."]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Scanctrl {
    #[inline(always)]
    fn default() -> Scanctrl {
        Scanctrl(0)
    }
}
impl core::fmt::Debug for Scanctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanctrl")
            .field("rep", &self.rep())
            .field("diff", &self.diff())
            .field("adj", &self.adj())
            .field("res", &self.res())
            .field("ref_", &self.ref_())
            .field("at", &self.at())
            .field("prsen", &self.prsen())
            .field("cmpen", &self.cmpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scanctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scanctrl {{ rep: {=bool:?}, diff: {=bool:?}, adj: {=bool:?}, res: {:?}, ref_: {:?}, at: {:?}, prsen: {=bool:?}, cmpen: {=bool:?} }}" , self . rep () , self . diff () , self . adj () , self . res () , self . ref_ () , self . at () , self . prsen () , self . cmpen ())
    }
}
#[doc = "Scan Control Register Continued."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scanctrlx(pub u32);
impl Scanctrlx {
    #[doc = "Scan Channel Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefsel(&self) -> super::vals::ScanctrlxVrefsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ScanctrlxVrefsel::from_bits(val as u8)
    }
    #[doc = "Scan Channel Reference Selection."]
    #[inline(always)]
    pub const fn set_vrefsel(&mut self, val: super::vals::ScanctrlxVrefsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Enable Fixed Scaling on VREF."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefattfix(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Fixed Scaling on VREF."]
    #[inline(always)]
    pub const fn set_vrefattfix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefatt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5."]
    #[inline(always)]
    pub const fn set_vrefatt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Code for VIN Attenuation Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn vinatt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Code for VIN Attenuation Factor."]
    #[inline(always)]
    pub const fn set_vinatt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Scan DV Level Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dvl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Scan DV Level Select."]
    #[inline(always)]
    pub const fn set_dvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Scan FIFO Overflow Action."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoofact(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Scan FIFO Overflow Action."]
    #[inline(always)]
    pub const fn set_fifoofact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Scan PRS Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsmode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Scan PRS Trigger Mode."]
    #[inline(always)]
    pub const fn set_prsmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Scan Sequence PRS Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 17usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Scan Sequence PRS Trigger Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Delay Next Conversion Start If CONVSTARTDELAYEN is Set."]
    #[must_use]
    #[inline(always)]
    pub const fn convstartdelay(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Delay Next Conversion Start If CONVSTARTDELAYEN is Set."]
    #[inline(always)]
    pub const fn set_convstartdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Enable Delaying Next Conversion Start."]
    #[must_use]
    #[inline(always)]
    pub const fn convstartdelayen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Delaying Next Conversion Start."]
    #[inline(always)]
    pub const fn set_convstartdelayen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Scanctrlx {
    #[inline(always)]
    fn default() -> Scanctrlx {
        Scanctrlx(0)
    }
}
impl core::fmt::Debug for Scanctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanctrlx")
            .field("vrefsel", &self.vrefsel())
            .field("vrefattfix", &self.vrefattfix())
            .field("vrefatt", &self.vrefatt())
            .field("vinatt", &self.vinatt())
            .field("dvl", &self.dvl())
            .field("fifoofact", &self.fifoofact())
            .field("prsmode", &self.prsmode())
            .field("prssel", &self.prssel())
            .field("convstartdelay", &self.convstartdelay())
            .field("convstartdelayen", &self.convstartdelayen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scanctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scanctrlx {{ vrefsel: {:?}, vrefattfix: {=bool:?}, vrefatt: {=u8:?}, vinatt: {=u8:?}, dvl: {=u8:?}, fifoofact: {=bool:?}, prsmode: {=bool:?}, prssel: {:?}, convstartdelay: {=u8:?}, convstartdelayen: {=bool:?} }}" , self . vrefsel () , self . vrefattfix () , self . vrefatt () , self . vinatt () , self . dvl () , self . fifoofact () , self . prsmode () , self . prssel () , self . convstartdelay () , self . convstartdelayen ())
    }
}
#[doc = "Scan Sequence Result Data + Data Source Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scandatax(pub u32);
impl Scandatax {
    #[doc = "Scan Conversion Result Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Conversion Result Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Scan Conversion Input ID."]
    #[must_use]
    #[inline(always)]
    pub const fn scaninputid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Scan Conversion Input ID."]
    #[inline(always)]
    pub const fn set_scaninputid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Scandatax {
    #[inline(always)]
    fn default() -> Scandatax {
        Scandatax(0)
    }
}
impl core::fmt::Debug for Scandatax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scandatax")
            .field("data", &self.data())
            .field("scaninputid", &self.scaninputid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scandatax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scandatax {{ data: {=u16:?}, scaninputid: {=u8:?} }}",
            self.data(),
            self.scaninputid()
        )
    }
}
#[doc = "Scan Sequence Result Data + Data Source Peek Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scandataxp(pub u32);
impl Scandataxp {
    #[doc = "Scan Conversion Result Data Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn datap(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Conversion Result Data Peek."]
    #[inline(always)]
    pub const fn set_datap(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Scan Conversion Data Source Peek."]
    #[must_use]
    #[inline(always)]
    pub const fn scaninputidpeek(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Scan Conversion Data Source Peek."]
    #[inline(always)]
    pub const fn set_scaninputidpeek(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Scandataxp {
    #[inline(always)]
    fn default() -> Scandataxp {
        Scandataxp(0)
    }
}
impl core::fmt::Debug for Scandataxp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scandataxp")
            .field("datap", &self.datap())
            .field("scaninputidpeek", &self.scaninputidpeek())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scandataxp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scandataxp {{ datap: {=u16:?}, scaninputidpeek: {=u8:?} }}",
            self.datap(),
            self.scaninputidpeek()
        )
    }
}
#[doc = "Scan FIFO Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scanfifoclear(pub u32);
impl Scanfifoclear {
    #[doc = "Clear Scan FIFO Content."]
    #[must_use]
    #[inline(always)]
    pub const fn scanfifoclear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Scan FIFO Content."]
    #[inline(always)]
    pub const fn set_scanfifoclear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Scanfifoclear {
    #[inline(always)]
    fn default() -> Scanfifoclear {
        Scanfifoclear(0)
    }
}
impl core::fmt::Debug for Scanfifoclear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanfifoclear")
            .field("scanfifoclear", &self.scanfifoclear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scanfifoclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scanfifoclear {{ scanfifoclear: {=bool:?} }}",
            self.scanfifoclear()
        )
    }
}
#[doc = "Scan FIFO Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scanfifocount(pub u32);
impl Scanfifocount {
    #[doc = "Scan Data Count."]
    #[must_use]
    #[inline(always)]
    pub const fn scandc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Scan Data Count."]
    #[inline(always)]
    pub const fn set_scandc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Scanfifocount {
    #[inline(always)]
    fn default() -> Scanfifocount {
        Scanfifocount(0)
    }
}
impl core::fmt::Debug for Scanfifocount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanfifocount")
            .field("scandc", &self.scandc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scanfifocount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scanfifocount {{ scandc: {=u8:?} }}", self.scandc())
    }
}
#[doc = "Input Selection Register for Scan Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scaninputsel(pub u32);
impl Scaninputsel {
    #[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn input0to7sel(&self) -> super::vals::Input0to7sel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Input0to7sel::from_bits(val as u8)
    }
    #[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK."]
    #[inline(always)]
    pub const fn set_input0to7sel(&mut self, val: super::vals::Input0to7sel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn input8to15sel(&self) -> super::vals::Input8to15sel {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Input8to15sel::from_bits(val as u8)
    }
    #[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK."]
    #[inline(always)]
    pub const fn set_input8to15sel(&mut self, val: super::vals::Input8to15sel) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn input16to23sel(&self) -> super::vals::Input16to23sel {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Input16to23sel::from_bits(val as u8)
    }
    #[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK."]
    #[inline(always)]
    pub const fn set_input16to23sel(&mut self, val: super::vals::Input16to23sel) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn input24to31sel(&self) -> super::vals::Input24to31sel {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Input24to31sel::from_bits(val as u8)
    }
    #[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK."]
    #[inline(always)]
    pub const fn set_input24to31sel(&mut self, val: super::vals::Input24to31sel) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Scaninputsel {
    #[inline(always)]
    fn default() -> Scaninputsel {
        Scaninputsel(0)
    }
}
impl core::fmt::Debug for Scaninputsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scaninputsel")
            .field("input0to7sel", &self.input0to7sel())
            .field("input8to15sel", &self.input8to15sel())
            .field("input16to23sel", &self.input16to23sel())
            .field("input24to31sel", &self.input24to31sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scaninputsel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scaninputsel {{ input0to7sel: {:?}, input8to15sel: {:?}, input16to23sel: {:?}, input24to31sel: {:?} }}" , self . input0to7sel () , self . input8to15sel () , self . input16to23sel () , self . input24to31sel ())
    }
}
#[doc = "Negative Input Select Register for Scan."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scannegsel(pub u32);
impl Scannegsel {
    #[doc = "Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input0negsel(&self) -> super::vals::Input0negsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Input0negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input0negsel(&mut self, val: super::vals::Input0negsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input2negsel(&self) -> super::vals::Input2negsel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Input2negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input2negsel(&mut self, val: super::vals::Input2negsel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input4negsel(&self) -> super::vals::Input4negsel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Input4negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input4negsel(&mut self, val: super::vals::Input4negsel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input6negsel(&self) -> super::vals::Input6negsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Input6negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input6negsel(&mut self, val: super::vals::Input6negsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input9negsel(&self) -> super::vals::Input9negsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Input9negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input9negsel(&mut self, val: super::vals::Input9negsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input11negsel(&self) -> super::vals::Input11negsel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Input11negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input11negsel(&mut self, val: super::vals::Input11negsel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input13negsel(&self) -> super::vals::Input13negsel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Input13negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input13negsel(&mut self, val: super::vals::Input13negsel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn input15negsel(&self) -> super::vals::Input15negsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Input15negsel::from_bits(val as u8)
    }
    #[doc = "Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode."]
    #[inline(always)]
    pub const fn set_input15negsel(&mut self, val: super::vals::Input15negsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Scannegsel {
    #[inline(always)]
    fn default() -> Scannegsel {
        Scannegsel(0)
    }
}
impl core::fmt::Debug for Scannegsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scannegsel")
            .field("input0negsel", &self.input0negsel())
            .field("input2negsel", &self.input2negsel())
            .field("input4negsel", &self.input4negsel())
            .field("input6negsel", &self.input6negsel())
            .field("input9negsel", &self.input9negsel())
            .field("input11negsel", &self.input11negsel())
            .field("input13negsel", &self.input13negsel())
            .field("input15negsel", &self.input15negsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scannegsel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Scannegsel {{ input0negsel: {:?}, input2negsel: {:?}, input4negsel: {:?}, input6negsel: {:?}, input9negsel: {:?}, input11negsel: {:?}, input13negsel: {:?}, input15negsel: {:?} }}" , self . input0negsel () , self . input2negsel () , self . input4negsel () , self . input6negsel () , self . input9negsel () , self . input11negsel () , self . input13negsel () , self . input15negsel ())
    }
}
#[doc = "Single Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlectrl(pub u32);
impl Singlectrl {
    #[doc = "Single Channel Repetitive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Repetitive Mode."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Channel Differential Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn diff(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Differential Mode."]
    #[inline(always)]
    pub const fn set_diff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Channel Result Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn adj(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Result Adjustment."]
    #[inline(always)]
    pub const fn set_adj(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Single Channel Resolution Select."]
    #[must_use]
    #[inline(always)]
    pub const fn res(&self) -> super::vals::SinglectrlRes {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::SinglectrlRes::from_bits(val as u8)
    }
    #[doc = "Single Channel Resolution Select."]
    #[inline(always)]
    pub const fn set_res(&mut self, val: super::vals::SinglectrlRes) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Single Channel Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> super::vals::SinglectrlRef {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::SinglectrlRef::from_bits(val as u8)
    }
    #[doc = "Single Channel Reference Selection."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: super::vals::SinglectrlRef) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Single Channel Positive Input Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn possel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Single Channel Positive Input Selection."]
    #[inline(always)]
    pub const fn set_possel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Single Channel Negative Input Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn negsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Single Channel Negative Input Selection."]
    #[inline(always)]
    pub const fn set_negsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Single Channel Acquisition Time."]
    #[must_use]
    #[inline(always)]
    pub const fn at(&self) -> super::vals::SinglectrlAt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::SinglectrlAt::from_bits(val as u8)
    }
    #[doc = "Single Channel Acquisition Time."]
    #[inline(always)]
    pub const fn set_at(&mut self, val: super::vals::SinglectrlAt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Single Channel PRS Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel PRS Trigger Enable."]
    #[inline(always)]
    pub const fn set_prsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Compare Logic Enable for Single Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Logic Enable for Single Channel."]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("rep", &self.rep())
            .field("diff", &self.diff())
            .field("adj", &self.adj())
            .field("res", &self.res())
            .field("ref_", &self.ref_())
            .field("possel", &self.possel())
            .field("negsel", &self.negsel())
            .field("at", &self.at())
            .field("prsen", &self.prsen())
            .field("cmpen", &self.cmpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Singlectrl {{ rep: {=bool:?}, diff: {=bool:?}, adj: {=bool:?}, res: {:?}, ref_: {:?}, possel: {=u8:?}, negsel: {=u8:?}, at: {:?}, prsen: {=bool:?}, cmpen: {=bool:?} }}" , self . rep () , self . diff () , self . adj () , self . res () , self . ref_ () , self . possel () , self . negsel () , self . at () , self . prsen () , self . cmpen ())
    }
}
#[doc = "Single Channel Control Register Continued."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlectrlx(pub u32);
impl Singlectrlx {
    #[doc = "Single Channel Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefsel(&self) -> super::vals::SinglectrlxVrefsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SinglectrlxVrefsel::from_bits(val as u8)
    }
    #[doc = "Single Channel Reference Selection."]
    #[inline(always)]
    pub const fn set_vrefsel(&mut self, val: super::vals::SinglectrlxVrefsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Enable Fixed Scaling on VREF."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefattfix(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Fixed Scaling on VREF."]
    #[inline(always)]
    pub const fn set_vrefattfix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefatt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5."]
    #[inline(always)]
    pub const fn set_vrefatt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Code for VIN Attenuation Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn vinatt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Code for VIN Attenuation Factor."]
    #[inline(always)]
    pub const fn set_vinatt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Single Channel DV Level Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dvl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Single Channel DV Level Select."]
    #[inline(always)]
    pub const fn set_dvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Single Channel FIFO Overflow Action."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoofact(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel FIFO Overflow Action."]
    #[inline(always)]
    pub const fn set_fifoofact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Single Channel PRS Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsmode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel PRS Trigger Mode."]
    #[inline(always)]
    pub const fn set_prsmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Single Channel PRS Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 17usize) & 0x0f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Single Channel PRS Trigger Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set."]
    #[must_use]
    #[inline(always)]
    pub const fn convstartdelay(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set."]
    #[inline(always)]
    pub const fn set_convstartdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Enable Delaying Next Conversion Start."]
    #[must_use]
    #[inline(always)]
    pub const fn convstartdelayen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Delaying Next Conversion Start."]
    #[inline(always)]
    pub const fn set_convstartdelayen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Singlectrlx {
    #[inline(always)]
    fn default() -> Singlectrlx {
        Singlectrlx(0)
    }
}
impl core::fmt::Debug for Singlectrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Singlectrlx")
            .field("vrefsel", &self.vrefsel())
            .field("vrefattfix", &self.vrefattfix())
            .field("vrefatt", &self.vrefatt())
            .field("vinatt", &self.vinatt())
            .field("dvl", &self.dvl())
            .field("fifoofact", &self.fifoofact())
            .field("prsmode", &self.prsmode())
            .field("prssel", &self.prssel())
            .field("convstartdelay", &self.convstartdelay())
            .field("convstartdelayen", &self.convstartdelayen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlectrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Singlectrlx {{ vrefsel: {:?}, vrefattfix: {=bool:?}, vrefatt: {=u8:?}, vinatt: {=u8:?}, dvl: {=u8:?}, fifoofact: {=bool:?}, prsmode: {=bool:?}, prssel: {:?}, convstartdelay: {=u8:?}, convstartdelayen: {=bool:?} }}" , self . vrefsel () , self . vrefattfix () , self . vrefatt () , self . vinatt () , self . dvl () , self . fifoofact () , self . prsmode () , self . prssel () , self . convstartdelay () , self . convstartdelayen ())
    }
}
#[doc = "Single FIFO Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlefifoclear(pub u32);
impl Singlefifoclear {
    #[doc = "Clear Single FIFO Content."]
    #[must_use]
    #[inline(always)]
    pub const fn singlefifoclear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Single FIFO Content."]
    #[inline(always)]
    pub const fn set_singlefifoclear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Singlefifoclear {
    #[inline(always)]
    fn default() -> Singlefifoclear {
        Singlefifoclear(0)
    }
}
impl core::fmt::Debug for Singlefifoclear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Singlefifoclear")
            .field("singlefifoclear", &self.singlefifoclear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlefifoclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Singlefifoclear {{ singlefifoclear: {=bool:?} }}",
            self.singlefifoclear()
        )
    }
}
#[doc = "Single FIFO Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Singlefifocount(pub u32);
impl Singlefifocount {
    #[doc = "Single Data Count."]
    #[must_use]
    #[inline(always)]
    pub const fn singledc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Single Data Count."]
    #[inline(always)]
    pub const fn set_singledc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Singlefifocount {
    #[inline(always)]
    fn default() -> Singlefifocount {
        Singlefifocount(0)
    }
}
impl core::fmt::Debug for Singlefifocount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Singlefifocount")
            .field("singledc", &self.singledc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Singlefifocount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Singlefifocount {{ singledc: {=u8:?} }}",
            self.singledc()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Single Channel Conversion Active."]
    #[must_use]
    #[inline(always)]
    pub const fn singleact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Conversion Active."]
    #[inline(always)]
    pub const fn set_singleact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Scan Conversion Active."]
    #[must_use]
    #[inline(always)]
    pub const fn scanact(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Conversion Active."]
    #[inline(always)]
    pub const fn set_scanact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Channel Reference Warmed Up."]
    #[must_use]
    #[inline(always)]
    pub const fn singlerefwarm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Reference Warmed Up."]
    #[inline(always)]
    pub const fn set_singlerefwarm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Scan Reference Warmed Up."]
    #[must_use]
    #[inline(always)]
    pub const fn scanrefwarm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Reference Warmed Up."]
    #[inline(always)]
    pub const fn set_scanrefwarm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Programming Error Status."]
    #[must_use]
    #[inline(always)]
    pub const fn progerr(&self) -> super::vals::Progerr {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Progerr::from_bits(val as u8)
    }
    #[doc = "Programming Error Status."]
    #[inline(always)]
    pub const fn set_progerr(&mut self, val: super::vals::Progerr) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ADC Warmed Up."]
    #[must_use]
    #[inline(always)]
    pub const fn warm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Warmed Up."]
    #[inline(always)]
    pub const fn set_warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Single Channel Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn singledv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Single Channel Data Valid."]
    #[inline(always)]
    pub const fn set_singledv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Scan Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn scandv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Scan Data Valid."]
    #[inline(always)]
    pub const fn set_scandv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("singleact", &self.singleact())
            .field("scanact", &self.scanact())
            .field("singlerefwarm", &self.singlerefwarm())
            .field("scanrefwarm", &self.scanrefwarm())
            .field("progerr", &self.progerr())
            .field("warm", &self.warm())
            .field("singledv", &self.singledv())
            .field("scandv", &self.scandv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ singleact: {=bool:?}, scanact: {=bool:?}, singlerefwarm: {=bool:?}, scanrefwarm: {=bool:?}, progerr: {:?}, warm: {=bool:?}, singledv: {=bool:?}, scandv: {=bool:?} }}" , self . singleact () , self . scanact () , self . singlerefwarm () , self . scanrefwarm () , self . progerr () , self . warm () , self . singledv () , self . scandv ())
    }
}
