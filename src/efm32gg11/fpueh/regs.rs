#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "FPIOC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpioc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FPIOC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FPDZC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpdzc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FPDZC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpdzc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FPUFC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpufc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FPUFC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpufc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FPOFC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpofc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FPOFC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpofc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FPIDC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpidc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FPIDC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpidc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FPIXC Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpixc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FPIXC Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fpixc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("fpioc", &self.fpioc())
            .field("fpdzc", &self.fpdzc())
            .field("fpufc", &self.fpufc())
            .field("fpofc", &self.fpofc())
            .field("fpidc", &self.fpidc())
            .field("fpixc", &self.fpixc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ fpioc: {=bool:?}, fpdzc: {=bool:?}, fpufc: {=bool:?}, fpofc: {=bool:?}, fpidc: {=bool:?}, fpixc: {=bool:?} }}" , self . fpioc () , self . fpdzc () , self . fpufc () , self . fpofc () , self . fpidc () , self . fpixc ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "FPU invalid operation."]
    #[must_use]
    #[inline(always)]
    pub const fn fpioc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FPU invalid operation."]
    #[inline(always)]
    pub const fn set_fpioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FPU divide-by-zero exception."]
    #[must_use]
    #[inline(always)]
    pub const fn fpdzc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FPU divide-by-zero exception."]
    #[inline(always)]
    pub const fn set_fpdzc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FPU underflow exception."]
    #[must_use]
    #[inline(always)]
    pub const fn fpufc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FPU underflow exception."]
    #[inline(always)]
    pub const fn set_fpufc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FPU overflow exception."]
    #[must_use]
    #[inline(always)]
    pub const fn fpofc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FPU overflow exception."]
    #[inline(always)]
    pub const fn set_fpofc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FPU input denormal exception."]
    #[must_use]
    #[inline(always)]
    pub const fn fpidc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FPU input denormal exception."]
    #[inline(always)]
    pub const fn set_fpidc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FPU inexact exception."]
    #[must_use]
    #[inline(always)]
    pub const fn fpixc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FPU inexact exception."]
    #[inline(always)]
    pub const fn set_fpixc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("fpioc", &self.fpioc())
            .field("fpdzc", &self.fpdzc())
            .field("fpufc", &self.fpufc())
            .field("fpofc", &self.fpofc())
            .field("fpidc", &self.fpidc())
            .field("fpixc", &self.fpixc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ fpioc: {=bool:?}, fpdzc: {=bool:?}, fpufc: {=bool:?}, fpofc: {=bool:?}, fpidc: {=bool:?}, fpixc: {=bool:?} }}" , self . fpioc () , self . fpdzc () , self . fpufc () , self . fpofc () , self . fpidc () , self . fpixc ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set FPIOC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpioc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPIOC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set FPDZC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpdzc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPDZC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpdzc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set FPUFC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpufc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPUFC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpufc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set FPOFC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpofc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPOFC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpofc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set FPIDC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpidc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPIDC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpidc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set FPIXC Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fpixc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set FPIXC Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fpixc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("fpioc", &self.fpioc())
            .field("fpdzc", &self.fpdzc())
            .field("fpufc", &self.fpufc())
            .field("fpofc", &self.fpofc())
            .field("fpidc", &self.fpidc())
            .field("fpixc", &self.fpixc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ fpioc: {=bool:?}, fpdzc: {=bool:?}, fpufc: {=bool:?}, fpofc: {=bool:?}, fpidc: {=bool:?}, fpixc: {=bool:?} }}" , self . fpioc () , self . fpdzc () , self . fpufc () , self . fpofc () , self . fpidc () , self . fpixc ())
    }
}
