#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "PPUPRIV Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ppupriv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PPUPRIV Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ppupriv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("ppupriv", &self.ppupriv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ien {{ ppupriv: {=bool:?} }}", self.ppupriv())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "PPU Privilege Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ppupriv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PPU Privilege Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ppupriv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("ppupriv", &self.ppupriv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If {{ ppupriv: {=bool:?} }}", self.ppupriv())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set PPUPRIV Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ppupriv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set PPUPRIV Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ppupriv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("ppupriv", &self.ppupriv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ifs {{ ppupriv: {=bool:?} }}", self.ppupriv())
    }
}
#[doc = "PPU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppuctrl(pub u32);
impl Ppuctrl {
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ppuctrl {
    #[inline(always)]
    fn default() -> Ppuctrl {
        Ppuctrl(0)
    }
}
impl core::fmt::Debug for Ppuctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppuctrl")
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppuctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ppuctrl {{ enable: {=bool:?} }}", self.enable())
    }
}
#[doc = "PPU Fault Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppufs(pub u32);
impl Ppufs {
    #[must_use]
    #[inline(always)]
    pub const fn periphid(&self) -> super::vals::Periphid {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Periphid::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_periphid(&mut self, val: super::vals::Periphid) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ppufs {
    #[inline(always)]
    fn default() -> Ppufs {
        Ppufs(0)
    }
}
impl core::fmt::Debug for Ppufs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppufs")
            .field("periphid", &self.periphid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppufs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ppufs {{ periphid: {:?} }}", self.periphid())
    }
}
#[doc = "PPU Privilege Access Type Descriptor 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppupatd0(pub u32);
impl Ppupatd0 {
    #[doc = "Analog Comparator 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 0 access control bit."]
    #[inline(always)]
    pub const fn set_acmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 1 access control bit."]
    #[inline(always)]
    pub const fn set_acmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Analog Comparator 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 1 access control bit."]
    #[inline(always)]
    pub const fn set_acmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Analog Comparator 3 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 3 access control bit."]
    #[inline(always)]
    pub const fn set_acmp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog to Digital Converter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Analog to Digital Converter 0 access control bit."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Analog to Digital Converter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Analog to Digital Converter 0 access control bit."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CAN 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn can0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CAN 0 access control bit."]
    #[inline(always)]
    pub const fn set_can0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CAN 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn can1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CAN 1 access control bit."]
    #[inline(always)]
    pub const fn set_can1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock Management Unit access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cmu(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Management Unit access control bit."]
    #[inline(always)]
    pub const fn set_cmu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CRYOTIMER access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cryotimer(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CRYOTIMER access control bit."]
    #[inline(always)]
    pub const fn set_cryotimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Advanced Encryption Standard Accelerator access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn crypto0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Advanced Encryption Standard Accelerator access control bit."]
    #[inline(always)]
    pub const fn set_crypto0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capacitive touch sense module access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn csen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capacitive touch sense module access control bit."]
    #[inline(always)]
    pub const fn set_csen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Digital to Analog Converter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn vdac0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Digital to Analog Converter 0 access control bit."]
    #[inline(always)]
    pub const fn set_vdac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Peripheral Reflex System access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn prs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral Reflex System access control bit."]
    #[inline(always)]
    pub const fn set_prs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External Bus Interface access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ebi(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External Bus Interface access control bit."]
    #[inline(always)]
    pub const fn set_ebi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Energy Management Unit access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn emu(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Management Unit access control bit."]
    #[inline(always)]
    pub const fn set_emu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Ethernet Controller access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn eth(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Controller access control bit."]
    #[inline(always)]
    pub const fn set_eth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FPU Exception Handler access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn fpueh(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Exception Handler access control bit."]
    #[inline(always)]
    pub const fn set_fpueh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "General Purpose CRC access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcrc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose CRC access control bit."]
    #[inline(always)]
    pub const fn set_gpcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "General purpose Input/Output access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "General purpose Input/Output access control bit."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "I2C 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 0 access control bit."]
    #[inline(always)]
    pub const fn set_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 1 access control bit."]
    #[inline(always)]
    pub const fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C 2 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 2 access control bit."]
    #[inline(always)]
    pub const fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Current Digital to Analog Converter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn idac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Current Digital to Analog Converter 0 access control bit."]
    #[inline(always)]
    pub const fn set_idac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory System Controller access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn msc(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Memory System Controller access control bit."]
    #[inline(always)]
    pub const fn set_msc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Liquid Crystal Display Controller access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Liquid Crystal Display Controller access control bit."]
    #[inline(always)]
    pub const fn set_lcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Linked Direct Memory Access Controller access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ldma(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Linked Direct Memory Access Controller access control bit."]
    #[inline(always)]
    pub const fn set_ldma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Low Energy Sensor Interface access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Sensor Interface access control bit."]
    #[inline(always)]
    pub const fn set_lesense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Low Energy Timer 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Timer 0 access control bit."]
    #[inline(always)]
    pub const fn set_letimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Low Energy Timer 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Timer 1 access control bit."]
    #[inline(always)]
    pub const fn set_letimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Low Energy UART 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy UART 0 access control bit."]
    #[inline(always)]
    pub const fn set_leuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Low Energy UART 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy UART 1 access control bit."]
    #[inline(always)]
    pub const fn set_leuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ppupatd0 {
    #[inline(always)]
    fn default() -> Ppupatd0 {
        Ppupatd0(0)
    }
}
impl core::fmt::Debug for Ppupatd0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppupatd0")
            .field("acmp0", &self.acmp0())
            .field("acmp1", &self.acmp1())
            .field("acmp2", &self.acmp2())
            .field("acmp3", &self.acmp3())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("can0", &self.can0())
            .field("can1", &self.can1())
            .field("cmu", &self.cmu())
            .field("cryotimer", &self.cryotimer())
            .field("crypto0", &self.crypto0())
            .field("csen", &self.csen())
            .field("vdac0", &self.vdac0())
            .field("prs", &self.prs())
            .field("ebi", &self.ebi())
            .field("emu", &self.emu())
            .field("eth", &self.eth())
            .field("fpueh", &self.fpueh())
            .field("gpcrc", &self.gpcrc())
            .field("gpio", &self.gpio())
            .field("i2c0", &self.i2c0())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("idac0", &self.idac0())
            .field("msc", &self.msc())
            .field("lcd", &self.lcd())
            .field("ldma", &self.ldma())
            .field("lesense", &self.lesense())
            .field("letimer0", &self.letimer0())
            .field("letimer1", &self.letimer1())
            .field("leuart0", &self.leuart0())
            .field("leuart1", &self.leuart1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppupatd0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ppupatd0 {{ acmp0: {=bool:?}, acmp1: {=bool:?}, acmp2: {=bool:?}, acmp3: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, can0: {=bool:?}, can1: {=bool:?}, cmu: {=bool:?}, cryotimer: {=bool:?}, crypto0: {=bool:?}, csen: {=bool:?}, vdac0: {=bool:?}, prs: {=bool:?}, ebi: {=bool:?}, emu: {=bool:?}, eth: {=bool:?}, fpueh: {=bool:?}, gpcrc: {=bool:?}, gpio: {=bool:?}, i2c0: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, idac0: {=bool:?}, msc: {=bool:?}, lcd: {=bool:?}, ldma: {=bool:?}, lesense: {=bool:?}, letimer0: {=bool:?}, letimer1: {=bool:?}, leuart0: {=bool:?}, leuart1: {=bool:?} }}" , self . acmp0 () , self . acmp1 () , self . acmp2 () , self . acmp3 () , self . adc0 () , self . adc1 () , self . can0 () , self . can1 () , self . cmu () , self . cryotimer () , self . crypto0 () , self . csen () , self . vdac0 () , self . prs () , self . ebi () , self . emu () , self . eth () , self . fpueh () , self . gpcrc () , self . gpio () , self . i2c0 () , self . i2c1 () , self . i2c2 () , self . idac0 () , self . msc () , self . lcd () , self . ldma () , self . lesense () , self . letimer0 () , self . letimer1 () , self . leuart0 () , self . leuart1 ())
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppupatd1(pub u32);
impl Ppupatd1 {
    #[doc = "Pulse Counter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pulse Counter 0 access control bit."]
    #[inline(always)]
    pub const fn set_pcnt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pulse Counter 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pulse Counter 1 access control bit."]
    #[inline(always)]
    pub const fn set_pcnt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pulse Counter 2 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pulse Counter 2 access control bit."]
    #[inline(always)]
    pub const fn set_pcnt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Quad-SPI access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn qspi0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Quad-SPI access control bit."]
    #[inline(always)]
    pub const fn set_qspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Management Unit access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn rmu(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Management Unit access control bit."]
    #[inline(always)]
    pub const fn set_rmu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Real-Time Counter access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Real-Time Counter access control bit."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Real-Time Counter and Calendar access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Real-Time Counter and Calendar access control bit."]
    #[inline(always)]
    pub const fn set_rtcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SDIO Controller access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Controller access control bit."]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Security Management Unit access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn smu(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Security Management Unit access control bit."]
    #[inline(always)]
    pub const fn set_smu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Timer 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 0 access control bit."]
    #[inline(always)]
    pub const fn set_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Timer 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 1 access control bit."]
    #[inline(always)]
    pub const fn set_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Timer 2 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 2 access control bit."]
    #[inline(always)]
    pub const fn set_timer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timer 3 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 3 access control bit."]
    #[inline(always)]
    pub const fn set_timer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Timer 4 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 4 access control bit."]
    #[inline(always)]
    pub const fn set_timer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Timer 5 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer5(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 5 access control bit."]
    #[inline(always)]
    pub const fn set_timer5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Timer 6 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn timer6(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 6 access control bit."]
    #[inline(always)]
    pub const fn set_timer6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "True Random Number Generator 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "True Random Number Generator 0 access control bit."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn uart0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0 access control bit."]
    #[inline(always)]
    pub const fn set_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1 access control bit."]
    #[inline(always)]
    pub const fn set_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit."]
    #[inline(always)]
    pub const fn set_usart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit."]
    #[inline(always)]
    pub const fn set_usart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit."]
    #[inline(always)]
    pub const fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit."]
    #[inline(always)]
    pub const fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit."]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit."]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Universal Serial Bus Interface access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Serial Bus Interface access control bit."]
    #[inline(always)]
    pub const fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog access control bit."]
    #[inline(always)]
    pub const fn set_wdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Watchdog access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog access control bit."]
    #[inline(always)]
    pub const fn set_wdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Wide Timer 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 0 access control bit."]
    #[inline(always)]
    pub const fn set_wtimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Wide Timer 0 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 0 access control bit."]
    #[inline(always)]
    pub const fn set_wtimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Wide Timer 2 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 2 access control bit."]
    #[inline(always)]
    pub const fn set_wtimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Wide Timer 3 access control bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 3 access control bit."]
    #[inline(always)]
    pub const fn set_wtimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ppupatd1 {
    #[inline(always)]
    fn default() -> Ppupatd1 {
        Ppupatd1(0)
    }
}
impl core::fmt::Debug for Ppupatd1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppupatd1")
            .field("pcnt0", &self.pcnt0())
            .field("pcnt1", &self.pcnt1())
            .field("pcnt2", &self.pcnt2())
            .field("qspi0", &self.qspi0())
            .field("rmu", &self.rmu())
            .field("rtc", &self.rtc())
            .field("rtcc", &self.rtcc())
            .field("sdio", &self.sdio())
            .field("smu", &self.smu())
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .field("timer2", &self.timer2())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("timer5", &self.timer5())
            .field("timer6", &self.timer6())
            .field("trng0", &self.trng0())
            .field("uart0", &self.uart0())
            .field("uart1", &self.uart1())
            .field("usart0", &self.usart0())
            .field("usart1", &self.usart1())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("usb", &self.usb())
            .field("wdog0", &self.wdog0())
            .field("wdog1", &self.wdog1())
            .field("wtimer0", &self.wtimer0())
            .field("wtimer1", &self.wtimer1())
            .field("wtimer2", &self.wtimer2())
            .field("wtimer3", &self.wtimer3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppupatd1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ppupatd1 {{ pcnt0: {=bool:?}, pcnt1: {=bool:?}, pcnt2: {=bool:?}, qspi0: {=bool:?}, rmu: {=bool:?}, rtc: {=bool:?}, rtcc: {=bool:?}, sdio: {=bool:?}, smu: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, timer5: {=bool:?}, timer6: {=bool:?}, trng0: {=bool:?}, uart0: {=bool:?}, uart1: {=bool:?}, usart0: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, usart3: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, usb: {=bool:?}, wdog0: {=bool:?}, wdog1: {=bool:?}, wtimer0: {=bool:?}, wtimer1: {=bool:?}, wtimer2: {=bool:?}, wtimer3: {=bool:?} }}" , self . pcnt0 () , self . pcnt1 () , self . pcnt2 () , self . qspi0 () , self . rmu () , self . rtc () , self . rtcc () , self . sdio () , self . smu () , self . timer0 () , self . timer1 () , self . timer2 () , self . timer3 () , self . timer4 () , self . timer5 () , self . timer6 () , self . trng0 () , self . uart0 () , self . uart1 () , self . usart0 () , self . usart1 () , self . usart2 () , self . usart3 () , self . usart4 () , self . usart5 () , self . usb () , self . wdog0 () , self . wdog1 () , self . wtimer0 () , self . wtimer1 () , self . wtimer2 () , self . wtimer3 ())
    }
}
