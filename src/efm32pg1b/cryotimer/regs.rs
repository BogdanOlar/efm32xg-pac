#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable CRYOTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable CRYOTIMER."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select Low Frequency Oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn oscsel(&self) -> super::vals::Oscsel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Oscsel::from_bits(val as u8)
    }
    #[doc = "Select Low Frequency Oscillator."]
    #[inline(always)]
    pub const fn set_oscsel(&mut self, val: super::vals::Oscsel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Prescaler Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::Presc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Presc::from_bits(val as u8)
    }
    #[doc = "Prescaler Setting."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::Presc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
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
            .field("debugrun", &self.debugrun())
            .field("oscsel", &self.oscsel())
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ en: {=bool:?}, debugrun: {=bool:?}, oscsel: {:?}, presc: {:?} }}",
            self.en(),
            self.debugrun(),
            self.oscsel(),
            self.presc()
        )
    }
}
#[doc = "Wake Up Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em4wuen(pub u32);
impl Em4wuen {
    #[doc = "EM4 Wake-up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Wake-up Enable."]
    #[inline(always)]
    pub const fn set_em4wu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em4wuen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Em4wuen {{ em4wu: {=bool:?} }}", self.em4wu())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "PERIOD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PERIOD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_period(&mut self, val: bool) {
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
            .field("period", &self.period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ien {{ period: {=bool:?} }}", self.period())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Wakeup Event/Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Event/Interrupt."]
    #[inline(always)]
    pub const fn set_period(&mut self, val: bool) {
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
            .field("period", &self.period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If {{ period: {=bool:?} }}", self.period())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set PERIOD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set PERIOD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_period(&mut self, val: bool) {
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
            .field("period", &self.period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ifs {{ period: {=bool:?} }}", self.period())
    }
}
#[doc = "Interrupt Duration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periodsel(pub u32);
impl Periodsel {
    #[doc = "Interrupts/Wakeup Events Period Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn periodsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Interrupts/Wakeup Events Period Setting."]
    #[inline(always)]
    pub const fn set_periodsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Periodsel {
    #[inline(always)]
    fn default() -> Periodsel {
        Periodsel(0)
    }
}
impl core::fmt::Debug for Periodsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periodsel")
            .field("periodsel", &self.periodsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periodsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Periodsel {{ periodsel: {=u8:?} }}", self.periodsel())
    }
}
