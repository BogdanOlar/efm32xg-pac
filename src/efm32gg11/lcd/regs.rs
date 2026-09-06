#[doc = "Animation Register a."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arega(pub u32);
impl Arega {
    #[doc = "Animation Register a Data."]
    #[must_use]
    #[inline(always)]
    pub const fn arega(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Animation Register a Data."]
    #[inline(always)]
    pub const fn set_arega(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Arega {
    #[inline(always)]
    fn default() -> Arega {
        Arega(0)
    }
}
impl core::fmt::Debug for Arega {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arega")
            .field("arega", &self.arega())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arega {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Arega {{ arega: {=u8:?} }}", self.arega())
    }
}
#[doc = "Animation Register B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aregb(pub u32);
impl Aregb {
    #[doc = "Animation Register B Data."]
    #[must_use]
    #[inline(always)]
    pub const fn aregb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Animation Register B Data."]
    #[inline(always)]
    pub const fn set_aregb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Aregb {
    #[inline(always)]
    fn default() -> Aregb {
        Aregb(0)
    }
}
impl core::fmt::Debug for Aregb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aregb")
            .field("aregb", &self.aregb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aregb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aregb {{ aregb: {=u8:?} }}", self.aregb())
    }
}
#[doc = "Blink and Animation Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bactrl(pub u32);
impl Bactrl {
    #[doc = "Blink Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn blinken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Blink Enable."]
    #[inline(always)]
    pub const fn set_blinken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Blank Display."]
    #[must_use]
    #[inline(always)]
    pub const fn blank(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Blank Display."]
    #[inline(always)]
    pub const fn set_blank(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Animation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Animation Enable."]
    #[inline(always)]
    pub const fn set_aen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Animate Register a Shift Control."]
    #[must_use]
    #[inline(always)]
    pub const fn aregasc(&self) -> super::vals::Aregasc {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Aregasc::from_bits(val as u8)
    }
    #[doc = "Animate Register a Shift Control."]
    #[inline(always)]
    pub const fn set_aregasc(&mut self, val: super::vals::Aregasc) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Animate Register B Shift Control."]
    #[must_use]
    #[inline(always)]
    pub const fn aregbsc(&self) -> super::vals::Aregbsc {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Aregbsc::from_bits(val as u8)
    }
    #[doc = "Animate Register B Shift Control."]
    #[inline(always)]
    pub const fn set_aregbsc(&mut self, val: super::vals::Aregbsc) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Animate Logic Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn alogsel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Animate Logic Function Select."]
    #[inline(always)]
    pub const fn set_alogsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Frame Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fcen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Counter Enable."]
    #[inline(always)]
    pub const fn set_fcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Counter Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn fcpresc(&self) -> super::vals::Fcpresc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Fcpresc::from_bits(val as u8)
    }
    #[doc = "Frame Counter Prescaler."]
    #[inline(always)]
    pub const fn set_fcpresc(&mut self, val: super::vals::Fcpresc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Frame Counter Top Value."]
    #[must_use]
    #[inline(always)]
    pub const fn fctop(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Frame Counter Top Value."]
    #[inline(always)]
    pub const fn set_fctop(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Animation Location."]
    #[must_use]
    #[inline(always)]
    pub const fn aloc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Animation Location."]
    #[inline(always)]
    pub const fn set_aloc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Bactrl {
    #[inline(always)]
    fn default() -> Bactrl {
        Bactrl(0)
    }
}
impl core::fmt::Debug for Bactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bactrl")
            .field("blinken", &self.blinken())
            .field("blank", &self.blank())
            .field("aen", &self.aen())
            .field("aregasc", &self.aregasc())
            .field("aregbsc", &self.aregbsc())
            .field("alogsel", &self.alogsel())
            .field("fcen", &self.fcen())
            .field("fcpresc", &self.fcpresc())
            .field("fctop", &self.fctop())
            .field("aloc", &self.aloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bactrl {{ blinken: {=bool:?}, blank: {=bool:?}, aen: {=bool:?}, aregasc: {:?}, aregbsc: {:?}, alogsel: {=bool:?}, fcen: {=bool:?}, fcpresc: {:?}, fctop: {=u8:?}, aloc: {=bool:?} }}" , self . blinken () , self . blank () , self . aen () , self . aregasc () , self . aregbsc () , self . alogsel () , self . fcen () , self . fcpresc () , self . fctop () , self . aloc ())
    }
}
#[doc = "Analog BIAS Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biasctrl(pub u32);
impl Biasctrl {
    #[doc = "SPEED Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "SPEED Adjustment."]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Buffer Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn bufdrv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer Drive Strength."]
    #[inline(always)]
    pub const fn set_bufdrv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Buffer Bias Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn bufbias(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Buffer Bias Setting."]
    #[inline(always)]
    pub const fn set_bufbias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
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
            .field("speed", &self.speed())
            .field("bufdrv", &self.bufdrv())
            .field("bufbias", &self.bufbias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Biasctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Biasctrl {{ speed: {=u8:?}, bufdrv: {=u8:?}, bufbias: {=u8:?} }}",
            self.speed(),
            self.bufdrv(),
            self.bufbias()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "LCD Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LCD Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update Data Control."]
    #[must_use]
    #[inline(always)]
    pub const fn udctrl(&self) -> super::vals::Udctrl {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Udctrl::from_bits(val as u8)
    }
    #[doc = "Update Data Control."]
    #[inline(always)]
    pub const fn set_udctrl(&mut self, val: super::vals::Udctrl) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Direct Segment Control."]
    #[must_use]
    #[inline(always)]
    pub const fn dsc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Segment Control."]
    #[inline(always)]
    pub const fn set_dsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("udctrl", &self.udctrl())
            .field("dsc", &self.dsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ en: {=bool:?}, udctrl: {:?}, dsc: {=bool:?} }}",
            self.en(),
            self.udctrl(),
            self.dsc()
        )
    }
}
#[doc = "Display Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dispctrl(pub u32);
impl Dispctrl {
    #[doc = "Mux Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Mux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mux::from_bits(val as u8)
    }
    #[doc = "Mux Configuration."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Mux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Waveform Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn wave(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform Selection."]
    #[inline(always)]
    pub const fn set_wave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Contrast Control."]
    #[must_use]
    #[inline(always)]
    pub const fn contrast(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Contrast Control."]
    #[inline(always)]
    pub const fn set_contrast(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Charge Redistribution Cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn chgrdst(&self) -> super::vals::Chgrdst {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Chgrdst::from_bits(val as u8)
    }
    #[doc = "Charge Redistribution Cycles."]
    #[inline(always)]
    pub const fn set_chgrdst(&mut self, val: super::vals::Chgrdst) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Bias Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn bias(&self) -> super::vals::Bias {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Bias::from_bits(val as u8)
    }
    #[doc = "Bias Configuration."]
    #[inline(always)]
    pub const fn set_bias(&mut self, val: super::vals::Bias) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Mode Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode Setting."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Dispctrl {
    #[inline(always)]
    fn default() -> Dispctrl {
        Dispctrl(0)
    }
}
impl core::fmt::Debug for Dispctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dispctrl")
            .field("mux", &self.mux())
            .field("wave", &self.wave())
            .field("contrast", &self.contrast())
            .field("chgrdst", &self.chgrdst())
            .field("bias", &self.bias())
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dispctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dispctrl {{ mux: {:?}, wave: {=bool:?}, contrast: {=u8:?}, chgrdst: {:?}, bias: {:?}, mode: {:?} }}" , self . mux () , self . wave () , self . contrast () , self . chgrdst () , self . bias () , self . mode ())
    }
}
#[doc = "Frame Rate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framerate(pub u32);
impl Framerate {
    #[doc = "Frame Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn frdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Frame Rate Divider."]
    #[inline(always)]
    pub const fn set_frdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Framerate {
    #[inline(always)]
    fn default() -> Framerate {
        Framerate(0)
    }
}
impl core::fmt::Debug for Framerate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Framerate")
            .field("frdiv", &self.frdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Framerate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Framerate {{ frdiv: {=u16:?} }}", self.frdiv())
    }
}
#[doc = "Freeze Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freeze(pub u32);
impl Freeze {
    #[doc = "Register Update Freeze."]
    #[must_use]
    #[inline(always)]
    pub const fn regfreeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Update Freeze."]
    #[inline(always)]
    pub const fn set_regfreeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LCD Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn lcdgate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LCD Gate."]
    #[inline(always)]
    pub const fn set_lcdgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Freeze {
    #[inline(always)]
    fn default() -> Freeze {
        Freeze(0)
    }
}
impl core::fmt::Debug for Freeze {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freeze")
            .field("regfreeze", &self.regfreeze())
            .field("lcdgate", &self.lcdgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freeze {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Freeze {{ regfreeze: {=bool:?}, lcdgate: {=bool:?} }}",
            self.regfreeze(),
            self.lcdgate()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "Frame Counter Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Counter Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fc(&mut self, val: bool) {
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
        f.debug_struct("Ien").field("fc", &self.fc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ien {{ fc: {=bool:?} }}", self.fc())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Frame Counter Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Counter Interrupt Flag."]
    #[inline(always)]
    pub const fn set_fc(&mut self, val: bool) {
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
        f.debug_struct("If").field("fc", &self.fc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "If {{ fc: {=bool:?} }}", self.fc())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Frame Counter Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Counter Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_fc(&mut self, val: bool) {
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
        f.debug_struct("Ifs").field("fc", &self.fc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ifs {{ fc: {=bool:?} }}", self.fc())
    }
}
#[doc = "Segment Data High Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd0h(pub u32);
impl Segd0h {
    #[doc = "COM0 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd0h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM0 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd0h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd0h {
    #[inline(always)]
    fn default() -> Segd0h {
        Segd0h(0)
    }
}
impl core::fmt::Debug for Segd0h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd0h")
            .field("segd0h", &self.segd0h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd0h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd0h {{ segd0h: {=u8:?} }}", self.segd0h())
    }
}
#[doc = "Segment Data High Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd1h(pub u32);
impl Segd1h {
    #[doc = "COM1 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd1h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM1 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd1h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd1h {
    #[inline(always)]
    fn default() -> Segd1h {
        Segd1h(0)
    }
}
impl core::fmt::Debug for Segd1h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd1h")
            .field("segd1h", &self.segd1h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd1h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd1h {{ segd1h: {=u8:?} }}", self.segd1h())
    }
}
#[doc = "Segment Data High Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd2h(pub u32);
impl Segd2h {
    #[doc = "COM2 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd2h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM2 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd2h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd2h {
    #[inline(always)]
    fn default() -> Segd2h {
        Segd2h(0)
    }
}
impl core::fmt::Debug for Segd2h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd2h")
            .field("segd2h", &self.segd2h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd2h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd2h {{ segd2h: {=u8:?} }}", self.segd2h())
    }
}
#[doc = "Segment Data High Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd3h(pub u32);
impl Segd3h {
    #[doc = "COM3 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd3h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM3 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd3h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd3h {
    #[inline(always)]
    fn default() -> Segd3h {
        Segd3h(0)
    }
}
impl core::fmt::Debug for Segd3h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd3h")
            .field("segd3h", &self.segd3h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd3h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd3h {{ segd3h: {=u8:?} }}", self.segd3h())
    }
}
#[doc = "Segment Data High Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd4h(pub u32);
impl Segd4h {
    #[doc = "COM0 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd4h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM0 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd4h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd4h {
    #[inline(always)]
    fn default() -> Segd4h {
        Segd4h(0)
    }
}
impl core::fmt::Debug for Segd4h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd4h")
            .field("segd4h", &self.segd4h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd4h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd4h {{ segd4h: {=u8:?} }}", self.segd4h())
    }
}
#[doc = "Segment Data High Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd5h(pub u32);
impl Segd5h {
    #[doc = "COM1 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd5h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM1 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd5h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd5h {
    #[inline(always)]
    fn default() -> Segd5h {
        Segd5h(0)
    }
}
impl core::fmt::Debug for Segd5h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd5h")
            .field("segd5h", &self.segd5h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd5h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd5h {{ segd5h: {=u8:?} }}", self.segd5h())
    }
}
#[doc = "Segment Data High Register 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd6h(pub u32);
impl Segd6h {
    #[doc = "COM2 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd6h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM2 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd6h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd6h {
    #[inline(always)]
    fn default() -> Segd6h {
        Segd6h(0)
    }
}
impl core::fmt::Debug for Segd6h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd6h")
            .field("segd6h", &self.segd6h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd6h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd6h {{ segd6h: {=u8:?} }}", self.segd6h())
    }
}
#[doc = "Segment Data High Register 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segd7h(pub u32);
impl Segd7h {
    #[doc = "COM3 Segment Data High."]
    #[must_use]
    #[inline(always)]
    pub const fn segd7h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COM3 Segment Data High."]
    #[inline(always)]
    pub const fn set_segd7h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segd7h {
    #[inline(always)]
    fn default() -> Segd7h {
        Segd7h(0)
    }
}
impl core::fmt::Debug for Segd7h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segd7h")
            .field("segd7h", &self.segd7h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segd7h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segd7h {{ segd7h: {=u8:?} }}", self.segd7h())
    }
}
#[doc = "Segment Enable (32 to 39)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segen2(pub u32);
impl Segen2 {
    #[doc = "Segment Enable (second Group)."]
    #[must_use]
    #[inline(always)]
    pub const fn segen2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Segment Enable (second Group)."]
    #[inline(always)]
    pub const fn set_segen2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Segen2 {
    #[inline(always)]
    fn default() -> Segen2 {
        Segen2(0)
    }
}
impl core::fmt::Debug for Segen2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Segen2")
            .field("segen2", &self.segen2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Segen2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Segen2 {{ segen2: {=u8:?} }}", self.segen2())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Current Animation State."]
    #[must_use]
    #[inline(always)]
    pub const fn astate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Current Animation State."]
    #[inline(always)]
    pub const fn set_astate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Blink State."]
    #[must_use]
    #[inline(always)]
    pub const fn blink(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Blink State."]
    #[inline(always)]
    pub const fn set_blink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("astate", &self.astate())
            .field("blink", &self.blink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ astate: {=u8:?}, blink: {=bool:?} }}",
            self.astate(),
            self.blink()
        )
    }
}
#[doc = "Synchronization Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc = "CTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CTRL Register Busy."]
    #[inline(always)]
    pub const fn set_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BACTRL Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn bactrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BACTRL Register Busy."]
    #[inline(always)]
    pub const fn set_bactrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AREGA Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn arega(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AREGA Register Busy."]
    #[inline(always)]
    pub const fn set_arega(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AREGB Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn aregb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AREGB Register Busy."]
    #[inline(always)]
    pub const fn set_aregb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SEGD0L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd0l(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD0L Register Busy."]
    #[inline(always)]
    pub const fn set_segd0l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SEGD1L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd1l(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD1L Register Busy."]
    #[inline(always)]
    pub const fn set_segd1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SEGD2L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd2l(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD2L Register Busy."]
    #[inline(always)]
    pub const fn set_segd2l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SEGD3L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd3l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD3L Register Busy."]
    #[inline(always)]
    pub const fn set_segd3l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SEGD0H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd0h(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD0H Register Busy."]
    #[inline(always)]
    pub const fn set_segd0h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SEGD1H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd1h(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD1H Register Busy."]
    #[inline(always)]
    pub const fn set_segd1h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SEGD2H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd2h(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD2H Register Busy."]
    #[inline(always)]
    pub const fn set_segd2h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SEGD3H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd3h(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD3H Register Busy."]
    #[inline(always)]
    pub const fn set_segd3h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SEGD4L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd4l(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD4L Register Busy."]
    #[inline(always)]
    pub const fn set_segd4l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SEGD5L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd5l(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD5L Register Busy."]
    #[inline(always)]
    pub const fn set_segd5l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SEGD6L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd6l(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD6L Register Busy."]
    #[inline(always)]
    pub const fn set_segd6l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SEGD7L Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd7l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD7L Register Busy."]
    #[inline(always)]
    pub const fn set_segd7l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SEGD4H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd4h(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD4H Register Busy."]
    #[inline(always)]
    pub const fn set_segd4h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SEGD5H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd5h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD5H Register Busy."]
    #[inline(always)]
    pub const fn set_segd5h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SEGD6H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd6h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD6H Register Busy."]
    #[inline(always)]
    pub const fn set_segd6h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SEGD7H Register Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn segd7h(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SEGD7H Register Busy."]
    #[inline(always)]
    pub const fn set_segd7h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("ctrl", &self.ctrl())
            .field("bactrl", &self.bactrl())
            .field("arega", &self.arega())
            .field("aregb", &self.aregb())
            .field("segd0l", &self.segd0l())
            .field("segd1l", &self.segd1l())
            .field("segd2l", &self.segd2l())
            .field("segd3l", &self.segd3l())
            .field("segd0h", &self.segd0h())
            .field("segd1h", &self.segd1h())
            .field("segd2h", &self.segd2h())
            .field("segd3h", &self.segd3h())
            .field("segd4l", &self.segd4l())
            .field("segd5l", &self.segd5l())
            .field("segd6l", &self.segd6l())
            .field("segd7l", &self.segd7l())
            .field("segd4h", &self.segd4h())
            .field("segd5h", &self.segd5h())
            .field("segd6h", &self.segd6h())
            .field("segd7h", &self.segd7h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Syncbusy {{ ctrl: {=bool:?}, bactrl: {=bool:?}, arega: {=bool:?}, aregb: {=bool:?}, segd0l: {=bool:?}, segd1l: {=bool:?}, segd2l: {=bool:?}, segd3l: {=bool:?}, segd0h: {=bool:?}, segd1h: {=bool:?}, segd2h: {=bool:?}, segd3h: {=bool:?}, segd4l: {=bool:?}, segd5l: {=bool:?}, segd6l: {=bool:?}, segd7l: {=bool:?}, segd4h: {=bool:?}, segd5h: {=bool:?}, segd6h: {=bool:?}, segd7h: {=bool:?} }}" , self . ctrl () , self . bactrl () , self . arega () , self . aregb () , self . segd0l () , self . segd1l () , self . segd2l () , self . segd3l () , self . segd0h () , self . segd1h () , self . segd2h () , self . segd3h () , self . segd4l () , self . segd5l () , self . segd6l () , self . segd7l () , self . segd4h () , self . segd5h () , self . segd6h () , self . segd7h ())
    }
}
