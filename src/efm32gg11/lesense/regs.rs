#[doc = "Alternative Excite Pin Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altexconf(pub u32);
impl Altexconf {
    #[doc = "ALTEX0 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf0(&self) -> super::vals::Idleconf0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Idleconf0::from_bits(val as u8)
    }
    #[doc = "ALTEX0 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf0(&mut self, val: super::vals::Idleconf0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ALTEX1 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf1(&self) -> super::vals::Idleconf1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Idleconf1::from_bits(val as u8)
    }
    #[doc = "ALTEX1 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf1(&mut self, val: super::vals::Idleconf1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "ALTEX2 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf2(&self) -> super::vals::Idleconf2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Idleconf2::from_bits(val as u8)
    }
    #[doc = "ALTEX2 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf2(&mut self, val: super::vals::Idleconf2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "ALTEX3 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf3(&self) -> super::vals::Idleconf3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Idleconf3::from_bits(val as u8)
    }
    #[doc = "ALTEX3 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf3(&mut self, val: super::vals::Idleconf3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "ALTEX4 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf4(&self) -> super::vals::Idleconf4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Idleconf4::from_bits(val as u8)
    }
    #[doc = "ALTEX4 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf4(&mut self, val: super::vals::Idleconf4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ALTEX5 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf5(&self) -> super::vals::Idleconf5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Idleconf5::from_bits(val as u8)
    }
    #[doc = "ALTEX5 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf5(&mut self, val: super::vals::Idleconf5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ALTEX6 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf6(&self) -> super::vals::Idleconf6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Idleconf6::from_bits(val as u8)
    }
    #[doc = "ALTEX6 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf6(&mut self, val: super::vals::Idleconf6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ALTEX7 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idleconf7(&self) -> super::vals::Idleconf7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Idleconf7::from_bits(val as u8)
    }
    #[doc = "ALTEX7 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_idleconf7(&mut self, val: super::vals::Idleconf7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "ALTEX0 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX0 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ALTEX1 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX1 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ALTEX2 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX2 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ALTEX3 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX3 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ALTEX4 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX4 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ALTEX5 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX5 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ALTEX6 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX6 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ALTEX7 Always Excite Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aex7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX7 Always Excite Enable."]
    #[inline(always)]
    pub const fn set_aex7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Altexconf {
    #[inline(always)]
    fn default() -> Altexconf {
        Altexconf(0)
    }
}
impl core::fmt::Debug for Altexconf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altexconf")
            .field("idleconf0", &self.idleconf0())
            .field("idleconf1", &self.idleconf1())
            .field("idleconf2", &self.idleconf2())
            .field("idleconf3", &self.idleconf3())
            .field("idleconf4", &self.idleconf4())
            .field("idleconf5", &self.idleconf5())
            .field("idleconf6", &self.idleconf6())
            .field("idleconf7", &self.idleconf7())
            .field("aex0", &self.aex0())
            .field("aex1", &self.aex1())
            .field("aex2", &self.aex2())
            .field("aex3", &self.aex3())
            .field("aex4", &self.aex4())
            .field("aex5", &self.aex5())
            .field("aex6", &self.aex6())
            .field("aex7", &self.aex7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altexconf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altexconf {{ idleconf0: {:?}, idleconf1: {:?}, idleconf2: {:?}, idleconf3: {:?}, idleconf4: {:?}, idleconf5: {:?}, idleconf6: {:?}, idleconf7: {:?}, aex0: {=bool:?}, aex1: {=bool:?}, aex2: {=bool:?}, aex3: {=bool:?}, aex4: {=bool:?}, aex5: {=bool:?}, aex6: {=bool:?}, aex7: {=bool:?} }}" , self . idleconf0 () , self . idleconf1 () , self . idleconf2 () , self . idleconf3 () , self . idleconf4 () , self . idleconf5 () , self . idleconf6 () , self . idleconf7 () , self . aex0 () , self . aex1 () , self . aex2 () , self . aex3 () , self . aex4 () , self . aex5 () , self . aex6 () , self . aex7 ())
    }
}
#[doc = "Bias Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biasctrl(pub u32);
impl Biasctrl {
    #[doc = "Select Bias Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn biasmode(&self) -> super::vals::Biasmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Biasmode::from_bits(val as u8)
    }
    #[doc = "Select Bias Mode."]
    #[inline(always)]
    pub const fn set_biasmode(&mut self, val: super::vals::Biasmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Biasctrl {
    #[inline(always)]
    fn default() -> Biasctrl {
        Biasctrl(0)
    }
}
impl core::fmt::Debug for Biasctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Biasctrl")
            .field("biasmode", &self.biasmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Biasctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Biasctrl {{ biasmode: {:?} }}", self.biasmode())
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf0Data(pub u32);
impl Buf0Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf0Data {
    #[inline(always)]
    fn default() -> Buf0Data {
        Buf0Data(0)
    }
}
impl core::fmt::Debug for Buf0Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf0Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf0Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf0Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf10Data(pub u32);
impl Buf10Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf10Data {
    #[inline(always)]
    fn default() -> Buf10Data {
        Buf10Data(0)
    }
}
impl core::fmt::Debug for Buf10Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf10Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf10Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf10Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf11Data(pub u32);
impl Buf11Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf11Data {
    #[inline(always)]
    fn default() -> Buf11Data {
        Buf11Data(0)
    }
}
impl core::fmt::Debug for Buf11Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf11Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf11Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf11Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf12Data(pub u32);
impl Buf12Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf12Data {
    #[inline(always)]
    fn default() -> Buf12Data {
        Buf12Data(0)
    }
}
impl core::fmt::Debug for Buf12Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf12Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf12Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf12Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf13Data(pub u32);
impl Buf13Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf13Data {
    #[inline(always)]
    fn default() -> Buf13Data {
        Buf13Data(0)
    }
}
impl core::fmt::Debug for Buf13Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf13Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf13Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf13Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf14Data(pub u32);
impl Buf14Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf14Data {
    #[inline(always)]
    fn default() -> Buf14Data {
        Buf14Data(0)
    }
}
impl core::fmt::Debug for Buf14Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf14Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf14Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf14Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf15Data(pub u32);
impl Buf15Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf15Data {
    #[inline(always)]
    fn default() -> Buf15Data {
        Buf15Data(0)
    }
}
impl core::fmt::Debug for Buf15Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf15Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf15Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf15Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf1Data(pub u32);
impl Buf1Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf1Data {
    #[inline(always)]
    fn default() -> Buf1Data {
        Buf1Data(0)
    }
}
impl core::fmt::Debug for Buf1Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf1Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf1Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf1Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf2Data(pub u32);
impl Buf2Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf2Data {
    #[inline(always)]
    fn default() -> Buf2Data {
        Buf2Data(0)
    }
}
impl core::fmt::Debug for Buf2Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf2Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf2Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf2Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf3Data(pub u32);
impl Buf3Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf3Data {
    #[inline(always)]
    fn default() -> Buf3Data {
        Buf3Data(0)
    }
}
impl core::fmt::Debug for Buf3Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf3Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf3Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf3Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf4Data(pub u32);
impl Buf4Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf4Data {
    #[inline(always)]
    fn default() -> Buf4Data {
        Buf4Data(0)
    }
}
impl core::fmt::Debug for Buf4Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf4Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf4Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf4Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf5Data(pub u32);
impl Buf5Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf5Data {
    #[inline(always)]
    fn default() -> Buf5Data {
        Buf5Data(0)
    }
}
impl core::fmt::Debug for Buf5Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf5Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf5Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf5Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf6Data(pub u32);
impl Buf6Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf6Data {
    #[inline(always)]
    fn default() -> Buf6Data {
        Buf6Data(0)
    }
}
impl core::fmt::Debug for Buf6Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf6Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf6Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf6Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf7Data(pub u32);
impl Buf7Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf7Data {
    #[inline(always)]
    fn default() -> Buf7Data {
        Buf7Data(0)
    }
}
impl core::fmt::Debug for Buf7Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf7Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf7Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf7Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf8Data(pub u32);
impl Buf8Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf8Data {
    #[inline(always)]
    fn default() -> Buf8Data {
        Buf8Data(0)
    }
}
impl core::fmt::Debug for Buf8Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf8Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf8Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf8Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Scan Results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buf9Data(pub u32);
impl Buf9Data {
    #[doc = "Scan Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Result Buffer."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn datasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_datasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Buf9Data {
    #[inline(always)]
    fn default() -> Buf9Data {
        Buf9Data(0)
    }
}
impl core::fmt::Debug for Buf9Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buf9Data")
            .field("data", &self.data())
            .field("datasrc", &self.datasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf9Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buf9Data {{ data: {=u16:?}, datasrc: {=u8:?} }}",
            self.data(),
            self.datasrc()
        )
    }
}
#[doc = "Result Buffer Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufdata(pub u32);
impl Bufdata {
    #[doc = "Result Data."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Result Data."]
    #[inline(always)]
    pub const fn set_bufdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Result Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdatasrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Data Source."]
    #[inline(always)]
    pub const fn set_bufdatasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Bufdata {
    #[inline(always)]
    fn default() -> Bufdata {
        Bufdata(0)
    }
}
impl core::fmt::Debug for Bufdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bufdata")
            .field("bufdata", &self.bufdata())
            .field("bufdatasrc", &self.bufdatasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bufdata {{ bufdata: {=u16:?}, bufdatasrc: {=u8:?} }}",
            self.bufdata(),
            self.bufdatasrc()
        )
    }
}
#[doc = "Scan Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Eval(pub u32);
impl Ch0Eval {
    #[doc = "Decision Threshold for Sensor Data."]
    #[must_use]
    #[inline(always)]
    pub const fn compthres(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Decision Threshold for Sensor Data."]
    #[inline(always)]
    pub const fn set_compthres(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Select Mode for Threshold Comparison."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Select Mode for Threshold Comparison."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Send Result to Decoder."]
    #[must_use]
    #[inline(always)]
    pub const fn decode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Send Result to Decoder."]
    #[inline(always)]
    pub const fn set_decode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable Storing of Sensor Sample in Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn strsample(&self) -> super::vals::Ch0EvalStrsample {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Ch0EvalStrsample::from_bits(val as u8)
    }
    #[doc = "Enable Storing of Sensor Sample in Result Buffer."]
    #[inline(always)]
    pub const fn set_strsample(&mut self, val: super::vals::Ch0EvalStrsample) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Enable Inversion of Result."]
    #[must_use]
    #[inline(always)]
    pub const fn scanresinv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Inversion of Result."]
    #[inline(always)]
    pub const fn set_scanresinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Configure Evaluation Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch0EvalMode {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Ch0EvalMode::from_bits(val as u8)
    }
    #[doc = "Configure Evaluation Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Ch0EvalMode) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for Ch0Eval {
    #[inline(always)]
    fn default() -> Ch0Eval {
        Ch0Eval(0)
    }
}
impl core::fmt::Debug for Ch0Eval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Eval")
            .field("compthres", &self.compthres())
            .field("comp", &self.comp())
            .field("decode", &self.decode())
            .field("strsample", &self.strsample())
            .field("scanresinv", &self.scanresinv())
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Eval {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch0Eval {{ compthres: {=u16:?}, comp: {=bool:?}, decode: {=bool:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}" , self . compthres () , self . comp () , self . decode () , self . strsample () , self . scanresinv () , self . mode ())
    }
}
#[doc = "Scan Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Interact(pub u32);
impl Ch0Interact {
    #[doc = "ACMP Threshold or VDAC Data."]
    #[must_use]
    #[inline(always)]
    pub const fn thres(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ACMP Threshold or VDAC Data."]
    #[inline(always)]
    pub const fn set_thres(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Select Sample Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sample(&self) -> super::vals::Ch0InteractSample {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ch0InteractSample::from_bits(val as u8)
    }
    #[doc = "Select Sample Mode."]
    #[inline(always)]
    pub const fn set_sample(&mut self, val: super::vals::Ch0InteractSample) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Enable Interrupt Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn setif(&self) -> super::vals::Ch0InteractSetif {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Ch0InteractSetif::from_bits(val as u8)
    }
    #[doc = "Enable Interrupt Generation."]
    #[inline(always)]
    pub const fn set_setif(&mut self, val: super::vals::Ch0InteractSetif) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Set GPIO Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn exmode(&self) -> super::vals::Ch0InteractExmode {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Ch0InteractExmode::from_bits(val as u8)
    }
    #[doc = "Set GPIO Mode."]
    #[inline(always)]
    pub const fn set_exmode(&mut self, val: super::vals::Ch0InteractExmode) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "Select Clock Used for Excitation Timing."]
    #[must_use]
    #[inline(always)]
    pub const fn exclk(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Select Clock Used for Excitation Timing."]
    #[inline(always)]
    pub const fn set_exclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Select Clock Used for Timing of Sample Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn sampleclk(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Select Clock Used for Timing of Sample Delay."]
    #[inline(always)]
    pub const fn set_sampleclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Use Alternative Excite Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn altex(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Use Alternative Excite Pin."]
    #[inline(always)]
    pub const fn set_altex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch0Interact {
    #[inline(always)]
    fn default() -> Ch0Interact {
        Ch0Interact(0)
    }
}
impl core::fmt::Debug for Ch0Interact {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Interact")
            .field("thres", &self.thres())
            .field("sample", &self.sample())
            .field("setif", &self.setif())
            .field("exmode", &self.exmode())
            .field("exclk", &self.exclk())
            .field("sampleclk", &self.sampleclk())
            .field("altex", &self.altex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Interact {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch0Interact {{ thres: {=u16:?}, sample: {:?}, setif: {:?}, exmode: {:?}, exclk: {=bool:?}, sampleclk: {=bool:?}, altex: {=bool:?} }}" , self . thres () , self . sample () , self . setif () , self . exmode () , self . exclk () , self . sampleclk () , self . altex ())
    }
}
#[doc = "Scan Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Timing(pub u32);
impl Ch0Timing {
    #[doc = "Set Excitation Time."]
    #[must_use]
    #[inline(always)]
    pub const fn extime(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Set Excitation Time."]
    #[inline(always)]
    pub const fn set_extime(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Set Sample Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn sampledly(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0xff;
        val as u8
    }
    #[doc = "Set Sample Delay."]
    #[inline(always)]
    pub const fn set_sampledly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
    }
    #[doc = "Set Measure Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn measuredly(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x03ff;
        val as u16
    }
    #[doc = "Set Measure Delay."]
    #[inline(always)]
    pub const fn set_measuredly(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
    }
}
impl Default for Ch0Timing {
    #[inline(always)]
    fn default() -> Ch0Timing {
        Ch0Timing(0)
    }
}
impl core::fmt::Debug for Ch0Timing {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Timing")
            .field("extime", &self.extime())
            .field("sampledly", &self.sampledly())
            .field("measuredly", &self.measuredly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Timing {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
            self.extime(),
            self.sampledly(),
            self.measuredly()
        )
    }
}
#[doc = "Channel Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chen(pub u32);
impl Chen {
    #[doc = "Enable Scan Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn chen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Enable Scan Channel."]
    #[inline(always)]
    pub const fn set_chen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Chen {
    #[inline(always)]
    fn default() -> Chen {
        Chen(0)
    }
}
impl core::fmt::Debug for Chen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chen").field("chen", &self.chen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chen {{ chen: {=u16:?} }}", self.chen())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Start Scanning of Sensors."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Scanning of Sensors."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop Scanning of Sensors."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Scanning of Sensors."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Decoder."]
    #[must_use]
    #[inline(always)]
    pub const fn decode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start Decoder."]
    #[inline(always)]
    pub const fn set_decode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Result Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn clearbuf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Result Buffer."]
    #[inline(always)]
    pub const fn set_clearbuf(&mut self, val: bool) {
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
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("decode", &self.decode())
            .field("clearbuf", &self.clearbuf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ start: {=bool:?}, stop: {=bool:?}, decode: {=bool:?}, clearbuf: {=bool:?} }}",
            self.start(),
            self.stop(),
            self.decode(),
            self.clearbuf()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Configure Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn scanmode(&self) -> super::vals::Scanmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Scanmode::from_bits(val as u8)
    }
    #[doc = "Configure Scan Mode."]
    #[inline(always)]
    pub const fn set_scanmode(&mut self, val: super::vals::Scanmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Scan Start PRS Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 2usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Scan Start PRS Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val.to_bits() as u32) & 0x1f) << 2usize);
    }
    #[doc = "Select Scan Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn scanconf(&self) -> super::vals::Scanconf {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Scanconf::from_bits(val as u8)
    }
    #[doc = "Select Scan Configuration."]
    #[inline(always)]
    pub const fn set_scanconf(&mut self, val: super::vals::Scanconf) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Alternative Excitation Map."]
    #[must_use]
    #[inline(always)]
    pub const fn altexmap(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Excitation Map."]
    #[inline(always)]
    pub const fn set_altexmap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Dual Sample Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dualsample(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Dual Sample Mode."]
    #[inline(always)]
    pub const fn set_dualsample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Result Buffer Overwrite."]
    #[must_use]
    #[inline(always)]
    pub const fn bufow(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Result Buffer Overwrite."]
    #[inline(always)]
    pub const fn set_bufow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable Storing of SCANRES."]
    #[must_use]
    #[inline(always)]
    pub const fn strscanres(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Storing of SCANRES."]
    #[inline(always)]
    pub const fn set_strscanres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Result Buffer Interrupt and DMA Trigger Level."]
    #[must_use]
    #[inline(always)]
    pub const fn bufidl(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Result Buffer Interrupt and DMA Trigger Level."]
    #[inline(always)]
    pub const fn set_bufidl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA Wake-up From EM2."]
    #[must_use]
    #[inline(always)]
    pub const fn dmawu(&self) -> super::vals::Dmawu {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dmawu::from_bits(val as u8)
    }
    #[doc = "DMA Wake-up From EM2."]
    #[inline(always)]
    pub const fn set_dmawu(&mut self, val: super::vals::Dmawu) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            .field("scanmode", &self.scanmode())
            .field("prssel", &self.prssel())
            .field("scanconf", &self.scanconf())
            .field("altexmap", &self.altexmap())
            .field("dualsample", &self.dualsample())
            .field("bufow", &self.bufow())
            .field("strscanres", &self.strscanres())
            .field("bufidl", &self.bufidl())
            .field("dmawu", &self.dmawu())
            .field("debugrun", &self.debugrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ scanmode: {:?}, prssel: {:?}, scanconf: {:?}, altexmap: {=bool:?}, dualsample: {=bool:?}, bufow: {=bool:?}, strscanres: {=bool:?}, bufidl: {=bool:?}, dmawu: {:?}, debugrun: {=bool:?} }}" , self . scanmode () , self . prssel () , self . scanconf () , self . altexmap () , self . dualsample () , self . bufow () , self . strscanres () , self . bufidl () , self . dmawu () , self . debugrun ())
    }
}
#[doc = "Current Channel Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Curch(pub u32);
impl Curch {
    #[doc = "Current Channel Index."]
    #[must_use]
    #[inline(always)]
    pub const fn curch(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Current Channel Index."]
    #[inline(always)]
    pub const fn set_curch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Curch {
    #[inline(always)]
    fn default() -> Curch {
        Curch(0)
    }
}
impl core::fmt::Debug for Curch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Curch")
            .field("curch", &self.curch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Curch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Curch {{ curch: {=u8:?} }}", self.curch())
    }
}
#[doc = "Decoder Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Decctrl(pub u32);
impl Decctrl {
    #[doc = "Disable the Decoder."]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the Decoder."]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Check of Current State."]
    #[must_use]
    #[inline(always)]
    pub const fn errchk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Check of Current State."]
    #[inline(always)]
    pub const fn set_errchk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Decoder to Channel Interrupt Mapping."]
    #[must_use]
    #[inline(always)]
    pub const fn intmap(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Decoder to Channel Interrupt Mapping."]
    #[inline(always)]
    pub const fn set_intmap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Decoder Hysteresis on PRS0 Output."]
    #[must_use]
    #[inline(always)]
    pub const fn hystprs0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Decoder Hysteresis on PRS0 Output."]
    #[inline(always)]
    pub const fn set_hystprs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Decoder Hysteresis on PRS1 Output."]
    #[must_use]
    #[inline(always)]
    pub const fn hystprs1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Decoder Hysteresis on PRS1 Output."]
    #[inline(always)]
    pub const fn set_hystprs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Decoder Hysteresis on PRS2 Output."]
    #[must_use]
    #[inline(always)]
    pub const fn hystprs2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Decoder Hysteresis on PRS2 Output."]
    #[inline(always)]
    pub const fn set_hystprs2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Decoder Hysteresis on Interrupt Requests."]
    #[must_use]
    #[inline(always)]
    pub const fn hystirq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Decoder Hysteresis on Interrupt Requests."]
    #[inline(always)]
    pub const fn set_hystirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Count Mode on Decoder PRS Channels 0 and 1."]
    #[must_use]
    #[inline(always)]
    pub const fn prscnt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Count Mode on Decoder PRS Channels 0 and 1."]
    #[inline(always)]
    pub const fn set_prscnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LESENSE Decoder Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LESENSE Decoder Input Configuration."]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LESENSE Decoder PRS Input 0 Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel0(&self) -> super::vals::Prssel0 {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::Prssel0::from_bits(val as u8)
    }
    #[doc = "LESENSE Decoder PRS Input 0 Configuration."]
    #[inline(always)]
    pub const fn set_prssel0(&mut self, val: super::vals::Prssel0) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "LESENSE Decoder PRS Input 1 Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel1(&self) -> super::vals::Prssel1 {
        let val = (self.0 >> 15usize) & 0x1f;
        super::vals::Prssel1::from_bits(val as u8)
    }
    #[doc = "LESENSE Decoder PRS Input 1 Configuration."]
    #[inline(always)]
    pub const fn set_prssel1(&mut self, val: super::vals::Prssel1) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val.to_bits() as u32) & 0x1f) << 15usize);
    }
    #[doc = "LESENSE Decoder PRS Input 2 Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel2(&self) -> super::vals::Prssel2 {
        let val = (self.0 >> 20usize) & 0x1f;
        super::vals::Prssel2::from_bits(val as u8)
    }
    #[doc = "LESENSE Decoder PRS Input 2 Configuration."]
    #[inline(always)]
    pub const fn set_prssel2(&mut self, val: super::vals::Prssel2) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val.to_bits() as u32) & 0x1f) << 20usize);
    }
    #[doc = "LESENSE Decoder PRS Input 3 Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel3(&self) -> super::vals::Prssel3 {
        let val = (self.0 >> 25usize) & 0x1f;
        super::vals::Prssel3::from_bits(val as u8)
    }
    #[doc = "LESENSE Decoder PRS Input 3 Configuration."]
    #[inline(always)]
    pub const fn set_prssel3(&mut self, val: super::vals::Prssel3) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val.to_bits() as u32) & 0x1f) << 25usize);
    }
}
impl Default for Decctrl {
    #[inline(always)]
    fn default() -> Decctrl {
        Decctrl(0)
    }
}
impl core::fmt::Debug for Decctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Decctrl")
            .field("disable", &self.disable())
            .field("errchk", &self.errchk())
            .field("intmap", &self.intmap())
            .field("hystprs0", &self.hystprs0())
            .field("hystprs1", &self.hystprs1())
            .field("hystprs2", &self.hystprs2())
            .field("hystirq", &self.hystirq())
            .field("prscnt", &self.prscnt())
            .field("input", &self.input())
            .field("prssel0", &self.prssel0())
            .field("prssel1", &self.prssel1())
            .field("prssel2", &self.prssel2())
            .field("prssel3", &self.prssel3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Decctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Decctrl {{ disable: {=bool:?}, errchk: {=bool:?}, intmap: {=bool:?}, hystprs0: {=bool:?}, hystprs1: {=bool:?}, hystprs2: {=bool:?}, hystirq: {=bool:?}, prscnt: {=bool:?}, input: {=bool:?}, prssel0: {:?}, prssel1: {:?}, prssel2: {:?}, prssel3: {:?} }}" , self . disable () , self . errchk () , self . intmap () , self . hystprs0 () , self . hystprs1 () , self . hystprs2 () , self . hystirq () , self . prscnt () , self . input () , self . prssel0 () , self . prssel1 () , self . prssel2 () , self . prssel3 ())
    }
}
#[doc = "Current Decoder State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Decstate(pub u32);
impl Decstate {
    #[doc = "Current Decoder State."]
    #[must_use]
    #[inline(always)]
    pub const fn decstate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Current Decoder State."]
    #[inline(always)]
    pub const fn set_decstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Decstate {
    #[inline(always)]
    fn default() -> Decstate {
        Decstate(0)
    }
}
impl core::fmt::Debug for Decstate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Decstate")
            .field("decstate", &self.decstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Decstate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Decstate {{ decstate: {=u8:?} }}", self.decstate())
    }
}
#[doc = "LESENSE Evaluation Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evalctrl(pub u32);
impl Evalctrl {
    #[doc = "Sliding Window and Step Detection Size."]
    #[must_use]
    #[inline(always)]
    pub const fn winsize(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sliding Window and Step Detection Size."]
    #[inline(always)]
    pub const fn set_winsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Evalctrl {
    #[inline(always)]
    fn default() -> Evalctrl {
        Evalctrl(0)
    }
}
impl core::fmt::Debug for Evalctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evalctrl")
            .field("winsize", &self.winsize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evalctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Evalctrl {{ winsize: {=u16:?} }}", self.winsize())
    }
}
#[doc = "GPIO Idle Phase Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idleconf(pub u32);
impl Idleconf {
    #[doc = "Channel 0 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0(&self) -> super::vals::Ch0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch0::from_bits(val as u8)
    }
    #[doc = "Channel 0 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch0(&mut self, val: super::vals::Ch0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Channel 1 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1(&self) -> super::vals::Ch1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Ch1::from_bits(val as u8)
    }
    #[doc = "Channel 1 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch1(&mut self, val: super::vals::Ch1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Channel 2 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2(&self) -> super::vals::Ch2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Ch2::from_bits(val as u8)
    }
    #[doc = "Channel 2 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch2(&mut self, val: super::vals::Ch2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Channel 3 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3(&self) -> super::vals::Ch3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ch3::from_bits(val as u8)
    }
    #[doc = "Channel 3 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch3(&mut self, val: super::vals::Ch3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Channel 4 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4(&self) -> super::vals::Ch4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Ch4::from_bits(val as u8)
    }
    #[doc = "Channel 4 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch4(&mut self, val: super::vals::Ch4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Channel 5 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5(&self) -> super::vals::Ch5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Ch5::from_bits(val as u8)
    }
    #[doc = "Channel 5 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch5(&mut self, val: super::vals::Ch5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel 6 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6(&self) -> super::vals::Ch6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Ch6::from_bits(val as u8)
    }
    #[doc = "Channel 6 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch6(&mut self, val: super::vals::Ch6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Channel 7 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7(&self) -> super::vals::Ch7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Ch7::from_bits(val as u8)
    }
    #[doc = "Channel 7 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch7(&mut self, val: super::vals::Ch7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Channel 8 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8(&self) -> super::vals::Ch8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch8::from_bits(val as u8)
    }
    #[doc = "Channel 8 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch8(&mut self, val: super::vals::Ch8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Channel 9 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9(&self) -> super::vals::Ch9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Ch9::from_bits(val as u8)
    }
    #[doc = "Channel 9 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch9(&mut self, val: super::vals::Ch9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Channel 10 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10(&self) -> super::vals::Ch10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch10::from_bits(val as u8)
    }
    #[doc = "Channel 10 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch10(&mut self, val: super::vals::Ch10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Channel 11 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11(&self) -> super::vals::Ch11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Ch11::from_bits(val as u8)
    }
    #[doc = "Channel 11 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch11(&mut self, val: super::vals::Ch11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Channel 12 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12(&self) -> super::vals::Ch12 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch12::from_bits(val as u8)
    }
    #[doc = "Channel 12 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch12(&mut self, val: super::vals::Ch12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Channel 13 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13(&self) -> super::vals::Ch13 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch13::from_bits(val as u8)
    }
    #[doc = "Channel 13 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch13(&mut self, val: super::vals::Ch13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Channel 14 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14(&self) -> super::vals::Ch14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch14::from_bits(val as u8)
    }
    #[doc = "Channel 14 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch14(&mut self, val: super::vals::Ch14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Channel 15 Idle Phase Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15(&self) -> super::vals::Ch15 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Ch15::from_bits(val as u8)
    }
    #[doc = "Channel 15 Idle Phase Configuration."]
    #[inline(always)]
    pub const fn set_ch15(&mut self, val: super::vals::Ch15) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Idleconf {
    #[inline(always)]
    fn default() -> Idleconf {
        Idleconf(0)
    }
}
impl core::fmt::Debug for Idleconf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idleconf")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .field("ch8", &self.ch8())
            .field("ch9", &self.ch9())
            .field("ch10", &self.ch10())
            .field("ch11", &self.ch11())
            .field("ch12", &self.ch12())
            .field("ch13", &self.ch13())
            .field("ch14", &self.ch14())
            .field("ch15", &self.ch15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idleconf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Idleconf {{ ch0: {:?}, ch1: {:?}, ch2: {:?}, ch3: {:?}, ch4: {:?}, ch5: {:?}, ch6: {:?}, ch7: {:?}, ch8: {:?}, ch9: {:?}, ch10: {:?}, ch11: {:?}, ch12: {:?}, ch13: {:?}, ch14: {:?}, ch15: {:?} }}" , self . ch0 () , self . ch1 () , self . ch2 () , self . ch3 () , self . ch4 () , self . ch5 () , self . ch6 () , self . ch7 () , self . ch8 () , self . ch9 () , self . ch10 () , self . ch11 () , self . ch12 () , self . ch13 () , self . ch14 () , self . ch15 ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "CH0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH2 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH2 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH3 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH3 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH4 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH4 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH5 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH5 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH6 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH6 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH7 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH7 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CH8 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CH8 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CH9 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CH9 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CH10 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CH10 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CH11 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CH11 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CH12 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CH12 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CH13 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CH13 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CH14 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CH14 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CH15 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CH15 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SCANCOMPLETE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scancomplete(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SCANCOMPLETE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scancomplete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DEC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dec(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DEC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DECERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn decerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DECERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_decerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BUFDATAV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdatav(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BUFDATAV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bufdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "BUFLEVEL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn buflevel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "BUFLEVEL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_buflevel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "BUFOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufof(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "BUFOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bufof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "CNTOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cntof(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CNTOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cntof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .field("ch8", &self.ch8())
            .field("ch9", &self.ch9())
            .field("ch10", &self.ch10())
            .field("ch11", &self.ch11())
            .field("ch12", &self.ch12())
            .field("ch13", &self.ch13())
            .field("ch14", &self.ch14())
            .field("ch15", &self.ch15())
            .field("scancomplete", &self.scancomplete())
            .field("dec", &self.dec())
            .field("decerr", &self.decerr())
            .field("bufdatav", &self.bufdatav())
            .field("buflevel", &self.buflevel())
            .field("bufof", &self.bufof())
            .field("cntof", &self.cntof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ ch0: {=bool:?}, ch1: {=bool:?}, ch2: {=bool:?}, ch3: {=bool:?}, ch4: {=bool:?}, ch5: {=bool:?}, ch6: {=bool:?}, ch7: {=bool:?}, ch8: {=bool:?}, ch9: {=bool:?}, ch10: {=bool:?}, ch11: {=bool:?}, ch12: {=bool:?}, ch13: {=bool:?}, ch14: {=bool:?}, ch15: {=bool:?}, scancomplete: {=bool:?}, dec: {=bool:?}, decerr: {=bool:?}, bufdatav: {=bool:?}, buflevel: {=bool:?}, bufof: {=bool:?}, cntof: {=bool:?} }}" , self . ch0 () , self . ch1 () , self . ch2 () , self . ch3 () , self . ch4 () , self . ch5 () , self . ch6 () , self . ch7 () , self . ch8 () , self . ch9 () , self . ch10 () , self . ch11 () , self . ch12 () , self . ch13 () , self . ch14 () , self . ch15 () , self . scancomplete () , self . dec () , self . decerr () , self . bufdatav () , self . buflevel () , self . bufof () , self . cntof ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "CH0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH3 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH3 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH4 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH4 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH5 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH5 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH6 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH6 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH7 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH7 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CH8 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CH8 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CH9 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CH9 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CH10 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CH10 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CH11 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CH11 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CH12 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CH12 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CH13 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CH13 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CH14 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CH14 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CH15 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CH15 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SCANCOMPLETE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scancomplete(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SCANCOMPLETE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scancomplete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DEC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dec(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DEC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DECERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn decerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DECERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_decerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BUFDATAV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdatav(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BUFDATAV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bufdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "BUFLEVEL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn buflevel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "BUFLEVEL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_buflevel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "BUFOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bufof(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "BUFOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bufof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "CNTOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cntof(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CNTOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cntof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .field("ch8", &self.ch8())
            .field("ch9", &self.ch9())
            .field("ch10", &self.ch10())
            .field("ch11", &self.ch11())
            .field("ch12", &self.ch12())
            .field("ch13", &self.ch13())
            .field("ch14", &self.ch14())
            .field("ch15", &self.ch15())
            .field("scancomplete", &self.scancomplete())
            .field("dec", &self.dec())
            .field("decerr", &self.decerr())
            .field("bufdatav", &self.bufdatav())
            .field("buflevel", &self.buflevel())
            .field("bufof", &self.bufof())
            .field("cntof", &self.cntof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ ch0: {=bool:?}, ch1: {=bool:?}, ch2: {=bool:?}, ch3: {=bool:?}, ch4: {=bool:?}, ch5: {=bool:?}, ch6: {=bool:?}, ch7: {=bool:?}, ch8: {=bool:?}, ch9: {=bool:?}, ch10: {=bool:?}, ch11: {=bool:?}, ch12: {=bool:?}, ch13: {=bool:?}, ch14: {=bool:?}, ch15: {=bool:?}, scancomplete: {=bool:?}, dec: {=bool:?}, decerr: {=bool:?}, bufdatav: {=bool:?}, buflevel: {=bool:?}, bufof: {=bool:?}, cntof: {=bool:?} }}" , self . ch0 () , self . ch1 () , self . ch2 () , self . ch3 () , self . ch4 () , self . ch5 () , self . ch6 () , self . ch7 () , self . ch8 () , self . ch9 () , self . ch10 () , self . ch11 () , self . ch12 () , self . ch13 () , self . ch14 () , self . ch15 () , self . scancomplete () , self . dec () , self . decerr () , self . bufdatav () , self . buflevel () , self . bufof () , self . cntof ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set CH0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set CH1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set CH2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set CH3 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH3 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set CH4 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH4 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set CH5 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH5 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set CH6 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH6 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set CH7 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH7 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set CH8 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH8 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set CH9 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH9 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set CH10 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH10 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set CH11 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH11 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set CH12 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH12 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set CH13 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH13 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set CH14 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH14 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set CH15 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH15 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set SCANCOMPLETE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scancomplete(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set SCANCOMPLETE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_scancomplete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set DEC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dec(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set DEC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set DECERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn decerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set DECERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_decerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set BUFDATAV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdatav(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set BUFDATAV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bufdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set BUFLEVEL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn buflevel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set BUFLEVEL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_buflevel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set BUFOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bufof(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set BUFOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_bufof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Set CNTOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cntof(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Set CNTOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cntof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .field("ch8", &self.ch8())
            .field("ch9", &self.ch9())
            .field("ch10", &self.ch10())
            .field("ch11", &self.ch11())
            .field("ch12", &self.ch12())
            .field("ch13", &self.ch13())
            .field("ch14", &self.ch14())
            .field("ch15", &self.ch15())
            .field("scancomplete", &self.scancomplete())
            .field("dec", &self.dec())
            .field("decerr", &self.decerr())
            .field("bufdatav", &self.bufdatav())
            .field("buflevel", &self.buflevel())
            .field("bufof", &self.bufof())
            .field("cntof", &self.cntof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ ch0: {=bool:?}, ch1: {=bool:?}, ch2: {=bool:?}, ch3: {=bool:?}, ch4: {=bool:?}, ch5: {=bool:?}, ch6: {=bool:?}, ch7: {=bool:?}, ch8: {=bool:?}, ch9: {=bool:?}, ch10: {=bool:?}, ch11: {=bool:?}, ch12: {=bool:?}, ch13: {=bool:?}, ch14: {=bool:?}, ch15: {=bool:?}, scancomplete: {=bool:?}, dec: {=bool:?}, decerr: {=bool:?}, bufdatav: {=bool:?}, buflevel: {=bool:?}, bufof: {=bool:?}, cntof: {=bool:?} }}" , self . ch0 () , self . ch1 () , self . ch2 () , self . ch3 () , self . ch4 () , self . ch5 () , self . ch6 () , self . ch7 () , self . ch8 () , self . ch9 () , self . ch10 () , self . ch11 () , self . ch12 () , self . ch13 () , self . ch14 () , self . ch15 () , self . scancomplete () , self . dec () , self . decerr () , self . bufdatav () , self . buflevel () , self . bufof () , self . cntof ())
    }
}
#[doc = "Peripheral Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perctrl(pub u32);
impl Perctrl {
    #[doc = "VDAC CH0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dacch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC CH0 Enable."]
    #[inline(always)]
    pub const fn set_dacch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VDAC CH1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dacch1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC CH1 Enable."]
    #[inline(always)]
    pub const fn set_dacch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VDAC CH0 Data Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dacch0data(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC CH0 Data Selection."]
    #[inline(always)]
    pub const fn set_dacch0data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDAC CH1 Data Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dacch1data(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC CH1 Data Selection."]
    #[inline(always)]
    pub const fn set_dacch1data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VDAC Startup Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn dacstartup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC Startup Configuration."]
    #[inline(always)]
    pub const fn set_dacstartup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "VDAC Conversion Trigger Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn dacconvtrig(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VDAC Conversion Trigger Configuration."]
    #[inline(always)]
    pub const fn set_dacconvtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ACMP0 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0mode(&self) -> super::vals::Acmp0mode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Acmp0mode::from_bits(val as u8)
    }
    #[doc = "ACMP0 Mode."]
    #[inline(always)]
    pub const fn set_acmp0mode(&mut self, val: super::vals::Acmp0mode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ACMP1 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1mode(&self) -> super::vals::Acmp1mode {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Acmp1mode::from_bits(val as u8)
    }
    #[doc = "ACMP1 Mode."]
    #[inline(always)]
    pub const fn set_acmp1mode(&mut self, val: super::vals::Acmp1mode) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Invert Analog Comparator 0 Output."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0inv(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Analog Comparator 0 Output."]
    #[inline(always)]
    pub const fn set_acmp0inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Invert Analog Comparator 1 Output."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1inv(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Analog Comparator 1 Output."]
    #[inline(always)]
    pub const fn set_acmp1inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ACMP0 Hysteresis Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0hysten(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ACMP0 Hysteresis Enable."]
    #[inline(always)]
    pub const fn set_acmp0hysten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ACMP1 Hysteresis Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1hysten(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ACMP1 Hysteresis Enable."]
    #[inline(always)]
    pub const fn set_acmp1hysten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ACMP and VDAC Duty Cycle Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn warmupmode(&self) -> super::vals::Warmupmode {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Warmupmode::from_bits(val as u8)
    }
    #[doc = "ACMP and VDAC Duty Cycle Mode."]
    #[inline(always)]
    pub const fn set_warmupmode(&mut self, val: super::vals::Warmupmode) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Perctrl {
    #[inline(always)]
    fn default() -> Perctrl {
        Perctrl(0)
    }
}
impl core::fmt::Debug for Perctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Perctrl")
            .field("dacch0en", &self.dacch0en())
            .field("dacch1en", &self.dacch1en())
            .field("dacch0data", &self.dacch0data())
            .field("dacch1data", &self.dacch1data())
            .field("dacstartup", &self.dacstartup())
            .field("dacconvtrig", &self.dacconvtrig())
            .field("acmp0mode", &self.acmp0mode())
            .field("acmp1mode", &self.acmp1mode())
            .field("acmp0inv", &self.acmp0inv())
            .field("acmp1inv", &self.acmp1inv())
            .field("acmp0hysten", &self.acmp0hysten())
            .field("acmp1hysten", &self.acmp1hysten())
            .field("warmupmode", &self.warmupmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Perctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Perctrl {{ dacch0en: {=bool:?}, dacch1en: {=bool:?}, dacch0data: {=bool:?}, dacch1data: {=bool:?}, dacstartup: {=bool:?}, dacconvtrig: {=bool:?}, acmp0mode: {:?}, acmp1mode: {:?}, acmp0inv: {=bool:?}, acmp1inv: {=bool:?}, acmp0hysten: {=bool:?}, acmp1hysten: {=bool:?}, warmupmode: {:?} }}" , self . dacch0en () , self . dacch1en () , self . dacch0data () , self . dacch1data () , self . dacstartup () , self . dacconvtrig () , self . acmp0mode () , self . acmp1mode () , self . acmp0inv () , self . acmp1inv () , self . acmp0hysten () , self . acmp1hysten () , self . warmupmode ())
    }
}
#[doc = "PRS Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsctrl(pub u32);
impl Prsctrl {
    #[doc = "Decoder State Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn deccmpval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Decoder State Compare Value."]
    #[inline(always)]
    pub const fn set_deccmpval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Decoder State Compare Value Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn deccmpmask(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Decoder State Compare Value Mask."]
    #[inline(always)]
    pub const fn set_deccmpmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Enable PRS Output DECCMP."]
    #[must_use]
    #[inline(always)]
    pub const fn deccmpen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PRS Output DECCMP."]
    #[inline(always)]
    pub const fn set_deccmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Prsctrl {
    #[inline(always)]
    fn default() -> Prsctrl {
        Prsctrl(0)
    }
}
impl core::fmt::Debug for Prsctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prsctrl")
            .field("deccmpval", &self.deccmpval())
            .field("deccmpmask", &self.deccmpmask())
            .field("deccmpen", &self.deccmpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prsctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prsctrl {{ deccmpval: {=u8:?}, deccmpmask: {=u8:?}, deccmpen: {=bool:?} }}",
            self.deccmpval(),
            self.deccmpmask(),
            self.deccmpen()
        )
    }
}
#[doc = "Result Buffer Pointers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptr(pub u32);
impl Ptr {
    #[doc = "Result Buffer Read Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Buffer Read Pointer."]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Result Buffer Write Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn wr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Result Buffer Write Pointer."]
    #[inline(always)]
    pub const fn set_wr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Ptr {
    #[inline(always)]
    fn default() -> Ptr {
        Ptr(0)
    }
}
impl core::fmt::Debug for Ptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptr")
            .field("rd", &self.rd())
            .field("wr", &self.wr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ptr {{ rd: {=u8:?}, wr: {=u8:?} }}",
            self.rd(),
            self.wr()
        )
    }
}
#[doc = "I/O Routing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CH0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH2 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH3 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH4 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4pen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH4 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH5 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH5 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH6 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6pen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH6 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH7 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7pen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH7 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CH8 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CH8 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch8pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CH9 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CH9 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch9pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CH10 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10pen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CH10 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch10pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CH11 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11pen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CH11 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch11pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CH12 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12pen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CH12 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch12pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CH13 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13pen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CH13 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch13pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CH14 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14pen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CH14 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch14pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CH15 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15pen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CH15 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch15pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ALTEX0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex0pen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX0 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ALTEX1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex1pen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX1 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ALTEX2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex2pen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX2 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ALTEX3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex3pen(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX3 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ALTEX4 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex4pen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX4 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ALTEX5 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex5pen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX5 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ALTEX6 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex6pen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX6 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ALTEX7 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altex7pen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ALTEX7 Pin Enable."]
    #[inline(always)]
    pub const fn set_altex7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("ch0pen", &self.ch0pen())
            .field("ch1pen", &self.ch1pen())
            .field("ch2pen", &self.ch2pen())
            .field("ch3pen", &self.ch3pen())
            .field("ch4pen", &self.ch4pen())
            .field("ch5pen", &self.ch5pen())
            .field("ch6pen", &self.ch6pen())
            .field("ch7pen", &self.ch7pen())
            .field("ch8pen", &self.ch8pen())
            .field("ch9pen", &self.ch9pen())
            .field("ch10pen", &self.ch10pen())
            .field("ch11pen", &self.ch11pen())
            .field("ch12pen", &self.ch12pen())
            .field("ch13pen", &self.ch13pen())
            .field("ch14pen", &self.ch14pen())
            .field("ch15pen", &self.ch15pen())
            .field("altex0pen", &self.altex0pen())
            .field("altex1pen", &self.altex1pen())
            .field("altex2pen", &self.altex2pen())
            .field("altex3pen", &self.altex3pen())
            .field("altex4pen", &self.altex4pen())
            .field("altex5pen", &self.altex5pen())
            .field("altex6pen", &self.altex6pen())
            .field("altex7pen", &self.altex7pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ ch0pen: {=bool:?}, ch1pen: {=bool:?}, ch2pen: {=bool:?}, ch3pen: {=bool:?}, ch4pen: {=bool:?}, ch5pen: {=bool:?}, ch6pen: {=bool:?}, ch7pen: {=bool:?}, ch8pen: {=bool:?}, ch9pen: {=bool:?}, ch10pen: {=bool:?}, ch11pen: {=bool:?}, ch12pen: {=bool:?}, ch13pen: {=bool:?}, ch14pen: {=bool:?}, ch15pen: {=bool:?}, altex0pen: {=bool:?}, altex1pen: {=bool:?}, altex2pen: {=bool:?}, altex3pen: {=bool:?}, altex4pen: {=bool:?}, altex5pen: {=bool:?}, altex6pen: {=bool:?}, altex7pen: {=bool:?} }}" , self . ch0pen () , self . ch1pen () , self . ch2pen () , self . ch3pen () , self . ch4pen () , self . ch5pen () , self . ch6pen () , self . ch7pen () , self . ch8pen () , self . ch9pen () , self . ch10pen () , self . ch11pen () , self . ch12pen () , self . ch13pen () , self . ch14pen () , self . ch15pen () , self . altex0pen () , self . altex1pen () , self . altex2pen () , self . altex3pen () , self . altex4pen () , self . altex5pen () , self . altex6pen () , self . altex7pen ())
    }
}
#[doc = "Scan Result Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scanres(pub u32);
impl Scanres {
    #[doc = "Scan Results."]
    #[must_use]
    #[inline(always)]
    pub const fn scanres(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn set_scanres(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Direction of Previous Step Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn stepdir(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Direction of Previous Step Detection."]
    #[inline(always)]
    pub const fn set_stepdir(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Scanres {
    #[inline(always)]
    fn default() -> Scanres {
        Scanres(0)
    }
}
impl core::fmt::Debug for Scanres {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanres")
            .field("scanres", &self.scanres())
            .field("stepdir", &self.stepdir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scanres {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scanres {{ scanres: {=u16:?}, stepdir: {=u16:?} }}",
            self.scanres(),
            self.stepdir()
        )
    }
}
#[doc = "Decoder Input Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sensorstate(pub u32);
impl Sensorstate {
    #[doc = "Decoder Input Register."]
    #[must_use]
    #[inline(always)]
    pub const fn sensorstate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Decoder Input Register."]
    #[inline(always)]
    pub const fn set_sensorstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Sensorstate {
    #[inline(always)]
    fn default() -> Sensorstate {
        Sensorstate(0)
    }
}
impl core::fmt::Debug for Sensorstate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sensorstate")
            .field("sensorstate", &self.sensorstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sensorstate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sensorstate {{ sensorstate: {=u8:?} }}",
            self.sensorstate()
        )
    }
}
#[doc = "State Transition Configuration a."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St0Tconfa(pub u32);
impl St0Tconfa {
    #[doc = "Sensor Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Sensor Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sensor Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Sensor Mask."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Next State Index."]
    #[must_use]
    #[inline(always)]
    pub const fn nextstate(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Next State Index."]
    #[inline(always)]
    pub const fn set_nextstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Enable State Descriptor Chaining."]
    #[must_use]
    #[inline(always)]
    pub const fn chain(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable State Descriptor Chaining."]
    #[inline(always)]
    pub const fn set_chain(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set Interrupt Flag Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn setif(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt Flag Enable."]
    #[inline(always)]
    pub const fn set_setif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Configure Transition Action."]
    #[must_use]
    #[inline(always)]
    pub const fn prsact(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Configure Transition Action."]
    #[inline(always)]
    pub const fn set_prsact(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for St0Tconfa {
    #[inline(always)]
    fn default() -> St0Tconfa {
        St0Tconfa(0)
    }
}
impl core::fmt::Debug for St0Tconfa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("St0Tconfa")
            .field("comp", &self.comp())
            .field("mask", &self.mask())
            .field("nextstate", &self.nextstate())
            .field("chain", &self.chain())
            .field("setif", &self.setif())
            .field("prsact", &self.prsact())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for St0Tconfa {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "St0Tconfa {{ comp: {=u8:?}, mask: {=u8:?}, nextstate: {=u8:?}, chain: {=bool:?}, setif: {=bool:?}, prsact: {=u8:?} }}" , self . comp () , self . mask () , self . nextstate () , self . chain () , self . setif () , self . prsact ())
    }
}
#[doc = "State Transition Configuration B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St0Tconfb(pub u32);
impl St0Tconfb {
    #[doc = "Sensor Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Sensor Compare Value."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sensor Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Sensor Mask."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Next State Index."]
    #[must_use]
    #[inline(always)]
    pub const fn nextstate(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Next State Index."]
    #[inline(always)]
    pub const fn set_nextstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Set Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn setif(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt Flag."]
    #[inline(always)]
    pub const fn set_setif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Configure Transition Action."]
    #[must_use]
    #[inline(always)]
    pub const fn prsact(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Configure Transition Action."]
    #[inline(always)]
    pub const fn set_prsact(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for St0Tconfb {
    #[inline(always)]
    fn default() -> St0Tconfb {
        St0Tconfb(0)
    }
}
impl core::fmt::Debug for St0Tconfb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("St0Tconfb")
            .field("comp", &self.comp())
            .field("mask", &self.mask())
            .field("nextstate", &self.nextstate())
            .field("setif", &self.setif())
            .field("prsact", &self.prsact())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for St0Tconfb {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "St0Tconfb {{ comp: {=u8:?}, mask: {=u8:?}, nextstate: {=u8:?}, setif: {=bool:?}, prsact: {=u8:?} }}" , self . comp () , self . mask () , self . nextstate () , self . setif () , self . prsact ())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Result Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdatav(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Result Data Valid."]
    #[inline(always)]
    pub const fn set_bufdatav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Result Buffer Half Full."]
    #[must_use]
    #[inline(always)]
    pub const fn bufhalffull(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Result Buffer Half Full."]
    #[inline(always)]
    pub const fn set_bufhalffull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Result Buffer Full."]
    #[must_use]
    #[inline(always)]
    pub const fn buffull(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Result Buffer Full."]
    #[inline(always)]
    pub const fn set_buffull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LESENSE Periodic Counter Running."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LESENSE Periodic Counter Running."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LESENSE Scan Active."]
    #[must_use]
    #[inline(always)]
    pub const fn scanactive(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LESENSE Scan Active."]
    #[inline(always)]
    pub const fn set_scanactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LESENSE VDAC Interface is Active."]
    #[must_use]
    #[inline(always)]
    pub const fn dacactive(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LESENSE VDAC Interface is Active."]
    #[inline(always)]
    pub const fn set_dacactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("bufdatav", &self.bufdatav())
            .field("bufhalffull", &self.bufhalffull())
            .field("buffull", &self.buffull())
            .field("running", &self.running())
            .field("scanactive", &self.scanactive())
            .field("dacactive", &self.dacactive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ bufdatav: {=bool:?}, bufhalffull: {=bool:?}, buffull: {=bool:?}, running: {=bool:?}, scanactive: {=bool:?}, dacactive: {=bool:?} }}" , self . bufdatav () , self . bufhalffull () , self . buffull () , self . running () , self . scanactive () , self . dacactive ())
    }
}
#[doc = "Synchronization Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc = "CMD Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CMD Register Busy."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syncbusy {{ cmd: {=bool:?} }}", self.cmd())
    }
}
#[doc = "Timing Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timctrl(pub u32);
impl Timctrl {
    #[doc = "Prescaling Factor for High Frequency Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn auxpresc(&self) -> super::vals::Auxpresc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Auxpresc::from_bits(val as u8)
    }
    #[doc = "Prescaling Factor for High Frequency Timer."]
    #[inline(always)]
    pub const fn set_auxpresc(&mut self, val: super::vals::Auxpresc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Prescaling Factor for Low Frequency Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn lfpresc(&self) -> super::vals::Lfpresc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Lfpresc::from_bits(val as u8)
    }
    #[doc = "Prescaling Factor for Low Frequency Timer."]
    #[inline(always)]
    pub const fn set_lfpresc(&mut self, val: super::vals::Lfpresc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Period Counter Prescaling."]
    #[must_use]
    #[inline(always)]
    pub const fn pcpresc(&self) -> super::vals::Pcpresc {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pcpresc::from_bits(val as u8)
    }
    #[doc = "Period Counter Prescaling."]
    #[inline(always)]
    pub const fn set_pcpresc(&mut self, val: super::vals::Pcpresc) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Period Counter Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pctop(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0xff;
        val as u8
    }
    #[doc = "Period Counter Top Value."]
    #[inline(always)]
    pub const fn set_pctop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
    }
    #[doc = "Start Delay Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn startdly(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Start Delay Configuration."]
    #[inline(always)]
    pub const fn set_startdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "AUXHFRCO Startup Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn auxstartup(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Startup Configuration."]
    #[inline(always)]
    pub const fn set_auxstartup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("auxpresc", &self.auxpresc())
            .field("lfpresc", &self.lfpresc())
            .field("pcpresc", &self.pcpresc())
            .field("pctop", &self.pctop())
            .field("startdly", &self.startdly())
            .field("auxstartup", &self.auxstartup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Timctrl {{ auxpresc: {:?}, lfpresc: {:?}, pcpresc: {:?}, pctop: {=u8:?}, startdly: {=u8:?}, auxstartup: {=bool:?} }}" , self . auxpresc () , self . lfpresc () , self . pcpresc () , self . pctop () , self . startdly () , self . auxstartup ())
    }
}
