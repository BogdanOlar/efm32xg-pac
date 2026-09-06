#[doc = "CC Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc0Ctrl(pub u32);
impl Cc0Ctrl {
    #[doc = "CC Channel Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Cc0CtrlMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cc0CtrlMode::from_bits(val as u8)
    }
    #[doc = "CC Channel Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Cc0CtrlMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Compare Match Output Action."]
    #[must_use]
    #[inline(always)]
    pub const fn cmoa(&self) -> super::vals::Cc0CtrlCmoa {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cc0CtrlCmoa::from_bits(val as u8)
    }
    #[doc = "Compare Match Output Action."]
    #[inline(always)]
    pub const fn set_cmoa(&mut self, val: super::vals::Cc0CtrlCmoa) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Input Capture Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn icedge(&self) -> super::vals::Cc0CtrlIcedge {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cc0CtrlIcedge::from_bits(val as u8)
    }
    #[doc = "Input Capture Edge Select."]
    #[inline(always)]
    pub const fn set_icedge(&mut self, val: super::vals::Cc0CtrlIcedge) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Compare/Capture Channel PRS Input Channel Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Compare/Capture Channel PRS Input Channel Selection."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
    #[doc = "Capture Compare Channel Comparison Base."]
    #[must_use]
    #[inline(always)]
    pub const fn compbase(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Compare Channel Comparison Base."]
    #[inline(always)]
    pub const fn set_compbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture Compare Channel Comparison Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn compmask(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "Capture Compare Channel Comparison Mask."]
    #[inline(always)]
    pub const fn set_compmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Day Capture/Compare Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn daycc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Day Capture/Compare Selection."]
    #[inline(always)]
    pub const fn set_daycc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Cc0Ctrl {
    #[inline(always)]
    fn default() -> Cc0Ctrl {
        Cc0Ctrl(0)
    }
}
impl core::fmt::Debug for Cc0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cc0Ctrl")
            .field("mode", &self.mode())
            .field("cmoa", &self.cmoa())
            .field("icedge", &self.icedge())
            .field("prssel", &self.prssel())
            .field("compbase", &self.compbase())
            .field("compmask", &self.compmask())
            .field("daycc", &self.daycc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cc0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cc0Ctrl {{ mode: {:?}, cmoa: {:?}, icedge: {:?}, prssel: {:?}, compbase: {=bool:?}, compmask: {=u8:?}, daycc: {=bool:?} }}" , self . mode () , self . cmoa () , self . icedge () , self . prssel () , self . compbase () , self . compmask () , self . daycc ())
    }
}
#[doc = "Capture/Compare Date Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc0Date(pub u32);
impl Cc0Date {
    #[doc = "Day of Month/week, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn dayu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Day of Month/week, Units."]
    #[inline(always)]
    pub const fn set_dayu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Day of Month/week, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn dayt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Day of Month/week, Tens."]
    #[inline(always)]
    pub const fn set_dayt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn monthu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month, Units."]
    #[inline(always)]
    pub const fn set_monthu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn montht(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month, Tens."]
    #[inline(always)]
    pub const fn set_montht(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Cc0Date {
    #[inline(always)]
    fn default() -> Cc0Date {
        Cc0Date(0)
    }
}
impl core::fmt::Debug for Cc0Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cc0Date")
            .field("dayu", &self.dayu())
            .field("dayt", &self.dayt())
            .field("monthu", &self.monthu())
            .field("montht", &self.montht())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cc0Date {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cc0Date {{ dayu: {=u8:?}, dayt: {=u8:?}, monthu: {=u8:?}, montht: {=bool:?} }}",
            self.dayu(),
            self.dayt(),
            self.monthu(),
            self.montht()
        )
    }
}
#[doc = "Capture/Compare Time Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc0Time(pub u32);
impl Cc0Time {
    #[doc = "Seconds, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn secu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Seconds, Units."]
    #[inline(always)]
    pub const fn set_secu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Seconds, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn sect(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Seconds, Tens."]
    #[inline(always)]
    pub const fn set_sect(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Minutes, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn minu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minutes, Units."]
    #[inline(always)]
    pub const fn set_minu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minutes, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn mint(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minutes, Tens."]
    #[inline(always)]
    pub const fn set_mint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Hours, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn houru(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hours, Units."]
    #[inline(always)]
    pub const fn set_houru(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hours, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn hourt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hours, Tens."]
    #[inline(always)]
    pub const fn set_hourt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Cc0Time {
    #[inline(always)]
    fn default() -> Cc0Time {
        Cc0Time(0)
    }
}
impl core::fmt::Debug for Cc0Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cc0Time")
            .field("secu", &self.secu())
            .field("sect", &self.sect())
            .field("minu", &self.minu())
            .field("mint", &self.mint())
            .field("houru", &self.houru())
            .field("hourt", &self.hourt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cc0Time {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cc0Time {{ secu: {=u8:?}, sect: {=u8:?}, minu: {=u8:?}, mint: {=u8:?}, houru: {=u8:?}, hourt: {=u8:?} }}" , self . secu () , self . sect () , self . minu () , self . mint () , self . houru () , self . hourt ())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Clear RTCC_STATUS Register."]
    #[must_use]
    #[inline(always)]
    pub const fn clrstatus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear RTCC_STATUS Register."]
    #[inline(always)]
    pub const fn set_clrstatus(&mut self, val: bool) {
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
            .field("clrstatus", &self.clrstatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd {{ clrstatus: {=bool:?} }}", self.clrstatus())
    }
}
#[doc = "Combined Pre-Counter and Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Combcnt(pub u32);
impl Combcnt {
    #[doc = "Pre-Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn precnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Pre-Counter Value."]
    #[inline(always)]
    pub const fn set_precnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cntlsb(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Counter Value."]
    #[inline(always)]
    pub const fn set_cntlsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Combcnt {
    #[inline(always)]
    fn default() -> Combcnt {
        Combcnt(0)
    }
}
impl core::fmt::Debug for Combcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Combcnt")
            .field("precnt", &self.precnt())
            .field("cntlsb", &self.cntlsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Combcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Combcnt {{ precnt: {=u16:?}, cntlsb: {=u32:?} }}",
            self.precnt(),
            self.cntlsb()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "RTCC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTCC Enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug Mode Run Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Run Enable."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pre-counter CCV0 Top Value Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn preccv0top(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-counter CCV0 Top Value Enable."]
    #[inline(always)]
    pub const fn set_preccv0top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CCV1 Top Value Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccv1top(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CCV1 Top Value Enable."]
    #[inline(always)]
    pub const fn set_ccv1top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter Prescaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cntpresc(&self) -> super::vals::Cntpresc {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Cntpresc::from_bits(val as u8)
    }
    #[doc = "Counter Prescaler Value."]
    #[inline(always)]
    pub const fn set_cntpresc(&mut self, val: super::vals::Cntpresc) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Counter Prescaler Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cnttick(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Prescaler Mode."]
    #[inline(always)]
    pub const fn set_cnttick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Backup Mode Timestamp Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bumodetsen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Backup Mode Timestamp Enable."]
    #[inline(always)]
    pub const fn set_bumodetsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Oscillator Failure Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn oscfdeten(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Failure Detection Enable."]
    #[inline(always)]
    pub const fn set_oscfdeten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Main Counter Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cntmode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Main Counter Mode."]
    #[inline(always)]
    pub const fn set_cntmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Leap Year Correction Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn lyearcorrdis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Leap Year Correction Disabled."]
    #[inline(always)]
    pub const fn set_lyearcorrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("enable", &self.enable())
            .field("debugrun", &self.debugrun())
            .field("preccv0top", &self.preccv0top())
            .field("ccv1top", &self.ccv1top())
            .field("cntpresc", &self.cntpresc())
            .field("cnttick", &self.cnttick())
            .field("bumodetsen", &self.bumodetsen())
            .field("oscfdeten", &self.oscfdeten())
            .field("cntmode", &self.cntmode())
            .field("lyearcorrdis", &self.lyearcorrdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ enable: {=bool:?}, debugrun: {=bool:?}, preccv0top: {=bool:?}, ccv1top: {=bool:?}, cntpresc: {:?}, cnttick: {=bool:?}, bumodetsen: {=bool:?}, oscfdeten: {=bool:?}, cntmode: {=bool:?}, lyearcorrdis: {=bool:?} }}" , self . enable () , self . debugrun () , self . preccv0top () , self . ccv1top () , self . cntpresc () , self . cnttick () , self . bumodetsen () , self . oscfdeten () , self . cntmode () , self . lyearcorrdis ())
    }
}
#[doc = "Date Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Date(pub u32);
impl Date {
    #[doc = "Day of Month, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn dayomu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Day of Month, Units."]
    #[inline(always)]
    pub const fn set_dayomu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Day of Month, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn dayomt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Day of Month, Tens."]
    #[inline(always)]
    pub const fn set_dayomt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn monthu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month, Units."]
    #[inline(always)]
    pub const fn set_monthu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn montht(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month, Tens."]
    #[inline(always)]
    pub const fn set_montht(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Year, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn yearu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Year, Units."]
    #[inline(always)]
    pub const fn set_yearu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Year, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn yeart(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Year, Tens."]
    #[inline(always)]
    pub const fn set_yeart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Day of Week."]
    #[must_use]
    #[inline(always)]
    pub const fn dayow(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Day of Week."]
    #[inline(always)]
    pub const fn set_dayow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Date {
    #[inline(always)]
    fn default() -> Date {
        Date(0)
    }
}
impl core::fmt::Debug for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Date")
            .field("dayomu", &self.dayomu())
            .field("dayomt", &self.dayomt())
            .field("monthu", &self.monthu())
            .field("montht", &self.montht())
            .field("yearu", &self.yearu())
            .field("yeart", &self.yeart())
            .field("dayow", &self.dayow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Date {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Date {{ dayomu: {=u8:?}, dayomt: {=u8:?}, monthu: {=u8:?}, montht: {=bool:?}, yearu: {=u8:?}, yeart: {=u8:?}, dayow: {=u8:?} }}" , self . dayomu () , self . dayomt () , self . monthu () , self . montht () , self . yearu () , self . yeart () , self . dayow ())
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
    #[doc = "OF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CC0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CC0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CC1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CC1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CC2 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CC2 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OSCFAIL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn oscfail(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OSCFAIL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_oscfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CNTTICK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cnttick(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CNTTICK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cnttick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "MINTICK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mintick(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "MINTICK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mintick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "HOURTICK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hourtick(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "HOURTICK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hourtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DAYTICK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn daytick(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DAYTICK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_daytick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAYOWOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dayowof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAYOWOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dayowof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "MONTHTICK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn monthtick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "MONTHTICK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_monthtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("of", &self.of())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("oscfail", &self.oscfail())
            .field("cnttick", &self.cnttick())
            .field("mintick", &self.mintick())
            .field("hourtick", &self.hourtick())
            .field("daytick", &self.daytick())
            .field("dayowof", &self.dayowof())
            .field("monthtick", &self.monthtick())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ of: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, oscfail: {=bool:?}, cnttick: {=bool:?}, mintick: {=bool:?}, hourtick: {=bool:?}, daytick: {=bool:?}, dayowof: {=bool:?}, monthtick: {=bool:?} }}" , self . of () , self . cc0 () , self . cc1 () , self . cc2 () , self . oscfail () , self . cnttick () , self . mintick () , self . hourtick () , self . daytick () , self . dayowof () , self . monthtick ())
    }
}
#[doc = "RTCC Interrupt Flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Oscillator Failure Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oscfail(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Failure Interrupt Flag."]
    #[inline(always)]
    pub const fn set_oscfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Main Counter Tick."]
    #[must_use]
    #[inline(always)]
    pub const fn cnttick(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Main Counter Tick."]
    #[inline(always)]
    pub const fn set_cnttick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Minute Tick."]
    #[must_use]
    #[inline(always)]
    pub const fn mintick(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Minute Tick."]
    #[inline(always)]
    pub const fn set_mintick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hour Tick."]
    #[must_use]
    #[inline(always)]
    pub const fn hourtick(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hour Tick."]
    #[inline(always)]
    pub const fn set_hourtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Day Tick."]
    #[must_use]
    #[inline(always)]
    pub const fn daytick(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Day Tick."]
    #[inline(always)]
    pub const fn set_daytick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Day of Week Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn dayowof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Day of Week Overflow."]
    #[inline(always)]
    pub const fn set_dayowof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Month Tick."]
    #[must_use]
    #[inline(always)]
    pub const fn monthtick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Month Tick."]
    #[inline(always)]
    pub const fn set_monthtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("of", &self.of())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("oscfail", &self.oscfail())
            .field("cnttick", &self.cnttick())
            .field("mintick", &self.mintick())
            .field("hourtick", &self.hourtick())
            .field("daytick", &self.daytick())
            .field("dayowof", &self.dayowof())
            .field("monthtick", &self.monthtick())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ of: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, oscfail: {=bool:?}, cnttick: {=bool:?}, mintick: {=bool:?}, hourtick: {=bool:?}, daytick: {=bool:?}, dayowof: {=bool:?}, monthtick: {=bool:?} }}" , self . of () , self . cc0 () , self . cc1 () , self . cc2 () , self . oscfail () , self . cnttick () , self . mintick () , self . hourtick () , self . daytick () , self . dayowof () , self . monthtick ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set OF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set OF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set CC0 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC0 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set CC1 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC1 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set CC2 Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cc2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set CC2 Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set OSCFAIL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oscfail(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set OSCFAIL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_oscfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set CNTTICK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cnttick(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set CNTTICK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cnttick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set MINTICK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mintick(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set MINTICK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_mintick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set HOURTICK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hourtick(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set HOURTICK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hourtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set DAYTICK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn daytick(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set DAYTICK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_daytick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set DAYOWOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dayowof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set DAYOWOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dayowof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set MONTHTICK Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn monthtick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set MONTHTICK Interrupt Flag."]
    #[inline(always)]
    pub const fn set_monthtick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("of", &self.of())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("oscfail", &self.oscfail())
            .field("cnttick", &self.cnttick())
            .field("mintick", &self.mintick())
            .field("hourtick", &self.hourtick())
            .field("daytick", &self.daytick())
            .field("dayowof", &self.dayowof())
            .field("monthtick", &self.monthtick())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ of: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, oscfail: {=bool:?}, cnttick: {=bool:?}, mintick: {=bool:?}, hourtick: {=bool:?}, daytick: {=bool:?}, dayowof: {=bool:?}, monthtick: {=bool:?} }}" , self . of () , self . cc0 () , self . cc1 () , self . cc2 () , self . oscfail () , self . cnttick () , self . mintick () , self . hourtick () , self . daytick () , self . dayowof () , self . monthtick ())
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
#[doc = "Retention RAM Power-down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerdown(pub u32);
impl Powerdown {
    #[doc = "Retention RAM Power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn ram(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Retention RAM Power-down."]
    #[inline(always)]
    pub const fn set_ram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Powerdown {
    #[inline(always)]
    fn default() -> Powerdown {
        Powerdown(0)
    }
}
impl core::fmt::Debug for Powerdown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Powerdown")
            .field("ram", &self.ram())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Powerdown {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Powerdown {{ ram: {=bool:?} }}", self.ram())
    }
}
#[doc = "Pre-Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Precnt(pub u32);
impl Precnt {
    #[doc = "Pre-Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn precnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Pre-Counter Value."]
    #[inline(always)]
    pub const fn set_precnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Precnt {
    #[inline(always)]
    fn default() -> Precnt {
        Precnt(0)
    }
}
impl core::fmt::Debug for Precnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Precnt")
            .field("precnt", &self.precnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Precnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Precnt {{ precnt: {=u16:?} }}", self.precnt())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Timestamp for Backup Mode Entry Stored."]
    #[must_use]
    #[inline(always)]
    pub const fn bumodets(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp for Backup Mode Entry Stored."]
    #[inline(always)]
    pub const fn set_bumodets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("bumodets", &self.bumodets())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ bumodets: {=bool:?} }}", self.bumodets())
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
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMD Register Busy."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
#[doc = "Time of Day Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time(pub u32);
impl Time {
    #[doc = "Seconds, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn secu(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Seconds, Units."]
    #[inline(always)]
    pub const fn set_secu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Seconds, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn sect(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Seconds, Tens."]
    #[inline(always)]
    pub const fn set_sect(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Minutes, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn minu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minutes, Units."]
    #[inline(always)]
    pub const fn set_minu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minutes, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn mint(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minutes, Tens."]
    #[inline(always)]
    pub const fn set_mint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Hours, Units."]
    #[must_use]
    #[inline(always)]
    pub const fn houru(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hours, Units."]
    #[inline(always)]
    pub const fn set_houru(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hours, Tens."]
    #[must_use]
    #[inline(always)]
    pub const fn hourt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hours, Tens."]
    #[inline(always)]
    pub const fn set_hourt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Time {
    #[inline(always)]
    fn default() -> Time {
        Time(0)
    }
}
impl core::fmt::Debug for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Time")
            .field("secu", &self.secu())
            .field("sect", &self.sect())
            .field("minu", &self.minu())
            .field("mint", &self.mint())
            .field("houru", &self.houru())
            .field("hourt", &self.hourt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Time {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Time {{ secu: {=u8:?}, sect: {=u8:?}, minu: {=u8:?}, mint: {=u8:?}, houru: {=u8:?}, hourt: {=u8:?} }}" , self . secu () , self . sect () , self . minu () , self . mint () , self . houru () , self . hourt ())
    }
}
