#[doc = "Port Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Drive Strength for Port."]
    #[must_use]
    #[inline(always)]
    pub const fn drive_strength(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Strength for Port."]
    #[inline(always)]
    pub const fn set_drive_strength(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Slewrate Limit for Port."]
    #[must_use]
    #[inline(always)]
    pub const fn slew_rate(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Slewrate Limit for Port."]
    #[inline(always)]
    pub const fn set_slew_rate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Data in Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn din_dis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Data in Disable."]
    #[inline(always)]
    pub const fn set_din_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Alternate Drive Strength for Port."]
    #[must_use]
    #[inline(always)]
    pub const fn drive_strength_alt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Drive Strength for Port."]
    #[inline(always)]
    pub const fn set_drive_strength_alt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Alternate Slewrate Limit for Port."]
    #[must_use]
    #[inline(always)]
    pub const fn slew_rate_alt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Alternate Slewrate Limit for Port."]
    #[inline(always)]
    pub const fn set_slew_rate_alt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Alternate Data in Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn din_dis_alt(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Data in Disable."]
    #[inline(always)]
    pub const fn set_din_dis_alt(&mut self, val: bool) {
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
            .field("drive_strength", &self.drive_strength())
            .field("slew_rate", &self.slew_rate())
            .field("din_dis", &self.din_dis())
            .field("drive_strength_alt", &self.drive_strength_alt())
            .field("slew_rate_alt", &self.slew_rate_alt())
            .field("din_dis_alt", &self.din_dis_alt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ drive_strength: {=bool:?}, slew_rate: {=u8:?}, din_dis: {=bool:?}, drive_strength_alt: {=bool:?}, slew_rate_alt: {=u8:?}, din_dis_alt: {=bool:?} }}" , self . drive_strength () , self . slew_rate () , self . din_dis () , self . drive_strength_alt () , self . slew_rate_alt () , self . din_dis_alt ())
    }
}
#[doc = "Port Data in Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Din(pub u32);
impl Din {
    #[doc = "Data in."]
    #[must_use]
    #[inline(always)]
    pub const fn din(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data in."]
    #[inline(always)]
    pub const fn set_din(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Din {
    #[inline(always)]
    fn default() -> Din {
        Din(0)
    }
}
impl core::fmt::Debug for Din {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Din").field("din", &self.din()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Din {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Din {{ din: {=u16:?} }}", self.din())
    }
}
#[doc = "Port Data Out Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dout(pub u32);
impl Dout {
    #[doc = "Data Out."]
    #[must_use]
    #[inline(always)]
    pub const fn dout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Out."]
    #[inline(always)]
    pub const fn set_dout(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dout {
    #[inline(always)]
    fn default() -> Dout {
        Dout(0)
    }
}
impl core::fmt::Debug for Dout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dout").field("dout", &self.dout()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dout {{ dout: {=u16:?} }}", self.dout())
    }
}
#[doc = "Port Data Out Toggle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Douttgl(pub u32);
impl Douttgl {
    #[doc = "Data Out Toggle."]
    #[must_use]
    #[inline(always)]
    pub const fn douttgl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Out Toggle."]
    #[inline(always)]
    pub const fn set_douttgl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Douttgl {
    #[inline(always)]
    fn default() -> Douttgl {
        Douttgl(0)
    }
}
impl core::fmt::Debug for Douttgl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Douttgl")
            .field("douttgl", &self.douttgl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Douttgl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Douttgl {{ douttgl: {=u16:?} }}", self.douttgl())
    }
}
#[doc = "EM4 Wake Up Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em4wuen(pub u32);
impl Em4wuen {
    #[doc = "EM4 Wake Up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wuen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "EM4 Wake Up Enable."]
    #[inline(always)]
    pub const fn set_em4wuen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Em4wuen {
    #[inline(always)]
    fn default() -> Em4wuen {
        Em4wuen(0)
    }
}
impl core::fmt::Debug for Em4wuen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Em4wuen")
            .field("em4wuen", &self.em4wuen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em4wuen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Em4wuen {{ em4wuen: {=u16:?} }}", self.em4wuen())
    }
}
#[doc = "External Interrupt Falling Edge Trigger Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extifall(pub u32);
impl Extifall {
    #[doc = "External Interrupt N Falling Edge Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn extifall(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "External Interrupt N Falling Edge Trigger Enable."]
    #[inline(always)]
    pub const fn set_extifall(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Extifall {
    #[inline(always)]
    fn default() -> Extifall {
        Extifall(0)
    }
}
impl core::fmt::Debug for Extifall {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extifall")
            .field("extifall", &self.extifall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extifall {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Extifall {{ extifall: {=u16:?} }}", self.extifall())
    }
}
#[doc = "External Interrupt Level Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extilevel(pub u32);
impl Extilevel {
    #[doc = "EM4 Wake Up Level for EM4WU0 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU0 Pin."]
    #[inline(always)]
    pub const fn set_em4wu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "EM4 Wake Up Level for EM4WU1 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU1 Pin."]
    #[inline(always)]
    pub const fn set_em4wu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EM4 Wake Up Level for EM4WU4 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU4 Pin."]
    #[inline(always)]
    pub const fn set_em4wu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "EM4 Wake Up Level for EM4WU8 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU8 Pin."]
    #[inline(always)]
    pub const fn set_em4wu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "EM4 Wake Up Level for EM4WU9 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU9 Pin."]
    #[inline(always)]
    pub const fn set_em4wu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "EM4 Wake Up Level for EM4WU12 Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake Up Level for EM4WU12 Pin."]
    #[inline(always)]
    pub const fn set_em4wu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Extilevel {
    #[inline(always)]
    fn default() -> Extilevel {
        Extilevel(0)
    }
}
impl core::fmt::Debug for Extilevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extilevel")
            .field("em4wu0", &self.em4wu0())
            .field("em4wu1", &self.em4wu1())
            .field("em4wu4", &self.em4wu4())
            .field("em4wu8", &self.em4wu8())
            .field("em4wu9", &self.em4wu9())
            .field("em4wu12", &self.em4wu12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extilevel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Extilevel {{ em4wu0: {=bool:?}, em4wu1: {=bool:?}, em4wu4: {=bool:?}, em4wu8: {=bool:?}, em4wu9: {=bool:?}, em4wu12: {=bool:?} }}" , self . em4wu0 () , self . em4wu1 () , self . em4wu4 () , self . em4wu8 () , self . em4wu9 () , self . em4wu12 ())
    }
}
#[doc = "External Interrupt Pin Select High Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extipinselh(pub u32);
impl Extipinselh {
    #[doc = "External Interrupt 8 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel8(&self) -> super::vals::Extipinsel8 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Extipinsel8::from_bits(val as u8)
    }
    #[doc = "External Interrupt 8 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel8(&mut self, val: super::vals::Extipinsel8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "External Interrupt 9 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel9(&self) -> super::vals::Extipinsel9 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Extipinsel9::from_bits(val as u8)
    }
    #[doc = "External Interrupt 9 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel9(&mut self, val: super::vals::Extipinsel9) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Interrupt 10 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel10(&self) -> super::vals::Extipinsel10 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Extipinsel10::from_bits(val as u8)
    }
    #[doc = "External Interrupt 10 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel10(&mut self, val: super::vals::Extipinsel10) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Interrupt 11 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel11(&self) -> super::vals::Extipinsel11 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Extipinsel11::from_bits(val as u8)
    }
    #[doc = "External Interrupt 11 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel11(&mut self, val: super::vals::Extipinsel11) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "External Interrupt 12 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel12(&self) -> super::vals::Extipinsel12 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Extipinsel12::from_bits(val as u8)
    }
    #[doc = "External Interrupt 12 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel12(&mut self, val: super::vals::Extipinsel12) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "External Interrupt 13 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel13(&self) -> super::vals::Extipinsel13 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Extipinsel13::from_bits(val as u8)
    }
    #[doc = "External Interrupt 13 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel13(&mut self, val: super::vals::Extipinsel13) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "External Interrupt 14 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel14(&self) -> super::vals::Extipinsel14 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Extipinsel14::from_bits(val as u8)
    }
    #[doc = "External Interrupt 14 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel14(&mut self, val: super::vals::Extipinsel14) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "External Interrupt 15 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel15(&self) -> super::vals::Extipinsel15 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Extipinsel15::from_bits(val as u8)
    }
    #[doc = "External Interrupt 15 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel15(&mut self, val: super::vals::Extipinsel15) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Extipinselh {
    #[inline(always)]
    fn default() -> Extipinselh {
        Extipinselh(0)
    }
}
impl core::fmt::Debug for Extipinselh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extipinselh")
            .field("extipinsel8", &self.extipinsel8())
            .field("extipinsel9", &self.extipinsel9())
            .field("extipinsel10", &self.extipinsel10())
            .field("extipinsel11", &self.extipinsel11())
            .field("extipinsel12", &self.extipinsel12())
            .field("extipinsel13", &self.extipinsel13())
            .field("extipinsel14", &self.extipinsel14())
            .field("extipinsel15", &self.extipinsel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extipinselh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Extipinselh {{ extipinsel8: {:?}, extipinsel9: {:?}, extipinsel10: {:?}, extipinsel11: {:?}, extipinsel12: {:?}, extipinsel13: {:?}, extipinsel14: {:?}, extipinsel15: {:?} }}" , self . extipinsel8 () , self . extipinsel9 () , self . extipinsel10 () , self . extipinsel11 () , self . extipinsel12 () , self . extipinsel13 () , self . extipinsel14 () , self . extipinsel15 ())
    }
}
#[doc = "External Interrupt Pin Select Low Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extipinsell(pub u32);
impl Extipinsell {
    #[doc = "External Interrupt 0 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel0(&self) -> super::vals::Extipinsel0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Extipinsel0::from_bits(val as u8)
    }
    #[doc = "External Interrupt 0 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel0(&mut self, val: super::vals::Extipinsel0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "External Interrupt 1 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel1(&self) -> super::vals::Extipinsel1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Extipinsel1::from_bits(val as u8)
    }
    #[doc = "External Interrupt 1 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel1(&mut self, val: super::vals::Extipinsel1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Interrupt 2 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel2(&self) -> super::vals::Extipinsel2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Extipinsel2::from_bits(val as u8)
    }
    #[doc = "External Interrupt 2 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel2(&mut self, val: super::vals::Extipinsel2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Interrupt 3 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel3(&self) -> super::vals::Extipinsel3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Extipinsel3::from_bits(val as u8)
    }
    #[doc = "External Interrupt 3 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel3(&mut self, val: super::vals::Extipinsel3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "External Interrupt 4 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel4(&self) -> super::vals::Extipinsel4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Extipinsel4::from_bits(val as u8)
    }
    #[doc = "External Interrupt 4 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel4(&mut self, val: super::vals::Extipinsel4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "External Interrupt 5 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel5(&self) -> super::vals::Extipinsel5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Extipinsel5::from_bits(val as u8)
    }
    #[doc = "External Interrupt 5 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel5(&mut self, val: super::vals::Extipinsel5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "External Interrupt 6 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel6(&self) -> super::vals::Extipinsel6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Extipinsel6::from_bits(val as u8)
    }
    #[doc = "External Interrupt 6 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel6(&mut self, val: super::vals::Extipinsel6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "External Interrupt 7 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipinsel7(&self) -> super::vals::Extipinsel7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Extipinsel7::from_bits(val as u8)
    }
    #[doc = "External Interrupt 7 Pin Select."]
    #[inline(always)]
    pub const fn set_extipinsel7(&mut self, val: super::vals::Extipinsel7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Extipinsell {
    #[inline(always)]
    fn default() -> Extipinsell {
        Extipinsell(0)
    }
}
impl core::fmt::Debug for Extipinsell {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extipinsell")
            .field("extipinsel0", &self.extipinsel0())
            .field("extipinsel1", &self.extipinsel1())
            .field("extipinsel2", &self.extipinsel2())
            .field("extipinsel3", &self.extipinsel3())
            .field("extipinsel4", &self.extipinsel4())
            .field("extipinsel5", &self.extipinsel5())
            .field("extipinsel6", &self.extipinsel6())
            .field("extipinsel7", &self.extipinsel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extipinsell {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Extipinsell {{ extipinsel0: {:?}, extipinsel1: {:?}, extipinsel2: {:?}, extipinsel3: {:?}, extipinsel4: {:?}, extipinsel5: {:?}, extipinsel6: {:?}, extipinsel7: {:?} }}" , self . extipinsel0 () , self . extipinsel1 () , self . extipinsel2 () , self . extipinsel3 () , self . extipinsel4 () , self . extipinsel5 () , self . extipinsel6 () , self . extipinsel7 ())
    }
}
#[doc = "External Interrupt Port Select High Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extipselh(pub u32);
impl Extipselh {
    #[doc = "External Interrupt 8 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel8(&self) -> super::vals::Extipsel8 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Extipsel8::from_bits(val as u8)
    }
    #[doc = "External Interrupt 8 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel8(&mut self, val: super::vals::Extipsel8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "External Interrupt 9 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel9(&self) -> super::vals::Extipsel9 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Extipsel9::from_bits(val as u8)
    }
    #[doc = "External Interrupt 9 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel9(&mut self, val: super::vals::Extipsel9) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "External Interrupt 10 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel10(&self) -> super::vals::Extipsel10 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Extipsel10::from_bits(val as u8)
    }
    #[doc = "External Interrupt 10 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel10(&mut self, val: super::vals::Extipsel10) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "External Interrupt 11 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel11(&self) -> super::vals::Extipsel11 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Extipsel11::from_bits(val as u8)
    }
    #[doc = "External Interrupt 11 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel11(&mut self, val: super::vals::Extipsel11) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "External Interrupt 12 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel12(&self) -> super::vals::Extipsel12 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Extipsel12::from_bits(val as u8)
    }
    #[doc = "External Interrupt 12 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel12(&mut self, val: super::vals::Extipsel12) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "External Interrupt 13 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel13(&self) -> super::vals::Extipsel13 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Extipsel13::from_bits(val as u8)
    }
    #[doc = "External Interrupt 13 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel13(&mut self, val: super::vals::Extipsel13) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "External Interrupt 14 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel14(&self) -> super::vals::Extipsel14 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Extipsel14::from_bits(val as u8)
    }
    #[doc = "External Interrupt 14 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel14(&mut self, val: super::vals::Extipsel14) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "External Interrupt 15 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel15(&self) -> super::vals::Extipsel15 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Extipsel15::from_bits(val as u8)
    }
    #[doc = "External Interrupt 15 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel15(&mut self, val: super::vals::Extipsel15) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Extipselh {
    #[inline(always)]
    fn default() -> Extipselh {
        Extipselh(0)
    }
}
impl core::fmt::Debug for Extipselh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extipselh")
            .field("extipsel8", &self.extipsel8())
            .field("extipsel9", &self.extipsel9())
            .field("extipsel10", &self.extipsel10())
            .field("extipsel11", &self.extipsel11())
            .field("extipsel12", &self.extipsel12())
            .field("extipsel13", &self.extipsel13())
            .field("extipsel14", &self.extipsel14())
            .field("extipsel15", &self.extipsel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extipselh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Extipselh {{ extipsel8: {:?}, extipsel9: {:?}, extipsel10: {:?}, extipsel11: {:?}, extipsel12: {:?}, extipsel13: {:?}, extipsel14: {:?}, extipsel15: {:?} }}" , self . extipsel8 () , self . extipsel9 () , self . extipsel10 () , self . extipsel11 () , self . extipsel12 () , self . extipsel13 () , self . extipsel14 () , self . extipsel15 ())
    }
}
#[doc = "External Interrupt Port Select Low Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extipsell(pub u32);
impl Extipsell {
    #[doc = "External Interrupt 0 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel0(&self) -> super::vals::Extipsel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Extipsel0::from_bits(val as u8)
    }
    #[doc = "External Interrupt 0 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel0(&mut self, val: super::vals::Extipsel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "External Interrupt 1 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel1(&self) -> super::vals::Extipsel1 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Extipsel1::from_bits(val as u8)
    }
    #[doc = "External Interrupt 1 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel1(&mut self, val: super::vals::Extipsel1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "External Interrupt 2 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel2(&self) -> super::vals::Extipsel2 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Extipsel2::from_bits(val as u8)
    }
    #[doc = "External Interrupt 2 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel2(&mut self, val: super::vals::Extipsel2) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "External Interrupt 3 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel3(&self) -> super::vals::Extipsel3 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Extipsel3::from_bits(val as u8)
    }
    #[doc = "External Interrupt 3 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel3(&mut self, val: super::vals::Extipsel3) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "External Interrupt 4 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel4(&self) -> super::vals::Extipsel4 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Extipsel4::from_bits(val as u8)
    }
    #[doc = "External Interrupt 4 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel4(&mut self, val: super::vals::Extipsel4) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "External Interrupt 5 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel5(&self) -> super::vals::Extipsel5 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Extipsel5::from_bits(val as u8)
    }
    #[doc = "External Interrupt 5 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel5(&mut self, val: super::vals::Extipsel5) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "External Interrupt 6 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel6(&self) -> super::vals::Extipsel6 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Extipsel6::from_bits(val as u8)
    }
    #[doc = "External Interrupt 6 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel6(&mut self, val: super::vals::Extipsel6) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "External Interrupt 7 Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn extipsel7(&self) -> super::vals::Extipsel7 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Extipsel7::from_bits(val as u8)
    }
    #[doc = "External Interrupt 7 Port Select."]
    #[inline(always)]
    pub const fn set_extipsel7(&mut self, val: super::vals::Extipsel7) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Extipsell {
    #[inline(always)]
    fn default() -> Extipsell {
        Extipsell(0)
    }
}
impl core::fmt::Debug for Extipsell {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extipsell")
            .field("extipsel0", &self.extipsel0())
            .field("extipsel1", &self.extipsel1())
            .field("extipsel2", &self.extipsel2())
            .field("extipsel3", &self.extipsel3())
            .field("extipsel4", &self.extipsel4())
            .field("extipsel5", &self.extipsel5())
            .field("extipsel6", &self.extipsel6())
            .field("extipsel7", &self.extipsel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extipsell {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Extipsell {{ extipsel0: {:?}, extipsel1: {:?}, extipsel2: {:?}, extipsel3: {:?}, extipsel4: {:?}, extipsel5: {:?}, extipsel6: {:?}, extipsel7: {:?} }}" , self . extipsel0 () , self . extipsel1 () , self . extipsel2 () , self . extipsel3 () , self . extipsel4 () , self . extipsel5 () , self . extipsel6 () , self . extipsel7 ())
    }
}
#[doc = "External Interrupt Rising Edge Trigger Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extirise(pub u32);
impl Extirise {
    #[doc = "External Interrupt N Rising Edge Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn extirise(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "External Interrupt N Rising Edge Trigger Enable."]
    #[inline(always)]
    pub const fn set_extirise(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Extirise {
    #[inline(always)]
    fn default() -> Extirise {
        Extirise(0)
    }
}
impl core::fmt::Debug for Extirise {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extirise")
            .field("extirise", &self.extirise())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extirise {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Extirise {{ extirise: {=u16:?} }}", self.extirise())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "EXT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "EXT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "EM4WU Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "EM4WU Interrupt Enable."]
    #[inline(always)]
    pub const fn set_em4wu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("ext", &self.ext())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ ext: {=u16:?}, em4wu: {=u16:?} }}",
            self.ext(),
            self.em4wu()
        )
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "External Pin Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "External Pin Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "EM4 Wake Up Pin Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "EM4 Wake Up Pin Interrupt Flag."]
    #[inline(always)]
    pub const fn set_em4wu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("ext", &self.ext())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "If {{ ext: {=u16:?}, em4wu: {=u16:?} }}",
            self.ext(),
            self.em4wu()
        )
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set EXT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set EXT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Set EM4WU Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Set EM4WU Interrupt Flag."]
    #[inline(always)]
    pub const fn set_em4wu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("ext", &self.ext())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ ext: {=u16:?}, em4wu: {=u16:?} }}",
            self.ext(),
            self.em4wu()
        )
    }
}
#[doc = "Input Sense Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Insense(pub u32);
impl Insense {
    #[doc = "Interrupt Sense Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Sense Enable."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EM4WU Interrupt Sense Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EM4WU Interrupt Sense Enable."]
    #[inline(always)]
    pub const fn set_em4wu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Insense {
    #[inline(always)]
    fn default() -> Insense {
        Insense(0)
    }
}
impl core::fmt::Debug for Insense {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Insense")
            .field("int", &self.int())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Insense {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Insense {{ int: {=bool:?}, em4wu: {=bool:?} }}",
            self.int(),
            self.em4wu()
        )
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
#[doc = "Port Pin Mode High Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modeh(pub u32);
impl Modeh {
    #[doc = "Pin 8 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode8(&self) -> super::vals::PaModehMode8 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PaModehMode8::from_bits(val as u8)
    }
    #[doc = "Pin 8 Mode."]
    #[inline(always)]
    pub const fn set_mode8(&mut self, val: super::vals::PaModehMode8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pin 9 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode9(&self) -> super::vals::PaModehMode9 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::PaModehMode9::from_bits(val as u8)
    }
    #[doc = "Pin 9 Mode."]
    #[inline(always)]
    pub const fn set_mode9(&mut self, val: super::vals::PaModehMode9) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Pin 10 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode10(&self) -> super::vals::PaModehMode10 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::PaModehMode10::from_bits(val as u8)
    }
    #[doc = "Pin 10 Mode."]
    #[inline(always)]
    pub const fn set_mode10(&mut self, val: super::vals::PaModehMode10) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Pin 11 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode11(&self) -> super::vals::PaModehMode11 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::PaModehMode11::from_bits(val as u8)
    }
    #[doc = "Pin 11 Mode."]
    #[inline(always)]
    pub const fn set_mode11(&mut self, val: super::vals::PaModehMode11) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Pin 12 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode12(&self) -> super::vals::PaModehMode12 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::PaModehMode12::from_bits(val as u8)
    }
    #[doc = "Pin 12 Mode."]
    #[inline(always)]
    pub const fn set_mode12(&mut self, val: super::vals::PaModehMode12) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Pin 13 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode13(&self) -> super::vals::PaModehMode13 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::PaModehMode13::from_bits(val as u8)
    }
    #[doc = "Pin 13 Mode."]
    #[inline(always)]
    pub const fn set_mode13(&mut self, val: super::vals::PaModehMode13) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Pin 14 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode14(&self) -> super::vals::PaModehMode14 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::PaModehMode14::from_bits(val as u8)
    }
    #[doc = "Pin 14 Mode."]
    #[inline(always)]
    pub const fn set_mode14(&mut self, val: super::vals::PaModehMode14) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Pin 15 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode15(&self) -> super::vals::PaModehMode15 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::PaModehMode15::from_bits(val as u8)
    }
    #[doc = "Pin 15 Mode."]
    #[inline(always)]
    pub const fn set_mode15(&mut self, val: super::vals::PaModehMode15) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Modeh {
    #[inline(always)]
    fn default() -> Modeh {
        Modeh(0)
    }
}
impl core::fmt::Debug for Modeh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modeh")
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modeh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Modeh {{ mode8: {:?}, mode9: {:?}, mode10: {:?}, mode11: {:?}, mode12: {:?}, mode13: {:?}, mode14: {:?}, mode15: {:?} }}" , self . mode8 () , self . mode9 () , self . mode10 () , self . mode11 () , self . mode12 () , self . mode13 () , self . mode14 () , self . mode15 ())
    }
}
#[doc = "Port Pin Mode Low Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Model(pub u32);
impl Model {
    #[doc = "Pin 0 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode0(&self) -> super::vals::PaModelMode0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PaModelMode0::from_bits(val as u8)
    }
    #[doc = "Pin 0 Mode."]
    #[inline(always)]
    pub const fn set_mode0(&mut self, val: super::vals::PaModelMode0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pin 1 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode1(&self) -> super::vals::PaModelMode1 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::PaModelMode1::from_bits(val as u8)
    }
    #[doc = "Pin 1 Mode."]
    #[inline(always)]
    pub const fn set_mode1(&mut self, val: super::vals::PaModelMode1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Pin 2 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode2(&self) -> super::vals::PaModelMode2 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::PaModelMode2::from_bits(val as u8)
    }
    #[doc = "Pin 2 Mode."]
    #[inline(always)]
    pub const fn set_mode2(&mut self, val: super::vals::PaModelMode2) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Pin 3 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode3(&self) -> super::vals::PaModelMode3 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::PaModelMode3::from_bits(val as u8)
    }
    #[doc = "Pin 3 Mode."]
    #[inline(always)]
    pub const fn set_mode3(&mut self, val: super::vals::PaModelMode3) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Pin 4 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode4(&self) -> super::vals::PaModelMode4 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::PaModelMode4::from_bits(val as u8)
    }
    #[doc = "Pin 4 Mode."]
    #[inline(always)]
    pub const fn set_mode4(&mut self, val: super::vals::PaModelMode4) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Pin 5 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode5(&self) -> super::vals::PaModelMode5 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::PaModelMode5::from_bits(val as u8)
    }
    #[doc = "Pin 5 Mode."]
    #[inline(always)]
    pub const fn set_mode5(&mut self, val: super::vals::PaModelMode5) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Pin 6 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode6(&self) -> super::vals::PaModelMode6 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::PaModelMode6::from_bits(val as u8)
    }
    #[doc = "Pin 6 Mode."]
    #[inline(always)]
    pub const fn set_mode6(&mut self, val: super::vals::PaModelMode6) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Pin 7 Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode7(&self) -> super::vals::PaModelMode7 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::PaModelMode7::from_bits(val as u8)
    }
    #[doc = "Pin 7 Mode."]
    #[inline(always)]
    pub const fn set_mode7(&mut self, val: super::vals::PaModelMode7) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Model {
    #[inline(always)]
    fn default() -> Model {
        Model(0)
    }
}
impl core::fmt::Debug for Model {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Model")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Model {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Model {{ mode0: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, mode4: {:?}, mode5: {:?}, mode6: {:?}, mode7: {:?} }}" , self . mode0 () , self . mode1 () , self . mode2 () , self . mode3 () , self . mode4 () , self . mode5 () , self . mode6 () , self . mode7 ())
    }
}
#[doc = "Over Voltage Disable for All Modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ovtdis(pub u32);
impl Ovtdis {
    #[doc = "Disable Over Voltage Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn ovtdis(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Disable Over Voltage Capability."]
    #[inline(always)]
    pub const fn set_ovtdis(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ovtdis {
    #[inline(always)]
    fn default() -> Ovtdis {
        Ovtdis(0)
    }
}
impl core::fmt::Debug for Ovtdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ovtdis")
            .field("ovtdis", &self.ovtdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ovtdis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ovtdis {{ ovtdis: {=u16:?} }}", self.ovtdis())
    }
}
#[doc = "Port Unlocked Pins Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinlockn(pub u32);
impl Pinlockn {
    #[doc = "Unlocked Pins."]
    #[must_use]
    #[inline(always)]
    pub const fn pinlockn(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Unlocked Pins."]
    #[inline(always)]
    pub const fn set_pinlockn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pinlockn {
    #[inline(always)]
    fn default() -> Pinlockn {
        Pinlockn(0)
    }
}
impl core::fmt::Debug for Pinlockn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinlockn")
            .field("pinlockn", &self.pinlockn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinlockn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinlockn {{ pinlockn: {=u16:?} }}", self.pinlockn())
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
    pub const fn swvloc(&self) -> super::vals::Swvloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Swvloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_swvloc(&mut self, val: super::vals::Swvloc) {
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
            .field("swvloc", &self.swvloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routeloc0 {{ swvloc: {:?} }}", self.swvloc())
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "Serial Wire Clock and JTAG Test Clock Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swclktckpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Wire Clock and JTAG Test Clock Pin Enable."]
    #[inline(always)]
    pub const fn set_swclktckpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Serial Wire Data and JTAG Test Mode Select Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swdiotmspen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Wire Data and JTAG Test Mode Select Pin Enable."]
    #[inline(always)]
    pub const fn set_swdiotmspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "JTAG Test Debug Output Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdopen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG Test Debug Output Pin Enable."]
    #[inline(always)]
    pub const fn set_tdopen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "JTAG Test Debug Input Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdipen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG Test Debug Input Pin Enable."]
    #[inline(always)]
    pub const fn set_tdipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Serial Wire Viewer Output Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swvpen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Wire Viewer Output Pin Enable."]
    #[inline(always)]
    pub const fn set_swvpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("swclktckpen", &self.swclktckpen())
            .field("swdiotmspen", &self.swdiotmspen())
            .field("tdopen", &self.tdopen())
            .field("tdipen", &self.tdipen())
            .field("swvpen", &self.swvpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ swclktckpen: {=bool:?}, swdiotmspen: {=bool:?}, tdopen: {=bool:?}, tdipen: {=bool:?}, swvpen: {=bool:?} }}" , self . swclktckpen () , self . swdiotmspen () , self . tdopen () , self . tdipen () , self . swvpen ())
    }
}
