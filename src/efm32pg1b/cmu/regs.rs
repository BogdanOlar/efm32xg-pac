#[doc = "ADC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcctrl(pub u32);
impl Adcctrl {
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
            .field("adc0clksel", &self.adc0clksel())
            .field("adc0clkinv", &self.adc0clkinv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcctrl {{ adc0clksel: {:?}, adc0clkinv: {=bool:?} }}",
            self.adc0clksel(),
            self.adc0clkinv()
        )
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
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Downsel::from_bits(val as u8)
    }
    #[doc = "Calibration Down-counter Select."]
    #[inline(always)]
    pub const fn set_downsel(&mut self, val: super::vals::Downsel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
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
    pub const fn prsupsel(&self) -> super::vals::Prsupsel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Prsupsel::from_bits(val as u8)
    }
    #[doc = "PRS Select for PRS Input When Selected in UPSEL."]
    #[inline(always)]
    pub const fn set_prsupsel(&mut self, val: super::vals::Prsupsel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "PRS Select for PRS Input When Selected in DOWNSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn prsdownsel(&self) -> super::vals::Prsdownsel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Prsdownsel::from_bits(val as u8)
    }
    #[doc = "PRS Select for PRS Input When Selected in DOWNSEL."]
    #[inline(always)]
    pub const fn set_prsdownsel(&mut self, val: super::vals::Prsdownsel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
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
    #[doc = "HFXO Shunt Current Optimization Start."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoshuntoptstart(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Shunt Current Optimization Start."]
    #[inline(always)]
    pub const fn set_hfxoshuntoptstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("hfxoshuntoptstart", &self.hfxoshuntoptstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ calstart: {=bool:?}, calstop: {=bool:?}, hfxopeakdetstart: {=bool:?}, hfxoshuntoptstart: {=bool:?} }}" , self . calstart () , self . calstop () , self . hfxopeakdetstart () , self . hfxoshuntoptstart ())
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
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Clkoutsel0::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 0."]
    #[inline(always)]
    pub const fn set_clkoutsel0(&mut self, val: super::vals::Clkoutsel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Clock Output Select 1."]
    #[must_use]
    #[inline(always)]
    pub const fn clkoutsel1(&self) -> super::vals::Clkoutsel1 {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::Clkoutsel1::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 1."]
    #[inline(always)]
    pub const fn set_clkoutsel1(&mut self, val: super::vals::Clkoutsel1) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
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
            .field("wshfle", &self.wshfle())
            .field("hfperclken", &self.hfperclken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ clkoutsel0: {:?}, clkoutsel1: {:?}, wshfle: {=bool:?}, hfperclken: {=bool:?} }}" , self . clkoutsel0 () , self . clkoutsel1 () , self . wshfle () , self . hfperclken ())
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
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dbg::from_bits(val as u8)
    }
    #[doc = "Debug Trace Clock."]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: super::vals::Dbg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
    pub const fn crypto(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Advanced Encryption Standard Accelerator Clock Enable."]
    #[inline(always)]
    pub const fn set_crypto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "General purpose Input/Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "General purpose Input/Output Clock Enable."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Peripheral Reflex System Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prs(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral Reflex System Clock Enable."]
    #[inline(always)]
    pub const fn set_prs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Linked Direct Memory Access Controller Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldma(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Linked Direct Memory Access Controller Clock Enable."]
    #[inline(always)]
    pub const fn set_ldma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "General Purpose CRC Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpcrc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose CRC Clock Enable."]
    #[inline(always)]
    pub const fn set_gpcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("crypto", &self.crypto())
            .field("gpio", &self.gpio())
            .field("prs", &self.prs())
            .field("ldma", &self.ldma())
            .field("gpcrc", &self.gpcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfbusclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfbusclken0 {{ le: {=bool:?}, crypto: {=bool:?}, gpio: {=bool:?}, prs: {=bool:?}, ldma: {=bool:?}, gpcrc: {=bool:?} }}" , self . le () , self . crypto () , self . gpio () , self . prs () , self . ldma () , self . gpcrc ())
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
    pub const fn timer(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_timer(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usart(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 2usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_usart(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 2usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Analog Comparator 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 4usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_acmp(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 4usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "CRYOTIMER Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cryotimer(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CRYOTIMER Clock Enable."]
    #[inline(always)]
    pub const fn set_cryotimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I2C 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c(&self, n: usize) -> bool {
        assert!(n < 1usize);
        let offs = 7usize + n * 0usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "I2C 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_i2c(&mut self, n: usize, val: bool) {
        assert!(n < 1usize);
        let offs = 7usize + n * 0usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self, n: usize) -> bool {
        assert!(n < 1usize);
        let offs = 8usize + n * 0usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Analog to Digital Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_adc(&mut self, n: usize, val: bool) {
        assert!(n < 1usize);
        let offs = 8usize + n * 0usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Current Digital to Analog Converter 0 Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn idac(&self, n: usize) -> bool {
        assert!(n < 1usize);
        let offs = 9usize + n * 0usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Current Digital to Analog Converter 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_idac(&mut self, n: usize, val: bool) {
        assert!(n < 1usize);
        let offs = 9usize + n * 0usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("timer[0]", &self.timer(0usize))
            .field("timer[1]", &self.timer(1usize))
            .field("usart[0]", &self.usart(0usize))
            .field("usart[1]", &self.usart(1usize))
            .field("acmp[0]", &self.acmp(0usize))
            .field("acmp[1]", &self.acmp(1usize))
            .field("cryotimer", &self.cryotimer())
            .field("i2c[0]", &self.i2c(0usize))
            .field("adc[0]", &self.adc(0usize))
            .field("idac[0]", &self.idac(0usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfperclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfperclken0 {{ timer[0]: {=bool:?}, timer[1]: {=bool:?}, usart[0]: {=bool:?}, usart[1]: {=bool:?}, acmp[0]: {=bool:?}, acmp[1]: {=bool:?}, cryotimer: {=bool:?}, i2c[0]: {=bool:?}, adc[0]: {=bool:?}, idac[0]: {=bool:?} }}" , self . timer (0usize) , self . timer (1usize) , self . usart (0usize) , self . usart (1usize) , self . acmp (0usize) , self . acmp (1usize) , self . cryotimer () , self . i2c (0usize) , self . adc (0usize) , self . idac (0usize))
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
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Hfclklepresc::from_bits(val as u8)
    }
    #[doc = "HFCLKLE Prescaler."]
    #[inline(always)]
    pub const fn set_hfclklepresc(&mut self, val: super::vals::Hfclklepresc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
#[doc = "HFXO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxoctrl(pub u32);
impl Hfxoctrl {
    #[doc = "HFXO Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdetshuntoptmode(&self) -> super::vals::Peakdetshuntoptmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Peakdetshuntoptmode::from_bits(val as u8)
    }
    #[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode."]
    #[inline(always)]
    pub const fn set_peakdetshuntoptmode(&mut self, val: super::vals::Peakdetshuntoptmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Low Power Mode Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lowpower(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Mode Control."]
    #[inline(always)]
    pub const fn set_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off."]
    #[must_use]
    #[inline(always)]
    pub const fn xti2gnd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off."]
    #[inline(always)]
    pub const fn set_xti2gnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off."]
    #[must_use]
    #[inline(always)]
    pub const fn xto2gnd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off."]
    #[inline(always)]
    pub const fn set_xto2gnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("peakdetshuntoptmode", &self.peakdetshuntoptmode())
            .field("lowpower", &self.lowpower())
            .field("xti2gnd", &self.xti2gnd())
            .field("xto2gnd", &self.xto2gnd())
            .field("lftimeout", &self.lftimeout())
            .field("autostartem0em1", &self.autostartem0em1())
            .field("autostartselem0em1", &self.autostartselem0em1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxoctrl {{ mode: {=bool:?}, peakdetshuntoptmode: {:?}, lowpower: {=bool:?}, xti2gnd: {=bool:?}, xto2gnd: {=bool:?}, lftimeout: {:?}, autostartem0em1: {=bool:?}, autostartselem0em1: {=bool:?} }}" , self . mode () , self . peakdetshuntoptmode () , self . lowpower () , self . xti2gnd () , self . xto2gnd () , self . lftimeout () , self . autostartem0em1 () , self . autostartselem0em1 ())
    }
}
#[doc = "HFXO Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxoctrl1(pub u32);
impl Hfxoctrl1 {
    #[doc = "Sets the Peak Detector amplitude detection threshold levels."]
    #[must_use]
    #[inline(always)]
    pub const fn peakdetthr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Sets the Peak Detector amplitude detection threshold levels."]
    #[inline(always)]
    pub const fn set_peakdetthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn reglvl(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_reglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[must_use]
    #[inline(always)]
    pub const fn xtibiasen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for internal use. Do not change."]
    #[inline(always)]
    pub const fn set_xtibiasen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("reglvl", &self.reglvl())
            .field("xtibiasen", &self.xtibiasen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxoctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfxoctrl1 {{ peakdetthr: {=u8:?}, reglvl: {=u8:?}, xtibiasen: {=bool:?} }}",
            self.peakdetthr(),
            self.reglvl(),
            self.xtibiasen()
        )
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
    pub const fn ibtrimxocore(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Sets the Startup Oscillator Core Bias Current."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
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
    #[doc = "This Field is Reserved. It Should Be Set to 0x9."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[doc = "This Field is Reserved. It Should Be Set to 0x9."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
    #[doc = "Sets the Regulator Output Current Level (shunt Regulator)."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets the Regulator Output Current Level (shunt Regulator)."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxostartupctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxostartupctrl {{ ibtrimxocore: {=u8:?}, ctune: {=u16:?}, reserved0: {=u8:?}, reserved1: {=u8:?} }}" , self . ibtrimxocore () , self . ctune () , self . reserved0 () , self . reserved1 ())
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
    pub const fn ibtrimxocore(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Sets the Steady State Regulator Output Current Level (shunt Regulator)."]
    #[must_use]
    #[inline(always)]
    pub const fn regish(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets the Steady State Regulator Output Current Level (shunt Regulator)."]
    #[inline(always)]
    pub const fn set_regish(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
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
    #[doc = "Controls Regulator Minimum Shunt Current Detection Relative to Nominal."]
    #[must_use]
    #[inline(always)]
    pub const fn regselilow(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Controls Regulator Minimum Shunt Current Detection Relative to Nominal."]
    #[inline(always)]
    pub const fn set_regselilow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
    #[doc = "Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA."]
    #[must_use]
    #[inline(always)]
    pub const fn regishupper(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA."]
    #[inline(always)]
    pub const fn set_regishupper(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
            .field("regish", &self.regish())
            .field("ctune", &self.ctune())
            .field("regselilow", &self.regselilow())
            .field("peakdeten", &self.peakdeten())
            .field("regishupper", &self.regishupper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxosteadystatectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxosteadystatectrl {{ ibtrimxocore: {=u8:?}, regish: {=u8:?}, ctune: {=u16:?}, regselilow: {=u8:?}, peakdeten: {=bool:?}, regishupper: {=u8:?} }}" , self . ibtrimxocore () , self . regish () , self . ctune () , self . regselilow () , self . peakdeten () , self . regishupper ())
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
    #[doc = "Wait Duration in HFXO Warm Startup Steady Wait State."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Wait Duration in HFXO Warm Startup Steady Wait State."]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
    #[doc = "Wait Duration in HFXO Shunt Current Optimization Wait State."]
    #[must_use]
    #[inline(always)]
    pub const fn shuntopttimeout(&self) -> super::vals::Shuntopttimeout {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Shuntopttimeout::from_bits(val as u8)
    }
    #[doc = "Wait Duration in HFXO Shunt Current Optimization Wait State."]
    #[inline(always)]
    pub const fn set_shuntopttimeout(&mut self, val: super::vals::Shuntopttimeout) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
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
            .field("reserved2", &self.reserved2())
            .field("peakdettimeout", &self.peakdettimeout())
            .field("shuntopttimeout", &self.shuntopttimeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxotimeoutctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfxotimeoutctrl {{ startuptimeout: {:?}, steadytimeout: {:?}, reserved2: {=u8:?}, peakdettimeout: {:?}, shuntopttimeout: {:?} }}" , self . startuptimeout () , self . steadytimeout () , self . reserved2 () , self . peakdettimeout () , self . shuntopttimeout ())
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
    pub const fn ibtrimxocore(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm."]
    #[inline(always)]
    pub const fn set_ibtrimxocore(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn regish(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm."]
    #[inline(always)]
    pub const fn set_regish(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
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
            .field("regish", &self.regish())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfxotrimstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfxotrimstatus {{ ibtrimxocore: {=u8:?}, regish: {=u8:?} }}",
            self.ibtrimxocore(),
            self.regish()
        )
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
    #[doc = "HFXOPEAKDETERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdeterr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HFXOPEAKDETERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxopeakdeterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "HFXOSHUNTOPTRDY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoshuntoptrdy(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HFXOSHUNTOPTRDY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hfxoshuntoptrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdeterr", &self.hfxopeakdeterr())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdeterr: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfxoshuntoptrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdeterr () , self . hfxopeakdetrdy () , self . hfxoshuntoptrdy () , self . hfrcodis () , self . lftimeouterr () , self . cmuerr ())
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
    #[doc = "HFXO Automatic Peak Detection Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdeterr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Automatic Peak Detection Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxopeakdeterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "HFXO Automatic Shunt Current Optimization Ready Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoshuntoptrdy(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Automatic Shunt Current Optimization Ready Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxoshuntoptrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdeterr", &self.hfxopeakdeterr())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdeterr: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfxoshuntoptrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdeterr () , self . hfxopeakdetrdy () , self . hfxoshuntoptrdy () , self . hfrcodis () , self . lftimeouterr () , self . cmuerr ())
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
    #[doc = "Set HFXOPEAKDETERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxopeakdeterr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXOPEAKDETERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxopeakdeterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "Set HFXOSHUNTOPTRDY Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoshuntoptrdy(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set HFXOSHUNTOPTRDY Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hfxoshuntoptrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdeterr", &self.hfxopeakdeterr())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ hfrcordy: {=bool:?}, hfxordy: {=bool:?}, lfrcordy: {=bool:?}, lfxordy: {=bool:?}, auxhfrcordy: {=bool:?}, calrdy: {=bool:?}, calof: {=bool:?}, hfxodiserr: {=bool:?}, hfxoautosw: {=bool:?}, hfxopeakdeterr: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfxoshuntoptrdy: {=bool:?}, hfrcodis: {=bool:?}, lftimeouterr: {=bool:?}, cmuerr: {=bool:?} }}" , self . hfrcordy () , self . hfxordy () , self . lfrcordy () , self . lfxordy () , self . auxhfrcordy () , self . calrdy () , self . calof () , self . hfxodiserr () , self . hfxoautosw () , self . hfxopeakdeterr () , self . hfxopeakdetrdy () , self . hfxoshuntoptrdy () , self . hfrcodis () , self . lftimeouterr () , self . cmuerr ())
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
    pub const fn letimer(&self, n: usize) -> bool {
        assert!(n < 1usize);
        let offs = 0usize + n * 0usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Timer 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_letimer(&mut self, n: usize, val: bool) {
        assert!(n < 1usize);
        let offs = 0usize + n * 0usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("letimer[0]", &self.letimer(0usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfaclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lfaclken0 {{ letimer[0]: {=bool:?} }}",
            self.letimer(0usize)
        )
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfapresc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfapresc0 {{ letimer0: {:?} }}", self.letimer0())
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
    pub const fn leuart(&self, n: usize) -> bool {
        assert!(n < 1usize);
        let offs = 0usize + n * 0usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Low Energy UART 0 Clock Enable."]
    #[inline(always)]
    pub const fn set_leuart(&mut self, n: usize, val: bool) {
        assert!(n < 1usize);
        let offs = 0usize + n * 0usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("leuart[0]", &self.leuart(0usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfbclken0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lfbclken0 {{ leuart[0]: {=bool:?} }}",
            self.leuart(0usize)
        )
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfbpresc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lfbpresc0 {{ leuart0: {:?} }}", self.leuart0())
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
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Rtcc::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter and Calendar Prescaler."]
    #[inline(always)]
    pub const fn set_rtcc(&mut self, val: super::vals::Rtcc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
            .field("timeout", &self.timeout())
            .field("gmccurtune", &self.gmccurtune())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lfrcoctrl {{ tuning: {=u16:?}, envref: {=bool:?}, enchop: {=bool:?}, endem: {=bool:?}, timeout: {:?}, gmccurtune: {=u8:?} }}" , self . tuning () , self . envref () , self . enchop () , self . endem () , self . timeout () , self . gmccurtune ())
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscencmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Oscencmd {{ hfrcoen: {=bool:?}, hfrcodis: {=bool:?}, hfxoen: {=bool:?}, hfxodis: {=bool:?}, auxhfrcoen: {=bool:?}, auxhfrcodis: {=bool:?}, lfrcoen: {=bool:?}, lfrcodis: {=bool:?}, lfxoen: {=bool:?}, lfxodis: {=bool:?} }}" , self . hfrcoen () , self . hfrcodis () , self . hfxoen () , self . hfxodis () , self . auxhfrcoen () , self . auxhfrcodis () , self . lfrcoen () , self . lfrcodis () , self . lfxoen () , self . lfxodis ())
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcntctrl {{ pcnt0clken: {=bool:?}, pcnt0clksel: {=bool:?} }}",
            self.pcnt0clken(),
            self.pcnt0clksel()
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ clkout0loc: {:?}, clkout1loc: {:?} }}",
            self.clkout0loc(),
            self.clkout1loc()
        )
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routepen {{ clkout0pen: {=bool:?}, clkout1pen: {=bool:?} }}",
            self.clkout0pen(),
            self.clkout1pen()
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
    #[doc = "HFXO is Required By Hardware."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoreq(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO is Required By Hardware."]
    #[inline(always)]
    pub const fn set_hfxoreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
    #[doc = "HFXO Shunt Current Optimization Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoshuntoptrdy(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Shunt Current Optimization Ready."]
    #[inline(always)]
    pub const fn set_hfxoshuntoptrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "HFXO Oscillation Amplitude is Too High."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoamphigh(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Oscillation Amplitude is Too High."]
    #[inline(always)]
    pub const fn set_hfxoamphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
    #[doc = "HFXO Regulator Shunt Current Too Low."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxoregilow(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "HFXO Regulator Shunt Current Too Low."]
    #[inline(always)]
    pub const fn set_hfxoregilow(&mut self, val: bool) {
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
            .field("calrdy", &self.calrdy())
            .field("hfxoreq", &self.hfxoreq())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfxoamphigh", &self.hfxoamphigh())
            .field("hfxoamplow", &self.hfxoamplow())
            .field("hfxoregilow", &self.hfxoregilow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ hfrcoens: {=bool:?}, hfrcordy: {=bool:?}, hfxoens: {=bool:?}, hfxordy: {=bool:?}, auxhfrcoens: {=bool:?}, auxhfrcordy: {=bool:?}, lfrcoens: {=bool:?}, lfrcordy: {=bool:?}, lfxoens: {=bool:?}, lfxordy: {=bool:?}, calrdy: {=bool:?}, hfxoreq: {=bool:?}, hfxopeakdetrdy: {=bool:?}, hfxoshuntoptrdy: {=bool:?}, hfxoamphigh: {=bool:?}, hfxoamplow: {=bool:?}, hfxoregilow: {=bool:?} }}" , self . hfrcoens () , self . hfrcordy () , self . hfxoens () , self . hfxordy () , self . auxhfrcoens () , self . auxhfrcordy () , self . lfrcoens () , self . lfrcordy () , self . lfxoens () , self . lfxordy () , self . calrdy () , self . hfxoreq () , self . hfxopeakdetrdy () , self . hfxoshuntoptrdy () , self . hfxoamphigh () , self . hfxoamplow () , self . hfxoregilow ())
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
            .field("lfeclken0", &self.lfeclken0())
            .field("lfepresc0", &self.lfepresc0())
            .field("hfrcobsy", &self.hfrcobsy())
            .field("auxhfrcobsy", &self.auxhfrcobsy())
            .field("lfrcobsy", &self.lfrcobsy())
            .field("lfrcovrefbsy", &self.lfrcovrefbsy())
            .field("hfxobsy", &self.hfxobsy())
            .field("lfxobsy", &self.lfxobsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syncbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Syncbusy {{ lfaclken0: {=bool:?}, lfapresc0: {=bool:?}, lfbclken0: {=bool:?}, lfbpresc0: {=bool:?}, lfeclken0: {=bool:?}, lfepresc0: {=bool:?}, hfrcobsy: {=bool:?}, auxhfrcobsy: {=bool:?}, lfrcobsy: {=bool:?}, lfrcovrefbsy: {=bool:?}, hfxobsy: {=bool:?}, lfxobsy: {=bool:?} }}" , self . lfaclken0 () , self . lfapresc0 () , self . lfbclken0 () , self . lfbpresc0 () , self . lfeclken0 () , self . lfepresc0 () , self . hfrcobsy () , self . auxhfrcobsy () , self . lfrcobsy () , self . lfrcovrefbsy () , self . hfxobsy () , self . lfxobsy ())
    }
}
#[doc = "ULFRCO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulfrcoctrl(pub u32);
impl Ulfrcoctrl {
    #[doc = "ULFRCO TUNING Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ULFRCO TUNING Value."]
    #[inline(always)]
    pub const fn set_tuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "ULFRCO Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::UlfrcoctrlMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::UlfrcoctrlMode::from_bits(val as u8)
    }
    #[doc = "ULFRCO Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::UlfrcoctrlMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)."]
    #[must_use]
    #[inline(always)]
    pub const fn restrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)."]
    #[inline(always)]
    pub const fn set_restrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Ulfrcoctrl {
    #[inline(always)]
    fn default() -> Ulfrcoctrl {
        Ulfrcoctrl(0)
    }
}
impl core::fmt::Debug for Ulfrcoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ulfrcoctrl")
            .field("tuning", &self.tuning())
            .field("mode", &self.mode())
            .field("restrim", &self.restrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ulfrcoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ulfrcoctrl {{ tuning: {=u8:?}, mode: {:?}, restrim: {=u8:?} }}",
            self.tuning(),
            self.mode(),
            self.restrim()
        )
    }
}
