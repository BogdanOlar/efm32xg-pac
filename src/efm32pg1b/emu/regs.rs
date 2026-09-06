#[doc = "Configurations Related to the Bias."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biasconf(pub u32);
impl Biasconf {
    #[doc = "NA DUTY in EM01."]
    #[must_use]
    #[inline(always)]
    pub const fn nadutyem01(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NA DUTY in EM01."]
    #[inline(always)]
    pub const fn set_nadutyem01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP in EM01."]
    #[must_use]
    #[inline(always)]
    pub const fn lpem01(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LP in EM01."]
    #[inline(always)]
    pub const fn set_lpem01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GMC in EM234."]
    #[must_use]
    #[inline(always)]
    pub const fn gmcem23(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GMC in EM234."]
    #[inline(always)]
    pub const fn set_gmcem23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "UADUTY in EM234."]
    #[must_use]
    #[inline(always)]
    pub const fn uadutyem23(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "UADUTY in EM234."]
    #[inline(always)]
    pub const fn set_uadutyem23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NA DUTY in EM234."]
    #[must_use]
    #[inline(always)]
    pub const fn nadutyem23(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NA DUTY in EM234."]
    #[inline(always)]
    pub const fn set_nadutyem23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LP in EM234."]
    #[must_use]
    #[inline(always)]
    pub const fn lpem23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LP in EM234."]
    #[inline(always)]
    pub const fn set_lpem23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Biasconf {
    #[inline(always)]
    fn default() -> Biasconf {
        Biasconf(0)
    }
}
impl core::fmt::Debug for Biasconf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Biasconf")
            .field("nadutyem01", &self.nadutyem01())
            .field("lpem01", &self.lpem01())
            .field("gmcem23", &self.gmcem23())
            .field("uadutyem23", &self.uadutyem23())
            .field("nadutyem23", &self.nadutyem23())
            .field("lpem23", &self.lpem23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Biasconf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Biasconf {{ nadutyem01: {=bool:?}, lpem01: {=bool:?}, gmcem23: {=bool:?}, uadutyem23: {=bool:?}, nadutyem23: {=bool:?}, lpem23: {=bool:?} }}" , self . nadutyem01 () , self . lpem01 () , self . gmcem23 () , self . uadutyem23 () , self . nadutyem23 () , self . lpem23 ())
    }
}
#[doc = "Test Control Register for Regulator and BIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biastestctrl(pub u32);
impl Biastestctrl {
    #[doc = "Reset Bias Ripple Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn bias_rip_reset(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Bias Ripple Counter."]
    #[inline(always)]
    pub const fn set_bias_rip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Biastestctrl {
    #[inline(always)]
    fn default() -> Biastestctrl {
        Biastestctrl(0)
    }
}
impl core::fmt::Debug for Biastestctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Biastestctrl")
            .field("bias_rip_reset", &self.bias_rip_reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Biastestctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Biastestctrl {{ bias_rip_reset: {=bool:?} }}",
            self.bias_rip_reset()
        )
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "EM4 Unlatch."]
    #[must_use]
    #[inline(always)]
    pub const fn em4unlatch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 Unlatch."]
    #[inline(always)]
    pub const fn set_em4unlatch(&mut self, val: bool) {
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
        f.debug_struct("Cmd")
            .field("em4unlatch", &self.em4unlatch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ em4unlatch: {=bool:?} }}", self.em4unlatch())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Energy Mode 2 Block."]
    #[must_use]
    #[inline(always)]
    pub const fn em2block(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Mode 2 Block."]
    #[inline(always)]
    pub const fn set_em2block(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("em2block", &self.em2block())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrl {{ em2block: {=bool:?} }}", self.em2block())
    }
}
#[doc = "DCDC Power Train PFET Current Limiter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcclimctrl(pub u32);
impl Dcdcclimctrl {
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn climblankdly(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_climblankdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Bypass Current Limit Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn byplimen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass Current Limit Enable."]
    #[inline(always)]
    pub const fn set_byplimen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Dcdcclimctrl {
    #[inline(always)]
    fn default() -> Dcdcclimctrl {
        Dcdcclimctrl(0)
    }
}
impl core::fmt::Debug for Dcdcclimctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdcclimctrl")
            .field("climblankdly", &self.climblankdly())
            .field("byplimen", &self.byplimen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdcclimctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdcclimctrl {{ climblankdly: {=u8:?}, byplimen: {=bool:?} }}",
            self.climblankdly(),
            self.byplimen()
        )
    }
}
#[doc = "DCDC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcctrl(pub u32);
impl Dcdcctrl {
    #[doc = "Regulator Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcmode(&self) -> super::vals::Dcdcmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dcdcmode::from_bits(val as u8)
    }
    #[doc = "Regulator Mode."]
    #[inline(always)]
    pub const fn set_dcdcmode(&mut self, val: super::vals::Dcdcmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DCDC Mode EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcmodeem23(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Mode EM23."]
    #[inline(always)]
    pub const fn set_dcdcmodeem23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DCDC Mode EM4H."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcmodeem4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Mode EM4H."]
    #[inline(always)]
    pub const fn set_dcdcmodeem4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Dcdcctrl {
    #[inline(always)]
    fn default() -> Dcdcctrl {
        Dcdcctrl(0)
    }
}
impl core::fmt::Debug for Dcdcctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdcctrl")
            .field("dcdcmode", &self.dcdcmode())
            .field("dcdcmodeem23", &self.dcdcmodeem23())
            .field("dcdcmodeem4", &self.dcdcmodeem4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdcctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdcctrl {{ dcdcmode: {:?}, dcdcmodeem23: {=bool:?}, dcdcmodeem4: {=bool:?} }}",
            self.dcdcmode(),
            self.dcdcmodeem23(),
            self.dcdcmodeem4()
        )
    }
}
#[doc = "DCDC Low Noise Compensator Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclncompctrl(pub u32);
impl Dcdclncompctrl {
    #[doc = "Low Noise Mode Compensator R1 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator R1 Trim Value."]
    #[inline(always)]
    pub const fn set_compenr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Low Noise Mode Compensator R2 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenr2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator R2 Trim Value."]
    #[inline(always)]
    pub const fn set_compenr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Low Noise Mode Compensator R3 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenr3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator R3 Trim Value."]
    #[inline(always)]
    pub const fn set_compenr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Low Noise Mode Compensator C1 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenc1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator C1 Trim Value."]
    #[inline(always)]
    pub const fn set_compenc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Low Noise Mode Compensator C2 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenc2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator C2 Trim Value."]
    #[inline(always)]
    pub const fn set_compenc2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Low Noise Mode Compensator C3 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compenc3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Low Noise Mode Compensator C3 Trim Value."]
    #[inline(always)]
    pub const fn set_compenc3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Dcdclncompctrl {
    #[inline(always)]
    fn default() -> Dcdclncompctrl {
        Dcdclncompctrl(0)
    }
}
impl core::fmt::Debug for Dcdclncompctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclncompctrl")
            .field("compenr1", &self.compenr1())
            .field("compenr2", &self.compenr2())
            .field("compenr3", &self.compenr3())
            .field("compenc1", &self.compenc1())
            .field("compenc2", &self.compenc2())
            .field("compenc3", &self.compenc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclncompctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcdclncompctrl {{ compenr1: {=u8:?}, compenr2: {=u8:?}, compenr3: {=u8:?}, compenc1: {=u8:?}, compenc2: {=u8:?}, compenc3: {=u8:?} }}" , self . compenr1 () , self . compenr2 () , self . compenr3 () , self . compenc1 () , self . compenc2 () , self . compenc3 ())
    }
}
#[doc = "DCDC Low Noise Controller Frequency Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclnfreqctrl(pub u32);
impl Dcdclnfreqctrl {
    #[doc = "LN Mode RCO Frequency Band Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn rcoband(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "LN Mode RCO Frequency Band Selection."]
    #[inline(always)]
    pub const fn set_rcoband(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn rcotrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_rcotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Dcdclnfreqctrl {
    #[inline(always)]
    fn default() -> Dcdclnfreqctrl {
        Dcdclnfreqctrl(0)
    }
}
impl core::fmt::Debug for Dcdclnfreqctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclnfreqctrl")
            .field("rcoband", &self.rcoband())
            .field("rcotrim", &self.rcotrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclnfreqctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdclnfreqctrl {{ rcoband: {=u8:?}, rcotrim: {=u8:?} }}",
            self.rcoband(),
            self.rcotrim()
        )
    }
}
#[doc = "DCDC Low Noise Voltage Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclnvctrl(pub u32);
impl Dcdclnvctrl {
    #[doc = "Low Noise Mode Feedback Attenuation."]
    #[must_use]
    #[inline(always)]
    pub const fn lnatt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Noise Mode Feedback Attenuation."]
    #[inline(always)]
    pub const fn set_lnatt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Noise Mode VREF Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn lnvref(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Low Noise Mode VREF Trim."]
    #[inline(always)]
    pub const fn set_lnvref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Dcdclnvctrl {
    #[inline(always)]
    fn default() -> Dcdclnvctrl {
        Dcdclnvctrl(0)
    }
}
impl core::fmt::Debug for Dcdclnvctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclnvctrl")
            .field("lnatt", &self.lnatt())
            .field("lnvref", &self.lnvref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclnvctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdclnvctrl {{ lnatt: {=bool:?}, lnvref: {=u8:?} }}",
            self.lnatt(),
            self.lnvref()
        )
    }
}
#[doc = "DCDC Low Power Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclpctrl(pub u32);
impl Dcdclpctrl {
    #[doc = "LP Mode Hysteresis Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmphyssel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "LP Mode Hysteresis Selection."]
    #[inline(always)]
    pub const fn set_lpcmphyssel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "LP Mode Duty Cycling Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpvrefdutyen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LP Mode Duty Cycling Enable."]
    #[inline(always)]
    pub const fn set_lpvrefdutyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn lpblank(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_lpblank(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
}
impl Default for Dcdclpctrl {
    #[inline(always)]
    fn default() -> Dcdclpctrl {
        Dcdclpctrl(0)
    }
}
impl core::fmt::Debug for Dcdclpctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclpctrl")
            .field("lpcmphyssel", &self.lpcmphyssel())
            .field("lpvrefdutyen", &self.lpvrefdutyen())
            .field("lpblank", &self.lpblank())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclpctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdclpctrl {{ lpcmphyssel: {=u8:?}, lpvrefdutyen: {=bool:?}, lpblank: {=u8:?} }}",
            self.lpcmphyssel(),
            self.lpvrefdutyen(),
            self.lpblank()
        )
    }
}
#[doc = "DCDC Low Power Voltage Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclpvctrl(pub u32);
impl Dcdclpvctrl {
    #[doc = "Low Power Feedback Attenuation."]
    #[must_use]
    #[inline(always)]
    pub const fn lpatt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Feedback Attenuation."]
    #[inline(always)]
    pub const fn set_lpatt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LP Mode Reference Selection for EM23 and EM4H."]
    #[must_use]
    #[inline(always)]
    pub const fn lpvref(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "LP Mode Reference Selection for EM23 and EM4H."]
    #[inline(always)]
    pub const fn set_lpvref(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Dcdclpvctrl {
    #[inline(always)]
    fn default() -> Dcdclpvctrl {
        Dcdclpvctrl(0)
    }
}
impl core::fmt::Debug for Dcdclpvctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclpvctrl")
            .field("lpatt", &self.lpatt())
            .field("lpvref", &self.lpvref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclpvctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdclpvctrl {{ lpatt: {=bool:?}, lpvref: {=u8:?} }}",
            self.lpatt(),
            self.lpvref()
        )
    }
}
#[doc = "DCDC Miscellaneous Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcmiscctrl(pub u32);
impl Dcdcmiscctrl {
    #[doc = "Force DCDC Into CCM Mode in Low Noise Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn lnforceccm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force DCDC Into CCM Mode in Low Noise Operation."]
    #[inline(always)]
    pub const fn set_lnforceccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFET Switch Number Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pfetcnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PFET Switch Number Selection."]
    #[inline(always)]
    pub const fn set_pfetcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "NFET Switch Number Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn nfetcnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "NFET Switch Number Selection."]
    #[inline(always)]
    pub const fn set_nfetcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Current Limit in Bypass Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn byplimsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Current Limit in Bypass Mode."]
    #[inline(always)]
    pub const fn set_byplimsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Current Limit Level Selection for Current Limiter in LP Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpclimilimsel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Current Limit Level Selection for Current Limiter in LP Mode."]
    #[inline(always)]
    pub const fn set_lpclimilimsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Current Limit Level Selection for Current Limiter in LN Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lnclimilimsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Current Limit Level Selection for Current Limiter in LN Mode."]
    #[inline(always)]
    pub const fn set_lnclimilimsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "LP Mode Comparator Bias Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmpbias(&self) -> super::vals::Lpcmpbias {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Lpcmpbias::from_bits(val as u8)
    }
    #[doc = "LP Mode Comparator Bias Selection."]
    #[inline(always)]
    pub const fn set_lpcmpbias(&mut self, val: super::vals::Lpcmpbias) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Dcdcmiscctrl {
    #[inline(always)]
    fn default() -> Dcdcmiscctrl {
        Dcdcmiscctrl(0)
    }
}
impl core::fmt::Debug for Dcdcmiscctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdcmiscctrl")
            .field("lnforceccm", &self.lnforceccm())
            .field("pfetcnt", &self.pfetcnt())
            .field("nfetcnt", &self.nfetcnt())
            .field("byplimsel", &self.byplimsel())
            .field("lpclimilimsel", &self.lpclimilimsel())
            .field("lnclimilimsel", &self.lnclimilimsel())
            .field("lpcmpbias", &self.lpcmpbias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdcmiscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcdcmiscctrl {{ lnforceccm: {=bool:?}, pfetcnt: {=u8:?}, nfetcnt: {=u8:?}, byplimsel: {=u8:?}, lpclimilimsel: {=u8:?}, lnclimilimsel: {=u8:?}, lpcmpbias: {:?} }}" , self . lnforceccm () , self . pfetcnt () , self . nfetcnt () , self . byplimsel () , self . lpclimilimsel () , self . lnclimilimsel () , self . lpcmpbias ())
    }
}
#[doc = "DCDC Read Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcsync(pub u32);
impl Dcdcsync {
    #[doc = "DCDC CTRL Register Transfer Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcctrlbusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC CTRL Register Transfer Busy."]
    #[inline(always)]
    pub const fn set_dcdcctrlbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dcdcsync {
    #[inline(always)]
    fn default() -> Dcdcsync {
        Dcdcsync(0)
    }
}
impl core::fmt::Debug for Dcdcsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdcsync")
            .field("dcdcctrlbusy", &self.dcdcctrlbusy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdcsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdcsync {{ dcdcctrlbusy: {=bool:?} }}",
            self.dcdcctrlbusy()
        )
    }
}
#[doc = "DCDC Controller Timing Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdctiming(pub u32);
impl Dcdctiming {
    #[doc = "Low Power Initialization Wait Time."]
    #[must_use]
    #[inline(always)]
    pub const fn lpinitwait(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Power Initialization Wait Time."]
    #[inline(always)]
    pub const fn set_lpinitwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "LN Mode Precharge Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn compenprchgen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LN Mode Precharge Enable."]
    #[inline(always)]
    pub const fn set_compenprchgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Low Noise Controller Initialization Wait Time."]
    #[must_use]
    #[inline(always)]
    pub const fn lnwait(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "Low Noise Controller Initialization Wait Time."]
    #[inline(always)]
    pub const fn set_lnwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait."]
    #[must_use]
    #[inline(always)]
    pub const fn bypwait(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait."]
    #[inline(always)]
    pub const fn set_bypwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Select Bias Duty Cycle Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn dutyscale(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Select Bias Duty Cycle Clock."]
    #[inline(always)]
    pub const fn set_dutyscale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Dcdctiming {
    #[inline(always)]
    fn default() -> Dcdctiming {
        Dcdctiming(0)
    }
}
impl core::fmt::Debug for Dcdctiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdctiming")
            .field("lpinitwait", &self.lpinitwait())
            .field("compenprchgen", &self.compenprchgen())
            .field("lnwait", &self.lnwait())
            .field("bypwait", &self.bypwait())
            .field("dutyscale", &self.dutyscale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdctiming {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcdctiming {{ lpinitwait: {=u8:?}, compenprchgen: {=bool:?}, lnwait: {=u8:?}, bypwait: {=u8:?}, dutyscale: {=u8:?} }}" , self . lpinitwait () , self . compenprchgen () , self . lnwait () , self . bypwait () , self . dutyscale ())
    }
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdczdetctrl(pub u32);
impl Dcdczdetctrl {
    #[doc = "Reverse Current Limit Level Selection for Zero Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn zdetilimsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Reverse Current Limit Level Selection for Zero Detector."]
    #[inline(always)]
    pub const fn set_zdetilimsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn zdetblankdly(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_zdetblankdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Dcdczdetctrl {
    #[inline(always)]
    fn default() -> Dcdczdetctrl {
        Dcdczdetctrl(0)
    }
}
impl core::fmt::Debug for Dcdczdetctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdczdetctrl")
            .field("zdetilimsel", &self.zdetilimsel())
            .field("zdetblankdly", &self.zdetblankdly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdczdetctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdczdetctrl {{ zdetilimsel: {=u8:?}, zdetblankdly: {=u8:?} }}",
            self.zdetilimsel(),
            self.zdetblankdly()
        )
    }
}
#[doc = "EM4 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em4ctrl(pub u32);
impl Em4ctrl {
    #[doc = "Energy Mode 4 State."]
    #[must_use]
    #[inline(always)]
    pub const fn em4state(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Mode 4 State."]
    #[inline(always)]
    pub const fn set_em4state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LFRCO Retain During EM4."]
    #[must_use]
    #[inline(always)]
    pub const fn retainlfrco(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Retain During EM4."]
    #[inline(always)]
    pub const fn set_retainlfrco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LFXO Retain During EM4."]
    #[must_use]
    #[inline(always)]
    pub const fn retainlfxo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Retain During EM4."]
    #[inline(always)]
    pub const fn set_retainlfxo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ULFRCO Retain During EM4S."]
    #[must_use]
    #[inline(always)]
    pub const fn retainulfrco(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ULFRCO Retain During EM4S."]
    #[inline(always)]
    pub const fn set_retainulfrco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EM4 IO Retention Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn em4ioretmode(&self) -> super::vals::Em4ioretmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Em4ioretmode::from_bits(val as u8)
    }
    #[doc = "EM4 IO Retention Disable."]
    #[inline(always)]
    pub const fn set_em4ioretmode(&mut self, val: super::vals::Em4ioretmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Energy Mode 4 Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn em4entry(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Energy Mode 4 Entry."]
    #[inline(always)]
    pub const fn set_em4entry(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Em4ctrl {
    #[inline(always)]
    fn default() -> Em4ctrl {
        Em4ctrl(0)
    }
}
impl core::fmt::Debug for Em4ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Em4ctrl")
            .field("em4state", &self.em4state())
            .field("retainlfrco", &self.retainlfrco())
            .field("retainlfxo", &self.retainlfxo())
            .field("retainulfrco", &self.retainulfrco())
            .field("em4ioretmode", &self.em4ioretmode())
            .field("em4entry", &self.em4entry())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em4ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Em4ctrl {{ em4state: {=bool:?}, retainlfrco: {=bool:?}, retainlfxo: {=bool:?}, retainulfrco: {=bool:?}, em4ioretmode: {:?}, em4entry: {=u8:?} }}" , self . em4state () , self . retainlfrco () , self . retainlfxo () , self . retainulfrco () , self . em4ioretmode () , self . em4entry ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "VMONAVDDFALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddfall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMONAVDDFALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VMONAVDDRISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddrise(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VMONAVDDRISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VMONALTAVDDFALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddfall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VMONALTAVDDFALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonaltavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VMONALTAVDDRISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddrise(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VMONALTAVDDRISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonaltavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VMONDVDDFALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddfall(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VMONDVDDFALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmondvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VMONDVDDRISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddrise(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "VMONDVDDRISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmondvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VMONIO0FALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0fall(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VMONIO0FALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonio0fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "VMONIO0RISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0rise(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VMONIO0RISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonio0rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VMONFVDDFALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddfall(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "VMONFVDDFALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonfvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VMONFVDDRISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddrise(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "VMONFVDDRISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonfvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PFETOVERCURRENTLIMIT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PFETOVERCURRENTLIMIT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NFETOVERCURRENTLIMIT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NFETOVERCURRENTLIMIT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_nfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DCDCLPRUNNING Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclprunning(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DCDCLPRUNNING Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcdclprunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DCDCLNRUNNING Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclnrunning(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DCDCLNRUNNING Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcdclnrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DCDCINBYPASS Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcinbypass(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DCDCINBYPASS Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcdcinbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "EM23WAKEUP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em23wakeup(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "EM23WAKEUP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_em23wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TEMP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn temp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TEMP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TEMPLOW Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn templow(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TEMPLOW Interrupt Enable."]
    #[inline(always)]
    pub const fn set_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TEMPHIGH Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn temphigh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TEMPHIGH Interrupt Enable."]
    #[inline(always)]
    pub const fn set_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("vmonavddfall", &self.vmonavddfall())
            .field("vmonavddrise", &self.vmonavddrise())
            .field("vmonaltavddfall", &self.vmonaltavddfall())
            .field("vmonaltavddrise", &self.vmonaltavddrise())
            .field("vmondvddfall", &self.vmondvddfall())
            .field("vmondvddrise", &self.vmondvddrise())
            .field("vmonio0fall", &self.vmonio0fall())
            .field("vmonio0rise", &self.vmonio0rise())
            .field("vmonfvddfall", &self.vmonfvddfall())
            .field("vmonfvddrise", &self.vmonfvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("em23wakeup", &self.em23wakeup())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonfvddfall: {=bool:?}, vmonfvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, em23wakeup: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonfvddfall () , self . vmonfvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . em23wakeup () , self . temp () , self . templow () , self . temphigh ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "VMON AVDD Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddfall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMON AVDD Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VMON AVDD Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddrise(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VMON AVDD Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Alternate VMON AVDD Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddfall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate VMON AVDD Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonaltavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Alternate VMON AVDD Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddrise(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate VMON AVDD Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonaltavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VMON DVDD Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddfall(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VMON DVDD Channel Fall."]
    #[inline(always)]
    pub const fn set_vmondvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VMON DVDD Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddrise(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "VMON DVDD Channel Rise."]
    #[inline(always)]
    pub const fn set_vmondvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VMON IOVDD0 Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0fall(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD0 Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonio0fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "VMON IOVDD0 Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0rise(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD0 Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonio0rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VMON VDDFLASH Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddfall(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "VMON VDDFLASH Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonfvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VMON VDDFLASH Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddrise(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "VMON VDDFLASH Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonfvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PFET Current Limit Hit."]
    #[must_use]
    #[inline(always)]
    pub const fn pfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PFET Current Limit Hit."]
    #[inline(always)]
    pub const fn set_pfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NFET Current Limit Hit."]
    #[must_use]
    #[inline(always)]
    pub const fn nfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NFET Current Limit Hit."]
    #[inline(always)]
    pub const fn set_nfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LP Mode is Running."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclprunning(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LP Mode is Running."]
    #[inline(always)]
    pub const fn set_dcdclprunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LN Mode is Running."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclnrunning(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LN Mode is Running."]
    #[inline(always)]
    pub const fn set_dcdclnrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DCDC is in Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcinbypass(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is in Bypass."]
    #[inline(always)]
    pub const fn set_dcdcinbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wakeup IRQ From EM2 and EM3."]
    #[must_use]
    #[inline(always)]
    pub const fn em23wakeup(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup IRQ From EM2 and EM3."]
    #[inline(always)]
    pub const fn set_em23wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "New Temperature Measurement Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn temp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "New Temperature Measurement Valid."]
    #[inline(always)]
    pub const fn set_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Temperature Low Limit Reached."]
    #[must_use]
    #[inline(always)]
    pub const fn templow(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Low Limit Reached."]
    #[inline(always)]
    pub const fn set_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Temperature High Limit Reached."]
    #[must_use]
    #[inline(always)]
    pub const fn temphigh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature High Limit Reached."]
    #[inline(always)]
    pub const fn set_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("vmonavddfall", &self.vmonavddfall())
            .field("vmonavddrise", &self.vmonavddrise())
            .field("vmonaltavddfall", &self.vmonaltavddfall())
            .field("vmonaltavddrise", &self.vmonaltavddrise())
            .field("vmondvddfall", &self.vmondvddfall())
            .field("vmondvddrise", &self.vmondvddrise())
            .field("vmonio0fall", &self.vmonio0fall())
            .field("vmonio0rise", &self.vmonio0rise())
            .field("vmonfvddfall", &self.vmonfvddfall())
            .field("vmonfvddrise", &self.vmonfvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("em23wakeup", &self.em23wakeup())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonfvddfall: {=bool:?}, vmonfvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, em23wakeup: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonfvddfall () , self . vmonfvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . em23wakeup () , self . temp () , self . templow () , self . temphigh ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set VMONAVDDFALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddfall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONAVDDFALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set VMONAVDDRISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavddrise(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONAVDDRISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set VMONALTAVDDFALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddfall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONALTAVDDFALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonaltavddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set VMONALTAVDDRISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavddrise(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONALTAVDDRISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonaltavddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set VMONDVDDFALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddfall(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONDVDDFALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmondvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set VMONDVDDRISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvddrise(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONDVDDRISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmondvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set VMONIO0FALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0fall(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONIO0FALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonio0fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set VMONIO0RISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0rise(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONIO0RISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonio0rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set VMONFVDDFALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddfall(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONFVDDFALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonfvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set VMONFVDDRISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvddrise(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONFVDDRISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonfvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set PFETOVERCURRENTLIMIT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set PFETOVERCURRENTLIMIT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set NFETOVERCURRENTLIMIT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nfetovercurrentlimit(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set NFETOVERCURRENTLIMIT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_nfetovercurrentlimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set DCDCLPRUNNING Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclprunning(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set DCDCLPRUNNING Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dcdclprunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set DCDCLNRUNNING Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdclnrunning(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set DCDCLNRUNNING Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dcdclnrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set DCDCINBYPASS Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcinbypass(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set DCDCINBYPASS Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dcdcinbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set EM23WAKEUP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn em23wakeup(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set EM23WAKEUP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_em23wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set TEMP Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn temp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set TEMP Interrupt Flag."]
    #[inline(always)]
    pub const fn set_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Set TEMPLOW Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn templow(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Set TEMPLOW Interrupt Flag."]
    #[inline(always)]
    pub const fn set_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set TEMPHIGH Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn temphigh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set TEMPHIGH Interrupt Flag."]
    #[inline(always)]
    pub const fn set_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("vmonavddfall", &self.vmonavddfall())
            .field("vmonavddrise", &self.vmonavddrise())
            .field("vmonaltavddfall", &self.vmonaltavddfall())
            .field("vmonaltavddrise", &self.vmonaltavddrise())
            .field("vmondvddfall", &self.vmondvddfall())
            .field("vmondvddrise", &self.vmondvddrise())
            .field("vmonio0fall", &self.vmonio0fall())
            .field("vmonio0rise", &self.vmonio0rise())
            .field("vmonfvddfall", &self.vmonfvddfall())
            .field("vmonfvddrise", &self.vmonfvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("em23wakeup", &self.em23wakeup())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonfvddfall: {=bool:?}, vmonfvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, em23wakeup: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonfvddfall () , self . vmonfvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . em23wakeup () , self . temp () , self . templow () , self . temphigh ())
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
#[doc = "Power Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrcfg(pub u32);
impl Pwrcfg {
    #[doc = "Power Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrcfg(&self) -> super::vals::Pwrcfg {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwrcfg::from_bits(val as u8)
    }
    #[doc = "Power Configuration."]
    #[inline(always)]
    pub const fn set_pwrcfg(&mut self, val: super::vals::Pwrcfg) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pwrcfg {
    #[inline(always)]
    fn default() -> Pwrcfg {
        Pwrcfg(0)
    }
}
impl core::fmt::Debug for Pwrcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwrcfg")
            .field("pwrcfg", &self.pwrcfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwrcfg {{ pwrcfg: {:?} }}", self.pwrcfg())
    }
}
#[doc = "Power Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrctrl(pub u32);
impl Pwrctrl {
    #[doc = "Analog Switch Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn anasw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Switch Selection."]
    #[inline(always)]
    pub const fn set_anasw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pwrctrl {
    #[inline(always)]
    fn default() -> Pwrctrl {
        Pwrctrl(0)
    }
}
impl core::fmt::Debug for Pwrctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwrctrl")
            .field("anasw", &self.anasw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwrctrl {{ anasw: {=bool:?} }}", self.anasw())
    }
}
#[doc = "Regulator and Supply Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrlock(pub u32);
impl Pwrlock {
    #[doc = "Regulator and Supply Configuration Lock Key."]
    #[must_use]
    #[inline(always)]
    pub const fn lockkey(&self) -> super::super::msc::vals::Lockkey {
        let val = (self.0 >> 0usize) & 0xffff;
        super::super::msc::vals::Lockkey::from_bits(val as u16)
    }
    #[doc = "Regulator and Supply Configuration Lock Key."]
    #[inline(always)]
    pub const fn set_lockkey(&mut self, val: super::super::msc::vals::Lockkey) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pwrlock {
    #[inline(always)]
    fn default() -> Pwrlock {
        Pwrlock(0)
    }
}
impl core::fmt::Debug for Pwrlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwrlock")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwrlock {{ lockkey: {:?} }}", self.lockkey())
    }
}
#[doc = "Memory Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram0ctrl(pub u32);
impl Ram0ctrl {
    #[doc = "RAM0 Blockset Power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn rampowerdown(&self) -> super::vals::Rampowerdown {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Rampowerdown::from_bits(val as u8)
    }
    #[doc = "RAM0 Blockset Power-down."]
    #[inline(always)]
    pub const fn set_rampowerdown(&mut self, val: super::vals::Rampowerdown) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ram0ctrl {
    #[inline(always)]
    fn default() -> Ram0ctrl {
        Ram0ctrl(0)
    }
}
impl core::fmt::Debug for Ram0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram0ctrl")
            .field("rampowerdown", &self.rampowerdown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ram0ctrl {{ rampowerdown: {:?} }}", self.rampowerdown())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "VMON Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonrdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMON Ready."]
    #[inline(always)]
    pub const fn set_vmonrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VMON AVDD Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonavdd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VMON AVDD Channel."]
    #[inline(always)]
    pub const fn set_vmonavdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Alternate VMON AVDD Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonaltavdd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate VMON AVDD Channel."]
    #[inline(always)]
    pub const fn set_vmonaltavdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VMON DVDD Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmondvdd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VMON DVDD Channel."]
    #[inline(always)]
    pub const fn set_vmondvdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VMON IOVDD0 Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD0 Channel."]
    #[inline(always)]
    pub const fn set_vmonio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VMON VDDFLASH Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonfvdd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VMON VDDFLASH Channel."]
    #[inline(always)]
    pub const fn set_vmonfvdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IO Retention Status."]
    #[must_use]
    #[inline(always)]
    pub const fn em4ioret(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "IO Retention Status."]
    #[inline(always)]
    pub const fn set_em4ioret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
            .field("vmonrdy", &self.vmonrdy())
            .field("vmonavdd", &self.vmonavdd())
            .field("vmonaltavdd", &self.vmonaltavdd())
            .field("vmondvdd", &self.vmondvdd())
            .field("vmonio0", &self.vmonio0())
            .field("vmonfvdd", &self.vmonfvdd())
            .field("em4ioret", &self.em4ioret())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ vmonrdy: {=bool:?}, vmonavdd: {=bool:?}, vmonaltavdd: {=bool:?}, vmondvdd: {=bool:?}, vmonio0: {=bool:?}, vmonfvdd: {=bool:?}, em4ioret: {=bool:?} }}" , self . vmonrdy () , self . vmonavdd () , self . vmonaltavdd () , self . vmondvdd () , self . vmonio0 () , self . vmonfvdd () , self . em4ioret ())
    }
}
#[doc = "Value of Last Temperature Measurement."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Temp(pub u32);
impl Temp {
    #[doc = "Temperature Measurement."]
    #[must_use]
    #[inline(always)]
    pub const fn temp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Temperature Measurement."]
    #[inline(always)]
    pub const fn set_temp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Temp {
    #[inline(always)]
    fn default() -> Temp {
        Temp(0)
    }
}
impl core::fmt::Debug for Temp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Temp").field("temp", &self.temp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Temp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Temp {{ temp: {=u8:?} }}", self.temp())
    }
}
#[doc = "Temperature Limits for Interrupt Generation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Templimits(pub u32);
impl Templimits {
    #[doc = "Temperature Low Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn templow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Temperature Low Limit."]
    #[inline(always)]
    pub const fn set_templow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Temperature High Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn temphigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Temperature High Limit."]
    #[inline(always)]
    pub const fn set_temphigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enable EM4 Wakeup Due to Low/high Temperature."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wuen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable EM4 Wakeup Due to Low/high Temperature."]
    #[inline(always)]
    pub const fn set_em4wuen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Templimits {
    #[inline(always)]
    fn default() -> Templimits {
        Templimits(0)
    }
}
impl core::fmt::Debug for Templimits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Templimits")
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .field("em4wuen", &self.em4wuen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Templimits {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Templimits {{ templow: {=u8:?}, temphigh: {=u8:?}, em4wuen: {=bool:?} }}",
            self.templow(),
            self.temphigh(),
            self.em4wuen()
        )
    }
}
#[doc = "Test Lock Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testlock(pub u32);
impl Testlock {
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
impl Default for Testlock {
    #[inline(always)]
    fn default() -> Testlock {
        Testlock(0)
    }
}
impl core::fmt::Debug for Testlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Testlock")
            .field("lockkey", &self.lockkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Testlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Testlock {{ lockkey: {:?} }}", self.lockkey())
    }
}
#[doc = "Alternate VMON AVDD Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmonaltavddctrl(pub u32);
impl Vmonaltavddctrl {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rise Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn risewu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rise Wakeup."]
    #[inline(always)]
    pub const fn set_risewu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Fall Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn fallwu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Fall Wakeup."]
    #[inline(always)]
    pub const fn set_fallwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Threshold Fine Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn thresfine(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Fine Adjust."]
    #[inline(always)]
    pub const fn set_thresfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Threshold Coarse Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn threscoarse(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Coarse Adjust."]
    #[inline(always)]
    pub const fn set_threscoarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Vmonaltavddctrl {
    #[inline(always)]
    fn default() -> Vmonaltavddctrl {
        Vmonaltavddctrl(0)
    }
}
impl core::fmt::Debug for Vmonaltavddctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmonaltavddctrl")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("thresfine", &self.thresfine())
            .field("threscoarse", &self.threscoarse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vmonaltavddctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmonaltavddctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, thresfine: {=u8:?}, threscoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . thresfine () , self . threscoarse ())
    }
}
#[doc = "VMON AVDD Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmonavddctrl(pub u32);
impl Vmonavddctrl {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rise Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn risewu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rise Wakeup."]
    #[inline(always)]
    pub const fn set_risewu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Fall Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn fallwu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Fall Wakeup."]
    #[inline(always)]
    pub const fn set_fallwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling Threshold Fine Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn fallthresfine(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Falling Threshold Fine Adjust."]
    #[inline(always)]
    pub const fn set_fallthresfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Falling Threshold Coarse Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn fallthrescoarse(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Falling Threshold Coarse Adjust."]
    #[inline(always)]
    pub const fn set_fallthrescoarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Rising Threshold Fine Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn risethresfine(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Rising Threshold Fine Adjust."]
    #[inline(always)]
    pub const fn set_risethresfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Rising Threshold Coarse Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn risethrescoarse(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Rising Threshold Coarse Adjust."]
    #[inline(always)]
    pub const fn set_risethrescoarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Vmonavddctrl {
    #[inline(always)]
    fn default() -> Vmonavddctrl {
        Vmonavddctrl(0)
    }
}
impl core::fmt::Debug for Vmonavddctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmonavddctrl")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("fallthresfine", &self.fallthresfine())
            .field("fallthrescoarse", &self.fallthrescoarse())
            .field("risethresfine", &self.risethresfine())
            .field("risethrescoarse", &self.risethrescoarse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vmonavddctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmonavddctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, fallthresfine: {=u8:?}, fallthrescoarse: {=u8:?}, risethresfine: {=u8:?}, risethrescoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . fallthresfine () , self . fallthrescoarse () , self . risethresfine () , self . risethrescoarse ())
    }
}
#[doc = "VMON DVDD Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmondvddctrl(pub u32);
impl Vmondvddctrl {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rise Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn risewu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rise Wakeup."]
    #[inline(always)]
    pub const fn set_risewu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Fall Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn fallwu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Fall Wakeup."]
    #[inline(always)]
    pub const fn set_fallwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Threshold Fine Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn thresfine(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Fine Adjust."]
    #[inline(always)]
    pub const fn set_thresfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Threshold Coarse Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn threscoarse(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Coarse Adjust."]
    #[inline(always)]
    pub const fn set_threscoarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Vmondvddctrl {
    #[inline(always)]
    fn default() -> Vmondvddctrl {
        Vmondvddctrl(0)
    }
}
impl core::fmt::Debug for Vmondvddctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmondvddctrl")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("thresfine", &self.thresfine())
            .field("threscoarse", &self.threscoarse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vmondvddctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmondvddctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, thresfine: {=u8:?}, threscoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . thresfine () , self . threscoarse ())
    }
}
#[doc = "VMON IOVDD0 Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmonio0ctrl(pub u32);
impl Vmonio0ctrl {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rise Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn risewu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rise Wakeup."]
    #[inline(always)]
    pub const fn set_risewu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Fall Wakeup."]
    #[must_use]
    #[inline(always)]
    pub const fn fallwu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Fall Wakeup."]
    #[inline(always)]
    pub const fn set_fallwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EM4 IO0 Retention Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn retdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 IO0 Retention Disable."]
    #[inline(always)]
    pub const fn set_retdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Threshold Fine Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn thresfine(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Fine Adjust."]
    #[inline(always)]
    pub const fn set_thresfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Threshold Coarse Adjust."]
    #[must_use]
    #[inline(always)]
    pub const fn threscoarse(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold Coarse Adjust."]
    #[inline(always)]
    pub const fn set_threscoarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Vmonio0ctrl {
    #[inline(always)]
    fn default() -> Vmonio0ctrl {
        Vmonio0ctrl(0)
    }
}
impl core::fmt::Debug for Vmonio0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmonio0ctrl")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("retdis", &self.retdis())
            .field("thresfine", &self.thresfine())
            .field("threscoarse", &self.threscoarse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vmonio0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmonio0ctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, retdis: {=bool:?}, thresfine: {=u8:?}, threscoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . retdis () , self . thresfine () , self . threscoarse ())
    }
}
