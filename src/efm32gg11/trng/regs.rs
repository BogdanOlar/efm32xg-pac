#[doc = "Main Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "TRNG Module Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Module Enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn testen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Test Enable."]
    #[inline(always)]
    pub const fn set_testen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Conditioning Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn condbypass(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Conditioning Bypass."]
    #[inline(always)]
    pub const fn set_condbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Enable for Repetition Count Test Failure."]
    #[must_use]
    #[inline(always)]
    pub const fn repcountien(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable for Repetition Count Test Failure."]
    #[inline(always)]
    pub const fn set_repcountien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)."]
    #[must_use]
    #[inline(always)]
    pub const fn apt64ien(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)."]
    #[inline(always)]
    pub const fn set_apt64ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)."]
    #[must_use]
    #[inline(always)]
    pub const fn apt4096ien(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)."]
    #[inline(always)]
    pub const fn set_apt4096ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Enable for FIFO Full."]
    #[must_use]
    #[inline(always)]
    pub const fn fullien(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable for FIFO Full."]
    #[inline(always)]
    pub const fn set_fullien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn softreset(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_softreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt enable for AIS31 preliminary noise alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn preien(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable for AIS31 preliminary noise alarm."]
    #[inline(always)]
    pub const fn set_preien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt enable for AIS31 noise alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almien(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable for AIS31 noise alarm."]
    #[inline(always)]
    pub const fn set_almien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Oscillator Force Run."]
    #[must_use]
    #[inline(always)]
    pub const fn forcerun(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Force Run."]
    #[inline(always)]
    pub const fn set_forcerun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NIST Start-up Test Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn bypnist(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NIST Start-up Test Bypass."]
    #[inline(always)]
    pub const fn set_bypnist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "AIS31 Start-up Test Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn bypais31(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub const fn set_bypais31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("enable", &self.enable())
            .field("testen", &self.testen())
            .field("condbypass", &self.condbypass())
            .field("repcountien", &self.repcountien())
            .field("apt64ien", &self.apt64ien())
            .field("apt4096ien", &self.apt4096ien())
            .field("fullien", &self.fullien())
            .field("softreset", &self.softreset())
            .field("preien", &self.preien())
            .field("almien", &self.almien())
            .field("forcerun", &self.forcerun())
            .field("bypnist", &self.bypnist())
            .field("bypais31", &self.bypais31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Control {{ enable: {=bool:?}, testen: {=bool:?}, condbypass: {=bool:?}, repcountien: {=bool:?}, apt64ien: {=bool:?}, apt4096ien: {=bool:?}, fullien: {=bool:?}, softreset: {=bool:?}, preien: {=bool:?}, almien: {=bool:?}, forcerun: {=bool:?}, bypnist: {=bool:?}, bypais31: {=bool:?} }}" , self . enable () , self . testen () , self . condbypass () , self . repcountien () , self . apt64ien () , self . apt4096ien () , self . fullien () , self . softreset () , self . preien () , self . almien () , self . forcerun () , self . bypnist () , self . bypais31 ())
    }
}
#[doc = "Initial Wait Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Initwaitval(pub u32);
impl Initwaitval {
    #[doc = "Wait counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Wait counter value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Initwaitval {
    #[inline(always)]
    fn default() -> Initwaitval {
        Initwaitval(0)
    }
}
impl core::fmt::Debug for Initwaitval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Initwaitval")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Initwaitval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Initwaitval {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Test Data Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn testdatabusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test Data Busy."]
    #[inline(always)]
    pub const fn set_testdatabusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Repetition Count Test Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn repcountif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Repetition Count Test Interrupt Status."]
    #[inline(always)]
    pub const fn set_repcountif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Adaptive Proportion test failure (64-sample window) interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn apt64if(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Adaptive Proportion test failure (64-sample window) interrupt status."]
    #[inline(always)]
    pub const fn set_apt64if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Adaptive Proportion test failure (4096-sample window) interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn apt4096if(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Adaptive Proportion test failure (4096-sample window) interrupt status."]
    #[inline(always)]
    pub const fn set_apt4096if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Full Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn fullif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Full Interrupt Status."]
    #[inline(always)]
    pub const fn set_fullif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "AIS31 Preliminary Noise Alarm interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn preif(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AIS31 Preliminary Noise Alarm interrupt status."]
    #[inline(always)]
    pub const fn set_preif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "AIS31 Noise Alarm interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn almif(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AIS31 Noise Alarm interrupt status."]
    #[inline(always)]
    pub const fn set_almif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("testdatabusy", &self.testdatabusy())
            .field("repcountif", &self.repcountif())
            .field("apt64if", &self.apt64if())
            .field("apt4096if", &self.apt4096if())
            .field("fullif", &self.fullif())
            .field("preif", &self.preif())
            .field("almif", &self.almif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ testdatabusy: {=bool:?}, repcountif: {=bool:?}, apt64if: {=bool:?}, apt4096if: {=bool:?}, fullif: {=bool:?}, preif: {=bool:?}, almif: {=bool:?} }}" , self . testdatabusy () , self . repcountif () , self . apt64if () , self . apt4096if () , self . fullif () , self . preif () , self . almif ())
    }
}
