#[doc = "ADC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcctrl(pub u32);
impl Adcctrl {
    #[doc = "ADC0 Clock Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0clkdiv(&self) -> super::vals::Adc0clkdiv {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Adc0clkdiv::from_bits(val as u8)
    }
    #[doc = "ADC0 Clock Prescaler."]
    #[inline(always)]
    pub const fn set_adc0clkdiv(&mut self, val: super::vals::Adc0clkdiv) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ADC0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0clksel(&self) -> super::vals::Adc0clksel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Adc0clksel::from_bits(val as u8)
    }
    #[doc = "ADC0 Clock Select."]
    #[inline(always)]
    pub const fn set_adc0clksel(&mut self, val: super::vals::Adc0clksel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Invert Clock Selected By ADC0CLKSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0clkinv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Clock Selected By ADC0CLKSEL."]
    #[inline(always)]
    pub const fn set_adc0clkinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ADC1 Clock Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1clkdiv(&self) -> super::vals::Adc1clkdiv {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Adc1clkdiv::from_bits(val as u8)
    }
    #[doc = "ADC1 Clock Prescaler."]
    #[inline(always)]
    pub const fn set_adc1clkdiv(&mut self, val: super::vals::Adc1clkdiv) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ADC1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1clksel(&self) -> super::vals::Adc1clksel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Adc1clksel::from_bits(val as u8)
    }
    #[doc = "ADC1 Clock Select."]
    #[inline(always)]
    pub const fn set_adc1clksel(&mut self, val: super::vals::Adc1clksel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Invert Clock Selected By ADC1CLKSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1clkinv(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Clock Selected By ADC1CLKSEL."]
    #[inline(always)]
    pub const fn set_adc1clkinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Adcctrl {
    #[inline(always)]
    fn default() -> Adcctrl {
        Adcctrl(0)
    }
}
impl core::fmt::Debug for Adcctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcctrl")
            .field("adc0clkdiv", &self.adc0clkdiv())
            .field("adc0clksel", &self.adc0clksel())
            .field("adc0clkinv", &self.adc0clkinv())
            .field("adc1clkdiv", &self.adc1clkdiv())
            .field("adc1clksel", &self.adc1clksel())
            .field("adc1clkinv", &self.adc1clkinv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adcctrl {{ adc0clkdiv: {:?}, adc0clksel: {:?}, adc0clkinv: {=bool:?}, adc1clkdiv: {:?}, adc1clksel: {:?}, adc1clkinv: {=bool:?} }}" , self . adc0clkdiv () , self . adc0clksel () , self . adc0clkinv () , self . adc1clkdiv () , self . adc1clksel () , self . adc1clkinv ())
    }
}
#[doc = "AUXHFRCO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Auxhfrcoctrl(pub u32);
impl Auxhfrcoctrl {
    #[doc = "AUXHFRCO Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "AUXHFRCO Tuning Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "AUXHFRCO Fine Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuning(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "AUXHFRCO Fine Tuning Value."]
    #[inline(always)]
    pub const fn set_finetuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "AUXHFRCO Frequency Range."]
    #[must_use]
    #[inline(always)]
    pub const fn freqrange(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AUXHFRCO Frequency Range."]
    #[inline(always)]
    pub const fn set_freqrange(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AUXHFRCO Comparator Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpbias(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "AUXHFRCO Comparator Bias Current."]
    #[inline(always)]
    pub const fn set_cmpbias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "AUXHFRCO LDO High Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ldohp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO LDO High Power Mode."]
    #[inline(always)]
    pub const fn set_ldohp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Locally Divide AUXHFRCO Clock Output."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::AuxhfrcoctrlClkdiv {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::AuxhfrcoctrlClkdiv::from_bits(val as u8)
    }
    #[doc = "Locally Divide AUXHFRCO Clock Output."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: super::vals::AuxhfrcoctrlClkdiv) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuningen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[inline(always)]
    pub const fn set_finetuningen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "AUXHFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[must_use]
    #[inline(always)]
    pub const fn vreftc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "AUXHFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[inline(always)]
    pub const fn set_vreftc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Auxhfrcoctrl {
    #[inline(always)]
    fn default() -> Auxhfrcoctrl {
        Auxhfrcoctrl(0)
    }
}
impl core::fmt::Debug for Auxhfrcoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Auxhfrcoctrl")
            .field("tuning", &self.tuning())
            .field("finetuning", &self.finetuning())
            .field("freqrange", &self.freqrange())
            .field("cmpbias", &self.cmpbias())
            .field("ldohp", &self.ldohp())
            .field("clkdiv", &self.clkdiv())
            .field("finetuningen", &self.finetuningen())
            .field("vreftc", &self.vreftc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Auxhfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Auxhfrcoctrl {{ tuning: {=u8:?}, finetuning: {=u8:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, ldohp: {=bool:?}, clkdiv: {:?}, finetuningen: {=bool:?}, vreftc: {=u8:?} }}" , self . tuning () , self . finetuning () , self . freqrange () , self . cmpbias () , self . ldohp () , self . clkdiv () , self . finetuningen () , self . vreftc ())
    }
}
#[doc = "Calibration Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calcnt(pub u32);
impl Calcnt {
    #[doc = "Calibration Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn calcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Calibration Counter."]
    #[inline(always)]
    pub const fn set_calcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Calcnt {
    #[inline(always)]
    fn default() -> Calcnt {
        Calcnt(0)
    }
}
impl core::fmt::Debug for Calcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calcnt")
            .field("calcnt", &self.calcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Calcnt {{ calcnt: {=u32:?} }}", self.calcnt())
    }
}
#[doc = "Calibration Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calctrl(pub u32);
impl Calctrl {
    #[doc = "Calibration Up-counter Select."]
    #[must_use]
    #[inline(always)]
    pub const fn upsel(&self) -> super::vals::Upsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Upsel::from_bits(val as u8)
    }
    #[doc = "Calibration Up-counter Select."]
    #[inline(always)]
    pub const fn set_upsel(&mut self, val: super::vals::Upsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Calibration Down-counter Select."]
    #[must_use]
    #[inline(always)]
    pub const fn downsel(&self) -> super::vals::Downsel {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Downsel::from_bits(val as u8)
    }
    #[doc = "Calibration Down-counter Select."]
    #[inline(always)]
    pub const fn set_downsel(&mut self, val: super::vals::Downsel) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Continuous Calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Calibration."]
    #[inline(always)]
    pub const fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PRS Select for PRS Input When Selected in UPSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn prsupsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 16usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Select for PRS Input When Selected in UPSEL."]
    #[inline(always)]
    pub const fn set_prsupsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "PRS Select for PRS Input When Selected in DOWNSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn prsdownsel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 24usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "PRS Select for PRS Input When Selected in DOWNSEL."]
    #[inline(always)]
    pub const fn set_prsdownsel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Calctrl {
    #[inline(always)]
    fn default() -> Calctrl {
        Calctrl(0)
    }
}
impl core::fmt::Debug for Calctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calctrl")
            .field("upsel", &self.upsel())
            .field("downsel", &self.downsel())
            .field("cont", &self.cont())
            .field("prsupsel", &self.prsupsel())
            .field("prsdownsel", &self.prsdownsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Calctrl {{ upsel: {:?}, downsel: {:?}, cont: {=bool:?}, prsupsel: {:?}, prsdownsel: {:?} }}" , self . upsel () , self . downsel () , self . cont () , self . prsupsel () , self . prsdownsel ())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Calibration Start."]
    #[must_use]
    #[inline(always)]
    pub const fn calstart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Start."]
    #[inline(always)]
    pub const fn set_calstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Calibration Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn calstop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Stop."]
    #[inline(always)]
    pub const fn set_calstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HFXO Peak Detection Start."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdetstart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Peak Detection Start."]
    #[inline(always)]
    pub const fn set_hfxopeakdetstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("calstart", &self.calstart())
            .field("calstop", &self.calstop())
            .field("hfxopeakdetstart", &self.hfxopeakdetstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ calstart: {=bool:?}, calstop: {=bool:?}, hfxopeakdetstart: {=bool:?} }}",
            self.calstart(),
            self.calstop(),
            self.hfxopeakdetstart()
        )
    }
}
#[doc = "CMU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Clock Output Select 0."]
    #[must_use]
    #[inline(always)]
    pub const fn clkoutsel0(&self) -> super::vals::Clkoutsel0 {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Clkoutsel0::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 0."]
    #[inline(always)]
    pub const fn set_clkoutsel0(&mut self, val: super::vals::Clkoutsel0) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Clock Output Select 1."]
    #[must_use]
    #[inline(always)]
    pub const fn clkoutsel1(&self) -> super::vals::Clkoutsel1 {
        let val = (self.0 >> 5usize) & 0x1f;
        super::vals::Clkoutsel1::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 1."]
    #[inline(always)]
    pub const fn set_clkoutsel1(&mut self, val: super::vals::Clkoutsel1) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val.to_bits() as u32) & 0x1f) << 5usize);
    }
    #[doc = "Clock Output Select 2."]
    #[must_use]
    #[inline(always)]
    pub const fn clkoutsel2(&self) -> super::vals::Clkoutsel2 {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::Clkoutsel2::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 2."]
    #[inline(always)]
    pub const fn set_clkoutsel2(&mut self, val: super::vals::Clkoutsel2) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "Wait State for High-Frequency LE Interface."]
    #[must_use]
    #[inline(always)]
    pub const fn wshfle(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wait State for High-Frequency LE Interface."]
    #[inline(always)]
    pub const fn set_wshfle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "HFPERCLK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfperclken(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "HFPERCLK Enable."]
    #[inline(always)]
    pub const fn set_hfperclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
            .field("clkoutsel0", &self.clkoutsel0())
            .field("clkoutsel1", &self.clkoutsel1())
            .field("clkoutsel2", &self.clkoutsel2())
            .field("wshfle", &self.wshfle())
            .field("hfperclken", &self.hfperclken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ clkoutsel0: {:?}, clkoutsel1: {:?}, clkoutsel2: {:?}, wshfle: {=bool:?}, hfperclken: {=bool:?} }}" , self . clkoutsel0 () , self . clkoutsel1 () , self . clkoutsel2 () , self . wshfle () , self . hfperclken ())
    }
}
#[doc = "Debug Trace Clock Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgclksel(pub u32);
impl Dbgclksel {
    #[doc = "Debug Trace Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn dbg(&self) -> super::vals::Dbg {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dbg::from_bits(val as u8)
    }
    #[doc = "Debug Trace Clock."]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: super::vals::Dbg) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Dbgclksel {
    #[inline(always)]
    fn default() -> Dbgclksel {
        Dbgclksel(0)
    }
}
impl core::fmt::Debug for Dbgclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgclksel")
            .field("dbg", &self.dbg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbgclksel {{ dbg: {:?} }}", self.dbg())
    }
}
#[doc = "DPLL Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpllctrl(pub u32);
impl Dpllctrl {
    #[doc = "Operating Mode Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operating Mode Control."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reference Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edgesel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Edge Select."]
    #[inline(always)]
    pub const fn set_edgesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Automatic Recovery Ctrl."]
    #[must_use]
    #[inline(always)]
    pub const fn autorecover(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Recovery Ctrl."]
    #[inline(always)]
    pub const fn set_autorecover(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reference Clock Selection Control."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Reference Clock Selection Control."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Dither Enable Control."]
    #[must_use]
    #[inline(always)]
    pub const fn dithen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Dither Enable Control."]
    #[inline(always)]
    pub const fn set_dithen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Dpllctrl {
    #[inline(always)]
    fn default() -> Dpllctrl {
        Dpllctrl(0)
    }
}
impl core::fmt::Debug for Dpllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dpllctrl")
            .field("mode", &self.mode())
            .field("edgesel", &self.edgesel())
            .field("autorecover", &self.autorecover())
            .field("refsel", &self.refsel())
            .field("dithen", &self.dithen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dpllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dpllctrl {{ mode: {=bool:?}, edgesel: {=bool:?}, autorecover: {=bool:?}, refsel: {:?}, dithen: {=bool:?} }}" , self . mode () , self . edgesel () , self . autorecover () , self . refsel () , self . dithen ())
    }
}
#[doc = "DPLL Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpllctrl1(pub u32);
impl Dpllctrl1 {
    #[doc = "Factor M."]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Factor M."]
    #[inline(always)]
    pub const fn set_m(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Factor N."]
    #[must_use]
    #[inline(always)]
    pub const fn n(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Factor N."]
    #[inline(always)]
    pub const fn set_n(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Dpllctrl1 {
    #[inline(always)]
    fn default() -> Dpllctrl1 {
        Dpllctrl1(0)
    }
}
impl core::fmt::Debug for Dpllctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dpllctrl1")
            .field("m", &self.m())
            .field("n", &self.n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dpllctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dpllctrl1 {{ m: {=u16:?}, n: {=u16:?} }}",
            self.m(),
            self.n()
        )
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freeze {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Freeze {{ regfreeze: {=bool:?} }}", self.regfreeze())
    }
}
#[doc = "High Frequency Bus Clock Enable Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfbusclken0(pub u32);
impl Hfbusclken0 {
    #[doc = "Low Energy Peripheral Interface Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn le(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Peripheral Interface Clock Enable."]
    #[inline(always)]
    pub const fn set_le(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Advanced Encryption Standard Accelerator Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crypto0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Advanced Encryption Standard Accelerator Clock Enable."]
    #[inline(always)]
    pub const fn set_crypto0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External Bus Interface Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ebi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External Bus Interface Clock Enable."]
    #[inline(always)]
    pub const fn set_ebi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ethernet Controller Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eth(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Controller Clock Enable."]
    #[inline(always)]
    pub const fn set_eth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SDIO Controller Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Controller Clock Enable."]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "General purpose Input/Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "General purpose Input/Output Clock Enable."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral Reflex System Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prs(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral Reflex System Clock Enable."]
    #[inline(always)]
    pub const fn set_prs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Linked Direct Memory Access Controller Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Linked Direct Memory Access Controller Clock Enable."]
    #[inline(always)]
    pub const fn set_ldma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "General Purpose CRC Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcrc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose CRC Clock Enable."]
    #[inline(always)]
    pub const fn set_gpcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Quad-SPI Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn qspi0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Quad-SPI Clock Enable."]
    #[inline(always)]
    pub const fn set_qspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Universal Serial Bus Interface Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Serial Bus Interface Clock Enable."]
    #[inline(always)]
    pub const fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Hfbusclken0 {
    #[inline(always)]
    fn default() -> Hfbusclken0 {
        Hfbusclken0(0)
    }
}
impl core::fmt::Debug for Hfbusclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfbusclken0")
            .field("le", &self.le())
            .field("crypto0", &self.crypto0())
            .field("ebi", &self.ebi())
            .field("eth", &self.eth())
            .field("sdio", &self.sdio())
            .field("gpio", &self.gpio())
            .field("prs", &self.prs())
            .field("ldma", &self.ldma())
            .field("gpcrc", &self.gpcrc())
            .field("qspi0", &self.qspi0())
            .field("usb", &self.usb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfbusclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfbusclken0 {{ le: {=bool:?}, crypto0: {=bool:?}, ebi: {=bool:?}, eth: {=bool:?}, sdio: {=bool:?}, gpio: {=bool:?}, prs: {=bool:?}, ldma: {=bool:?}, gpcrc: {=bool:?}, qspi0: {=bool:?}, usb: {=bool:?} }}" , self . le () , self . crypto0 () , self . ebi () , self . eth () , self . sdio () , self . gpio () , self . prs () , self . ldma () , self . gpcrc () , self . qspi0 () , self . usb ())
    }
}
#[doc = "High Frequency Bus Clock Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfbuspresc(pub u32);
impl Hfbuspresc {
    #[doc = "HFBUSCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfbusprescPresc {
        let val = (self.0 >> 8usize) & 0x01ff;
        super::vals::HfbusprescPresc::from_bits(val as u16)
    }
    #[doc = "HFBUSCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfbusprescPresc) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val.to_bits() as u32) & 0x01ff) << 8usize);
    }
}
impl Default for Hfbuspresc {
    #[inline(always)]
    fn default() -> Hfbuspresc {
        Hfbuspresc(0)
    }
}
impl core::fmt::Debug for Hfbuspresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfbuspresc")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfbuspresc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfbuspresc {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Clock Select Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclksel(pub u32);
impl Hfclksel {
    #[doc = "HFCLK Select."]
    #[must_use]
    #[inline(always)]
    pub const fn hf(&self) -> super::vals::Hf {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Hf::from_bits(val as u8)
    }
    #[doc = "HFCLK Select."]
    #[inline(always)]
    pub const fn set_hf(&mut self, val: super::vals::Hf) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Hfclksel {
    #[inline(always)]
    fn default() -> Hfclksel {
        Hfclksel(0)
    }
}
impl core::fmt::Debug for Hfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfclksel").field("hf", &self.hf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfclksel {{ hf: {:?} }}", self.hf())
    }
}
#[doc = "HFCLK Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkstatus(pub u32);
impl Hfclkstatus {
    #[doc = "HFCLK Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn selected(&self) -> super::vals::Selected {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Selected::from_bits(val as u8)
    }
    #[doc = "HFCLK Selected."]
    #[inline(always)]
    pub const fn set_selected(&mut self, val: super::vals::Selected) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Hfclkstatus {
    #[inline(always)]
    fn default() -> Hfclkstatus {
        Hfclkstatus(0)
    }
}
impl core::fmt::Debug for Hfclkstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfclkstatus")
            .field("selected", &self.selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfclkstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfclkstatus {{ selected: {:?} }}", self.selected())
    }
}
#[doc = "High Frequency Core Clock Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfcorepresc(pub u32);
impl Hfcorepresc {
    #[doc = "HFCORECLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfcoreprescPresc {
        let val = (self.0 >> 8usize) & 0x01ff;
        super::vals::HfcoreprescPresc::from_bits(val as u16)
    }
    #[doc = "HFCORECLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfcoreprescPresc) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val.to_bits() as u32) & 0x01ff) << 8usize);
    }
}
impl Default for Hfcorepresc {
    #[inline(always)]
    fn default() -> Hfcorepresc {
        Hfcorepresc(0)
    }
}
impl core::fmt::Debug for Hfcorepresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfcorepresc")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfcorepresc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfcorepresc {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Export Clock Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfexppresc(pub u32);
impl Hfexppresc {
    #[doc = "HFEXPCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfexpprescPresc {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::HfexpprescPresc::from_bits(val as u8)
    }
    #[doc = "HFEXPCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfexpprescPresc) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for Hfexppresc {
    #[inline(always)]
    fn default() -> Hfexppresc {
        Hfexppresc(0)
    }
}
impl core::fmt::Debug for Hfexppresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfexppresc")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfexppresc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfexppresc {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfperclken0(pub u32);
impl Hfperclken0 {
    #[doc = "Timer 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer 2 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 2 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timer 3 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 3 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Timer 4 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 4 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer 5 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 5 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Timer 6 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timer6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 6 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Analog Comparator 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_acmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Analog Comparator 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_acmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog Comparator 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_acmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Analog Comparator 3 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 3 Clock Enable."]
    #[inline(always)]
    pub const fn set_acmp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "I2C 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "I2C 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "I2C 2 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 2 Clock Enable."]
    #[inline(always)]
    pub const fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "CRYOTIMER Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cryotimer(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CRYOTIMER Clock Enable."]
    #[inline(always)]
    pub const fn set_cryotimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Current Digital to Analog Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn idac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Current Digital to Analog Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_idac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "True Random Number Generator 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "True Random Number Generator 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Hfperclken0 {
    #[inline(always)]
    fn default() -> Hfperclken0 {
        Hfperclken0(0)
    }
}
impl core::fmt::Debug for Hfperclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfperclken0")
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .field("timer2", &self.timer2())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("timer5", &self.timer5())
            .field("timer6", &self.timer6())
            .field("usart0", &self.usart0())
            .field("usart1", &self.usart1())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("acmp0", &self.acmp0())
            .field("acmp1", &self.acmp1())
            .field("acmp2", &self.acmp2())
            .field("acmp3", &self.acmp3())
            .field("i2c0", &self.i2c0())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cryotimer", &self.cryotimer())
            .field("idac0", &self.idac0())
            .field("trng0", &self.trng0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfperclken0 {{ timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, timer5: {=bool:?}, timer6: {=bool:?}, usart0: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, usart3: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, acmp0: {=bool:?}, acmp1: {=bool:?}, acmp2: {=bool:?}, acmp3: {=bool:?}, i2c0: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cryotimer: {=bool:?}, idac0: {=bool:?}, trng0: {=bool:?} }}" , self . timer0 () , self . timer1 () , self . timer2 () , self . timer3 () , self . timer4 () , self . timer5 () , self . timer6 () , self . usart0 () , self . usart1 () , self . usart2 () , self . usart3 () , self . usart4 () , self . usart5 () , self . acmp0 () , self . acmp1 () , self . acmp2 () , self . acmp3 () , self . i2c0 () , self . i2c1 () , self . i2c2 () , self . adc0 () , self . adc1 () , self . cryotimer () , self . idac0 () , self . trng0 ())
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfperclken1(pub u32);
impl Hfperclken1 {
    #[doc = "Wide Timer 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_wtimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wide Timer 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_wtimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wide Timer 2 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 2 Clock Enable."]
    #[inline(always)]
    pub const fn set_wtimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Wide Timer 3 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wtimer3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Wide Timer 3 Clock Enable."]
    #[inline(always)]
    pub const fn set_wtimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uart0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CAN 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn can0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CAN 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_can0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CAN 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn can1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CAN 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_can1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Digital to Analog Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdac0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Digital to Analog Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_vdac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capacitive touch sense module Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn csen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capacitive touch sense module Clock Enable."]
    #[inline(always)]
    pub const fn set_csen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Hfperclken1 {
    #[inline(always)]
    fn default() -> Hfperclken1 {
        Hfperclken1(0)
    }
}
impl core::fmt::Debug for Hfperclken1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfperclken1")
            .field("wtimer0", &self.wtimer0())
            .field("wtimer1", &self.wtimer1())
            .field("wtimer2", &self.wtimer2())
            .field("wtimer3", &self.wtimer3())
            .field("uart0", &self.uart0())
            .field("uart1", &self.uart1())
            .field("can0", &self.can0())
            .field("can1", &self.can1())
            .field("vdac0", &self.vdac0())
            .field("csen", &self.csen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperclken1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfperclken1 {{ wtimer0: {=bool:?}, wtimer1: {=bool:?}, wtimer2: {=bool:?}, wtimer3: {=bool:?}, uart0: {=bool:?}, uart1: {=bool:?}, can0: {=bool:?}, can1: {=bool:?}, vdac0: {=bool:?}, csen: {=bool:?} }}" , self . wtimer0 () , self . wtimer1 () , self . wtimer2 () , self . wtimer3 () , self . uart0 () , self . uart1 () , self . can0 () , self . can1 () , self . vdac0 () , self . csen ())
    }
}
#[doc = "High Frequency Peripheral Clock Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfperpresc(pub u32);
impl Hfperpresc {
    #[doc = "HFPERCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfperprescPresc {
        let val = (self.0 >> 8usize) & 0x01ff;
        super::vals::HfperprescPresc::from_bits(val as u16)
    }
    #[doc = "HFPERCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfperprescPresc) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val.to_bits() as u32) & 0x01ff) << 8usize);
    }
}
impl Default for Hfperpresc {
    #[inline(always)]
    fn default() -> Hfperpresc {
        Hfperpresc(0)
    }
}
impl core::fmt::Debug for Hfperpresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfperpresc")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperpresc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfperpresc {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Peripheral Clock Prescaler B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfperprescb(pub u32);
impl Hfperprescb {
    #[doc = "HFPERCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfperprescbPresc {
        let val = (self.0 >> 8usize) & 0x01ff;
        super::vals::HfperprescbPresc::from_bits(val as u16)
    }
    #[doc = "HFPERCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfperprescbPresc) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val.to_bits() as u32) & 0x01ff) << 8usize);
    }
}
impl Default for Hfperprescb {
    #[inline(always)]
    fn default() -> Hfperprescb {
        Hfperprescb(0)
    }
}
impl core::fmt::Debug for Hfperprescb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfperprescb")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperprescb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfperprescb {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Peripheral Clock Prescaler C Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfperprescc(pub u32);
impl Hfperprescc {
    #[doc = "HFPERCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfperpresccPresc {
        let val = (self.0 >> 8usize) & 0x01ff;
        super::vals::HfperpresccPresc::from_bits(val as u16)
    }
    #[doc = "HFPERCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfperpresccPresc) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val.to_bits() as u32) & 0x01ff) << 8usize);
    }
}
impl Default for Hfperprescc {
    #[inline(always)]
    fn default() -> Hfperprescc {
        Hfperprescc(0)
    }
}
impl core::fmt::Debug for Hfperprescc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfperprescc")
            .field("presc", &self.presc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperprescc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfperprescc {{ presc: {:?} }}", self.presc())
    }
}
#[doc = "High Frequency Clock Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfpresc(pub u32);
impl Hfpresc {
    #[doc = "HFCLK Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::HfprescPresc {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::HfprescPresc::from_bits(val as u8)
    }
    #[doc = "HFCLK Prescaler."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::HfprescPresc) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "HFCLKLE Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn hfclklepresc(&self) -> super::vals::Hfclklepresc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Hfclklepresc::from_bits(val as u8)
    }
    #[doc = "HFCLKLE Prescaler."]
    #[inline(always)]
    pub const fn set_hfclklepresc(&mut self, val: super::vals::Hfclklepresc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Hfpresc {
    #[inline(always)]
    fn default() -> Hfpresc {
        Hfpresc(0)
    }
}
impl core::fmt::Debug for Hfpresc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfpresc")
            .field("presc", &self.presc())
            .field("hfclklepresc", &self.hfclklepresc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfpresc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfpresc {{ presc: {:?}, hfclklepresc: {:?} }}",
            self.presc(),
            self.hfclklepresc()
        )
    }
}
#[doc = "HFRCO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfrcoctrl(pub u32);
impl Hfrcoctrl {
    #[doc = "HFRCO Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "HFRCO Tuning Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "HFRCO Fine Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuning(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "HFRCO Fine Tuning Value."]
    #[inline(always)]
    pub const fn set_finetuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "HFRCO Frequency Range."]
    #[must_use]
    #[inline(always)]
    pub const fn freqrange(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "HFRCO Frequency Range."]
    #[inline(always)]
    pub const fn set_freqrange(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "HFRCO Comparator Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpbias(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "HFRCO Comparator Bias Current."]
    #[inline(always)]
    pub const fn set_cmpbias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "HFRCO LDO High Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ldohp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO LDO High Power Mode."]
    #[inline(always)]
    pub const fn set_ldohp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Locally Divide HFRCO Clock Output."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::HfrcoctrlClkdiv {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::HfrcoctrlClkdiv::from_bits(val as u8)
    }
    #[doc = "Locally Divide HFRCO Clock Output."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: super::vals::HfrcoctrlClkdiv) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuningen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[inline(always)]
    pub const fn set_finetuningen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "HFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[must_use]
    #[inline(always)]
    pub const fn vreftc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "HFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[inline(always)]
    pub const fn set_vreftc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Hfrcoctrl {
    #[inline(always)]
    fn default() -> Hfrcoctrl {
        Hfrcoctrl(0)
    }
}
impl core::fmt::Debug for Hfrcoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfrcoctrl")
            .field("tuning", &self.tuning())
            .field("finetuning", &self.finetuning())
            .field("freqrange", &self.freqrange())
            .field("cmpbias", &self.cmpbias())
            .field("ldohp", &self.ldohp())
            .field("clkdiv", &self.clkdiv())
            .field("finetuningen", &self.finetuningen())
            .field("vreftc", &self.vreftc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfrcoctrl {{ tuning: {=u8:?}, finetuning: {=u8:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, ldohp: {=bool:?}, clkdiv: {:?}, finetuningen: {=bool:?}, vreftc: {=u8:?} }}" , self . tuning () , self . finetuning () , self . freqrange () , self . cmpbias () , self . ldohp () , self . clkdiv () , self . finetuningen () , self . vreftc ())
    }
}
#[doc = "HFRCO Spread Spectrum Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfrcoss(pub u32);
impl Hfrcoss {
    #[doc = "Spread Spectrum Amplitude."]
    #[must_use]
    #[inline(always)]
    pub const fn ssamp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Spread Spectrum Amplitude."]
    #[inline(always)]
    pub const fn set_ssamp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Spread Spectrum Update Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn ssinv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Spread Spectrum Update Interval."]
    #[inline(always)]
    pub const fn set_ssinv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Hfrcoss {
    #[inline(always)]
    fn default() -> Hfrcoss {
        Hfrcoss(0)
    }
}
impl core::fmt::Debug for Hfrcoss {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfrcoss")
            .field("ssamp", &self.ssamp())
            .field("ssinv", &self.ssinv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfrcoss {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfrcoss {{ ssamp: {=u8:?}, ssinv: {=u8:?} }}",
            self.ssamp(),
            self.ssinv()
        )
    }
}
#[doc = "HFXO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxoctrl(pub u32);
impl Hfxoctrl {
    #[doc = "HFXO Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::HfxoctrlMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::HfxoctrlMode::from_bits(val as u8)
    }
    #[doc = "HFXO Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::HfxoctrlMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxox2en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)."]
    #[inline(always)]
    pub const fn set_hfxox2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HFXO Automatic Peak Detection Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdetmode(&self) -> super::vals::Peakdetmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Peakdetmode::from_bits(val as u8)
    }
    #[doc = "HFXO Automatic Peak Detection Mode."]
    #[inline(always)]
    pub const fn set_peakdetmode(&mut self, val: super::vals::Peakdetmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "HFXO Low Frequency Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn lftimeout(&self) -> super::vals::Lftimeout {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Lftimeout::from_bits(val as u8)
    }
    #[doc = "HFXO Low Frequency Timeout."]
    #[inline(always)]
    pub const fn set_lftimeout(&mut self, val: super::vals::Lftimeout) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3."]
    #[must_use]
    #[inline(always)]
    pub const fn autostartem0em1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3."]
    #[inline(always)]
    pub const fn set_autostartem0em1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3."]
    #[must_use]
    #[inline(always)]
    pub const fn autostartselem0em1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3."]
    #[inline(always)]
    pub const fn set_autostartselem0em1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Hfxoctrl {
    #[inline(always)]
    fn default() -> Hfxoctrl {
        Hfxoctrl(0)
    }
}
impl core::fmt::Debug for Hfxoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxoctrl")
            .field("mode", &self.mode())
            .field("hfxox2en", &self.hfxox2en())
            .field("peakdetmode", &self.peakdetmode())
            .field("lftimeout", &self.lftimeout())
            .field("autostartem0em1", &self.autostartem0em1())
            .field("autostartselem0em1", &self.autostartselem0em1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxoctrl {{ mode: {:?}, hfxox2en: {=bool:?}, peakdetmode: {:?}, lftimeout: {:?}, autostartem0em1: {=bool:?}, autostartselem0em1: {=bool:?} }}" , self . mode () , self . hfxox2en () , self . peakdetmode () , self . lftimeout () , self . autostartem0em1 () , self . autostartselem0em1 ())
    }
}
#[doc = "HFXO Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxoctrl1(pub u32);
impl Hfxoctrl1 {
    #[doc = "Sets the Amplitude Detection Level (mV)."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdetthr(&self) -> super::vals::Peakdetthr {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Peakdetthr::from_bits(val as u8)
    }
    #[doc = "Sets the Amplitude Detection Level (mV)."]
    #[inline(always)]
    pub const fn set_peakdetthr(&mut self, val: super::vals::Peakdetthr) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
}
impl Default for Hfxoctrl1 {
    #[inline(always)]
    fn default() -> Hfxoctrl1 {
        Hfxoctrl1(0)
    }
}
impl core::fmt::Debug for Hfxoctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxoctrl1")
            .field("peakdetthr", &self.peakdetthr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxoctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hfxoctrl1 {{ peakdetthr: {:?} }}", self.peakdetthr())
    }
}
#[doc = "HFXO Startup Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxostartupctrl(pub u32);
impl Hfxostartupctrl {
    #[doc = "Sets the Startup Oscillator Core Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn ibtrimxocore(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Sets the Startup Oscillator Core Bias Current."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Sets Oscillator Tuning Capacitance."]
    #[must_use]
    #[inline(always)]
    pub const fn ctune(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x01ff;
        val as u16
    }
    #[doc = "Sets Oscillator Tuning Capacitance."]
    #[inline(always)]
    pub const fn set_ctune(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
    }
}
impl Default for Hfxostartupctrl {
    #[inline(always)]
    fn default() -> Hfxostartupctrl {
        Hfxostartupctrl(0)
    }
}
impl core::fmt::Debug for Hfxostartupctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxostartupctrl")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("ctune", &self.ctune())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxostartupctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfxostartupctrl {{ ibtrimxocore: {=u16:?}, ctune: {=u16:?} }}",
            self.ibtrimxocore(),
            self.ctune()
        )
    }
}
#[doc = "HFXO Steady State Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxosteadystatectrl(pub u32);
impl Hfxosteadystatectrl {
    #[doc = "Sets the Steady State Oscillator Core Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn ibtrimxocore(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Sets Oscillator Tuning Capacitance."]
    #[must_use]
    #[inline(always)]
    pub const fn ctune(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x01ff;
        val as u16
    }
    #[doc = "Sets Oscillator Tuning Capacitance."]
    #[inline(always)]
    pub const fn set_ctune(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
    }
    #[doc = "Enables Oscillator Peak Detectors."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdeten(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Oscillator Peak Detectors."]
    #[inline(always)]
    pub const fn set_peakdeten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO."]
    #[must_use]
    #[inline(always)]
    pub const fn peakmonen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO."]
    #[inline(always)]
    pub const fn set_peakmonen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Hfxosteadystatectrl {
    #[inline(always)]
    fn default() -> Hfxosteadystatectrl {
        Hfxosteadystatectrl(0)
    }
}
impl core::fmt::Debug for Hfxosteadystatectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxosteadystatectrl")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("ctune", &self.ctune())
            .field("peakdeten", &self.peakdeten())
            .field("peakmonen", &self.peakmonen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxosteadystatectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxosteadystatectrl {{ ibtrimxocore: {=u16:?}, ctune: {=u16:?}, peakdeten: {=bool:?}, peakmonen: {=bool:?} }}" , self . ibtrimxocore () , self . ctune () , self . peakdeten () , self . peakmonen ())
    }
}
#[doc = "HFXO Timeout Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxotimeoutctrl(pub u32);
impl Hfxotimeoutctrl {
    #[doc = "Wait Duration in HFXO Startup Enable Wait State."]
    #[must_use]
    #[inline(always)]
    pub const fn startuptimeout(&self) -> super::vals::Startuptimeout {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Startuptimeout::from_bits(val as u8)
    }
    #[doc = "Wait Duration in HFXO Startup Enable Wait State."]
    #[inline(always)]
    pub const fn set_startuptimeout(&mut self, val: super::vals::Startuptimeout) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Wait Duration in HFXO Startup Steady Wait State."]
    #[must_use]
    #[inline(always)]
    pub const fn steadytimeout(&self) -> super::vals::Steadytimeout {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Steadytimeout::from_bits(val as u8)
    }
    #[doc = "Wait Duration in HFXO Startup Steady Wait State."]
    #[inline(always)]
    pub const fn set_steadytimeout(&mut self, val: super::vals::Steadytimeout) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Wait Duration in HFXO Peak Detection Wait State."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdettimeout(&self) -> super::vals::Peakdettimeout {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Peakdettimeout::from_bits(val as u8)
    }
    #[doc = "Wait Duration in HFXO Peak Detection Wait State."]
    #[inline(always)]
    pub const fn set_peakdettimeout(&mut self, val: super::vals::Peakdettimeout) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Hfxotimeoutctrl {
    #[inline(always)]
    fn default() -> Hfxotimeoutctrl {
        Hfxotimeoutctrl(0)
    }
}
impl core::fmt::Debug for Hfxotimeoutctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxotimeoutctrl")
            .field("startuptimeout", &self.startuptimeout())
            .field("steadytimeout", &self.steadytimeout())
            .field("peakdettimeout", &self.peakdettimeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxotimeoutctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfxotimeoutctrl {{ startuptimeout: {:?}, steadytimeout: {:?}, peakdettimeout: {:?} }}",
            self.startuptimeout(),
            self.steadytimeout(),
            self.peakdettimeout()
        )
    }
}
#[doc = "HFXO Trim Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxotrimstatus(pub u32);
impl Hfxotrimstatus {
    #[doc = "Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn ibtrimxocore(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)."]
    #[must_use]
    #[inline(always)]
    pub const fn ibtrimxocoremon(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)."]
    #[inline(always)]
    pub const fn set_ibtrimxocoremon(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Peak Detection Algorithm Found a Value for IBTRIMXOCORE."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Peak Detection Algorithm Found a Value for IBTRIMXOCORE."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON."]
    #[must_use]
    #[inline(always)]
    pub const fn monvalid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON."]
    #[inline(always)]
    pub const fn set_monvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hfxotrimstatus {
    #[inline(always)]
    fn default() -> Hfxotrimstatus {
        Hfxotrimstatus(0)
    }
}
impl core::fmt::Debug for Hfxotrimstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfxotrimstatus")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("ibtrimxocoremon", &self.ibtrimxocoremon())
            .field("valid", &self.valid())
            .field("monvalid", &self.monvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxotrimstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxotrimstatus {{ ibtrimxocore: {=u16:?}, ibtrimxocoremon: {=u16:?}, valid: {=bool:?}, monvalid: {=bool:?} }}" , self . ibtrimxocore () , self . ibtrimxocoremon () , self . valid () , self . monvalid ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "HFRCORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcordy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HFXORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxordy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HFXORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LFRCORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcordy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LFXORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxordy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LFXORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AUXHFRCORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcordy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_auxhfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CALRDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn calrdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CALRDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_calrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CALOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn calof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CALOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_calof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USHFRCORDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcordy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCORDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ushfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "HFXODISERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxodiserr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HFXODISERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxodiserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HFXOAUTOSW Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoautosw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HFXOAUTOSW Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxoautosw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HFXOPEAKDETRDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdetrdy(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HFXOPEAKDETRDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxopeakdetrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "HFRCODIS Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcodis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCODIS Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LFTIMEOUTERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lftimeouterr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LFTIMEOUTERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lftimeouterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DPLLRDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllrdy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DPLLRDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dpllrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DPLLLOCKFAILLOW Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfaillow(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DPLLLOCKFAILLOW Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dplllockfaillow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DPLLLOCKFAILHIGH Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfailhigh(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DPLLLOCKFAILHIGH Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dplllockfailhigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LFXOEDGE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxoedge(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LFXOEDGE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lfxoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LFRCOEDGE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcoedge(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCOEDGE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ULFRCOEDGE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ulfrcoedge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ULFRCOEDGE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ulfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "CMUERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmuerr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "CMUERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cmuerr(&mut self, val: bool) {
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
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxordy", &self.hfxordy())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxordy", &self.lfxordy())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("calrdy", &self.calrdy())
            .field("calof", &self.calof())
            .field("ushfrcordy", &self.ushfrcordy())
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("dpllrdy", &self.dpllrdy())
            .field("dplllockfaillow", &self.dplllockfaillow())
            .field("dplllockfailhigh", &self.dplllockfailhigh())
            .field("lfxoedge", &self.lfxoedge())
            .field("lfrcoedge", &self.lfrcoedge())
            .field("ulfrcoedge", &self.ulfrcoedge())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, ushfrcordy: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, dpllrdy: {=bool:?}, dplllockfaillow: {=bool:?}, dplllockfailhigh: {=bool:?}, lfxoedge: {=bool:?}, lfrcoedge: {=bool:?}, ulfrcoedge: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . ushfrcordy () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdetrdy () , self . hfrcodis () , self . lftimeouterr () , self . dpllrdy () , self . dplllockfaillow () , self . dplllockfailhigh () , self . lfxoedge () , self . lfrcoedge () , self . ulfrcoedge () , self . cmuerr ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "HFRCO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcordy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HFXO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxordy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LFRCO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcordy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LFXO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxordy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AUXHFRCO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcordy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_auxhfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Calibration Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn calrdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_calrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Calibration Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn calof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_calof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USHFRCO Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcordy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ushfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "HFXO Disable Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxodiserr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Disable Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxodiserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HFXO Automatic Switch Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoautosw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Automatic Switch Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxoautosw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HFXO Automatic Peak Detection Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdetrdy(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Automatic Peak Detection Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxopeakdetrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "HFRCO Disable Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcodis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Disable Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Low Frequency Timeout Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lftimeouterr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency Timeout Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lftimeouterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DPLL Lock Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllrdy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Lock Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dpllrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DPLL Lock Failure Low Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfaillow(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Lock Failure Low Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dplllockfaillow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DPLL Lock Failure Low Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfailhigh(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Lock Failure Low Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dplllockfailhigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LFXO Clock Edge Detected Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxoedge(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Clock Edge Detected Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfxoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LFRCO Clock Edge Detected Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcoedge(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Clock Edge Detected Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ULFRCO Clock Edge Detected Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ulfrcoedge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ULFRCO Clock Edge Detected Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ulfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "CMU Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmuerr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "CMU Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmuerr(&mut self, val: bool) {
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
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxordy", &self.hfxordy())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxordy", &self.lfxordy())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("calrdy", &self.calrdy())
            .field("calof", &self.calof())
            .field("ushfrcordy", &self.ushfrcordy())
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("dpllrdy", &self.dpllrdy())
            .field("dplllockfaillow", &self.dplllockfaillow())
            .field("dplllockfailhigh", &self.dplllockfailhigh())
            .field("lfxoedge", &self.lfxoedge())
            .field("lfrcoedge", &self.lfrcoedge())
            .field("ulfrcoedge", &self.ulfrcoedge())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, ushfrcordy: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, dpllrdy: {=bool:?}, dplllockfaillow: {=bool:?}, dplllockfailhigh: {=bool:?}, lfxoedge: {=bool:?}, lfrcoedge: {=bool:?}, ulfrcoedge: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . ushfrcordy () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdetrdy () , self . hfrcodis () , self . lftimeouterr () , self . dpllrdy () , self . dplllockfaillow () , self . dplllockfailhigh () , self . lfxoedge () , self . lfrcoedge () , self . ulfrcoedge () , self . cmuerr ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set HFRCORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcordy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFRCORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set HFXORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxordy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set LFRCORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcordy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set LFRCORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set LFXORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxordy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set LFXORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set AUXHFRCORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcordy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set AUXHFRCORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_auxhfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set CALRDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn calrdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set CALRDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_calrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set CALOF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn calof(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set CALOF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_calof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set USHFRCORDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcordy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set USHFRCORDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ushfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set HFXODISERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxodiserr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXODISERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxodiserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set HFXOAUTOSW Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoautosw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXOAUTOSW Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxoautosw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set HFXOPEAKDETRDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdetrdy(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXOPEAKDETRDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxopeakdetrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set HFRCODIS Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcodis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFRCODIS Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set LFTIMEOUTERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lftimeouterr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set LFTIMEOUTERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lftimeouterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set DPLLRDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllrdy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set DPLLRDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dpllrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set DPLLLOCKFAILLOW Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfaillow(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set DPLLLOCKFAILLOW Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dplllockfaillow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set DPLLLOCKFAILHIGH Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dplllockfailhigh(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set DPLLLOCKFAILHIGH Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dplllockfailhigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set LFXOEDGE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxoedge(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set LFXOEDGE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfxoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set LFRCOEDGE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcoedge(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set LFRCOEDGE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set ULFRCOEDGE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ulfrcoedge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set ULFRCOEDGE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ulfrcoedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Set CMUERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmuerr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set CMUERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_cmuerr(&mut self, val: bool) {
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
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxordy", &self.hfxordy())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxordy", &self.lfxordy())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("calrdy", &self.calrdy())
            .field("calof", &self.calof())
            .field("ushfrcordy", &self.ushfrcordy())
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("dpllrdy", &self.dpllrdy())
            .field("dplllockfaillow", &self.dplllockfaillow())
            .field("dplllockfailhigh", &self.dplllockfailhigh())
            .field("lfxoedge", &self.lfxoedge())
            .field("lfrcoedge", &self.lfrcoedge())
            .field("ulfrcoedge", &self.ulfrcoedge())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, ushfrcordy: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, dpllrdy: {=bool:?}, dplllockfaillow: {=bool:?}, dplllockfailhigh: {=bool:?}, lfxoedge: {=bool:?}, lfrcoedge: {=bool:?}, ulfrcoedge: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . ushfrcordy () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdetrdy () , self . hfrcodis () , self . lftimeouterr () , self . dpllrdy () , self . dplllockfaillow () , self . dplllockfailhigh () , self . lfxoedge () , self . lfrcoedge () , self . ulfrcoedge () , self . cmuerr ())
    }
}
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfaclken0(pub u32);
impl Lfaclken0 {
    #[doc = "Low Energy Timer 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_letimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Energy Timer 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Timer 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_letimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Energy Sensor Interface Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Sensor Interface Clock Enable."]
    #[inline(always)]
    pub const fn set_lesense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Liquid Crystal Display Controller Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Liquid Crystal Display Controller Clock Enable."]
    #[inline(always)]
    pub const fn set_lcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Real-Time Counter Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Real-Time Counter Clock Enable."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Lfaclken0 {
    #[inline(always)]
    fn default() -> Lfaclken0 {
        Lfaclken0(0)
    }
}
impl core::fmt::Debug for Lfaclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfaclken0")
            .field("letimer0", &self.letimer0())
            .field("letimer1", &self.letimer1())
            .field("lesense", &self.lesense())
            .field("lcd", &self.lcd())
            .field("rtc", &self.rtc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfaclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lfaclken0 {{ letimer0: {=bool:?}, letimer1: {=bool:?}, lesense: {=bool:?}, lcd: {=bool:?}, rtc: {=bool:?} }}" , self . letimer0 () , self . letimer1 () , self . lesense () , self . lcd () , self . rtc ())
    }
}
#[doc = "Low Frequency A Clock Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfaclksel(pub u32);
impl Lfaclksel {
    #[doc = "Clock Select for LFA."]
    #[must_use]
    #[inline(always)]
    pub const fn lfa(&self) -> super::vals::Lfa {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lfa::from_bits(val as u8)
    }
    #[doc = "Clock Select for LFA."]
    #[inline(always)]
    pub const fn set_lfa(&mut self, val: super::vals::Lfa) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lfaclksel {
    #[inline(always)]
    fn default() -> Lfaclksel {
        Lfaclksel(0)
    }
}
impl core::fmt::Debug for Lfaclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfaclksel")
            .field("lfa", &self.lfa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfaclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfaclksel {{ lfa: {:?} }}", self.lfa())
    }
}
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfapresc0(pub u32);
impl Lfapresc0 {
    #[doc = "Low Energy Timer 0 Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer0(&self) -> super::vals::Letimer0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Letimer0::from_bits(val as u8)
    }
    #[doc = "Low Energy Timer 0 Prescaler."]
    #[inline(always)]
    pub const fn set_letimer0(&mut self, val: super::vals::Letimer0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Low Energy Timer 1 Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn letimer1(&self) -> super::vals::Letimer1 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Letimer1::from_bits(val as u8)
    }
    #[doc = "Low Energy Timer 1 Prescaler."]
    #[inline(always)]
    pub const fn set_letimer1(&mut self, val: super::vals::Letimer1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Low Energy Sensor Interface Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn lesense(&self) -> super::vals::Lesense {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lesense::from_bits(val as u8)
    }
    #[doc = "Low Energy Sensor Interface Prescaler."]
    #[inline(always)]
    pub const fn set_lesense(&mut self, val: super::vals::Lesense) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Liquid Crystal Display Controller Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd(&self) -> super::vals::Lcd {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Lcd::from_bits(val as u8)
    }
    #[doc = "Liquid Crystal Display Controller Prescaler."]
    #[inline(always)]
    pub const fn set_lcd(&mut self, val: super::vals::Lcd) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Real-Time Counter Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> super::vals::Rtc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Rtc::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter Prescaler."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: super::vals::Rtc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Lfapresc0 {
    #[inline(always)]
    fn default() -> Lfapresc0 {
        Lfapresc0(0)
    }
}
impl core::fmt::Debug for Lfapresc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfapresc0")
            .field("letimer0", &self.letimer0())
            .field("letimer1", &self.letimer1())
            .field("lesense", &self.lesense())
            .field("lcd", &self.lcd())
            .field("rtc", &self.rtc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfapresc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lfapresc0 {{ letimer0: {:?}, letimer1: {:?}, lesense: {:?}, lcd: {:?}, rtc: {:?} }}",
            self.letimer0(),
            self.letimer1(),
            self.lesense(),
            self.lcd(),
            self.rtc()
        )
    }
}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfbclken0(pub u32);
impl Lfbclken0 {
    #[doc = "Low Energy UART 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy UART 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_leuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Energy UART 1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy UART 1 Clock Enable."]
    #[inline(always)]
    pub const fn set_leuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn systick(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Enable."]
    #[inline(always)]
    pub const fn set_systick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capacitive touch sense module Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn csen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capacitive touch sense module Clock Enable."]
    #[inline(always)]
    pub const fn set_csen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Lfbclken0 {
    #[inline(always)]
    fn default() -> Lfbclken0 {
        Lfbclken0(0)
    }
}
impl core::fmt::Debug for Lfbclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfbclken0")
            .field("leuart0", &self.leuart0())
            .field("leuart1", &self.leuart1())
            .field("systick", &self.systick())
            .field("csen", &self.csen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfbclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lfbclken0 {{ leuart0: {=bool:?}, leuart1: {=bool:?}, systick: {=bool:?}, csen: {=bool:?} }}" , self . leuart0 () , self . leuart1 () , self . systick () , self . csen ())
    }
}
#[doc = "Low Frequency B Clock Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfbclksel(pub u32);
impl Lfbclksel {
    #[doc = "Clock Select for LFB."]
    #[must_use]
    #[inline(always)]
    pub const fn lfb(&self) -> super::vals::Lfb {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lfb::from_bits(val as u8)
    }
    #[doc = "Clock Select for LFB."]
    #[inline(always)]
    pub const fn set_lfb(&mut self, val: super::vals::Lfb) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lfbclksel {
    #[inline(always)]
    fn default() -> Lfbclksel {
        Lfbclksel(0)
    }
}
impl core::fmt::Debug for Lfbclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfbclksel")
            .field("lfb", &self.lfb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfbclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfbclksel {{ lfb: {:?} }}", self.lfb())
    }
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfbpresc0(pub u32);
impl Lfbpresc0 {
    #[doc = "Low Energy UART 0 Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart0(&self) -> super::vals::Leuart0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Leuart0::from_bits(val as u8)
    }
    #[doc = "Low Energy UART 0 Prescaler."]
    #[inline(always)]
    pub const fn set_leuart0(&mut self, val: super::vals::Leuart0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Low Energy UART 1 Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn leuart1(&self) -> super::vals::Leuart1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Leuart1::from_bits(val as u8)
    }
    #[doc = "Low Energy UART 1 Prescaler."]
    #[inline(always)]
    pub const fn set_leuart1(&mut self, val: super::vals::Leuart1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn systick(&self) -> super::vals::Systick {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Systick::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_systick(&mut self, val: super::vals::Systick) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Capacitive touch sense module Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn csen(&self) -> super::vals::Csen {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Csen::from_bits(val as u8)
    }
    #[doc = "Capacitive touch sense module Prescaler."]
    #[inline(always)]
    pub const fn set_csen(&mut self, val: super::vals::Csen) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Lfbpresc0 {
    #[inline(always)]
    fn default() -> Lfbpresc0 {
        Lfbpresc0(0)
    }
}
impl core::fmt::Debug for Lfbpresc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfbpresc0")
            .field("leuart0", &self.leuart0())
            .field("leuart1", &self.leuart1())
            .field("systick", &self.systick())
            .field("csen", &self.csen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfbpresc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lfbpresc0 {{ leuart0: {:?}, leuart1: {:?}, systick: {:?}, csen: {:?} }}",
            self.leuart0(),
            self.leuart1(),
            self.systick(),
            self.csen()
        )
    }
}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfcclken0(pub u32);
impl Lfcclken0 {
    #[doc = "Universal Serial Bus Interface Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Universal Serial Bus Interface Clock Enable."]
    #[inline(always)]
    pub const fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lfcclken0 {
    #[inline(always)]
    fn default() -> Lfcclken0 {
        Lfcclken0(0)
    }
}
impl core::fmt::Debug for Lfcclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfcclken0")
            .field("usb", &self.usb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfcclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfcclken0 {{ usb: {=bool:?} }}", self.usb())
    }
}
#[doc = "Low Frequency C Clock Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfcclksel(pub u32);
impl Lfcclksel {
    #[doc = "Clock Select for LFC."]
    #[must_use]
    #[inline(always)]
    pub const fn lfc(&self) -> super::vals::Lfc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lfc::from_bits(val as u8)
    }
    #[doc = "Clock Select for LFC."]
    #[inline(always)]
    pub const fn set_lfc(&mut self, val: super::vals::Lfc) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lfcclksel {
    #[inline(always)]
    fn default() -> Lfcclksel {
        Lfcclksel(0)
    }
}
impl core::fmt::Debug for Lfcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfcclksel")
            .field("lfc", &self.lfc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfcclksel {{ lfc: {:?} }}", self.lfc())
    }
}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfeclken0(pub u32);
impl Lfeclken0 {
    #[doc = "Real-Time Counter and Calendar Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Real-Time Counter and Calendar Clock Enable."]
    #[inline(always)]
    pub const fn set_rtcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lfeclken0 {
    #[inline(always)]
    fn default() -> Lfeclken0 {
        Lfeclken0(0)
    }
}
impl core::fmt::Debug for Lfeclken0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfeclken0")
            .field("rtcc", &self.rtcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfeclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfeclken0 {{ rtcc: {=bool:?} }}", self.rtcc())
    }
}
#[doc = "Low Frequency E Clock Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfeclksel(pub u32);
impl Lfeclksel {
    #[doc = "Clock Select for LFE."]
    #[must_use]
    #[inline(always)]
    pub const fn lfe(&self) -> super::vals::Lfe {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lfe::from_bits(val as u8)
    }
    #[doc = "Clock Select for LFE."]
    #[inline(always)]
    pub const fn set_lfe(&mut self, val: super::vals::Lfe) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lfeclksel {
    #[inline(always)]
    fn default() -> Lfeclksel {
        Lfeclksel(0)
    }
}
impl core::fmt::Debug for Lfeclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfeclksel")
            .field("lfe", &self.lfe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfeclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfeclksel {{ lfe: {:?} }}", self.lfe())
    }
}
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfepresc0(pub u32);
impl Lfepresc0 {
    #[doc = "Real-Time Counter and Calendar Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcc(&self) -> super::vals::Rtcc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rtcc::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter and Calendar Prescaler."]
    #[inline(always)]
    pub const fn set_rtcc(&mut self, val: super::vals::Rtcc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lfepresc0 {
    #[inline(always)]
    fn default() -> Lfepresc0 {
        Lfepresc0(0)
    }
}
impl core::fmt::Debug for Lfepresc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfepresc0")
            .field("rtcc", &self.rtcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfepresc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfepresc0 {{ rtcc: {:?} }}", self.rtcc())
    }
}
#[doc = "LFRCO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfrcoctrl(pub u32);
impl Lfrcoctrl {
    #[doc = "LFRCO Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "LFRCO Tuning Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Enable Duty Cycling of Vref."]
    #[must_use]
    #[inline(always)]
    pub const fn envref(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Duty Cycling of Vref."]
    #[inline(always)]
    pub const fn set_envref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable Comparator Chopping."]
    #[must_use]
    #[inline(always)]
    pub const fn enchop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Comparator Chopping."]
    #[inline(always)]
    pub const fn set_enchop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable Dynamic Element Matching."]
    #[must_use]
    #[inline(always)]
    pub const fn endem(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Dynamic Element Matching."]
    #[inline(always)]
    pub const fn set_endem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Control Vref Update Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefupdate(&self) -> super::vals::Vrefupdate {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Vrefupdate::from_bits(val as u8)
    }
    #[doc = "Control Vref Update Rate."]
    #[inline(always)]
    pub const fn set_vrefupdate(&mut self, val: super::vals::Vrefupdate) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LFRCO Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> super::vals::LfrcoctrlTimeout {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::LfrcoctrlTimeout::from_bits(val as u8)
    }
    #[doc = "LFRCO Timeout."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: super::vals::LfrcoctrlTimeout) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Tuning of Gmc Current."]
    #[must_use]
    #[inline(always)]
    pub const fn gmccurtune(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Tuning of Gmc Current."]
    #[inline(always)]
    pub const fn set_gmccurtune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Lfrcoctrl {
    #[inline(always)]
    fn default() -> Lfrcoctrl {
        Lfrcoctrl(0)
    }
}
impl core::fmt::Debug for Lfrcoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfrcoctrl")
            .field("tuning", &self.tuning())
            .field("envref", &self.envref())
            .field("enchop", &self.enchop())
            .field("endem", &self.endem())
            .field("vrefupdate", &self.vrefupdate())
            .field("timeout", &self.timeout())
            .field("gmccurtune", &self.gmccurtune())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lfrcoctrl {{ tuning: {=u16:?}, envref: {=bool:?}, enchop: {=bool:?}, endem: {=bool:?}, vrefupdate: {:?}, timeout: {:?}, gmccurtune: {=u8:?} }}" , self . tuning () , self . envref () , self . enchop () , self . endem () , self . vrefupdate () , self . timeout () , self . gmccurtune ())
    }
}
#[doc = "LFXO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfxoctrl(pub u32);
impl Lfxoctrl {
    #[doc = "LFXO Internal Capacitor Array Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "LFXO Internal Capacitor Array Tuning Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "LFXO Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::LfxoctrlMode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LfxoctrlMode::from_bits(val as u8)
    }
    #[doc = "LFXO Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::LfxoctrlMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LFXO Startup Gain."]
    #[must_use]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "LFXO Startup Gain."]
    #[inline(always)]
    pub const fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "LFXO High XTAL Oscillation Amplitude Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn highampl(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO High XTAL Oscillation Amplitude Enable."]
    #[inline(always)]
    pub const fn set_highampl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LFXO AGC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn agc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO AGC Enable."]
    #[inline(always)]
    pub const fn set_agc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "LFXO Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cur(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "LFXO Current Trim."]
    #[inline(always)]
    pub const fn set_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LFXO Buffer Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn bufcur(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Buffer Bias Current."]
    #[inline(always)]
    pub const fn set_bufcur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LFXO Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> super::vals::LfxoctrlTimeout {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::LfxoctrlTimeout::from_bits(val as u8)
    }
    #[doc = "LFXO Timeout."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: super::vals::LfxoctrlTimeout) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Lfxoctrl {
    #[inline(always)]
    fn default() -> Lfxoctrl {
        Lfxoctrl(0)
    }
}
impl core::fmt::Debug for Lfxoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lfxoctrl")
            .field("tuning", &self.tuning())
            .field("mode", &self.mode())
            .field("gain", &self.gain())
            .field("highampl", &self.highampl())
            .field("agc", &self.agc())
            .field("cur", &self.cur())
            .field("bufcur", &self.bufcur())
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfxoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lfxoctrl {{ tuning: {=u8:?}, mode: {:?}, gain: {=u8:?}, highampl: {=bool:?}, agc: {=bool:?}, cur: {=u8:?}, bufcur: {=bool:?}, timeout: {:?} }}" , self . tuning () , self . mode () , self . gain () , self . highampl () , self . agc () , self . cur () , self . bufcur () , self . timeout ())
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
#[doc = "Oscillator Enable/Disable Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscencmd(pub u32);
impl Oscencmd {
    #[doc = "HFRCO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcoen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Enable."]
    #[inline(always)]
    pub const fn set_hfrcoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HFRCO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcodis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Disable."]
    #[inline(always)]
    pub const fn set_hfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HFXO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Enable."]
    #[inline(always)]
    pub const fn set_hfxoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HFXO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxodis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Disable."]
    #[inline(always)]
    pub const fn set_hfxodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AUXHFRCO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcoen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Enable."]
    #[inline(always)]
    pub const fn set_auxhfrcoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "AUXHFRCO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcodis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Disable."]
    #[inline(always)]
    pub const fn set_auxhfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LFRCO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcoen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Enable."]
    #[inline(always)]
    pub const fn set_lfrcoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LFRCO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcodis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Disable."]
    #[inline(always)]
    pub const fn set_lfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LFXO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Enable."]
    #[inline(always)]
    pub const fn set_lfxoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LFXO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxodis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Disable."]
    #[inline(always)]
    pub const fn set_lfxodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "USHFRCO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcoen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Enable."]
    #[inline(always)]
    pub const fn set_ushfrcoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "USHFRCO Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcodis(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Disable."]
    #[inline(always)]
    pub const fn set_ushfrcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DPLL Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Enable."]
    #[inline(always)]
    pub const fn set_dpllen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DPLL Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn dplldis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Disable."]
    #[inline(always)]
    pub const fn set_dplldis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Oscencmd {
    #[inline(always)]
    fn default() -> Oscencmd {
        Oscencmd(0)
    }
}
impl core::fmt::Debug for Oscencmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscencmd")
            .field("hfrcoen", &self.hfrcoen())
            .field("hfrcodis", &self.hfrcodis())
            .field("hfxoen", &self.hfxoen())
            .field("hfxodis", &self.hfxodis())
            .field("auxhfrcoen", &self.auxhfrcoen())
            .field("auxhfrcodis", &self.auxhfrcodis())
            .field("lfrcoen", &self.lfrcoen())
            .field("lfrcodis", &self.lfrcodis())
            .field("lfxoen", &self.lfxoen())
            .field("lfxodis", &self.lfxodis())
            .field("ushfrcoen", &self.ushfrcoen())
            .field("ushfrcodis", &self.ushfrcodis())
            .field("dpllen", &self.dpllen())
            .field("dplldis", &self.dplldis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscencmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Oscencmd {{ hfrcoen: {=bool:?}, hfrcodis: {=bool:?}, hfxoen: {=bool:?}, hfxodis: {=bool:?}, auxhfrcoen: {=bool:?}, auxhfrcodis: {=bool:?}, lfrcoen: {=bool:?}, lfrcodis: {=bool:?}, lfxoen: {=bool:?}, lfxodis: {=bool:?}, ushfrcoen: {=bool:?}, ushfrcodis: {=bool:?}, dpllen: {=bool:?}, dplldis: {=bool:?} }}" , self . hfrcoen () , self . hfrcodis () , self . hfxoen () , self . hfxodis () , self . auxhfrcoen () , self . auxhfrcodis () , self . lfrcoen () , self . lfrcodis () , self . lfxoen () , self . lfxodis () , self . ushfrcoen () , self . ushfrcodis () , self . dpllen () , self . dplldis ())
    }
}
#[doc = "PCNT Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntctrl(pub u32);
impl Pcntctrl {
    #[doc = "PCNT0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0clken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT0 Clock Enable."]
    #[inline(always)]
    pub const fn set_pcnt0clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PCNT0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt0clksel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT0 Clock Select."]
    #[inline(always)]
    pub const fn set_pcnt0clksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PCNT1 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1clken(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT1 Clock Enable."]
    #[inline(always)]
    pub const fn set_pcnt1clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PCNT1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt1clksel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT1 Clock Select."]
    #[inline(always)]
    pub const fn set_pcnt1clksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PCNT2 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2clken(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT2 Clock Enable."]
    #[inline(always)]
    pub const fn set_pcnt2clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PCNT2 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnt2clksel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PCNT2 Clock Select."]
    #[inline(always)]
    pub const fn set_pcnt2clksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pcntctrl {
    #[inline(always)]
    fn default() -> Pcntctrl {
        Pcntctrl(0)
    }
}
impl core::fmt::Debug for Pcntctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcntctrl")
            .field("pcnt0clken", &self.pcnt0clken())
            .field("pcnt0clksel", &self.pcnt0clksel())
            .field("pcnt1clken", &self.pcnt1clken())
            .field("pcnt1clksel", &self.pcnt1clksel())
            .field("pcnt2clken", &self.pcnt2clken())
            .field("pcnt2clksel", &self.pcnt2clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pcntctrl {{ pcnt0clken: {=bool:?}, pcnt0clksel: {=bool:?}, pcnt1clken: {=bool:?}, pcnt1clksel: {=bool:?}, pcnt2clken: {=bool:?}, pcnt2clksel: {=bool:?} }}" , self . pcnt0clken () , self . pcnt0clksel () , self . pcnt1clken () , self . pcnt1clksel () , self . pcnt2clken () , self . pcnt2clksel ())
    }
}
#[doc = "QSPI Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspictrl(pub u32);
impl Qspictrl {
    #[doc = "QSPI0 Reference Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn qspi0clksel(&self) -> super::vals::Qspi0clksel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qspi0clksel::from_bits(val as u8)
    }
    #[doc = "QSPI0 Reference Clock Select."]
    #[inline(always)]
    pub const fn set_qspi0clksel(&mut self, val: super::vals::Qspi0clksel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "QSPI0 Reference Clock Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn qspi0clkdis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "QSPI0 Reference Clock Disable."]
    #[inline(always)]
    pub const fn set_qspi0clkdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Qspictrl {
    #[inline(always)]
    fn default() -> Qspictrl {
        Qspictrl(0)
    }
}
impl core::fmt::Debug for Qspictrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qspictrl")
            .field("qspi0clksel", &self.qspi0clksel())
            .field("qspi0clkdis", &self.qspi0clkdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qspictrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qspictrl {{ qspi0clksel: {:?}, qspi0clkdis: {=bool:?} }}",
            self.qspi0clksel(),
            self.qspi0clkdis()
        )
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
    pub const fn clkout0loc(&self) -> super::vals::Clkout0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Clkout0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_clkout0loc(&mut self, val: super::vals::Clkout0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout1loc(&self) -> super::vals::Clkout1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Clkout1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_clkout1loc(&mut self, val: super::vals::Clkout1loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout2loc(&self) -> super::vals::Clkout2loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Clkout2loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_clkout2loc(&mut self, val: super::vals::Clkout2loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
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
            .field("clkout0loc", &self.clkout0loc())
            .field("clkout1loc", &self.clkout1loc())
            .field("clkout2loc", &self.clkout2loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ clkout0loc: {:?}, clkout1loc: {:?}, clkout2loc: {:?} }}",
            self.clkout0loc(),
            self.clkout1loc(),
            self.clkout2loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin0loc(&self) -> super::vals::Clkin0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Clkin0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_clkin0loc(&mut self, val: super::vals::Clkin0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Routeloc1 {
    #[inline(always)]
    fn default() -> Routeloc1 {
        Routeloc1(0)
    }
}
impl core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc1")
            .field("clkin0loc", &self.clkin0loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routeloc1 {{ clkin0loc: {:?} }}", self.clkin0loc())
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CLKOUT0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CLKOUT0 Pin Enable."]
    #[inline(always)]
    pub const fn set_clkout0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CLKOUT1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CLKOUT1 Pin Enable."]
    #[inline(always)]
    pub const fn set_clkout1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CLKOUT2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout2pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CLKOUT2 Pin Enable."]
    #[inline(always)]
    pub const fn set_clkout2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CLKIN0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin0pen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "CLKIN0 Pin Enable."]
    #[inline(always)]
    pub const fn set_clkin0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("clkout0pen", &self.clkout0pen())
            .field("clkout1pen", &self.clkout1pen())
            .field("clkout2pen", &self.clkout2pen())
            .field("clkin0pen", &self.clkin0pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ clkout0pen: {=bool:?}, clkout1pen: {=bool:?}, clkout2pen: {=bool:?}, clkin0pen: {=bool:?} }}" , self . clkout0pen () , self . clkout1pen () , self . clkout2pen () , self . clkin0pen ())
    }
}
#[doc = "SDIO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdioctrl(pub u32);
impl Sdioctrl {
    #[doc = "SDIO Reference Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sdioclksel(&self) -> super::vals::Sdioclksel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sdioclksel::from_bits(val as u8)
    }
    #[doc = "SDIO Reference Clock Select."]
    #[inline(always)]
    pub const fn set_sdioclksel(&mut self, val: super::vals::Sdioclksel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SDIO Reference Clock Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdioclkdis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Reference Clock Disable."]
    #[inline(always)]
    pub const fn set_sdioclkdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sdioctrl {
    #[inline(always)]
    fn default() -> Sdioctrl {
        Sdioctrl(0)
    }
}
impl core::fmt::Debug for Sdioctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdioctrl")
            .field("sdioclksel", &self.sdioclksel())
            .field("sdioclkdis", &self.sdioclkdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdioctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdioctrl {{ sdioclksel: {:?}, sdioclkdis: {=bool:?} }}",
            self.sdioclksel(),
            self.sdioclkdis()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "HFRCO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcoens(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Enable Status."]
    #[inline(always)]
    pub const fn set_hfrcoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HFRCO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcordy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Ready."]
    #[inline(always)]
    pub const fn set_hfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HFXO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoens(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Enable Status."]
    #[inline(always)]
    pub const fn set_hfxoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HFXO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxordy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Ready."]
    #[inline(always)]
    pub const fn set_hfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AUXHFRCO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcoens(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Enable Status."]
    #[inline(always)]
    pub const fn set_auxhfrcoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "AUXHFRCO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcordy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Ready."]
    #[inline(always)]
    pub const fn set_auxhfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LFRCO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcoens(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Enable Status."]
    #[inline(always)]
    pub const fn set_lfrcoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LFRCO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcordy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Ready."]
    #[inline(always)]
    pub const fn set_lfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LFXO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxoens(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Enable Status."]
    #[inline(always)]
    pub const fn set_lfxoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LFXO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxordy(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Ready."]
    #[inline(always)]
    pub const fn set_lfxordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "USHFRCO Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcoens(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Enable Status."]
    #[inline(always)]
    pub const fn set_ushfrcoens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "USHFRCO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcordy(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Ready."]
    #[inline(always)]
    pub const fn set_ushfrcordy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DPLL Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllens(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Enable Status."]
    #[inline(always)]
    pub const fn set_dpllens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DPLL Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn dpllrdy(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DPLL Ready."]
    #[inline(always)]
    pub const fn set_dpllrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Calibration Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn calrdy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Ready."]
    #[inline(always)]
    pub const fn set_calrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SDIO Clock Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn sdioclkens(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Clock Enabled Status."]
    #[inline(always)]
    pub const fn set_sdioclkens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "QSPI0 Clock Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn qspi0clkens(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "QSPI0 Clock Enabled Status."]
    #[inline(always)]
    pub const fn set_qspi0clkens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "HFXO Peak Detection Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdetrdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Peak Detection Ready."]
    #[inline(always)]
    pub const fn set_hfxopeakdetrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "HFXO Amplitude Tuning Value Too Low."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoamplow(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Amplitude Tuning Value Too Low."]
    #[inline(always)]
    pub const fn set_hfxoamplow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LFXO Clock Phase."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxophase(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Clock Phase."]
    #[inline(always)]
    pub const fn set_lfxophase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LFRCO Clock Phase."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcophase(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Clock Phase."]
    #[inline(always)]
    pub const fn set_lfrcophase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ULFRCO Clock Phase."]
    #[must_use]
    #[inline(always)]
    pub const fn ulfrcophase(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ULFRCO Clock Phase."]
    #[inline(always)]
    pub const fn set_ulfrcophase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("hfrcoens", &self.hfrcoens())
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxoens", &self.hfxoens())
            .field("hfxordy", &self.hfxordy())
            .field("auxhfrcoens", &self.auxhfrcoens())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("lfrcoens", &self.lfrcoens())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxoens", &self.lfxoens())
            .field("lfxordy", &self.lfxordy())
            .field("ushfrcoens", &self.ushfrcoens())
            .field("ushfrcordy", &self.ushfrcordy())
            .field("dpllens", &self.dpllens())
            .field("dpllrdy", &self.dpllrdy())
            .field("calrdy", &self.calrdy())
            .field("sdioclkens", &self.sdioclkens())
            .field("qspi0clkens", &self.qspi0clkens())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoamplow", &self.hfxoamplow())
            .field("lfxophase", &self.lfxophase())
            .field("lfrcophase", &self.lfrcophase())
            .field("ulfrcophase", &self.ulfrcophase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ hfrcoens: {=bool:?}, hfrcordy: {=bool:?}, hfxoens: {=bool:?}, hfxordy: {=bool:?}, auxhfrcoens: {=bool:?}, auxhfrcordy: {=bool:?}, lfrcoens: {=bool:?}, lfrcordy: {=bool:?}, lfxoens: {=bool:?}, lfxordy: {=bool:?}, ushfrcoens: {=bool:?}, ushfrcordy: {=bool:?}, dpllens: {=bool:?}, dpllrdy: {=bool:?}, calrdy: {=bool:?}, sdioclkens: {=bool:?}, qspi0clkens: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfxoamplow: {=bool:?}, lfxophase: {=bool:?}, lfrcophase: {=bool:?}, ulfrcophase: {=bool:?} }}" , self . hfrcoens () , self . hfrcordy () , self . hfxoens () , self . hfxordy () , self . auxhfrcoens () , self . auxhfrcordy () , self . lfrcoens () , self . lfrcordy () , self . lfxoens () , self . lfxordy () , self . ushfrcoens () , self . ushfrcordy () , self . dpllens () , self . dpllrdy () , self . calrdy () , self . sdioclkens () , self . qspi0clkens () , self . hfxopeakdetrdy () , self . hfxoamplow () , self . lfxophase () , self . lfrcophase () , self . ulfrcophase ())
    }
}
#[doc = "Synchronization Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc = "Low Frequency a Clock Enable 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfaclken0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency a Clock Enable 0 Busy."]
    #[inline(always)]
    pub const fn set_lfaclken0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Frequency a Prescaler 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfapresc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency a Prescaler 0 Busy."]
    #[inline(always)]
    pub const fn set_lfapresc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low Frequency B Clock Enable 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfbclken0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency B Clock Enable 0 Busy."]
    #[inline(always)]
    pub const fn set_lfbclken0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Low Frequency B Prescaler 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfbpresc0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency B Prescaler 0 Busy."]
    #[inline(always)]
    pub const fn set_lfbpresc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low Frequency C Clock Enable 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfcclken0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency C Clock Enable 0 Busy."]
    #[inline(always)]
    pub const fn set_lfcclken0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Low Frequency E Clock Enable 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfeclken0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency E Clock Enable 0 Busy."]
    #[inline(always)]
    pub const fn set_lfeclken0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Frequency E Prescaler 0 Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfepresc0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Low Frequency E Prescaler 0 Busy."]
    #[inline(always)]
    pub const fn set_lfepresc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "HFRCO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn hfrcobsy(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HFRCO Busy."]
    #[inline(always)]
    pub const fn set_hfrcobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "AUXHFRCO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn auxhfrcobsy(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "AUXHFRCO Busy."]
    #[inline(always)]
    pub const fn set_auxhfrcobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LFRCO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcobsy(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO Busy."]
    #[inline(always)]
    pub const fn set_lfrcobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LFRCO VREF Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfrcovrefbsy(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LFRCO VREF Busy."]
    #[inline(always)]
    pub const fn set_lfrcovrefbsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "HFXO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxobsy(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Busy."]
    #[inline(always)]
    pub const fn set_hfxobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LFXO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn lfxobsy(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LFXO Busy."]
    #[inline(always)]
    pub const fn set_lfxobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "USHFRCO Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn ushfrcobsy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO Busy."]
    #[inline(always)]
    pub const fn set_ushfrcobsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("lfaclken0", &self.lfaclken0())
            .field("lfapresc0", &self.lfapresc0())
            .field("lfbclken0", &self.lfbclken0())
            .field("lfbpresc0", &self.lfbpresc0())
            .field("lfcclken0", &self.lfcclken0())
            .field("lfeclken0", &self.lfeclken0())
            .field("lfepresc0", &self.lfepresc0())
            .field("hfrcobsy", &self.hfrcobsy())
            .field("auxhfrcobsy", &self.auxhfrcobsy())
            .field("lfrcobsy", &self.lfrcobsy())
            .field("lfrcovrefbsy", &self.lfrcovrefbsy())
            .field("hfxobsy", &self.hfxobsy())
            .field("lfxobsy", &self.lfxobsy())
            .field("ushfrcobsy", &self.ushfrcobsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Syncbusy {{ lfaclken0: {=bool:?}, lfapresc0: {=bool:?}, lfbclken0: {=bool:?}, lfbpresc0: {=bool:?}, lfcclken0: {=bool:?}, lfeclken0: {=bool:?}, lfepresc0: {=bool:?}, hfrcobsy: {=bool:?}, auxhfrcobsy: {=bool:?}, lfrcobsy: {=bool:?}, lfrcovrefbsy: {=bool:?}, hfxobsy: {=bool:?}, lfxobsy: {=bool:?}, ushfrcobsy: {=bool:?} }}" , self . lfaclken0 () , self . lfapresc0 () , self . lfbclken0 () , self . lfbpresc0 () , self . lfcclken0 () , self . lfeclken0 () , self . lfepresc0 () , self . hfrcobsy () , self . auxhfrcobsy () , self . lfrcobsy () , self . lfrcovrefbsy () , self . hfxobsy () , self . lfxobsy () , self . ushfrcobsy ())
    }
}
#[doc = "USB Clock Recovery Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcrctrl(pub u32);
impl Usbcrctrl {
    #[doc = "Clock Recovery Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usbcren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Recovery Enable."]
    #[inline(always)]
    pub const fn set_usbcren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Speed Clock Recovery Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn usblscrmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Speed Clock Recovery Mode."]
    #[inline(always)]
    pub const fn set_usblscrmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Usbcrctrl {
    #[inline(always)]
    fn default() -> Usbcrctrl {
        Usbcrctrl(0)
    }
}
impl core::fmt::Debug for Usbcrctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcrctrl")
            .field("usbcren", &self.usbcren())
            .field("usblscrmd", &self.usblscrmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcrctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbcrctrl {{ usbcren: {=bool:?}, usblscrmd: {=bool:?} }}",
            self.usbcren(),
            self.usblscrmd()
        )
    }
}
#[doc = "USB Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbctrl(pub u32);
impl Usbctrl {
    #[doc = "USB Rate Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn usbclksel(&self) -> super::vals::Usbclksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usbclksel::from_bits(val as u8)
    }
    #[doc = "USB Rate Clock Select."]
    #[inline(always)]
    pub const fn set_usbclksel(&mut self, val: super::vals::Usbclksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "USB Rate Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usbclken(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB Rate Clock Enable."]
    #[inline(always)]
    pub const fn set_usbclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Usbctrl {
    #[inline(always)]
    fn default() -> Usbctrl {
        Usbctrl(0)
    }
}
impl core::fmt::Debug for Usbctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbctrl")
            .field("usbclksel", &self.usbclksel())
            .field("usbclken", &self.usbclken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbctrl {{ usbclksel: {:?}, usbclken: {=bool:?} }}",
            self.usbclksel(),
            self.usbclken()
        )
    }
}
#[doc = "USHFRCO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ushfrcoctrl(pub u32);
impl Ushfrcoctrl {
    #[doc = "USHFRCO Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USHFRCO Tuning Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "USHFRCO Fine Tuning Value."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuning(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "USHFRCO Fine Tuning Value."]
    #[inline(always)]
    pub const fn set_finetuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "USHFRCO Frequency Range."]
    #[must_use]
    #[inline(always)]
    pub const fn freqrange(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "USHFRCO Frequency Range."]
    #[inline(always)]
    pub const fn set_freqrange(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "USHFRCO Comparator Bias Current."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpbias(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "USHFRCO Comparator Bias Current."]
    #[inline(always)]
    pub const fn set_cmpbias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "USHFRCO LDO High Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ldohp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "USHFRCO LDO High Power Mode."]
    #[inline(always)]
    pub const fn set_ldohp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Locally Divide USHFRCO Clock Output."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::UshfrcoctrlClkdiv {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::UshfrcoctrlClkdiv::from_bits(val as u8)
    }
    #[doc = "Locally Divide USHFRCO Clock Output."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: super::vals::UshfrcoctrlClkdiv) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn finetuningen(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Reference for Fine Tuning."]
    #[inline(always)]
    pub const fn set_finetuningen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USHFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[must_use]
    #[inline(always)]
    pub const fn vreftc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "USHFRCO Temperature Coefficient Trim on Comparator Reference."]
    #[inline(always)]
    pub const fn set_vreftc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ushfrcoctrl {
    #[inline(always)]
    fn default() -> Ushfrcoctrl {
        Ushfrcoctrl(0)
    }
}
impl core::fmt::Debug for Ushfrcoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ushfrcoctrl")
            .field("tuning", &self.tuning())
            .field("finetuning", &self.finetuning())
            .field("freqrange", &self.freqrange())
            .field("cmpbias", &self.cmpbias())
            .field("ldohp", &self.ldohp())
            .field("clkdiv", &self.clkdiv())
            .field("finetuningen", &self.finetuningen())
            .field("vreftc", &self.vreftc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ushfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ushfrcoctrl {{ tuning: {=u8:?}, finetuning: {=u8:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, ldohp: {=bool:?}, clkdiv: {:?}, finetuningen: {=bool:?}, vreftc: {=u8:?} }}" , self . tuning () , self . finetuning () , self . freqrange () , self . cmpbias () , self . ldohp () , self . clkdiv () , self . finetuningen () , self . vreftc ())
    }
}
