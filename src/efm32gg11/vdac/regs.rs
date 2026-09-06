#[doc = "Calibration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cal(pub u32);
impl Cal {
    #[doc = "Input Buffer Offset Calibration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn offsettrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Input Buffer Offset Calibration Value."]
    #[inline(always)]
    pub const fn set_offsettrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Gain Error Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gainerrtrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Gain Error Trim Value."]
    #[inline(always)]
    pub const fn set_gainerrtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Gain Error Trim Value for CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn gainerrtrimch1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Gain Error Trim Value for CH1."]
    #[inline(always)]
    pub const fn set_gainerrtrimch1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Cal {
    #[inline(always)]
    fn default() -> Cal {
        Cal(0)
    }
}
impl core::fmt::Debug for Cal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cal")
            .field("offsettrim", &self.offsettrim())
            .field("gainerrtrim", &self.gainerrtrim())
            .field("gainerrtrimch1", &self.gainerrtrimch1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cal {{ offsettrim: {=u8:?}, gainerrtrim: {=u8:?}, gainerrtrimch1: {=u8:?} }}",
            self.offsettrim(),
            self.gainerrtrim(),
            self.gainerrtrimch1()
        )
    }
}
#[doc = "Channel 0 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0ctrl(pub u32);
impl Ch0ctrl {
    #[doc = "Conversion Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn convmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Mode."]
    #[inline(always)]
    pub const fn set_convmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 0 Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn trigmode(&self) -> super::vals::Ch0ctrlTrigmode {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Ch0ctrlTrigmode::from_bits(val as u8)
    }
    #[doc = "Channel 0 Trigger Mode."]
    #[inline(always)]
    pub const fn set_trigmode(&mut self, val: super::vals::Ch0ctrlTrigmode) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Channel 0 PRS Asynchronous Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsasync(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 PRS Asynchronous Enable."]
    #[inline(always)]
    pub const fn set_prsasync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 0 PRS Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 12usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Channel 0 PRS Trigger Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val.to_bits() as u32) & 0x1f) << 12usize);
    }
}
impl Default for Ch0ctrl {
    #[inline(always)]
    fn default() -> Ch0ctrl {
        Ch0ctrl(0)
    }
}
impl core::fmt::Debug for Ch0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0ctrl")
            .field("convmode", &self.convmode())
            .field("trigmode", &self.trigmode())
            .field("prsasync", &self.prsasync())
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0ctrl {{ convmode: {=bool:?}, trigmode: {:?}, prsasync: {=bool:?}, prssel: {:?} }}",
            self.convmode(),
            self.trigmode(),
            self.prsasync(),
            self.prssel()
        )
    }
}
#[doc = "Channel 0 Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0data(pub u32);
impl Ch0data {
    #[doc = "Channel 0 Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel 0 Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Ch0data {
    #[inline(always)]
    fn default() -> Ch0data {
        Ch0data(0)
    }
}
impl core::fmt::Debug for Ch0data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0data")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch0data {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Channel 1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1ctrl(pub u32);
impl Ch1ctrl {
    #[doc = "Conversion Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn convmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Mode."]
    #[inline(always)]
    pub const fn set_convmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn trigmode(&self) -> super::vals::Ch1ctrlTrigmode {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Ch1ctrlTrigmode::from_bits(val as u8)
    }
    #[doc = "Channel 1 Trigger Mode."]
    #[inline(always)]
    pub const fn set_trigmode(&mut self, val: super::vals::Ch1ctrlTrigmode) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Channel 1 PRS Asynchronous Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsasync(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 PRS Asynchronous Enable."]
    #[inline(always)]
    pub const fn set_prsasync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 1 PRS Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 12usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "Channel 1 PRS Trigger Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val.to_bits() as u32) & 0x1f) << 12usize);
    }
}
impl Default for Ch1ctrl {
    #[inline(always)]
    fn default() -> Ch1ctrl {
        Ch1ctrl(0)
    }
}
impl core::fmt::Debug for Ch1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1ctrl")
            .field("convmode", &self.convmode())
            .field("trigmode", &self.trigmode())
            .field("prsasync", &self.prsasync())
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch1ctrl {{ convmode: {=bool:?}, trigmode: {:?}, prsasync: {=bool:?}, prssel: {:?} }}",
            self.convmode(),
            self.trigmode(),
            self.prsasync(),
            self.prssel()
        )
    }
}
#[doc = "Channel 1 Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1data(pub u32);
impl Ch1data {
    #[doc = "Channel 1 Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel 1 Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Ch1data {
    #[inline(always)]
    fn default() -> Ch1data {
        Ch1data(0)
    }
}
impl core::fmt::Debug for Ch1data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1data")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch1data {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "DAC Channel 0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 0 Enable."]
    #[inline(always)]
    pub const fn set_ch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC Channel 0 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 0 Disable."]
    #[inline(always)]
    pub const fn set_ch0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAC Channel 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 1 Enable."]
    #[inline(always)]
    pub const fn set_ch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DAC Channel 1 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1dis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 1 Disable."]
    #[inline(always)]
    pub const fn set_ch1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OPA0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Enable."]
    #[inline(always)]
    pub const fn set_opa0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OPA0 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0dis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Disable."]
    #[inline(always)]
    pub const fn set_opa0dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OPA1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Enable."]
    #[inline(always)]
    pub const fn set_opa1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OPA1 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1dis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Disable."]
    #[inline(always)]
    pub const fn set_opa1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OPA2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Enable."]
    #[inline(always)]
    pub const fn set_opa2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPA2 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2dis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Disable."]
    #[inline(always)]
    pub const fn set_opa2dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPA3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Enable."]
    #[inline(always)]
    pub const fn set_opa3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "OPA3 Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3dis(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Disable."]
    #[inline(always)]
    pub const fn set_opa3dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("ch0en", &self.ch0en())
            .field("ch0dis", &self.ch0dis())
            .field("ch1en", &self.ch1en())
            .field("ch1dis", &self.ch1dis())
            .field("opa0en", &self.opa0en())
            .field("opa0dis", &self.opa0dis())
            .field("opa1en", &self.opa1en())
            .field("opa1dis", &self.opa1dis())
            .field("opa2en", &self.opa2en())
            .field("opa2dis", &self.opa2dis())
            .field("opa3en", &self.opa3en())
            .field("opa3dis", &self.opa3dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ ch0en: {=bool:?}, ch0dis: {=bool:?}, ch1en: {=bool:?}, ch1dis: {=bool:?}, opa0en: {=bool:?}, opa0dis: {=bool:?}, opa1en: {=bool:?}, opa1dis: {=bool:?}, opa2en: {=bool:?}, opa2dis: {=bool:?}, opa3en: {=bool:?}, opa3dis: {=bool:?} }}" , self . ch0en () , self . ch0dis () , self . ch1en () , self . ch1dis () , self . opa0en () , self . opa0dis () , self . opa1en () , self . opa1dis () , self . opa2en () , self . opa2dis () , self . opa3en () , self . opa3dis ())
    }
}
#[doc = "Combined Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Combdata(pub u32);
impl Combdata {
    #[doc = "Channel 0 Data."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel 0 Data."]
    #[inline(always)]
    pub const fn set_ch0data(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Channel 1 Data."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1data(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel 1 Data."]
    #[inline(always)]
    pub const fn set_ch1data(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Combdata {
    #[inline(always)]
    fn default() -> Combdata {
        Combdata(0)
    }
}
impl core::fmt::Debug for Combdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Combdata")
            .field("ch0data", &self.ch0data())
            .field("ch1data", &self.ch1data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Combdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Combdata {{ ch0data: {=u16:?}, ch1data: {=u16:?} }}",
            self.ch0data(),
            self.ch1data()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Differential Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn diff(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Differential Mode."]
    #[inline(always)]
    pub const fn set_diff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sine Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sinemode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Sine Mode."]
    #[inline(always)]
    pub const fn set_sinemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PRS Controlled Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn outenprs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PRS Controlled Output Enable."]
    #[inline(always)]
    pub const fn set_outenprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 0 Start Reset Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0prescrst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Start Reset Prescaler."]
    #[inline(always)]
    pub const fn set_ch0prescrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Reference Selection."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Prescaler Setting for DAC Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn presc(&self) -> super::vals::Presc {
        let val = (self.0 >> 16usize) & 0x7f;
        super::vals::Presc::from_bits(val as u8)
    }
    #[doc = "Prescaler Setting for DAC Clock."]
    #[inline(always)]
    pub const fn set_presc(&mut self, val: super::vals::Presc) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
    }
    #[doc = "Refresh Period."]
    #[must_use]
    #[inline(always)]
    pub const fn refreshperiod(&self) -> super::vals::Refreshperiod {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Refreshperiod::from_bits(val as u8)
    }
    #[doc = "Refresh Period."]
    #[inline(always)]
    pub const fn set_refreshperiod(&mut self, val: super::vals::Refreshperiod) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Warm-up Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn warmupmode(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Warm-up Mode."]
    #[inline(always)]
    pub const fn set_warmupmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Clock Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dacclkmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Mode."]
    #[inline(always)]
    pub const fn set_dacclkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("diff", &self.diff())
            .field("sinemode", &self.sinemode())
            .field("outenprs", &self.outenprs())
            .field("ch0prescrst", &self.ch0prescrst())
            .field("refsel", &self.refsel())
            .field("presc", &self.presc())
            .field("refreshperiod", &self.refreshperiod())
            .field("warmupmode", &self.warmupmode())
            .field("dacclkmode", &self.dacclkmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ diff: {=bool:?}, sinemode: {=bool:?}, outenprs: {=bool:?}, ch0prescrst: {=bool:?}, refsel: {:?}, presc: {:?}, refreshperiod: {:?}, warmupmode: {=bool:?}, dacclkmode: {=bool:?} }}" , self . diff () , self . sinemode () , self . outenprs () , self . ch0prescrst () , self . refsel () , self . presc () , self . refreshperiod () , self . warmupmode () , self . dacclkmode ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "CH0CD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0cd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0CD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch0cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1CD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1cd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1CD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch1cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH0OF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH0OF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch0of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH1OF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH1OF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH0UF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH0UF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch0uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH1UF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1uf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH1UF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch1uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH0BL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0bl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH0BL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch0bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH1BL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1bl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH1BL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ch1bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EM23ERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn em23err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EM23ERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_em23err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "OPA0APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0aportconflict(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa0aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OPA1APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1aportconflict(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa1aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OPA2APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2aportconflict(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa2aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OPA3APORTCONFLICT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3aportconflict(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3APORTCONFLICT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa3aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OPA0PRSTIMEDERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0prstimederr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0PRSTIMEDERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa0prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPA1PRSTIMEDERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1prstimederr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1PRSTIMEDERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa1prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPA2PRSTIMEDERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2prstimederr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2PRSTIMEDERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa2prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "OPA3PRSTIMEDERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3prstimederr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3PRSTIMEDERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa3prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "OPA0OUTVALID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0outvalid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0OUTVALID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa0outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "OPA1OUTVALID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1outvalid(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1OUTVALID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa1outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "OPA2OUTVALID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2outvalid(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2OUTVALID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa2outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OPA3OUTVALID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3outvalid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3OUTVALID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_opa3outvalid(&mut self, val: bool) {
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
            .field("ch0cd", &self.ch0cd())
            .field("ch1cd", &self.ch1cd())
            .field("ch0of", &self.ch0of())
            .field("ch1of", &self.ch1of())
            .field("ch0uf", &self.ch0uf())
            .field("ch1uf", &self.ch1uf())
            .field("ch0bl", &self.ch0bl())
            .field("ch1bl", &self.ch1bl())
            .field("em23err", &self.em23err())
            .field("opa0aportconflict", &self.opa0aportconflict())
            .field("opa1aportconflict", &self.opa1aportconflict())
            .field("opa2aportconflict", &self.opa2aportconflict())
            .field("opa3aportconflict", &self.opa3aportconflict())
            .field("opa0prstimederr", &self.opa0prstimederr())
            .field("opa1prstimederr", &self.opa1prstimederr())
            .field("opa2prstimederr", &self.opa2prstimederr())
            .field("opa3prstimederr", &self.opa3prstimederr())
            .field("opa0outvalid", &self.opa0outvalid())
            .field("opa1outvalid", &self.opa1outvalid())
            .field("opa2outvalid", &self.opa2outvalid())
            .field("opa3outvalid", &self.opa3outvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ ch0cd: {=bool:?}, ch1cd: {=bool:?}, ch0of: {=bool:?}, ch1of: {=bool:?}, ch0uf: {=bool:?}, ch1uf: {=bool:?}, ch0bl: {=bool:?}, ch1bl: {=bool:?}, em23err: {=bool:?}, opa0aportconflict: {=bool:?}, opa1aportconflict: {=bool:?}, opa2aportconflict: {=bool:?}, opa3aportconflict: {=bool:?}, opa0prstimederr: {=bool:?}, opa1prstimederr: {=bool:?}, opa2prstimederr: {=bool:?}, opa3prstimederr: {=bool:?}, opa0outvalid: {=bool:?}, opa1outvalid: {=bool:?}, opa2outvalid: {=bool:?}, opa3outvalid: {=bool:?} }}" , self . ch0cd () , self . ch1cd () , self . ch0of () , self . ch1of () , self . ch0uf () , self . ch1uf () , self . ch0bl () , self . ch1bl () , self . em23err () , self . opa0aportconflict () , self . opa1aportconflict () , self . opa2aportconflict () , self . opa3aportconflict () , self . opa0prstimederr () , self . opa1prstimederr () , self . opa2prstimederr () , self . opa3prstimederr () , self . opa0outvalid () , self . opa1outvalid () , self . opa2outvalid () , self . opa3outvalid ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Channel 0 Conversion Done Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0cd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Conversion Done Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Conversion Done Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1cd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Conversion Done Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 0 Data Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Data Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 1 Data Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Data Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 0 Data Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Data Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 1 Data Underflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1uf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Data Underflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 0 Buffer Level Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0bl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Buffer Level Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 1 Buffer Level Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1bl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Buffer Level Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EM2/3 Entry Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn em23err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EM2/3 Entry Error Flag."]
    #[inline(always)]
    pub const fn set_em23err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "OPA0 Bus Conflict Output Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0aportconflict(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Bus Conflict Output Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OPA1 Bus Conflict Output Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1aportconflict(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Bus Conflict Output Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OPA2 Bus Conflict Output Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2aportconflict(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Bus Conflict Output Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OPA3 Bus Conflict Output Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3aportconflict(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Bus Conflict Output Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OPA0 PRS Trigger Mode Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0prstimederr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 PRS Trigger Mode Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPA1 PRS Trigger Mode Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1prstimederr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 PRS Trigger Mode Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPA2 PRS Trigger Mode Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2prstimederr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 PRS Trigger Mode Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "OPA3 PRS Trigger Mode Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3prstimederr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 PRS Trigger Mode Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "OPA0 Output Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0outvalid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Output Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "OPA1 Output Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1outvalid(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Output Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "OPA3 Output Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2outvalid(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Output Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OPA3 Output Valid Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3outvalid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Output Valid Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3outvalid(&mut self, val: bool) {
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
            .field("ch0cd", &self.ch0cd())
            .field("ch1cd", &self.ch1cd())
            .field("ch0of", &self.ch0of())
            .field("ch1of", &self.ch1of())
            .field("ch0uf", &self.ch0uf())
            .field("ch1uf", &self.ch1uf())
            .field("ch0bl", &self.ch0bl())
            .field("ch1bl", &self.ch1bl())
            .field("em23err", &self.em23err())
            .field("opa0aportconflict", &self.opa0aportconflict())
            .field("opa1aportconflict", &self.opa1aportconflict())
            .field("opa2aportconflict", &self.opa2aportconflict())
            .field("opa3aportconflict", &self.opa3aportconflict())
            .field("opa0prstimederr", &self.opa0prstimederr())
            .field("opa1prstimederr", &self.opa1prstimederr())
            .field("opa2prstimederr", &self.opa2prstimederr())
            .field("opa3prstimederr", &self.opa3prstimederr())
            .field("opa0outvalid", &self.opa0outvalid())
            .field("opa1outvalid", &self.opa1outvalid())
            .field("opa2outvalid", &self.opa2outvalid())
            .field("opa3outvalid", &self.opa3outvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ ch0cd: {=bool:?}, ch1cd: {=bool:?}, ch0of: {=bool:?}, ch1of: {=bool:?}, ch0uf: {=bool:?}, ch1uf: {=bool:?}, ch0bl: {=bool:?}, ch1bl: {=bool:?}, em23err: {=bool:?}, opa0aportconflict: {=bool:?}, opa1aportconflict: {=bool:?}, opa2aportconflict: {=bool:?}, opa3aportconflict: {=bool:?}, opa0prstimederr: {=bool:?}, opa1prstimederr: {=bool:?}, opa2prstimederr: {=bool:?}, opa3prstimederr: {=bool:?}, opa0outvalid: {=bool:?}, opa1outvalid: {=bool:?}, opa2outvalid: {=bool:?}, opa3outvalid: {=bool:?} }}" , self . ch0cd () , self . ch1cd () , self . ch0of () , self . ch1of () , self . ch0uf () , self . ch1uf () , self . ch0bl () , self . ch1bl () , self . em23err () , self . opa0aportconflict () , self . opa1aportconflict () , self . opa2aportconflict () , self . opa3aportconflict () , self . opa0prstimederr () , self . opa1prstimederr () , self . opa2prstimederr () , self . opa3prstimederr () , self . opa0outvalid () , self . opa1outvalid () , self . opa2outvalid () , self . opa3outvalid ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set CH0CD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0cd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH0CD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set CH1CD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1cd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH1CD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set CH0OF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH0OF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set CH1OF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH1OF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set CH0UF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH0UF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch0uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set CH1UF Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1uf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set CH1UF Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ch1uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set EM23ERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn em23err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set EM23ERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_em23err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set OPA0APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0aportconflict(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA0APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set OPA1APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1aportconflict(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA1APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set OPA2APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2aportconflict(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA2APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set OPA3APORTCONFLICT Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3aportconflict(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA3APORTCONFLICT Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set OPA0PRSTIMEDERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0prstimederr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA0PRSTIMEDERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set OPA1PRSTIMEDERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1prstimederr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA1PRSTIMEDERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Set OPA2PRSTIMEDERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2prstimederr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA2PRSTIMEDERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set OPA3PRSTIMEDERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3prstimederr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA3PRSTIMEDERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3prstimederr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set OPA0OUTVALID Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0outvalid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA0OUTVALID Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa0outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set OPA1OUTVALID Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1outvalid(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA1OUTVALID Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa1outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Set OPA2OUTVALID Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2outvalid(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA2OUTVALID Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa2outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set OPA3OUTVALID Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3outvalid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set OPA3OUTVALID Interrupt Flag."]
    #[inline(always)]
    pub const fn set_opa3outvalid(&mut self, val: bool) {
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
            .field("ch0cd", &self.ch0cd())
            .field("ch1cd", &self.ch1cd())
            .field("ch0of", &self.ch0of())
            .field("ch1of", &self.ch1of())
            .field("ch0uf", &self.ch0uf())
            .field("ch1uf", &self.ch1uf())
            .field("em23err", &self.em23err())
            .field("opa0aportconflict", &self.opa0aportconflict())
            .field("opa1aportconflict", &self.opa1aportconflict())
            .field("opa2aportconflict", &self.opa2aportconflict())
            .field("opa3aportconflict", &self.opa3aportconflict())
            .field("opa0prstimederr", &self.opa0prstimederr())
            .field("opa1prstimederr", &self.opa1prstimederr())
            .field("opa2prstimederr", &self.opa2prstimederr())
            .field("opa3prstimederr", &self.opa3prstimederr())
            .field("opa0outvalid", &self.opa0outvalid())
            .field("opa1outvalid", &self.opa1outvalid())
            .field("opa2outvalid", &self.opa2outvalid())
            .field("opa3outvalid", &self.opa3outvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ ch0cd: {=bool:?}, ch1cd: {=bool:?}, ch0of: {=bool:?}, ch1of: {=bool:?}, ch0uf: {=bool:?}, ch1uf: {=bool:?}, em23err: {=bool:?}, opa0aportconflict: {=bool:?}, opa1aportconflict: {=bool:?}, opa2aportconflict: {=bool:?}, opa3aportconflict: {=bool:?}, opa0prstimederr: {=bool:?}, opa1prstimederr: {=bool:?}, opa2prstimederr: {=bool:?}, opa3prstimederr: {=bool:?}, opa0outvalid: {=bool:?}, opa1outvalid: {=bool:?}, opa2outvalid: {=bool:?}, opa3outvalid: {=bool:?} }}" , self . ch0cd () , self . ch1cd () , self . ch0of () , self . ch1of () , self . ch0uf () , self . ch1uf () , self . em23err () , self . opa0aportconflict () , self . opa1aportconflict () , self . opa2aportconflict () , self . opa3aportconflict () , self . opa0prstimederr () , self . opa1prstimederr () , self . opa2prstimederr () , self . opa3prstimederr () , self . opa0outvalid () , self . opa1outvalid () , self . opa2outvalid () , self . opa3outvalid ())
    }
}
#[doc = "Operational Amplifier APORT Conflict Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Aportconflict(pub u32);
impl Opa0Aportconflict {
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xconflict(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport1xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yconflict(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport1yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2xconflict(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport2xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2yconflict(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport2yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3xconflict(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport3xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3yconflict(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport3yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4xconflict(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport4xconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4yconflict(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral."]
    #[inline(always)]
    pub const fn set_aport4yconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Opa0Aportconflict {
    #[inline(always)]
    fn default() -> Opa0Aportconflict {
        Opa0Aportconflict(0)
    }
}
impl core::fmt::Debug for Opa0Aportconflict {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Aportconflict")
            .field("aport1xconflict", &self.aport1xconflict())
            .field("aport1yconflict", &self.aport1yconflict())
            .field("aport2xconflict", &self.aport2xconflict())
            .field("aport2yconflict", &self.aport2yconflict())
            .field("aport3xconflict", &self.aport3xconflict())
            .field("aport3yconflict", &self.aport3yconflict())
            .field("aport4xconflict", &self.aport4xconflict())
            .field("aport4yconflict", &self.aport4yconflict())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Aportconflict {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Aportconflict {{ aport1xconflict: {=bool:?}, aport1yconflict: {=bool:?}, aport2xconflict: {=bool:?}, aport2yconflict: {=bool:?}, aport3xconflict: {=bool:?}, aport3yconflict: {=bool:?}, aport4xconflict: {=bool:?}, aport4yconflict: {=bool:?} }}" , self . aport1xconflict () , self . aport1yconflict () , self . aport2xconflict () , self . aport2yconflict () , self . aport3xconflict () , self . aport3yconflict () , self . aport4xconflict () , self . aport4yconflict ())
    }
}
#[doc = "Operational Amplifier APORT Request Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Aportreq(pub u32);
impl Opa0Aportreq {
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1xreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[inline(always)]
    pub const fn set_aport1xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport1yreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT1X is Requested."]
    #[inline(always)]
    pub const fn set_aport1yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2xreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2X is Requested."]
    #[inline(always)]
    pub const fn set_aport2xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1 If the Bus Connected to APORT2Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport2yreq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT2Y is Requested."]
    #[inline(always)]
    pub const fn set_aport2yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1 If the Bus Connected to APORT3X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3xreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3X is Requested."]
    #[inline(always)]
    pub const fn set_aport3xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 If the Bus Connected to APORT3Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport3yreq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT3Y is Requested."]
    #[inline(always)]
    pub const fn set_aport3yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1 If the Bus Connected to APORT4X is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4xreq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4X is Requested."]
    #[inline(always)]
    pub const fn set_aport4xreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1 If the Bus Connected to APORT4Y is Requested."]
    #[must_use]
    #[inline(always)]
    pub const fn aport4yreq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1 If the Bus Connected to APORT4Y is Requested."]
    #[inline(always)]
    pub const fn set_aport4yreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Opa0Aportreq {
    #[inline(always)]
    fn default() -> Opa0Aportreq {
        Opa0Aportreq(0)
    }
}
impl core::fmt::Debug for Opa0Aportreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Aportreq")
            .field("aport1xreq", &self.aport1xreq())
            .field("aport1yreq", &self.aport1yreq())
            .field("aport2xreq", &self.aport2xreq())
            .field("aport2yreq", &self.aport2yreq())
            .field("aport3xreq", &self.aport3xreq())
            .field("aport3yreq", &self.aport3yreq())
            .field("aport4xreq", &self.aport4xreq())
            .field("aport4yreq", &self.aport4yreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Aportreq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Aportreq {{ aport1xreq: {=bool:?}, aport1yreq: {=bool:?}, aport2xreq: {=bool:?}, aport2yreq: {=bool:?}, aport3xreq: {=bool:?}, aport3yreq: {=bool:?}, aport4xreq: {=bool:?}, aport4yreq: {=bool:?} }}" , self . aport1xreq () , self . aport1yreq () , self . aport2xreq () , self . aport2yreq () , self . aport3xreq () , self . aport3yreq () , self . aport4xreq () , self . aport4yreq ())
    }
}
#[doc = "Operational Amplifier Calibration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Cal(pub u32);
impl Opa0Cal {
    #[doc = "Compensation Cap Cm1 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cm1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Compensation Cap Cm1 Trim Value."]
    #[inline(always)]
    pub const fn set_cm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Compensation Cap Cm2 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cm2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "Compensation Cap Cm2 Trim Value."]
    #[inline(always)]
    pub const fn set_cm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "Compensation Cap Cm3 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cm3(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Compensation Cap Cm3 Trim Value."]
    #[inline(always)]
    pub const fn set_cm3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Gm Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gm(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Gm Trim Value."]
    #[inline(always)]
    pub const fn set_gm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Gm3 Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gm3(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "Gm3 Trim Value."]
    #[inline(always)]
    pub const fn set_gm3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "OPAx Non-Inverting Input Offset Configuration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "OPAx Non-Inverting Input Offset Configuration Value."]
    #[inline(always)]
    pub const fn set_offsetp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "OPAx Inverting Input Offset Configuration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetn(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "OPAx Inverting Input Offset Configuration Value."]
    #[inline(always)]
    pub const fn set_offsetn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
}
impl Default for Opa0Cal {
    #[inline(always)]
    fn default() -> Opa0Cal {
        Opa0Cal(0)
    }
}
impl core::fmt::Debug for Opa0Cal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Cal")
            .field("cm1", &self.cm1())
            .field("cm2", &self.cm2())
            .field("cm3", &self.cm3())
            .field("gm", &self.gm())
            .field("gm3", &self.gm3())
            .field("offsetp", &self.offsetp())
            .field("offsetn", &self.offsetn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Cal {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Cal {{ cm1: {=u8:?}, cm2: {=u8:?}, cm3: {=u8:?}, gm: {=u8:?}, gm3: {=u8:?}, offsetp: {=u8:?}, offsetn: {=u8:?} }}" , self . cm1 () , self . cm2 () , self . cm3 () , self . gm () , self . gm3 () , self . offsetp () , self . offsetn ())
    }
}
#[doc = "Operational Amplifier Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Ctrl(pub u32);
impl Opa0Ctrl {
    #[doc = "OPAx Operation Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn drivestrength(&self) -> super::vals::Opa0CtrlDrivestrength {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Opa0CtrlDrivestrength::from_bits(val as u8)
    }
    #[doc = "OPAx Operation Mode."]
    #[inline(always)]
    pub const fn set_drivestrength(&mut self, val: super::vals::Opa0CtrlDrivestrength) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "OPAx Unity Gain Bandwidth Scale."]
    #[must_use]
    #[inline(always)]
    pub const fn incbw(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Unity Gain Bandwidth Scale."]
    #[inline(always)]
    pub const fn set_incbw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "High Common Mode Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn hcmdis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "High Common Mode Disable."]
    #[inline(always)]
    pub const fn set_hcmdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Scale OPAx Output Driving Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn outscale(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Scale OPAx Output Driving Strength."]
    #[inline(always)]
    pub const fn set_outscale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OPAx PRS Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prsen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx PRS Trigger Enable."]
    #[inline(always)]
    pub const fn set_prsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAx PRS Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prsmode(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx PRS Trigger Mode."]
    #[inline(always)]
    pub const fn set_prsmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAx PRS Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::super::prs::vals::Prssel {
        let val = (self.0 >> 10usize) & 0x1f;
        super::super::prs::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "OPAx PRS Trigger Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::super::prs::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "OPAx PRS Output Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prsoutmode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx PRS Output Select."]
    #[inline(always)]
    pub const fn set_prsoutmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "APORT Bus Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportxmasterdis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus Master Disable."]
    #[inline(always)]
    pub const fn set_aportxmasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "APORT Bus Master Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportymasterdis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "APORT Bus Master Disable."]
    #[inline(always)]
    pub const fn set_aportymasterdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Opa0Ctrl {
    #[inline(always)]
    fn default() -> Opa0Ctrl {
        Opa0Ctrl(0)
    }
}
impl core::fmt::Debug for Opa0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Ctrl")
            .field("drivestrength", &self.drivestrength())
            .field("incbw", &self.incbw())
            .field("hcmdis", &self.hcmdis())
            .field("outscale", &self.outscale())
            .field("prsen", &self.prsen())
            .field("prsmode", &self.prsmode())
            .field("prssel", &self.prssel())
            .field("prsoutmode", &self.prsoutmode())
            .field("aportxmasterdis", &self.aportxmasterdis())
            .field("aportymasterdis", &self.aportymasterdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Ctrl {{ drivestrength: {:?}, incbw: {=bool:?}, hcmdis: {=bool:?}, outscale: {=bool:?}, prsen: {=bool:?}, prsmode: {=bool:?}, prssel: {:?}, prsoutmode: {=bool:?}, aportxmasterdis: {=bool:?}, aportymasterdis: {=bool:?} }}" , self . drivestrength () , self . incbw () , self . hcmdis () , self . outscale () , self . prsen () , self . prsmode () , self . prssel () , self . prsoutmode () , self . aportxmasterdis () , self . aportymasterdis ())
    }
}
#[doc = "Operational Amplifier Mux Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Mux(pub u32);
impl Opa0Mux {
    #[doc = "OPAx Non-inverting Input Mux."]
    #[must_use]
    #[inline(always)]
    pub const fn possel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPAx Non-inverting Input Mux."]
    #[inline(always)]
    pub const fn set_possel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "OPAx Inverting Input Mux."]
    #[must_use]
    #[inline(always)]
    pub const fn negsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "OPAx Inverting Input Mux."]
    #[inline(always)]
    pub const fn set_negsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "OPAx Resistor Ladder Input Mux."]
    #[must_use]
    #[inline(always)]
    pub const fn resinmux(&self) -> super::vals::Opa0MuxResinmux {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Opa0MuxResinmux::from_bits(val as u8)
    }
    #[doc = "OPAx Resistor Ladder Input Mux."]
    #[inline(always)]
    pub const fn set_resinmux(&mut self, val: super::vals::Opa0MuxResinmux) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "OPAx Dedicated 3x Gain Resistor Ladder."]
    #[must_use]
    #[inline(always)]
    pub const fn gain3x(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Dedicated 3x Gain Resistor Ladder."]
    #[inline(always)]
    pub const fn set_gain3x(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPAx Resistor Ladder Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ressel(&self) -> super::vals::Opa0MuxRessel {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Opa0MuxRessel::from_bits(val as u8)
    }
    #[doc = "OPAx Resistor Ladder Select."]
    #[inline(always)]
    pub const fn set_ressel(&mut self, val: super::vals::Opa0MuxRessel) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Opa0Mux {
    #[inline(always)]
    fn default() -> Opa0Mux {
        Opa0Mux(0)
    }
}
impl core::fmt::Debug for Opa0Mux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Mux")
            .field("possel", &self.possel())
            .field("negsel", &self.negsel())
            .field("resinmux", &self.resinmux())
            .field("gain3x", &self.gain3x())
            .field("ressel", &self.ressel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Mux {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Mux {{ possel: {=u8:?}, negsel: {=u8:?}, resinmux: {:?}, gain3x: {=bool:?}, ressel: {:?} }}" , self . possel () , self . negsel () , self . resinmux () , self . gain3x () , self . ressel ())
    }
}
#[doc = "Operational Amplifier Output Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Out(pub u32);
impl Opa0Out {
    #[doc = "OPAx Main Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mainouten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Main Output Enable."]
    #[inline(always)]
    pub const fn set_mainouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OPAx Alternative Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altouten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Alternative Output Enable."]
    #[inline(always)]
    pub const fn set_altouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OPAx Aport Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aportouten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Aport Output Enable."]
    #[inline(always)]
    pub const fn set_aportouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OPAx Main and Alternative Output Short."]
    #[must_use]
    #[inline(always)]
    pub const fn short(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OPAx Main and Alternative Output Short."]
    #[inline(always)]
    pub const fn set_short(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OPAx Output Enable Value."]
    #[must_use]
    #[inline(always)]
    pub const fn altoutpaden(&self) -> super::vals::Opa0OutAltoutpaden {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Opa0OutAltoutpaden::from_bits(val as u8)
    }
    #[doc = "OPAx Output Enable Value."]
    #[inline(always)]
    pub const fn set_altoutpaden(&mut self, val: super::vals::Opa0OutAltoutpaden) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
    #[doc = "OPAx APORT Output."]
    #[must_use]
    #[inline(always)]
    pub const fn aportoutsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPAx APORT Output."]
    #[inline(always)]
    pub const fn set_aportoutsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Opa0Out {
    #[inline(always)]
    fn default() -> Opa0Out {
        Opa0Out(0)
    }
}
impl core::fmt::Debug for Opa0Out {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Out")
            .field("mainouten", &self.mainouten())
            .field("altouten", &self.altouten())
            .field("aportouten", &self.aportouten())
            .field("short", &self.short())
            .field("altoutpaden", &self.altoutpaden())
            .field("aportoutsel", &self.aportoutsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Out {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opa0Out {{ mainouten: {=bool:?}, altouten: {=bool:?}, aportouten: {=bool:?}, short: {=bool:?}, altoutpaden: {:?}, aportoutsel: {=u8:?} }}" , self . mainouten () , self . altouten () , self . aportouten () , self . short () , self . altoutpaden () , self . aportoutsel ())
    }
}
#[doc = "Operational Amplifier Timer Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa0Timer(pub u32);
impl Opa0Timer {
    #[doc = "OPAx Startup Delay Count Value."]
    #[must_use]
    #[inline(always)]
    pub const fn startupdly(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OPAx Startup Delay Count Value."]
    #[inline(always)]
    pub const fn set_startupdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OPAx Warmup Time Count Value."]
    #[must_use]
    #[inline(always)]
    pub const fn warmuptime(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "OPAx Warmup Time Count Value."]
    #[inline(always)]
    pub const fn set_warmuptime(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "OPAx Output Settling Timeout Value."]
    #[must_use]
    #[inline(always)]
    pub const fn settletime(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "OPAx Output Settling Timeout Value."]
    #[inline(always)]
    pub const fn set_settletime(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Opa0Timer {
    #[inline(always)]
    fn default() -> Opa0Timer {
        Opa0Timer(0)
    }
}
impl core::fmt::Debug for Opa0Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opa0Timer")
            .field("startupdly", &self.startupdly())
            .field("warmuptime", &self.warmuptime())
            .field("settletime", &self.settletime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opa0Timer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opa0Timer {{ startupdly: {=u8:?}, warmuptime: {=u8:?}, settletime: {=u16:?} }}",
            self.startupdly(),
            self.warmuptime(),
            self.settletime()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Channel 0 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0ens(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Enabled Status."]
    #[inline(always)]
    pub const fn set_ch0ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1ens(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Enabled Status."]
    #[inline(always)]
    pub const fn set_ch1ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 0 Buffer Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0bl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Buffer Level."]
    #[inline(always)]
    pub const fn set_ch0bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 1 Buffer Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1bl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Buffer Level."]
    #[inline(always)]
    pub const fn set_ch1bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 0 Warm."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0warm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Warm."]
    #[inline(always)]
    pub const fn set_ch0warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 1 Warm."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1warm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Warm."]
    #[inline(always)]
    pub const fn set_ch1warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPA0 Bus Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0aportconflict(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Bus Conflict Output."]
    #[inline(always)]
    pub const fn set_opa0aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OPA1 Bus Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1aportconflict(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Bus Conflict Output."]
    #[inline(always)]
    pub const fn set_opa1aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OPA2 Bus Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2aportconflict(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Bus Conflict Output."]
    #[inline(always)]
    pub const fn set_opa2aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OPA3 Bus Conflict Output."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3aportconflict(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Bus Conflict Output."]
    #[inline(always)]
    pub const fn set_opa3aportconflict(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OPA0 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0ens(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Enabled Status."]
    #[inline(always)]
    pub const fn set_opa0ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPA1 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1ens(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Enabled Status."]
    #[inline(always)]
    pub const fn set_opa1ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPA2 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2ens(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Enabled Status."]
    #[inline(always)]
    pub const fn set_opa2ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "OPA3 Enabled Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3ens(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Enabled Status."]
    #[inline(always)]
    pub const fn set_opa3ens(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "OPA0 Warm Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0warm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Warm Status."]
    #[inline(always)]
    pub const fn set_opa0warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "OPA1 Warm Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1warm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Warm Status."]
    #[inline(always)]
    pub const fn set_opa1warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OPA2 Warm Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2warm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Warm Status."]
    #[inline(always)]
    pub const fn set_opa2warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "OPA3 Warm Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3warm(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Warm Status."]
    #[inline(always)]
    pub const fn set_opa3warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "OPA0 Output Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa0outvalid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "OPA0 Output Valid Status."]
    #[inline(always)]
    pub const fn set_opa0outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "OPA1 Output Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa1outvalid(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "OPA1 Output Valid Status."]
    #[inline(always)]
    pub const fn set_opa1outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "OPA2 Output Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa2outvalid(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "OPA2 Output Valid Status."]
    #[inline(always)]
    pub const fn set_opa2outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OPA3 Output Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn opa3outvalid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OPA3 Output Valid Status."]
    #[inline(always)]
    pub const fn set_opa3outvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ch0ens", &self.ch0ens())
            .field("ch1ens", &self.ch1ens())
            .field("ch0bl", &self.ch0bl())
            .field("ch1bl", &self.ch1bl())
            .field("ch0warm", &self.ch0warm())
            .field("ch1warm", &self.ch1warm())
            .field("opa0aportconflict", &self.opa0aportconflict())
            .field("opa1aportconflict", &self.opa1aportconflict())
            .field("opa2aportconflict", &self.opa2aportconflict())
            .field("opa3aportconflict", &self.opa3aportconflict())
            .field("opa0ens", &self.opa0ens())
            .field("opa1ens", &self.opa1ens())
            .field("opa2ens", &self.opa2ens())
            .field("opa3ens", &self.opa3ens())
            .field("opa0warm", &self.opa0warm())
            .field("opa1warm", &self.opa1warm())
            .field("opa2warm", &self.opa2warm())
            .field("opa3warm", &self.opa3warm())
            .field("opa0outvalid", &self.opa0outvalid())
            .field("opa1outvalid", &self.opa1outvalid())
            .field("opa2outvalid", &self.opa2outvalid())
            .field("opa3outvalid", &self.opa3outvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ ch0ens: {=bool:?}, ch1ens: {=bool:?}, ch0bl: {=bool:?}, ch1bl: {=bool:?}, ch0warm: {=bool:?}, ch1warm: {=bool:?}, opa0aportconflict: {=bool:?}, opa1aportconflict: {=bool:?}, opa2aportconflict: {=bool:?}, opa3aportconflict: {=bool:?}, opa0ens: {=bool:?}, opa1ens: {=bool:?}, opa2ens: {=bool:?}, opa3ens: {=bool:?}, opa0warm: {=bool:?}, opa1warm: {=bool:?}, opa2warm: {=bool:?}, opa3warm: {=bool:?}, opa0outvalid: {=bool:?}, opa1outvalid: {=bool:?}, opa2outvalid: {=bool:?}, opa3outvalid: {=bool:?} }}" , self . ch0ens () , self . ch1ens () , self . ch0bl () , self . ch1bl () , self . ch0warm () , self . ch1warm () , self . opa0aportconflict () , self . opa1aportconflict () , self . opa2aportconflict () , self . opa3aportconflict () , self . opa0ens () , self . opa1ens () , self . opa2ens () , self . opa3ens () , self . opa0warm () , self . opa1warm () , self . opa2warm () , self . opa3warm () , self . opa0outvalid () , self . opa1outvalid () , self . opa2outvalid () , self . opa3outvalid ())
    }
}
