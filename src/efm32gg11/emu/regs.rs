#[doc = "Backup Power Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buctrl(pub u32);
impl Buctrl {
    #[doc = "Enable Backup Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Backup Mode."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Backup Mode Status Export."]
    #[must_use]
    #[inline(always)]
    pub const fn staten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Backup Mode Status Export."]
    #[inline(always)]
    pub const fn set_staten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable BU_VIN Probing."]
    #[must_use]
    #[inline(always)]
    pub const fn buvinprobeen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable BU_VIN Probing."]
    #[inline(always)]
    pub const fn set_buvinprobeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BU_VOUT Resistor Select."]
    #[must_use]
    #[inline(always)]
    pub const fn voutres(&self) -> super::vals::Voutres {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Voutres::from_bits(val as u8)
    }
    #[doc = "BU_VOUT Resistor Select."]
    #[inline(always)]
    pub const fn set_voutres(&mut self, val: super::vals::Voutres) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Power Domain Resistor Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrres(&self) -> super::vals::Pwrres {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pwrres::from_bits(val as u8)
    }
    #[doc = "Power Domain Resistor Select."]
    #[inline(always)]
    pub const fn set_pwrres(&mut self, val: super::vals::Pwrres) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Power Connection Configuration in Backup Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn buactpwrcon(&self) -> super::vals::Buactpwrcon {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Buactpwrcon::from_bits(val as u8)
    }
    #[doc = "Power Connection Configuration in Backup Mode."]
    #[inline(always)]
    pub const fn set_buactpwrcon(&mut self, val: super::vals::Buactpwrcon) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Power Connection Configuration When Not in Backup Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn buinactpwrcon(&self) -> super::vals::Buinactpwrcon {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Buinactpwrcon::from_bits(val as u8)
    }
    #[doc = "Power Connection Configuration When Not in Backup Mode."]
    #[inline(always)]
    pub const fn set_buinactpwrcon(&mut self, val: super::vals::Buinactpwrcon) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Disable MAIN-BU Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn dismaxcomp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Disable MAIN-BU Comparator."]
    #[inline(always)]
    pub const fn set_dismaxcomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Buctrl {
    #[inline(always)]
    fn default() -> Buctrl {
        Buctrl(0)
    }
}
impl core::fmt::Debug for Buctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buctrl")
            .field("en", &self.en())
            .field("staten", &self.staten())
            .field("buvinprobeen", &self.buvinprobeen())
            .field("voutres", &self.voutres())
            .field("pwrres", &self.pwrres())
            .field("buactpwrcon", &self.buactpwrcon())
            .field("buinactpwrcon", &self.buinactpwrcon())
            .field("dismaxcomp", &self.dismaxcomp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Buctrl {{ en: {=bool:?}, staten: {=bool:?}, buvinprobeen: {=bool:?}, voutres: {:?}, pwrres: {:?}, buactpwrcon: {:?}, buinactpwrcon: {:?}, dismaxcomp: {=bool:?} }}" , self . en () , self . staten () , self . buvinprobeen () , self . voutres () , self . pwrres () , self . buactpwrcon () , self . buinactpwrcon () , self . dismaxcomp ())
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
    #[doc = "EM01 Voltage Scale Command to Scale to Voltage Scale Level 0."]
    #[must_use]
    #[inline(always)]
    pub const fn em01vscale0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EM01 Voltage Scale Command to Scale to Voltage Scale Level 0."]
    #[inline(always)]
    pub const fn set_em01vscale0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EM01 Voltage Scale Command to Scale to Voltage Scale Level 2."]
    #[must_use]
    #[inline(always)]
    pub const fn em01vscale2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EM01 Voltage Scale Command to Scale to Voltage Scale Level 2."]
    #[inline(always)]
    pub const fn set_em01vscale2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            .field("em01vscale0", &self.em01vscale0())
            .field("em01vscale2", &self.em01vscale2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ em4unlatch: {=bool:?}, em01vscale0: {=bool:?}, em01vscale2: {=bool:?} }}",
            self.em4unlatch(),
            self.em01vscale0(),
            self.em01vscale2()
        )
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
    #[doc = "Disable BOD in EM2."]
    #[must_use]
    #[inline(always)]
    pub const fn em2boddis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disable BOD in EM2."]
    #[inline(always)]
    pub const fn set_em2boddis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn em01ld(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_em01ld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn em23vscaleautowsen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage."]
    #[inline(always)]
    pub const fn set_em23vscaleautowsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EM23 Voltage Scale."]
    #[must_use]
    #[inline(always)]
    pub const fn em23vscale(&self) -> super::vals::Em23vscale {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Em23vscale::from_bits(val as u8)
    }
    #[doc = "EM23 Voltage Scale."]
    #[inline(always)]
    pub const fn set_em23vscale(&mut self, val: super::vals::Em23vscale) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "EM4H Voltage Scale."]
    #[must_use]
    #[inline(always)]
    pub const fn em4hvscale(&self) -> super::vals::Em4hvscale {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Em4hvscale::from_bits(val as u8)
    }
    #[doc = "EM4H Voltage Scale."]
    #[inline(always)]
    pub const fn set_em4hvscale(&mut self, val: super::vals::Em4hvscale) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
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
            .field("em2boddis", &self.em2boddis())
            .field("em01ld", &self.em01ld())
            .field("em23vscaleautowsen", &self.em23vscaleautowsen())
            .field("em23vscale", &self.em23vscale())
            .field("em4hvscale", &self.em4hvscale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ em2block: {=bool:?}, em2boddis: {=bool:?}, em01ld: {=bool:?}, em23vscaleautowsen: {=bool:?}, em23vscale: {:?}, em4hvscale: {:?} }}" , self . em2block () , self . em2boddis () , self . em01ld () , self . em23vscaleautowsen () , self . em23vscale () , self . em4hvscale ())
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
    #[doc = "LP Mode Hysteresis Selection for EM23 and EM4H."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmphysselem234h(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "LP Mode Hysteresis Selection for EM23 and EM4H."]
    #[inline(always)]
    pub const fn set_lpcmphysselem234h(&mut self, val: u8) {
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
            .field("lpcmphysselem234h", &self.lpcmphysselem234h())
            .field("lpvrefdutyen", &self.lpvrefdutyen())
            .field("lpblank", &self.lpblank())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclpctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcdclpctrl {{ lpcmphysselem234h: {=u8:?}, lpvrefdutyen: {=bool:?}, lpblank: {=u8:?} }}" , self . lpcmphysselem234h () , self . lpvrefdutyen () , self . lpblank ())
    }
}
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdclpem01cfg(pub u32);
impl Dcdclpem01cfg {
    #[doc = "LP Mode Comparator Bias Selection for EM01."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmpbiasem01(&self) -> super::vals::Lpcmpbiasem01 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lpcmpbiasem01::from_bits(val as u8)
    }
    #[doc = "LP Mode Comparator Bias Selection for EM01."]
    #[inline(always)]
    pub const fn set_lpcmpbiasem01(&mut self, val: super::vals::Lpcmpbiasem01) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LP Mode Hysteresis Selection for EM01."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmphysselem01(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "LP Mode Hysteresis Selection for EM01."]
    #[inline(always)]
    pub const fn set_lpcmphysselem01(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Dcdclpem01cfg {
    #[inline(always)]
    fn default() -> Dcdclpem01cfg {
        Dcdclpem01cfg(0)
    }
}
impl core::fmt::Debug for Dcdclpem01cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdclpem01cfg")
            .field("lpcmpbiasem01", &self.lpcmpbiasem01())
            .field("lpcmphysselem01", &self.lpcmphysselem01())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdclpem01cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdclpem01cfg {{ lpcmpbiasem01: {:?}, lpcmphysselem01: {=u8:?} }}",
            self.lpcmpbiasem01(),
            self.lpcmphysselem01()
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
    #[doc = "Disable LP Mode Hysteresis in the State Machine Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmphysdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LP Mode Hysteresis in the State Machine Control."]
    #[inline(always)]
    pub const fn set_lpcmphysdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator Threshold on the High Side."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmphyshi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Threshold on the High Side."]
    #[inline(always)]
    pub const fn set_lpcmphyshi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM."]
    #[must_use]
    #[inline(always)]
    pub const fn lnforceccmimm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM."]
    #[inline(always)]
    pub const fn set_lnforceccmimm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
    #[doc = "LP Mode Comparator Bias Selection for EM23 or EM4H."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmpbiasem234h(&self) -> super::vals::Lpcmpbiasem234h {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Lpcmpbiasem234h::from_bits(val as u8)
    }
    #[doc = "LP Mode Comparator Bias Selection for EM23 or EM4H."]
    #[inline(always)]
    pub const fn set_lpcmpbiasem234h(&mut self, val: super::vals::Lpcmpbiasem234h) {
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
            .field("lpcmphysdis", &self.lpcmphysdis())
            .field("lpcmphyshi", &self.lpcmphyshi())
            .field("lnforceccmimm", &self.lnforceccmimm())
            .field("pfetcnt", &self.pfetcnt())
            .field("nfetcnt", &self.nfetcnt())
            .field("byplimsel", &self.byplimsel())
            .field("lpclimilimsel", &self.lpclimilimsel())
            .field("lnclimilimsel", &self.lnclimilimsel())
            .field("lpcmpbiasem234h", &self.lpcmpbiasem234h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdcmiscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcdcmiscctrl {{ lnforceccm: {=bool:?}, lpcmphysdis: {=bool:?}, lpcmphyshi: {=bool:?}, lnforceccmimm: {=bool:?}, pfetcnt: {=u8:?}, nfetcnt: {=u8:?}, byplimsel: {=u8:?}, lpclimilimsel: {=u8:?}, lnclimilimsel: {=u8:?}, lpcmpbiasem234h: {:?} }}" , self . lnforceccm () , self . lpcmphysdis () , self . lpcmphyshi () , self . lnforceccmimm () , self . pfetcnt () , self . nfetcnt () , self . byplimsel () , self . lpclimilimsel () , self . lnclimilimsel () , self . lpcmpbiasem234h ())
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
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em23pernoretaincmd(pub u32);
impl Em23pernoretaincmd {
    #[doc = "Clears Status Bit of ACMP0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0unlock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ACMP0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_acmp0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears Status Bit of ACMP1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1unlock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ACMP1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_acmp1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clears Status Bit of PCNT0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0unlock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of PCNT0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_pcnt0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clears Status Bit of PCNT1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of PCNT1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_pcnt1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clears Status Bit of PCNT2 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2unlock(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of PCNT2 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_pcnt2unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clears Status Bit of I2C0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c0unlock(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of I2C0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_i2c0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clears Status Bit of I2C1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1unlock(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of I2C1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_i2c1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Clears Status Bit of DAC0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0unlock(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of DAC0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_dac0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clears Status Bit of IDAC0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn idac0unlock(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of IDAC0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_idac0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clears Status Bit of ADC0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0unlock(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ADC0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_adc0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clears Status Bit of LETIMER0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0unlock(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LETIMER0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_letimer0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Clears Status Bit of WDOG0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0unlock(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of WDOG0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_wdog0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Clears Status Bit of WDOG1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1unlock(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of WDOG1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_wdog1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clears Status Bit of LESENSE0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense0unlock(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LESENSE0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_lesense0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Clears Status Bit of CSEN and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn csenunlock(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of CSEN and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_csenunlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Clears Status Bit of LEUART0 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0unlock(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LEUART0 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_leuart0unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Clears Status Bit of LEUART1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1unlock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LEUART1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_leuart1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Clears Status Bit of LCD and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn lcdunlock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LCD and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_lcdunlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Clears Status Bit of LETIMER1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1unlock(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of LETIMER1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_letimer1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Clears Status Bit of I2C2 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c2unlock(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of I2C2 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_i2c2unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Clears Status Bit of ADC1 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1unlock(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ADC1 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_adc1unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Clears Status Bit of ACMP2 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2unlock(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ACMP2 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_acmp2unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clears Status Bit of ACMP3 and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3unlock(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of ACMP3 and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_acmp3unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Clears Status Bit of RTC and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcunlock(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of RTC and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_rtcunlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Clears Status Bit of USB and Unlocks Access to It."]
    #[must_use]
    #[inline(always)]
    pub const fn usbunlock(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Clears Status Bit of USB and Unlocks Access to It."]
    #[inline(always)]
    pub const fn set_usbunlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Em23pernoretaincmd {
    #[inline(always)]
    fn default() -> Em23pernoretaincmd {
        Em23pernoretaincmd(0)
    }
}
impl core::fmt::Debug for Em23pernoretaincmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Em23pernoretaincmd")
            .field("acmp0unlock", &self.acmp0unlock())
            .field("acmp1unlock", &self.acmp1unlock())
            .field("pcnt0unlock", &self.pcnt0unlock())
            .field("pcnt1unlock", &self.pcnt1unlock())
            .field("pcnt2unlock", &self.pcnt2unlock())
            .field("i2c0unlock", &self.i2c0unlock())
            .field("i2c1unlock", &self.i2c1unlock())
            .field("dac0unlock", &self.dac0unlock())
            .field("idac0unlock", &self.idac0unlock())
            .field("adc0unlock", &self.adc0unlock())
            .field("letimer0unlock", &self.letimer0unlock())
            .field("wdog0unlock", &self.wdog0unlock())
            .field("wdog1unlock", &self.wdog1unlock())
            .field("lesense0unlock", &self.lesense0unlock())
            .field("csenunlock", &self.csenunlock())
            .field("leuart0unlock", &self.leuart0unlock())
            .field("leuart1unlock", &self.leuart1unlock())
            .field("lcdunlock", &self.lcdunlock())
            .field("letimer1unlock", &self.letimer1unlock())
            .field("i2c2unlock", &self.i2c2unlock())
            .field("adc1unlock", &self.adc1unlock())
            .field("acmp2unlock", &self.acmp2unlock())
            .field("acmp3unlock", &self.acmp3unlock())
            .field("rtcunlock", &self.rtcunlock())
            .field("usbunlock", &self.usbunlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em23pernoretaincmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Em23pernoretaincmd {{ acmp0unlock: {=bool:?}, acmp1unlock: {=bool:?}, pcnt0unlock: {=bool:?}, pcnt1unlock: {=bool:?}, pcnt2unlock: {=bool:?}, i2c0unlock: {=bool:?}, i2c1unlock: {=bool:?}, dac0unlock: {=bool:?}, idac0unlock: {=bool:?}, adc0unlock: {=bool:?}, letimer0unlock: {=bool:?}, wdog0unlock: {=bool:?}, wdog1unlock: {=bool:?}, lesense0unlock: {=bool:?}, csenunlock: {=bool:?}, leuart0unlock: {=bool:?}, leuart1unlock: {=bool:?}, lcdunlock: {=bool:?}, letimer1unlock: {=bool:?}, i2c2unlock: {=bool:?}, adc1unlock: {=bool:?}, acmp2unlock: {=bool:?}, acmp3unlock: {=bool:?}, rtcunlock: {=bool:?}, usbunlock: {=bool:?} }}" , self . acmp0unlock () , self . acmp1unlock () , self . pcnt0unlock () , self . pcnt1unlock () , self . pcnt2unlock () , self . i2c0unlock () , self . i2c1unlock () , self . dac0unlock () , self . idac0unlock () , self . adc0unlock () , self . letimer0unlock () , self . wdog0unlock () , self . wdog1unlock () , self . lesense0unlock () , self . csenunlock () , self . leuart0unlock () , self . leuart1unlock () , self . lcdunlock () , self . letimer1unlock () , self . i2c2unlock () , self . adc1unlock () , self . acmp2unlock () , self . acmp3unlock () , self . rtcunlock () , self . usbunlock ())
    }
}
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em23pernoretainctrl(pub u32);
impl Em23pernoretainctrl {
    #[doc = "Allow Power Down of ACMP0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0dis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ACMP0 During EM23."]
    #[inline(always)]
    pub const fn set_acmp0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow Power Down of ACMP1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ACMP1 During EM23."]
    #[inline(always)]
    pub const fn set_acmp1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Power Down of PCNT0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0dis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of PCNT0 During EM23."]
    #[inline(always)]
    pub const fn set_pcnt0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allow Power Down of PCNT1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1dis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of PCNT1 During EM23."]
    #[inline(always)]
    pub const fn set_pcnt1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Allow Power Down of PCNT2 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2dis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of PCNT2 During EM23."]
    #[inline(always)]
    pub const fn set_pcnt2dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Allow Power Down of I2C0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c0dis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of I2C0 During EM23."]
    #[inline(always)]
    pub const fn set_i2c0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Allow Power Down of I2C1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1dis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of I2C1 During EM23."]
    #[inline(always)]
    pub const fn set_i2c1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Allow Power Down of DAC0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn vdac0dis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of DAC0 During EM23."]
    #[inline(always)]
    pub const fn set_vdac0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Allow Power Down of IDAC0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn idac0dis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of IDAC0 During EM23."]
    #[inline(always)]
    pub const fn set_idac0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow Power Down of ADC0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0dis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ADC0 During EM23."]
    #[inline(always)]
    pub const fn set_adc0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Allow Power Down of LETIMER0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0dis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LETIMER0 During EM23."]
    #[inline(always)]
    pub const fn set_letimer0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Allow Power Down of WDOG0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0dis(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of WDOG0 During EM23."]
    #[inline(always)]
    pub const fn set_wdog0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Allow Power Down of WDOG1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1dis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of WDOG1 During EM23."]
    #[inline(always)]
    pub const fn set_wdog1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Allow Power Down of LESENSE0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense0dis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LESENSE0 During EM23."]
    #[inline(always)]
    pub const fn set_lesense0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Allow Power Down of CSEN During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn csendis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of CSEN During EM23."]
    #[inline(always)]
    pub const fn set_csendis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Allow Power Down of LEUART0 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0dis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LEUART0 During EM23."]
    #[inline(always)]
    pub const fn set_leuart0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow Power Down of LEUART1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1dis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LEUART1 During EM23."]
    #[inline(always)]
    pub const fn set_leuart1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow Power Down of LCD During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn lcddis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LCD During EM23."]
    #[inline(always)]
    pub const fn set_lcddis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Allow Power Down of LETIMER1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1dis(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of LETIMER1 During EM23."]
    #[inline(always)]
    pub const fn set_letimer1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Allow Power Down of I2C2 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c2dis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of I2C2 During EM23."]
    #[inline(always)]
    pub const fn set_i2c2dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Allow Power Down of ADC1 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1dis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ADC1 During EM23."]
    #[inline(always)]
    pub const fn set_adc1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Allow Power Down of ACMP2 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2dis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ACMP2 During EM23."]
    #[inline(always)]
    pub const fn set_acmp2dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Allow Power Down of ACMP3 During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3dis(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of ACMP3 During EM23."]
    #[inline(always)]
    pub const fn set_acmp3dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Allow Power Down of RTC During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcdis(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of RTC During EM23."]
    #[inline(always)]
    pub const fn set_rtcdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Allow Power Down of USB During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn usbdis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Power Down of USB During EM23."]
    #[inline(always)]
    pub const fn set_usbdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Em23pernoretainctrl {
    #[inline(always)]
    fn default() -> Em23pernoretainctrl {
        Em23pernoretainctrl(0)
    }
}
impl core::fmt::Debug for Em23pernoretainctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Em23pernoretainctrl")
            .field("acmp0dis", &self.acmp0dis())
            .field("acmp1dis", &self.acmp1dis())
            .field("pcnt0dis", &self.pcnt0dis())
            .field("pcnt1dis", &self.pcnt1dis())
            .field("pcnt2dis", &self.pcnt2dis())
            .field("i2c0dis", &self.i2c0dis())
            .field("i2c1dis", &self.i2c1dis())
            .field("vdac0dis", &self.vdac0dis())
            .field("idac0dis", &self.idac0dis())
            .field("adc0dis", &self.adc0dis())
            .field("letimer0dis", &self.letimer0dis())
            .field("wdog0dis", &self.wdog0dis())
            .field("wdog1dis", &self.wdog1dis())
            .field("lesense0dis", &self.lesense0dis())
            .field("csendis", &self.csendis())
            .field("leuart0dis", &self.leuart0dis())
            .field("leuart1dis", &self.leuart1dis())
            .field("lcddis", &self.lcddis())
            .field("letimer1dis", &self.letimer1dis())
            .field("i2c2dis", &self.i2c2dis())
            .field("adc1dis", &self.adc1dis())
            .field("acmp2dis", &self.acmp2dis())
            .field("acmp3dis", &self.acmp3dis())
            .field("rtcdis", &self.rtcdis())
            .field("usbdis", &self.usbdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em23pernoretainctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Em23pernoretainctrl {{ acmp0dis: {=bool:?}, acmp1dis: {=bool:?}, pcnt0dis: {=bool:?}, pcnt1dis: {=bool:?}, pcnt2dis: {=bool:?}, i2c0dis: {=bool:?}, i2c1dis: {=bool:?}, vdac0dis: {=bool:?}, idac0dis: {=bool:?}, adc0dis: {=bool:?}, letimer0dis: {=bool:?}, wdog0dis: {=bool:?}, wdog1dis: {=bool:?}, lesense0dis: {=bool:?}, csendis: {=bool:?}, leuart0dis: {=bool:?}, leuart1dis: {=bool:?}, lcddis: {=bool:?}, letimer1dis: {=bool:?}, i2c2dis: {=bool:?}, adc1dis: {=bool:?}, acmp2dis: {=bool:?}, acmp3dis: {=bool:?}, rtcdis: {=bool:?}, usbdis: {=bool:?} }}" , self . acmp0dis () , self . acmp1dis () , self . pcnt0dis () , self . pcnt1dis () , self . pcnt2dis () , self . i2c0dis () , self . i2c1dis () , self . vdac0dis () , self . idac0dis () , self . adc0dis () , self . letimer0dis () , self . wdog0dis () , self . wdog1dis () , self . lesense0dis () , self . csendis () , self . leuart0dis () , self . leuart1dis () , self . lcddis () , self . letimer1dis () , self . i2c2dis () , self . adc1dis () , self . acmp2dis () , self . acmp3dis () , self . rtcdis () , self . usbdis ())
    }
}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Em23pernoretainstatus(pub u32);
impl Em23pernoretainstatus {
    #[doc = "Indicates If ACMP0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0locked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ACMP0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_acmp0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates If ACMP1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1locked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ACMP1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_acmp1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates If PCNT0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0locked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If PCNT0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_pcnt0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates If PCNT1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1locked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If PCNT1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_pcnt1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates If PCNT2 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2locked(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If PCNT2 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_pcnt2locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates If I2C0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c0locked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If I2C0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_i2c0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates If I2C1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1locked(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If I2C1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_i2c1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates If DAC0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0locked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If DAC0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_dac0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates If IDAC0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn idac0locked(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If IDAC0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_idac0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates If ADC0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0locked(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ADC0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_adc0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates If LETIMER0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0locked(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LETIMER0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_letimer0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates If WDOG0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog0locked(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If WDOG0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_wdog0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates If WDOG1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1locked(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If WDOG1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_wdog1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates If LESENSE0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense0locked(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LESENSE0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_lesense0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Indicates If CSEN Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn csenlocked(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If CSEN Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_csenlocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Indicates If LEUART0 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0locked(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LEUART0 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_leuart0locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Indicates If LEUART1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1locked(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LEUART1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_leuart1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates If LCD Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn lcdlocked(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LCD Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_lcdlocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Indicates If LETIMER1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1locked(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If LETIMER1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_letimer1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Indicates If I2C2 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c2locked(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If I2C2 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_i2c2locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Indicates If ADC1 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1locked(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ADC1 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_adc1locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Indicates If ACMP2 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2locked(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ACMP2 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_acmp2locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Indicates If ACMP3 Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3locked(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If ACMP3 Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_acmp3locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Indicates If RTC Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn rtclocked(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If RTC Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_rtclocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates If USB Powered Down During EM23."]
    #[must_use]
    #[inline(always)]
    pub const fn usblocked(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If USB Powered Down During EM23."]
    #[inline(always)]
    pub const fn set_usblocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Em23pernoretainstatus {
    #[inline(always)]
    fn default() -> Em23pernoretainstatus {
        Em23pernoretainstatus(0)
    }
}
impl core::fmt::Debug for Em23pernoretainstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Em23pernoretainstatus")
            .field("acmp0locked", &self.acmp0locked())
            .field("acmp1locked", &self.acmp1locked())
            .field("pcnt0locked", &self.pcnt0locked())
            .field("pcnt1locked", &self.pcnt1locked())
            .field("pcnt2locked", &self.pcnt2locked())
            .field("i2c0locked", &self.i2c0locked())
            .field("i2c1locked", &self.i2c1locked())
            .field("dac0locked", &self.dac0locked())
            .field("idac0locked", &self.idac0locked())
            .field("adc0locked", &self.adc0locked())
            .field("letimer0locked", &self.letimer0locked())
            .field("wdog0locked", &self.wdog0locked())
            .field("wdog1locked", &self.wdog1locked())
            .field("lesense0locked", &self.lesense0locked())
            .field("csenlocked", &self.csenlocked())
            .field("leuart0locked", &self.leuart0locked())
            .field("leuart1locked", &self.leuart1locked())
            .field("lcdlocked", &self.lcdlocked())
            .field("letimer1locked", &self.letimer1locked())
            .field("i2c2locked", &self.i2c2locked())
            .field("adc1locked", &self.adc1locked())
            .field("acmp2locked", &self.acmp2locked())
            .field("acmp3locked", &self.acmp3locked())
            .field("rtclocked", &self.rtclocked())
            .field("usblocked", &self.usblocked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Em23pernoretainstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Em23pernoretainstatus {{ acmp0locked: {=bool:?}, acmp1locked: {=bool:?}, pcnt0locked: {=bool:?}, pcnt1locked: {=bool:?}, pcnt2locked: {=bool:?}, i2c0locked: {=bool:?}, i2c1locked: {=bool:?}, dac0locked: {=bool:?}, idac0locked: {=bool:?}, adc0locked: {=bool:?}, letimer0locked: {=bool:?}, wdog0locked: {=bool:?}, wdog1locked: {=bool:?}, lesense0locked: {=bool:?}, csenlocked: {=bool:?}, leuart0locked: {=bool:?}, leuart1locked: {=bool:?}, lcdlocked: {=bool:?}, letimer1locked: {=bool:?}, i2c2locked: {=bool:?}, adc1locked: {=bool:?}, acmp2locked: {=bool:?}, acmp3locked: {=bool:?}, rtclocked: {=bool:?}, usblocked: {=bool:?} }}" , self . acmp0locked () , self . acmp1locked () , self . pcnt0locked () , self . pcnt1locked () , self . pcnt2locked () , self . i2c0locked () , self . i2c1locked () , self . dac0locked () , self . idac0locked () , self . adc0locked () , self . letimer0locked () , self . wdog0locked () , self . wdog1locked () , self . lesense0locked () , self . csenlocked () , self . leuart0locked () , self . leuart1locked () , self . lcdlocked () , self . letimer1locked () , self . i2c2locked () , self . adc1locked () , self . acmp2locked () , self . acmp3locked () , self . rtclocked () , self . usblocked ())
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
    #[doc = "VMONIO1FALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1fall(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VMONIO1FALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonio1fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VMONIO1RISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1rise(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "VMONIO1RISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonio1rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "R5VREADY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vready(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "R5VREADY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_r5vready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "VMONBUVDDFALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddfall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "VMONBUVDDFALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonbuvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "VMONBUVDDRISE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddrise(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "VMONBUVDDRISE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vmonbuvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "BURDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn burdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "BURDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_burdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "R5VVSINT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vvsint(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "R5VVSINT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_r5vvsint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    #[doc = "VSCALEDONE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vscaledone(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "VSCALEDONE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vscaledone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("vmonio1fall", &self.vmonio1fall())
            .field("vmonio1rise", &self.vmonio1rise())
            .field("r5vready", &self.r5vready())
            .field("vmonbuvddfall", &self.vmonbuvddfall())
            .field("vmonbuvddrise", &self.vmonbuvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("burdy", &self.burdy())
            .field("r5vvsint", &self.r5vvsint())
            .field("em23wakeup", &self.em23wakeup())
            .field("vscaledone", &self.vscaledone())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonio1fall: {=bool:?}, vmonio1rise: {=bool:?}, r5vready: {=bool:?}, vmonbuvddfall: {=bool:?}, vmonbuvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, burdy: {=bool:?}, r5vvsint: {=bool:?}, em23wakeup: {=bool:?}, vscaledone: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonio1fall () , self . vmonio1rise () , self . r5vready () , self . vmonbuvddfall () , self . vmonbuvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . burdy () , self . r5vvsint () , self . em23wakeup () , self . vscaledone () , self . temp () , self . templow () , self . temphigh ())
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
    #[doc = "VMON IOVDD1 Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1fall(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD1 Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonio1fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VMON IOVDD1 Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1rise(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD1 Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonio1rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "5V Regulator is Ready to Use."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vready(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "5V Regulator is Ready to Use."]
    #[inline(always)]
    pub const fn set_r5vready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "VMON BACKUP Channel Fall."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddfall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "VMON BACKUP Channel Fall."]
    #[inline(always)]
    pub const fn set_vmonbuvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "VMON BUVDD Channel Rise."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddrise(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "VMON BUVDD Channel Rise."]
    #[inline(always)]
    pub const fn set_vmonbuvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Backup Functionality Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn burdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Backup Functionality Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_burdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "5V Regulator Voltage Update Done."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vvsint(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "5V Regulator Voltage Update Done."]
    #[inline(always)]
    pub const fn set_r5vvsint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    #[doc = "Voltage Scale Steps Done IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn vscaledone(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Scale Steps Done IRQ."]
    #[inline(always)]
    pub const fn set_vscaledone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("vmonio1fall", &self.vmonio1fall())
            .field("vmonio1rise", &self.vmonio1rise())
            .field("r5vready", &self.r5vready())
            .field("vmonbuvddfall", &self.vmonbuvddfall())
            .field("vmonbuvddrise", &self.vmonbuvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("burdy", &self.burdy())
            .field("r5vvsint", &self.r5vvsint())
            .field("em23wakeup", &self.em23wakeup())
            .field("vscaledone", &self.vscaledone())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonio1fall: {=bool:?}, vmonio1rise: {=bool:?}, r5vready: {=bool:?}, vmonbuvddfall: {=bool:?}, vmonbuvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, burdy: {=bool:?}, r5vvsint: {=bool:?}, em23wakeup: {=bool:?}, vscaledone: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonio1fall () , self . vmonio1rise () , self . r5vready () , self . vmonbuvddfall () , self . vmonbuvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . burdy () , self . r5vvsint () , self . em23wakeup () , self . vscaledone () , self . temp () , self . templow () , self . temphigh ())
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
    #[doc = "Set VMONIO1FALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1fall(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONIO1FALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonio1fall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set VMONIO1RISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1rise(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONIO1RISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonio1rise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set R5VREADY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vready(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set R5VREADY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_r5vready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set VMONBUVDDFALL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddfall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONBUVDDFALL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonbuvddfall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set VMONBUVDDRISE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvddrise(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set VMONBUVDDRISE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vmonbuvddrise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Set BURDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn burdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Set BURDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_burdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set R5VVSINT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn r5vvsint(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set R5VVSINT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_r5vvsint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    #[doc = "Set VSCALEDONE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vscaledone(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Set VSCALEDONE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vscaledone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("vmonio1fall", &self.vmonio1fall())
            .field("vmonio1rise", &self.vmonio1rise())
            .field("r5vready", &self.r5vready())
            .field("vmonbuvddfall", &self.vmonbuvddfall())
            .field("vmonbuvddrise", &self.vmonbuvddrise())
            .field("pfetovercurrentlimit", &self.pfetovercurrentlimit())
            .field("nfetovercurrentlimit", &self.nfetovercurrentlimit())
            .field("dcdclprunning", &self.dcdclprunning())
            .field("dcdclnrunning", &self.dcdclnrunning())
            .field("dcdcinbypass", &self.dcdcinbypass())
            .field("burdy", &self.burdy())
            .field("r5vvsint", &self.r5vvsint())
            .field("em23wakeup", &self.em23wakeup())
            .field("vscaledone", &self.vscaledone())
            .field("temp", &self.temp())
            .field("templow", &self.templow())
            .field("temphigh", &self.temphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ vmonavddfall: {=bool:?}, vmonavddrise: {=bool:?}, vmonaltavddfall: {=bool:?}, vmonaltavddrise: {=bool:?}, vmondvddfall: {=bool:?}, vmondvddrise: {=bool:?}, vmonio0fall: {=bool:?}, vmonio0rise: {=bool:?}, vmonio1fall: {=bool:?}, vmonio1rise: {=bool:?}, r5vready: {=bool:?}, vmonbuvddfall: {=bool:?}, vmonbuvddrise: {=bool:?}, pfetovercurrentlimit: {=bool:?}, nfetovercurrentlimit: {=bool:?}, dcdclprunning: {=bool:?}, dcdclnrunning: {=bool:?}, dcdcinbypass: {=bool:?}, burdy: {=bool:?}, r5vvsint: {=bool:?}, em23wakeup: {=bool:?}, vscaledone: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}" , self . vmonavddfall () , self . vmonavddrise () , self . vmonaltavddfall () , self . vmonaltavddrise () , self . vmondvddfall () , self . vmondvddrise () , self . vmonio0fall () , self . vmonio0rise () , self . vmonio1fall () , self . vmonio1rise () , self . r5vready () , self . vmonbuvddfall () , self . vmonbuvddrise () , self . pfetovercurrentlimit () , self . nfetovercurrentlimit () , self . dcdclprunning () , self . dcdclnrunning () , self . dcdcinbypass () , self . burdy () , self . r5vvsint () , self . em23wakeup () , self . vscaledone () , self . temp () , self . templow () , self . temphigh ())
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
    #[doc = "This Field Selects the Input Supply Pin for the Digital LDO."]
    #[must_use]
    #[inline(always)]
    pub const fn regpwrsel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This Field Selects the Input Supply Pin for the Digital LDO."]
    #[inline(always)]
    pub const fn set_regpwrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Allows Immediate Switching of ANASW and REGPWRSEL Bitfields."]
    #[must_use]
    #[inline(always)]
    pub const fn immediatepwrswitch(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Allows Immediate Switching of ANASW and REGPWRSEL Bitfields."]
    #[inline(always)]
    pub const fn set_immediatepwrswitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("regpwrsel", &self.regpwrsel())
            .field("immediatepwrswitch", &self.immediatepwrswitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwrctrl {{ anasw: {=bool:?}, regpwrsel: {=bool:?}, immediatepwrswitch: {=bool:?} }}",
            self.anasw(),
            self.regpwrsel(),
            self.immediatepwrswitch()
        )
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
#[doc = "5V Regulator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5vadcctrl(pub u32);
impl R5vadcctrl {
    #[doc = "Enable the 5V Subsystem ADC MUX."]
    #[must_use]
    #[inline(always)]
    pub const fn enamux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 5V Subsystem ADC MUX."]
    #[inline(always)]
    pub const fn set_enamux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC Mux Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn amuxsel(&self) -> super::vals::Amuxsel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Amuxsel::from_bits(val as u8)
    }
    #[doc = "ADC Mux Selection."]
    #[inline(always)]
    pub const fn set_amuxsel(&mut self, val: super::vals::Amuxsel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for R5vadcctrl {
    #[inline(always)]
    fn default() -> R5vadcctrl {
        R5vadcctrl(0)
    }
}
impl core::fmt::Debug for R5vadcctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5vadcctrl")
            .field("enamux", &self.enamux())
            .field("amuxsel", &self.amuxsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5vadcctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "R5vadcctrl {{ enamux: {=bool:?}, amuxsel: {:?} }}",
            self.enamux(),
            self.amuxsel()
        )
    }
}
#[doc = "5V Regulator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5vctrl(pub u32);
impl R5vctrl {
    #[doc = "5V Regulator Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "5V Regulator Bypass."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable EM4 Wakeup Due to VBUS Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn em4wuen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable EM4 Wakeup Due to VBUS Detection."]
    #[inline(always)]
    pub const fn set_em4wuen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn imonen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS."]
    #[inline(always)]
    pub const fn set_imonen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "5V Input Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmode(&self) -> super::vals::Inputmode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Inputmode::from_bits(val as u8)
    }
    #[doc = "5V Input Mode."]
    #[inline(always)]
    pub const fn set_inputmode(&mut self, val: super::vals::Inputmode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for R5vctrl {
    #[inline(always)]
    fn default() -> R5vctrl {
        R5vctrl(0)
    }
}
impl core::fmt::Debug for R5vctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5vctrl")
            .field("bypass", &self.bypass())
            .field("em4wuen", &self.em4wuen())
            .field("imonen", &self.imonen())
            .field("inputmode", &self.inputmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5vctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "R5vctrl {{ bypass: {=bool:?}, em4wuen: {=bool:?}, imonen: {=bool:?}, inputmode: {:?} }}" , self . bypass () , self . em4wuen () , self . imonen () , self . inputmode ())
    }
}
#[doc = "5V Detector Enables."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5vdetctrl(pub u32);
impl R5vdetctrl {
    #[doc = "VREGI Detector Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn vregidetdis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VREGI Detector Disable."]
    #[inline(always)]
    pub const fn set_vregidetdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Detector Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdetdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detector Disable."]
    #[inline(always)]
    pub const fn set_vbusdetdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VREGO Detector Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn vregodetdis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VREGO Detector Disable."]
    #[inline(always)]
    pub const fn set_vregodetdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for R5vdetctrl {
    #[inline(always)]
    fn default() -> R5vdetctrl {
        R5vdetctrl(0)
    }
}
impl core::fmt::Debug for R5vdetctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5vdetctrl")
            .field("vregidetdis", &self.vregidetdis())
            .field("vbusdetdis", &self.vbusdetdis())
            .field("vregodetdis", &self.vregodetdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5vdetctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "R5vdetctrl {{ vregidetdis: {=bool:?}, vbusdetdis: {=bool:?}, vregodetdis: {=bool:?} }}" , self . vregidetdis () , self . vbusdetdis () , self . vregodetdis ())
    }
}
#[doc = "5V Regulator Voltage Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5voutlevel(pub u32);
impl R5voutlevel {
    #[doc = "5V Regulator Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn outlevel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "5V Regulator Voltage."]
    #[inline(always)]
    pub const fn set_outlevel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for R5voutlevel {
    #[inline(always)]
    fn default() -> R5voutlevel {
        R5voutlevel(0)
    }
}
impl core::fmt::Debug for R5voutlevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5voutlevel")
            .field("outlevel", &self.outlevel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5voutlevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "R5voutlevel {{ outlevel: {=u8:?} }}", self.outlevel())
    }
}
#[doc = "5V Detector Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5vstatus(pub u32);
impl R5vstatus {
    #[doc = "VREGI Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn vregidet(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VREGI Detected."]
    #[inline(always)]
    pub const fn set_vregidet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB VBUS Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB VBUS Detected."]
    #[inline(always)]
    pub const fn set_vbusdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VREGO Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn vregodet(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VREGO Detected."]
    #[inline(always)]
    pub const fn set_vregodet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output of the Supply Comparator Between VBUS and VREGI."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusgtvregi(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output of the Supply Comparator Between VBUS and VREGI."]
    #[inline(always)]
    pub const fn set_vbusgtvregi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Regulator Dropout Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ldodropoutdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Regulator Dropout Detection."]
    #[inline(always)]
    pub const fn set_ldodropoutdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates If the Regulator is Going Through a Cold Start."]
    #[must_use]
    #[inline(always)]
    pub const fn coldstart(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates If the Regulator is Going Through a Cold Start."]
    #[inline(always)]
    pub const fn set_coldstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for R5vstatus {
    #[inline(always)]
    fn default() -> R5vstatus {
        R5vstatus(0)
    }
}
impl core::fmt::Debug for R5vstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5vstatus")
            .field("vregidet", &self.vregidet())
            .field("vbusdet", &self.vbusdet())
            .field("vregodet", &self.vregodet())
            .field("vbusgtvregi", &self.vbusgtvregi())
            .field("ldodropoutdet", &self.ldodropoutdet())
            .field("coldstart", &self.coldstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5vstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "R5vstatus {{ vregidet: {=bool:?}, vbusdet: {=bool:?}, vregodet: {=bool:?}, vbusgtvregi: {=bool:?}, ldodropoutdet: {=bool:?}, coldstart: {=bool:?} }}" , self . vregidet () , self . vbusdet () , self . vregodet () , self . vbusgtvregi () , self . ldodropoutdet () , self . coldstart ())
    }
}
#[doc = "5V Read Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R5vsync(pub u32);
impl R5vsync {
    #[doc = "5V Regulator Voltage Register Transfer Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn outlevelbusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "5V Regulator Voltage Register Transfer Busy."]
    #[inline(always)]
    pub const fn set_outlevelbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for R5vsync {
    #[inline(always)]
    fn default() -> R5vsync {
        R5vsync(0)
    }
}
impl core::fmt::Debug for R5vsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R5vsync")
            .field("outlevelbusy", &self.outlevelbusy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R5vsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "R5vsync {{ outlevelbusy: {=bool:?} }}",
            self.outlevelbusy()
        )
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
    pub const fn rampowerdown(&self) -> super::vals::Ram0ctrlRampowerdown {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ram0ctrlRampowerdown::from_bits(val as u8)
    }
    #[doc = "RAM0 Blockset Power-down."]
    #[inline(always)]
    pub const fn set_rampowerdown(&mut self, val: super::vals::Ram0ctrlRampowerdown) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
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
#[doc = "Memory Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram1ctrl(pub u32);
impl Ram1ctrl {
    #[doc = "RAM1 Blockset Power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn rampowerdown(&self) -> super::vals::Ram1ctrlRampowerdown {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ram1ctrlRampowerdown::from_bits(val as u8)
    }
    #[doc = "RAM1 Blockset Power-down."]
    #[inline(always)]
    pub const fn set_rampowerdown(&mut self, val: super::vals::Ram1ctrlRampowerdown) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ram1ctrl {
    #[inline(always)]
    fn default() -> Ram1ctrl {
        Ram1ctrl(0)
    }
}
impl core::fmt::Debug for Ram1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram1ctrl")
            .field("rampowerdown", &self.rampowerdown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ram1ctrl {{ rampowerdown: {:?} }}", self.rampowerdown())
    }
}
#[doc = "Memory Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram2ctrl(pub u32);
impl Ram2ctrl {
    #[doc = "RAM2 Blockset Power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn rampowerdown(&self) -> super::vals::Ram2ctrlRampowerdown {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ram2ctrlRampowerdown::from_bits(val as u8)
    }
    #[doc = "RAM2 Blockset Power-down."]
    #[inline(always)]
    pub const fn set_rampowerdown(&mut self, val: super::vals::Ram2ctrlRampowerdown) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ram2ctrl {
    #[inline(always)]
    fn default() -> Ram2ctrl {
        Ram2ctrl(0)
    }
}
impl core::fmt::Debug for Ram2ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ram2ctrl")
            .field("rampowerdown", &self.rampowerdown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ram2ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ram2ctrl {{ rampowerdown: {:?} }}", self.rampowerdown())
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
    #[doc = "VMON IOVDD1 Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "VMON IOVDD1 Channel."]
    #[inline(always)]
    pub const fn set_vmonio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VMON BUVDD Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn vmonbuvdd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VMON BUVDD Channel."]
    #[inline(always)]
    pub const fn set_vmonbuvdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Backup Mode Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn burdy(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Backup Mode Ready."]
    #[inline(always)]
    pub const fn set_burdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Current Voltage Scale Value."]
    #[must_use]
    #[inline(always)]
    pub const fn vscale(&self) -> super::vals::Vscale {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Vscale::from_bits(val as u8)
    }
    #[doc = "Current Voltage Scale Value."]
    #[inline(always)]
    pub const fn set_vscale(&mut self, val: super::vals::Vscale) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "System is Busy Scaling Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vscalebusy(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "System is Busy Scaling Voltage."]
    #[inline(always)]
    pub const fn set_vscalebusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
    #[doc = "Temperature Measurement Active."]
    #[must_use]
    #[inline(always)]
    pub const fn tempactive(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Measurement Active."]
    #[inline(always)]
    pub const fn set_tempactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("vmonio1", &self.vmonio1())
            .field("vmonbuvdd", &self.vmonbuvdd())
            .field("burdy", &self.burdy())
            .field("vscale", &self.vscale())
            .field("vscalebusy", &self.vscalebusy())
            .field("em4ioret", &self.em4ioret())
            .field("tempactive", &self.tempactive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ vmonrdy: {=bool:?}, vmonavdd: {=bool:?}, vmonaltavdd: {=bool:?}, vmondvdd: {=bool:?}, vmonio0: {=bool:?}, vmonio1: {=bool:?}, vmonbuvdd: {=bool:?}, burdy: {=bool:?}, vscale: {:?}, vscalebusy: {=bool:?}, em4ioret: {=bool:?}, tempactive: {=bool:?} }}" , self . vmonrdy () , self . vmonavdd () , self . vmonaltavdd () , self . vmondvdd () , self . vmonio0 () , self . vmonio1 () , self . vmonbuvdd () , self . burdy () , self . vscale () , self . vscalebusy () , self . em4ioret () , self . tempactive ())
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
#[doc = "VMON BUVDD Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmonbuvddctrl(pub u32);
impl Vmonbuvddctrl {
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
impl Default for Vmonbuvddctrl {
    #[inline(always)]
    fn default() -> Vmonbuvddctrl {
        Vmonbuvddctrl(0)
    }
}
impl core::fmt::Debug for Vmonbuvddctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmonbuvddctrl")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("thresfine", &self.thresfine())
            .field("threscoarse", &self.threscoarse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vmonbuvddctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmonbuvddctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, thresfine: {=u8:?}, threscoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . thresfine () , self . threscoarse ())
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
#[doc = "VMON IOVDD1 Channel Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmonio1ctrl(pub u32);
impl Vmonio1ctrl {
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
    #[doc = "EM4 IO1 Retention Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn retdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EM4 IO1 Retention Disable."]
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
impl Default for Vmonio1ctrl {
    #[inline(always)]
    fn default() -> Vmonio1ctrl {
        Vmonio1ctrl(0)
    }
}
impl core::fmt::Debug for Vmonio1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vmonio1ctrl")
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
impl defmt::Format for Vmonio1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vmonio1ctrl {{ en: {=bool:?}, risewu: {=bool:?}, fallwu: {=bool:?}, retdis: {=bool:?}, thresfine: {=u8:?}, threscoarse: {=u8:?} }}" , self . en () , self . risewu () , self . fallwu () , self . retdis () , self . thresfine () , self . threscoarse ())
    }
}
