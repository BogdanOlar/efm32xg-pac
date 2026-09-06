#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Initialization Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization Enable."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: bool) {
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
        f.debug_struct("Cmd").field("init", &self.init()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ init: {=bool:?} }}", self.init())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "CRC Functionality Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Functionality Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polynomial Select."]
    #[must_use]
    #[inline(always)]
    pub const fn polysel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Polynomial Select."]
    #[inline(always)]
    pub const fn set_polysel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Byte Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bytemode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Mode Enable."]
    #[inline(always)]
    pub const fn set_bytemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Byte-level Bit Reverse Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bitreverse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Byte-level Bit Reverse Enable."]
    #[inline(always)]
    pub const fn set_bitreverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Byte Reverse Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bytereverse(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Reverse Mode."]
    #[inline(always)]
    pub const fn set_bytereverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Auto Init Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoinit(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Init Enable."]
    #[inline(always)]
    pub const fn set_autoinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("polysel", &self.polysel())
            .field("bytemode", &self.bytemode())
            .field("bitreverse", &self.bitreverse())
            .field("bytereverse", &self.bytereverse())
            .field("autoinit", &self.autoinit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, polysel: {=bool:?}, bytemode: {=bool:?}, bitreverse: {=bool:?}, bytereverse: {=bool:?}, autoinit: {=bool:?} }}" , self . en () , self . polysel () , self . bytemode () , self . bitreverse () , self . bytereverse () , self . autoinit ())
    }
}
#[doc = "Input 8-bit Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputdatabyte(pub u32);
impl Inputdatabyte {
    #[doc = "Input Data for 8-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inputdatabyte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Data for 8-bit."]
    #[inline(always)]
    pub const fn set_inputdatabyte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Inputdatabyte {
    #[inline(always)]
    fn default() -> Inputdatabyte {
        Inputdatabyte(0)
    }
}
impl core::fmt::Debug for Inputdatabyte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inputdatabyte")
            .field("inputdatabyte", &self.inputdatabyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inputdatabyte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inputdatabyte {{ inputdatabyte: {=u8:?} }}",
            self.inputdatabyte()
        )
    }
}
#[doc = "Input 16-bit Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputdatahword(pub u32);
impl Inputdatahword {
    #[doc = "Input Data for 16-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inputdatahword(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Input Data for 16-bit."]
    #[inline(always)]
    pub const fn set_inputdatahword(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Inputdatahword {
    #[inline(always)]
    fn default() -> Inputdatahword {
        Inputdatahword(0)
    }
}
impl core::fmt::Debug for Inputdatahword {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inputdatahword")
            .field("inputdatahword", &self.inputdatahword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inputdatahword {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inputdatahword {{ inputdatahword: {=u16:?} }}",
            self.inputdatahword()
        )
    }
}
#[doc = "CRC Polynomial Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poly(pub u32);
impl Poly {
    #[doc = "CRC Polynomial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn poly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CRC Polynomial Value."]
    #[inline(always)]
    pub const fn set_poly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Poly {
    #[inline(always)]
    fn default() -> Poly {
        Poly(0)
    }
}
impl core::fmt::Debug for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Poly").field("poly", &self.poly()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Poly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Poly {{ poly: {=u16:?} }}", self.poly())
    }
}
