#[doc = "Charger Detect Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdconf(pub u32);
impl Cdconf {
    #[doc = "DCD Timeout (TDCD_TIMEOUT) Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdtoconf(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "DCD Timeout (TDCD_TIMEOUT) Configuration."]
    #[inline(always)]
    pub const fn set_dcdtoconf(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Cdconf {
    #[inline(always)]
    fn default() -> Cdconf {
        Cdconf(0)
    }
}
impl core::fmt::Debug for Cdconf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdconf")
            .field("dcdtoconf", &self.dcdtoconf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdconf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdconf {{ dcdtoconf: {=u16:?} }}", self.dcdtoconf())
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Start Charger Detection Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn startcd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Charger Detection Enabled."]
    #[inline(always)]
    pub const fn set_startcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start Charger Detection in Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn stopcd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start Charger Detection in Progress."]
    #[inline(always)]
    pub const fn set_stopcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("startcd", &self.startcd())
            .field("stopcd", &self.stopcd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ startcd: {=bool:?}, stopcd: {=bool:?} }}",
            self.startcd(),
            self.stopcd()
        )
    }
}
#[doc = "System Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "VBUSEN Active Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusenap(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUSEN Active Polarity."]
    #[inline(always)]
    pub const fn set_vbusenap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PHY Power."]
    #[must_use]
    #[inline(always)]
    pub const fn selfpowered(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Power."]
    #[inline(always)]
    pub const fn set_selfpowered(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Low Energy Mode Oscillator Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lemoscctrl(&self) -> super::vals::Lemoscctrl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Lemoscctrl::from_bits(val as u8)
    }
    #[doc = "Low Energy Mode Oscillator Control."]
    #[inline(always)]
    pub const fn set_lemoscctrl(&mut self, val: super::vals::Lemoscctrl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Low Energy Mode USB PHY Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lemphyctrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Mode USB PHY Control."]
    #[inline(always)]
    pub const fn set_lemphyctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Low Energy Mode on Bus Idle Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lemidleen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Mode on Bus Idle Enable."]
    #[inline(always)]
    pub const fn set_lemidleen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ID Pull-up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn idcden(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pull-up Enable."]
    #[inline(always)]
    pub const fn set_idcden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OTG CLKC Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn otgclkcdis(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "OTG CLKC Disable."]
    #[inline(always)]
    pub const fn set_otgclkcdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OTG ID Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn otgidindis(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Input Disable."]
    #[inline(always)]
    pub const fn set_otgidindis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "OTG Control Signals to PHY Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn otgphyctrldis(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Control Signals to PHY Disable."]
    #[inline(always)]
    pub const fn set_otgphyctrldis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Data Contact Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcden(&self) -> super::vals::Dcden {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dcden::from_bits(val as u8)
    }
    #[doc = "Data Contact Detection Enable."]
    #[inline(always)]
    pub const fn set_dcden(&mut self, val: super::vals::Dcden) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Primary Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pden(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Primary Detection Enable."]
    #[inline(always)]
    pub const fn set_pden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Secondary Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Enable."]
    #[inline(always)]
    pub const fn set_sden(&mut self, val: bool) {
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
            .field("vbusenap", &self.vbusenap())
            .field("selfpowered", &self.selfpowered())
            .field("lemoscctrl", &self.lemoscctrl())
            .field("lemphyctrl", &self.lemphyctrl())
            .field("lemidleen", &self.lemidleen())
            .field("idcden", &self.idcden())
            .field("otgclkcdis", &self.otgclkcdis())
            .field("otgidindis", &self.otgidindis())
            .field("otgphyctrldis", &self.otgphyctrldis())
            .field("dcden", &self.dcden())
            .field("pden", &self.pden())
            .field("sden", &self.sden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ vbusenap: {=bool:?}, selfpowered: {=bool:?}, lemoscctrl: {:?}, lemphyctrl: {=bool:?}, lemidleen: {=bool:?}, idcden: {=bool:?}, otgclkcdis: {=bool:?}, otgidindis: {=bool:?}, otgphyctrldis: {=bool:?}, dcden: {:?}, pden: {=bool:?}, sden: {=bool:?} }}" , self . vbusenap () , self . selfpowered () , self . lemoscctrl () , self . lemphyctrl () , self . lemidleen () , self . idcden () , self . otgclkcdis () , self . otgidindis () , self . otgphyctrldis () , self . dcden () , self . pden () , self . sden ())
    }
}
#[doc = "Device All Endpoints Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daint(pub u32);
impl Daint {
    #[doc = "IN Endpoint 0 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 0 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IN Endpoint 1 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 1 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IN Endpoint 2 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 2 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IN Endpoint 3 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 3 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Endpoint 4 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 4 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Endpoint 5 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 5 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint 6 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepint6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 6 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_inepint6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Endpoint 0 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 0 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OUT Endpoint 1 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 1 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OUT Endpoint 2 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 2 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OUT Endpoint 3 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 3 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OUT Endpoint 4 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 4 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OUT Endpoint 5 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 5 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OUT Endpoint 6 Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepint6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 6 Interrupt Bit."]
    #[inline(always)]
    pub const fn set_outepint6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Daint {
    #[inline(always)]
    fn default() -> Daint {
        Daint(0)
    }
}
impl core::fmt::Debug for Daint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Daint")
            .field("inepint0", &self.inepint0())
            .field("inepint1", &self.inepint1())
            .field("inepint2", &self.inepint2())
            .field("inepint3", &self.inepint3())
            .field("inepint4", &self.inepint4())
            .field("inepint5", &self.inepint5())
            .field("inepint6", &self.inepint6())
            .field("outepint0", &self.outepint0())
            .field("outepint1", &self.outepint1())
            .field("outepint2", &self.outepint2())
            .field("outepint3", &self.outepint3())
            .field("outepint4", &self.outepint4())
            .field("outepint5", &self.outepint5())
            .field("outepint6", &self.outepint6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Daint {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Daint {{ inepint0: {=bool:?}, inepint1: {=bool:?}, inepint2: {=bool:?}, inepint3: {=bool:?}, inepint4: {=bool:?}, inepint5: {=bool:?}, inepint6: {=bool:?}, outepint0: {=bool:?}, outepint1: {=bool:?}, outepint2: {=bool:?}, outepint3: {=bool:?}, outepint4: {=bool:?}, outepint5: {=bool:?}, outepint6: {=bool:?} }}" , self . inepint0 () , self . inepint1 () , self . inepint2 () , self . inepint3 () , self . inepint4 () , self . inepint5 () , self . inepint6 () , self . outepint0 () , self . outepint1 () , self . outepint2 () , self . outepint3 () , self . outepint4 () , self . outepint5 () , self . outepint6 ())
    }
}
#[doc = "Device All Endpoints Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daintmsk(pub u32);
impl Daintmsk {
    #[doc = "IN Endpoint 0 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 0 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IN Endpoint 1 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 1 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IN Endpoint 2 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 2 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IN Endpoint 3 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 3 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Endpoint 4 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 4 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Endpoint 5 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 5 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint 6 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inepmsk6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint 6 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_inepmsk6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Endpoint 0 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 0 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OUT Endpoint 1 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 1 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OUT Endpoint 2 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 2 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OUT Endpoint 3 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 3 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OUT Endpoint 4 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 4 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OUT Endpoint 5 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 5 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OUT Endpoint 6 Interrupt mask Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn outepmsk6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoint 6 Interrupt mask Bit."]
    #[inline(always)]
    pub const fn set_outepmsk6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Daintmsk {
    #[inline(always)]
    fn default() -> Daintmsk {
        Daintmsk(0)
    }
}
impl core::fmt::Debug for Daintmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Daintmsk")
            .field("inepmsk0", &self.inepmsk0())
            .field("inepmsk1", &self.inepmsk1())
            .field("inepmsk2", &self.inepmsk2())
            .field("inepmsk3", &self.inepmsk3())
            .field("inepmsk4", &self.inepmsk4())
            .field("inepmsk5", &self.inepmsk5())
            .field("inepmsk6", &self.inepmsk6())
            .field("outepmsk0", &self.outepmsk0())
            .field("outepmsk1", &self.outepmsk1())
            .field("outepmsk2", &self.outepmsk2())
            .field("outepmsk3", &self.outepmsk3())
            .field("outepmsk4", &self.outepmsk4())
            .field("outepmsk5", &self.outepmsk5())
            .field("outepmsk6", &self.outepmsk6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Daintmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Daintmsk {{ inepmsk0: {=bool:?}, inepmsk1: {=bool:?}, inepmsk2: {=bool:?}, inepmsk3: {=bool:?}, inepmsk4: {=bool:?}, inepmsk5: {=bool:?}, inepmsk6: {=bool:?}, outepmsk0: {=bool:?}, outepmsk1: {=bool:?}, outepmsk2: {=bool:?}, outepmsk3: {=bool:?}, outepmsk4: {=bool:?}, outepmsk5: {=bool:?}, outepmsk6: {=bool:?} }}" , self . inepmsk0 () , self . inepmsk1 () , self . inepmsk2 () , self . inepmsk3 () , self . inepmsk4 () , self . inepmsk5 () , self . inepmsk6 () , self . outepmsk0 () , self . outepmsk1 () , self . outepmsk2 () , self . outepmsk3 () , self . outepmsk4 () , self . outepmsk5 () , self . outepmsk6 ())
    }
}
#[doc = "Data TRIM 1 Values for USB DP and DM."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dattrim1(pub u32);
impl Dattrim1 {
    #[doc = "Trim for DP and DM Output Impedance for Both FS and LS."]
    #[must_use]
    #[inline(always)]
    pub const fn rout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim for DP and DM Output Impedance for Both FS and LS."]
    #[inline(always)]
    pub const fn set_rout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Enables Delay of Pull in TX Mode for Both FS and LS."]
    #[must_use]
    #[inline(always)]
    pub const fn endlypullup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Delay of Pull in TX Mode for Both FS and LS."]
    #[inline(always)]
    pub const fn set_endlypullup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Trim for Rising Crossover Voltage in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn dlypullupfs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for Rising Crossover Voltage in FS."]
    #[inline(always)]
    pub const fn set_dlypullupfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Trim for Falling Crossover Voltage in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn vcrsfs(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for Falling Crossover Voltage in FS."]
    #[inline(always)]
    pub const fn set_vcrsfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Trim for DM Fall Time in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn tfdmfs(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for DM Fall Time in FS."]
    #[inline(always)]
    pub const fn set_tfdmfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Trim for DM Rise Time in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn trdmfs(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for DM Rise Time in FS."]
    #[inline(always)]
    pub const fn set_trdmfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Trim for DP Fall Time in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn tfdpfs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for DP Fall Time in FS."]
    #[inline(always)]
    pub const fn set_tfdpfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Trim for DP Rise Time in FS."]
    #[must_use]
    #[inline(always)]
    pub const fn trdpfs(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for DP Rise Time in FS."]
    #[inline(always)]
    pub const fn set_trdpfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for Dattrim1 {
    #[inline(always)]
    fn default() -> Dattrim1 {
        Dattrim1(0)
    }
}
impl core::fmt::Debug for Dattrim1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dattrim1")
            .field("rout", &self.rout())
            .field("endlypullup", &self.endlypullup())
            .field("dlypullupfs", &self.dlypullupfs())
            .field("vcrsfs", &self.vcrsfs())
            .field("tfdmfs", &self.tfdmfs())
            .field("trdmfs", &self.trdmfs())
            .field("tfdpfs", &self.tfdpfs())
            .field("trdpfs", &self.trdpfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dattrim1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dattrim1 {{ rout: {=u8:?}, endlypullup: {=bool:?}, dlypullupfs: {=u8:?}, vcrsfs: {=u8:?}, tfdmfs: {=u8:?}, trdmfs: {=u8:?}, tfdpfs: {=u8:?}, trdpfs: {=u8:?} }}" , self . rout () , self . endlypullup () , self . dlypullupfs () , self . vcrsfs () , self . tfdmfs () , self . trdmfs () , self . tfdpfs () , self . trdpfs ())
    }
}
#[doc = "Device Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcfg(pub u32);
impl Dcfg {
    #[doc = "Device Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn devspd(&self) -> super::vals::Devspd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Devspd::from_bits(val as u8)
    }
    #[doc = "Device Speed."]
    #[inline(always)]
    pub const fn set_devspd(&mut self, val: super::vals::Devspd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-Zero-Length Status OUT Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn nzstsouthshk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Zero-Length Status OUT Handshake."]
    #[inline(always)]
    pub const fn set_nzstsouthshk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable 32 kHz Suspend Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ena32khzsusp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 32 kHz Suspend Mode."]
    #[inline(always)]
    pub const fn set_ena32khzsusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn devaddr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn set_devaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
    }
    #[doc = "Periodic Frame Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn perfrint(&self) -> super::vals::Perfrint {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Perfrint::from_bits(val as u8)
    }
    #[doc = "Periodic Frame Interval."]
    #[inline(always)]
    pub const fn set_perfrint(&mut self, val: super::vals::Perfrint) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "Enable Device OUT NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn endevoutnak(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Device OUT NAK."]
    #[inline(always)]
    pub const fn set_endevoutnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn xcvrdly(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_xcvrdly(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn erraticintmsk(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_erraticintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Resume Validation Period."]
    #[must_use]
    #[inline(always)]
    pub const fn resvalid(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "Resume Validation Period."]
    #[inline(always)]
    pub const fn set_resvalid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Dcfg {
    #[inline(always)]
    fn default() -> Dcfg {
        Dcfg(0)
    }
}
impl core::fmt::Debug for Dcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcfg")
            .field("devspd", &self.devspd())
            .field("nzstsouthshk", &self.nzstsouthshk())
            .field("ena32khzsusp", &self.ena32khzsusp())
            .field("devaddr", &self.devaddr())
            .field("perfrint", &self.perfrint())
            .field("endevoutnak", &self.endevoutnak())
            .field("xcvrdly", &self.xcvrdly())
            .field("erraticintmsk", &self.erraticintmsk())
            .field("resvalid", &self.resvalid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcfg {{ devspd: {:?}, nzstsouthshk: {=bool:?}, ena32khzsusp: {=bool:?}, devaddr: {=u8:?}, perfrint: {:?}, endevoutnak: {=bool:?}, xcvrdly: {=bool:?}, erraticintmsk: {=bool:?}, resvalid: {=u8:?} }}" , self . devspd () , self . nzstsouthshk () , self . ena32khzsusp () , self . devaddr () , self . perfrint () , self . endevoutnak () , self . xcvrdly () , self . erraticintmsk () , self . resvalid ())
    }
}
#[doc = "Device Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dctl(pub u32);
impl Dctl {
    #[doc = "Remote Wakeup Signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn rmtwkupsig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Wakeup Signaling."]
    #[inline(always)]
    pub const fn set_rmtwkupsig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Soft Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn sftdiscon(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Soft Disconnect."]
    #[inline(always)]
    pub const fn set_sftdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Global Non-periodic IN NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn gnpinnaksts(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Global Non-periodic IN NAK Status."]
    #[inline(always)]
    pub const fn set_gnpinnaksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Global OUT NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn goutnaksts(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Global OUT NAK Status."]
    #[inline(always)]
    pub const fn set_goutnaksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tstctl(&self) -> super::vals::Tstctl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Tstctl::from_bits(val as u8)
    }
    #[doc = "Test Control."]
    #[inline(always)]
    pub const fn set_tstctl(&mut self, val: super::vals::Tstctl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Set Global Non-periodic IN NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn sgnpinnak(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set Global Non-periodic IN NAK."]
    #[inline(always)]
    pub const fn set_sgnpinnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clear Global Non-periodic IN NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cgnpinnak(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Global Non-periodic IN NAK."]
    #[inline(always)]
    pub const fn set_cgnpinnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set Global OUT NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn sgoutnak(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set Global OUT NAK."]
    #[inline(always)]
    pub const fn set_sgoutnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clear Global OUT NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cgoutnak(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Global OUT NAK."]
    #[inline(always)]
    pub const fn set_cgoutnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power-On Programming Done."]
    #[must_use]
    #[inline(always)]
    pub const fn pwronprgdone(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power-On Programming Done."]
    #[inline(always)]
    pub const fn set_pwronprgdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Ignore Frame number For Isochronous End points."]
    #[must_use]
    #[inline(always)]
    pub const fn ignrfrmnum(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Frame number For Isochronous End points."]
    #[inline(always)]
    pub const fn set_ignrfrmnum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK on Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nakonbble(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK on Babble Error."]
    #[inline(always)]
    pub const fn set_nakonbble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Dctl {
    #[inline(always)]
    fn default() -> Dctl {
        Dctl(0)
    }
}
impl core::fmt::Debug for Dctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dctl")
            .field("rmtwkupsig", &self.rmtwkupsig())
            .field("sftdiscon", &self.sftdiscon())
            .field("gnpinnaksts", &self.gnpinnaksts())
            .field("goutnaksts", &self.goutnaksts())
            .field("tstctl", &self.tstctl())
            .field("sgnpinnak", &self.sgnpinnak())
            .field("cgnpinnak", &self.cgnpinnak())
            .field("sgoutnak", &self.sgoutnak())
            .field("cgoutnak", &self.cgoutnak())
            .field("pwronprgdone", &self.pwronprgdone())
            .field("ignrfrmnum", &self.ignrfrmnum())
            .field("nakonbble", &self.nakonbble())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dctl {{ rmtwkupsig: {=bool:?}, sftdiscon: {=bool:?}, gnpinnaksts: {=bool:?}, goutnaksts: {=bool:?}, tstctl: {:?}, sgnpinnak: {=bool:?}, cgnpinnak: {=bool:?}, sgoutnak: {=bool:?}, cgoutnak: {=bool:?}, pwronprgdone: {=bool:?}, ignrfrmnum: {=bool:?}, nakonbble: {=bool:?} }}" , self . rmtwkupsig () , self . sftdiscon () , self . gnpinnaksts () , self . goutnaksts () , self . tstctl () , self . sgnpinnak () , self . cgnpinnak () , self . sgoutnak () , self . cgoutnak () , self . pwronprgdone () , self . ignrfrmnum () , self . nakonbble ())
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0Ctl(pub u32);
impl Diep0Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep0CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep0CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep0CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep0Ctl {
    #[inline(always)]
    fn default() -> Diep0Ctl {
        Diep0Ctl(0)
    }
}
impl core::fmt::Debug for Diep0Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep0Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0Dtxfsts(pub u32);
impl Diep0Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep0Dtxfsts {
    #[inline(always)]
    fn default() -> Diep0Dtxfsts {
        Diep0Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep0Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep0Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0Int(pub u32);
impl Diep0Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep0Int {
    #[inline(always)]
    fn default() -> Diep0Int {
        Diep0Int(0)
    }
}
impl core::fmt::Debug for Diep0Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep0Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0Tsiz(pub u32);
impl Diep0Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep0Tsiz {
    #[inline(always)]
    fn default() -> Diep0Tsiz {
        Diep0Tsiz(0)
    }
}
impl core::fmt::Debug for Diep0Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep0Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device Control IN Endpoint 0 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0ctl(pub u32);
impl Diep0ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> super::vals::Diep0ctlMps {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Diep0ctlMps::from_bits(val as u8)
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: super::vals::Diep0ctlMps) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep0ctl {
    #[inline(always)]
    fn default() -> Diep0ctl {
        Diep0ctl(0)
    }
}
impl core::fmt::Debug for Diep0ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep0ctl {{ mps: {:?}, usbactep: {=bool:?}, naksts: {=bool:?}, eptype: {=u8:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint 0 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0int(pub u32);
impl Diep0int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep0int {
    #[inline(always)]
    fn default() -> Diep0int {
        Diep0int(0)
    }
}
impl core::fmt::Debug for Diep0int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep0int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint 0 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0tsiz(pub u32);
impl Diep0tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
}
impl Default for Diep0tsiz {
    #[inline(always)]
    fn default() -> Diep0tsiz {
        Diep0tsiz(0)
    }
}
impl core::fmt::Debug for Diep0tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep0tsiz {{ xfersize: {=u8:?}, pktcnt: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep0txfsts(pub u32);
impl Diep0txfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep0txfsts {
    #[inline(always)]
    fn default() -> Diep0txfsts {
        Diep0txfsts(0)
    }
}
impl core::fmt::Debug for Diep0txfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep0txfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep0txfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep0txfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep1Ctl(pub u32);
impl Diep1Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep1CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep1CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep1CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep1Ctl {
    #[inline(always)]
    fn default() -> Diep1Ctl {
        Diep1Ctl(0)
    }
}
impl core::fmt::Debug for Diep1Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep1Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep1Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep1Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep1Dtxfsts(pub u32);
impl Diep1Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep1Dtxfsts {
    #[inline(always)]
    fn default() -> Diep1Dtxfsts {
        Diep1Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep1Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep1Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep1Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep1Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep1Int(pub u32);
impl Diep1Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep1Int {
    #[inline(always)]
    fn default() -> Diep1Int {
        Diep1Int(0)
    }
}
impl core::fmt::Debug for Diep1Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep1Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep1Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep1Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep1Tsiz(pub u32);
impl Diep1Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep1Tsiz {
    #[inline(always)]
    fn default() -> Diep1Tsiz {
        Diep1Tsiz(0)
    }
}
impl core::fmt::Debug for Diep1Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep1Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep1Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep1Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep2Ctl(pub u32);
impl Diep2Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep2CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep2CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep2CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep2Ctl {
    #[inline(always)]
    fn default() -> Diep2Ctl {
        Diep2Ctl(0)
    }
}
impl core::fmt::Debug for Diep2Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep2Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep2Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep2Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep2Dtxfsts(pub u32);
impl Diep2Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep2Dtxfsts {
    #[inline(always)]
    fn default() -> Diep2Dtxfsts {
        Diep2Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep2Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep2Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep2Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep2Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep2Int(pub u32);
impl Diep2Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep2Int {
    #[inline(always)]
    fn default() -> Diep2Int {
        Diep2Int(0)
    }
}
impl core::fmt::Debug for Diep2Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep2Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep2Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep2Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep2Tsiz(pub u32);
impl Diep2Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep2Tsiz {
    #[inline(always)]
    fn default() -> Diep2Tsiz {
        Diep2Tsiz(0)
    }
}
impl core::fmt::Debug for Diep2Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep2Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep2Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep2Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep3Ctl(pub u32);
impl Diep3Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep3CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep3CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep3CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep3Ctl {
    #[inline(always)]
    fn default() -> Diep3Ctl {
        Diep3Ctl(0)
    }
}
impl core::fmt::Debug for Diep3Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep3Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep3Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep3Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep3Dtxfsts(pub u32);
impl Diep3Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep3Dtxfsts {
    #[inline(always)]
    fn default() -> Diep3Dtxfsts {
        Diep3Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep3Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep3Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep3Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep3Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep3Int(pub u32);
impl Diep3Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep3Int {
    #[inline(always)]
    fn default() -> Diep3Int {
        Diep3Int(0)
    }
}
impl core::fmt::Debug for Diep3Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep3Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep3Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep3Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep3Tsiz(pub u32);
impl Diep3Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep3Tsiz {
    #[inline(always)]
    fn default() -> Diep3Tsiz {
        Diep3Tsiz(0)
    }
}
impl core::fmt::Debug for Diep3Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep3Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep3Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep3Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep4Ctl(pub u32);
impl Diep4Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep4CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep4CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep4CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep4Ctl {
    #[inline(always)]
    fn default() -> Diep4Ctl {
        Diep4Ctl(0)
    }
}
impl core::fmt::Debug for Diep4Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep4Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep4Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep4Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep4Dtxfsts(pub u32);
impl Diep4Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep4Dtxfsts {
    #[inline(always)]
    fn default() -> Diep4Dtxfsts {
        Diep4Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep4Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep4Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep4Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep4Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep4Int(pub u32);
impl Diep4Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep4Int {
    #[inline(always)]
    fn default() -> Diep4Int {
        Diep4Int(0)
    }
}
impl core::fmt::Debug for Diep4Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep4Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep4Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep4Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep4Tsiz(pub u32);
impl Diep4Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep4Tsiz {
    #[inline(always)]
    fn default() -> Diep4Tsiz {
        Diep4Tsiz(0)
    }
}
impl core::fmt::Debug for Diep4Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep4Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep4Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep4Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device Control IN Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep5Ctl(pub u32);
impl Diep5Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even or Odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Diep5CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Diep5CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Diep5CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TxFIFO Number."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO Number."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Diep5Ctl {
    #[inline(always)]
    fn default() -> Diep5Ctl {
        Diep5Ctl(0)
    }
}
impl core::fmt::Debug for Diep5Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep5Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep5Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep5Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep5Dtxfsts(pub u32);
impl Diep5Dtxfsts {
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn spcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_spcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diep5Dtxfsts {
    #[inline(always)]
    fn default() -> Diep5Dtxfsts {
        Diep5Dtxfsts(0)
    }
}
impl core::fmt::Debug for Diep5Dtxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep5Dtxfsts")
            .field("spcavail", &self.spcavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep5Dtxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Diep5Dtxfsts {{ spcavail: {=u16:?} }}", self.spcavail())
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep5Int(pub u32);
impl Diep5Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfemp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO is Empty."]
    #[inline(always)]
    pub const fn set_intkntxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received with EP Mismatch."]
    #[inline(always)]
    pub const fn set_intknepmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective."]
    #[inline(always)]
    pub const fn set_inepnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfemp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Empty."]
    #[inline(always)]
    pub const fn set_txfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fifo Underrun."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun."]
    #[inline(always)]
    pub const fn set_txfifoundrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diep5Int {
    #[inline(always)]
    fn default() -> Diep5Int {
        Diep5Int(0)
    }
}
impl core::fmt::Debug for Diep5Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep5Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep5Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diep5Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, timeout: {=bool:?}, intkntxfemp: {=bool:?}, intknepmis: {=bool:?}, inepnakeff: {=bool:?}, txfemp: {=bool:?}, txfifoundrn: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . timeout () , self . intkntxfemp () , self . intknepmis () , self . inepnakeff () , self . txfemp () , self . txfifoundrn () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt ())
    }
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diep5Tsiz(pub u32);
impl Diep5Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Multi Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Diep5Tsiz {
    #[inline(always)]
    fn default() -> Diep5Tsiz {
        Diep5Tsiz(0)
    }
}
impl core::fmt::Debug for Diep5Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diep5Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diep5Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diep5Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, mc: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.mc()
        )
    }
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepempmsk(pub u32);
impl Diepempmsk {
    #[doc = "IN EP Tx FIFO Empty Interrupt Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ineptxfempmsk(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "IN EP Tx FIFO Empty Interrupt Mask Bits."]
    #[inline(always)]
    pub const fn set_ineptxfempmsk(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Diepempmsk {
    #[inline(always)]
    fn default() -> Diepempmsk {
        Diepempmsk(0)
    }
}
impl core::fmt::Debug for Diepempmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diepempmsk")
            .field("ineptxfempmsk", &self.ineptxfempmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diepempmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Diepempmsk {{ ineptxfempmsk: {=u16:?} }}",
            self.ineptxfempmsk()
        )
    }
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepmsk(pub u32);
impl Diepmsk {
    #[doc = "Transfer Completed Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercomplmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt Mask."]
    #[inline(always)]
    pub const fn set_xfercomplmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbldmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt Mask."]
    #[inline(always)]
    pub const fn set_epdisbldmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberrmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error Mask."]
    #[inline(always)]
    pub const fn set_ahberrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timeout Condition Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn timeoutmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Condition Mask."]
    #[inline(always)]
    pub const fn set_timeoutmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IN Token Received When TxFIFO Empty Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn intkntxfempmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token Received When TxFIFO Empty Mask."]
    #[inline(always)]
    pub const fn set_intkntxfempmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN Token received with EP Mismatch Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn intknepmismsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IN Token received with EP Mismatch Mask."]
    #[inline(always)]
    pub const fn set_intknepmismsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IN Endpoint NAK Effective Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn inepnakeffmsk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint NAK Effective Mask."]
    #[inline(always)]
    pub const fn set_inepnakeffmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Fifo Underrun Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifoundrnmsk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fifo Underrun Mask."]
    #[inline(always)]
    pub const fn set_txfifoundrnmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "NAK interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn nakmsk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK interrupt Mask."]
    #[inline(always)]
    pub const fn set_nakmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Diepmsk {
    #[inline(always)]
    fn default() -> Diepmsk {
        Diepmsk(0)
    }
}
impl core::fmt::Debug for Diepmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Diepmsk")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("epdisbldmsk", &self.epdisbldmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("timeoutmsk", &self.timeoutmsk())
            .field("intkntxfempmsk", &self.intkntxfempmsk())
            .field("intknepmismsk", &self.intknepmismsk())
            .field("inepnakeffmsk", &self.inepnakeffmsk())
            .field("txfifoundrnmsk", &self.txfifoundrnmsk())
            .field("nakmsk", &self.nakmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Diepmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Diepmsk {{ xfercomplmsk: {=bool:?}, epdisbldmsk: {=bool:?}, ahberrmsk: {=bool:?}, timeoutmsk: {=bool:?}, intkntxfempmsk: {=bool:?}, intknepmismsk: {=bool:?}, inepnakeffmsk: {=bool:?}, txfifoundrnmsk: {=bool:?}, nakmsk: {=bool:?} }}" , self . xfercomplmsk () , self . epdisbldmsk () , self . ahberrmsk () , self . timeoutmsk () , self . intkntxfempmsk () , self . intknepmismsk () , self . inepnakeffmsk () , self . txfifoundrnmsk () , self . nakmsk ())
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf1(pub u32);
impl Dieptxf1 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf1 {
    #[inline(always)]
    fn default() -> Dieptxf1 {
        Dieptxf1(0)
    }
}
impl core::fmt::Debug for Dieptxf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf1")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf1 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf2(pub u32);
impl Dieptxf2 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf2 {
    #[inline(always)]
    fn default() -> Dieptxf2 {
        Dieptxf2(0)
    }
}
impl core::fmt::Debug for Dieptxf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf2")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf2 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf3(pub u32);
impl Dieptxf3 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf3 {
    #[inline(always)]
    fn default() -> Dieptxf3 {
        Dieptxf3(0)
    }
}
impl core::fmt::Debug for Dieptxf3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf3")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf3 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf4(pub u32);
impl Dieptxf4 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf4 {
    #[inline(always)]
    fn default() -> Dieptxf4 {
        Dieptxf4(0)
    }
}
impl core::fmt::Debug for Dieptxf4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf4")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf4 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf5(pub u32);
impl Dieptxf5 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf5 {
    #[inline(always)]
    fn default() -> Dieptxf5 {
        Dieptxf5(0)
    }
}
impl core::fmt::Debug for Dieptxf5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf5")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf5 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf6(pub u32);
impl Dieptxf6 {
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "IN Endpoint FIFOn Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_inepntxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn inepntxfdep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "IN Endpoint TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_inepntxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dieptxf6 {
    #[inline(always)]
    fn default() -> Dieptxf6 {
        Dieptxf6(0)
    }
}
impl core::fmt::Debug for Dieptxf6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieptxf6")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieptxf6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieptxf6 {{ inepntxfstaddr: {=u16:?}, inepntxfdep: {=u16:?} }}",
            self.inepntxfstaddr(),
            self.inepntxfdep()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0Ctl(pub u32);
impl Doep0Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep0CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep0CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep0CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep0Ctl {
    #[inline(always)]
    fn default() -> Doep0Ctl {
        Doep0Ctl(0)
    }
}
impl core::fmt::Debug for Doep0Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep0Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0Int(pub u32);
impl Doep0Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep0Int {
    #[inline(always)]
    fn default() -> Doep0Int {
        Doep0Int(0)
    }
}
impl core::fmt::Debug for Doep0Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep0Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0Tsiz(pub u32);
impl Doep0Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep0TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep0TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep0TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep0Tsiz {
    #[inline(always)]
    fn default() -> Doep0Tsiz {
        Doep0Tsiz(0)
    }
}
impl core::fmt::Debug for Doep0Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep0Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint 0 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0ctl(pub u32);
impl Doep0ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> super::vals::Doep0ctlMps {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Doep0ctlMps::from_bits(val as u8)
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: super::vals::Doep0ctlMps) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep0ctl {
    #[inline(always)]
    fn default() -> Doep0ctl {
        Doep0ctl(0)
    }
}
impl core::fmt::Debug for Doep0ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep0ctl {{ mps: {:?}, usbactep: {=bool:?}, naksts: {=bool:?}, eptype: {=u8:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint 0 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0int(pub u32);
impl Doep0int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep0int {
    #[inline(always)]
    fn default() -> Doep0int {
        Doep0int(0)
    }
}
impl core::fmt::Debug for Doep0int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep0int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep0tsiz(pub u32);
impl Doep0tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn supcnt(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_supcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep0tsiz {
    #[inline(always)]
    fn default() -> Doep0tsiz {
        Doep0tsiz(0)
    }
}
impl core::fmt::Debug for Doep0tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep0tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("supcnt", &self.supcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep0tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep0tsiz {{ xfersize: {=u8:?}, pktcnt: {=bool:?}, supcnt: {=u8:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.supcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep1Ctl(pub u32);
impl Doep1Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep1CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep1CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep1CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep1Ctl {
    #[inline(always)]
    fn default() -> Doep1Ctl {
        Doep1Ctl(0)
    }
}
impl core::fmt::Debug for Doep1Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep1Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep1Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep1Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep1Int(pub u32);
impl Doep1Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep1Int {
    #[inline(always)]
    fn default() -> Doep1Int {
        Doep1Int(0)
    }
}
impl core::fmt::Debug for Doep1Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep1Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep1Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep1Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep1Tsiz(pub u32);
impl Doep1Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep1TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep1TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep1TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep1Tsiz {
    #[inline(always)]
    fn default() -> Doep1Tsiz {
        Doep1Tsiz(0)
    }
}
impl core::fmt::Debug for Doep1Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep1Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep1Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep1Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep2Ctl(pub u32);
impl Doep2Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep2CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep2CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep2CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep2Ctl {
    #[inline(always)]
    fn default() -> Doep2Ctl {
        Doep2Ctl(0)
    }
}
impl core::fmt::Debug for Doep2Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep2Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep2Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep2Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep2Int(pub u32);
impl Doep2Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep2Int {
    #[inline(always)]
    fn default() -> Doep2Int {
        Doep2Int(0)
    }
}
impl core::fmt::Debug for Doep2Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep2Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep2Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep2Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep2Tsiz(pub u32);
impl Doep2Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep2TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep2TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep2TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep2Tsiz {
    #[inline(always)]
    fn default() -> Doep2Tsiz {
        Doep2Tsiz(0)
    }
}
impl core::fmt::Debug for Doep2Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep2Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep2Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep2Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep3Ctl(pub u32);
impl Doep3Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep3CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep3CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep3CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep3Ctl {
    #[inline(always)]
    fn default() -> Doep3Ctl {
        Doep3Ctl(0)
    }
}
impl core::fmt::Debug for Doep3Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep3Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep3Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep3Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep3Int(pub u32);
impl Doep3Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep3Int {
    #[inline(always)]
    fn default() -> Doep3Int {
        Doep3Int(0)
    }
}
impl core::fmt::Debug for Doep3Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep3Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep3Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep3Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep3Tsiz(pub u32);
impl Doep3Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep3TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep3TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep3TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep3Tsiz {
    #[inline(always)]
    fn default() -> Doep3Tsiz {
        Doep3Tsiz(0)
    }
}
impl core::fmt::Debug for Doep3Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep3Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep3Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep3Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep4Ctl(pub u32);
impl Doep4Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep4CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep4CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep4CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep4Ctl {
    #[inline(always)]
    fn default() -> Doep4Ctl {
        Doep4Ctl(0)
    }
}
impl core::fmt::Debug for Doep4Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep4Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep4Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep4Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep4Int(pub u32);
impl Doep4Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep4Int {
    #[inline(always)]
    fn default() -> Doep4Int {
        Doep4Int(0)
    }
}
impl core::fmt::Debug for Doep4Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep4Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep4Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep4Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep4Tsiz(pub u32);
impl Doep4Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep4TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep4TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep4TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep4Tsiz {
    #[inline(always)]
    fn default() -> Doep4Tsiz {
        Doep4Tsiz(0)
    }
}
impl core::fmt::Debug for Doep4Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep4Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep4Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep4Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device Control OUT Endpoint x+1 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep5Ctl(pub u32);
impl Doep5Ctl {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "USB Active Endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn usbactep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Active Endpoint."]
    #[inline(always)]
    pub const fn set_usbactep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn dpideof(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Data PID / Even-odd Frame."]
    #[inline(always)]
    pub const fn set_dpideof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NAK Status."]
    #[must_use]
    #[inline(always)]
    pub const fn naksts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Status."]
    #[inline(always)]
    pub const fn set_naksts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Doep5CtlEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Doep5CtlEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Doep5CtlEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Snoop Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn snp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Snoop Mode."]
    #[inline(always)]
    pub const fn set_snp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "STALL Handshake."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Handshake."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Clear NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn cnak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clear NAK."]
    #[inline(always)]
    pub const fn set_cnak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn snak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set NAK."]
    #[inline(always)]
    pub const fn set_snak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd0pidef(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA0 PID / Even Frame."]
    #[inline(always)]
    pub const fn set_setd0pidef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn setd1pidof(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Set DATA1 PID / Odd Frame."]
    #[inline(always)]
    pub const fn set_setd1pidof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Endpoint Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disable."]
    #[inline(always)]
    pub const fn set_epdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Enable."]
    #[inline(always)]
    pub const fn set_epena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Doep5Ctl {
    #[inline(always)]
    fn default() -> Doep5Ctl {
        Doep5Ctl(0)
    }
}
impl core::fmt::Debug for Doep5Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep5Ctl")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpideof", &self.dpideof())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pidef", &self.setd0pidef())
            .field("setd1pidof", &self.setd1pidof())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep5Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep5Ctl {{ mps: {=u16:?}, usbactep: {=bool:?}, dpideof: {=bool:?}, naksts: {=bool:?}, eptype: {:?}, snp: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, setd0pidef: {=bool:?}, setd1pidof: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mps () , self . usbactep () , self . dpideof () , self . naksts () , self . eptype () , self . snp () , self . stall () , self . cnak () , self . snak () , self . setd0pidef () , self . setd1pidof () , self . epdis () , self . epena ())
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep5Int(pub u32);
impl Doep5Int {
    #[doc = "Transfer Completed Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbld(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt."]
    #[inline(always)]
    pub const fn set_epdisbld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Phase Done."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Phase Done."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received When Endpoint Disabled."]
    #[inline(always)]
    pub const fn set_outtknepdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received For Control Write."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received For Control Write."]
    #[inline(always)]
    pub const fn set_stsphsercvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error."]
    #[inline(always)]
    pub const fn set_outpkterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Packet Drop Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktdrpsts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Drop Status."]
    #[inline(always)]
    pub const fn set_pktdrpsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bbleerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nakintrpt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_nakintrpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn stuppktrcvd(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_stuppktrcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Doep5Int {
    #[inline(always)]
    fn default() -> Doep5Int {
        Doep5Int(0)
    }
}
impl core::fmt::Debug for Doep5Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep5Int")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("setup", &self.setup())
            .field("outtknepdis", &self.outtknepdis())
            .field("stsphsercvd", &self.stsphsercvd())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterr", &self.outpkterr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("stuppktrcvd", &self.stuppktrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep5Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doep5Int {{ xfercompl: {=bool:?}, epdisbld: {=bool:?}, ahberr: {=bool:?}, setup: {=bool:?}, outtknepdis: {=bool:?}, stsphsercvd: {=bool:?}, back2backsetup: {=bool:?}, outpkterr: {=bool:?}, pktdrpsts: {=bool:?}, bbleerr: {=bool:?}, nakintrpt: {=bool:?}, stuppktrcvd: {=bool:?} }}" , self . xfercompl () , self . epdisbld () , self . ahberr () , self . setup () , self . outtknepdis () , self . stsphsercvd () , self . back2backsetup () , self . outpkterr () , self . pktdrpsts () , self . bbleerr () , self . nakintrpt () , self . stuppktrcvd ())
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doep5Tsiz(pub u32);
impl Doep5Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdpidsupcnt(&self) -> super::vals::Doep5TsizRxdpidsupcnt {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Doep5TsizRxdpidsupcnt::from_bits(val as u8)
    }
    #[doc = "Receive Data PID / SETUP Packet Count."]
    #[inline(always)]
    pub const fn set_rxdpidsupcnt(&mut self, val: super::vals::Doep5TsizRxdpidsupcnt) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Doep5Tsiz {
    #[inline(always)]
    fn default() -> Doep5Tsiz {
        Doep5Tsiz(0)
    }
}
impl core::fmt::Debug for Doep5Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doep5Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpidsupcnt", &self.rxdpidsupcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doep5Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Doep5Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, rxdpidsupcnt: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.rxdpidsupcnt()
        )
    }
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doepmsk(pub u32);
impl Doepmsk {
    #[doc = "Transfer Completed Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercomplmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Interrupt Mask."]
    #[inline(always)]
    pub const fn set_xfercomplmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Disabled Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn epdisbldmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Disabled Interrupt Mask."]
    #[inline(always)]
    pub const fn set_epdisbldmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberrmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SETUP Phase Done Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn setupmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP Phase Done Mask."]
    #[inline(always)]
    pub const fn set_setupmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OUT Token Received when Endpoint Disabled Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn outtknepdismsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Token Received when Endpoint Disabled Mask."]
    #[inline(always)]
    pub const fn set_outtknepdismsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Phase Received Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn stsphsercvdmsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Phase Received Mask."]
    #[inline(always)]
    pub const fn set_stsphsercvdmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Back-to-Back SETUP Packets Received Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn back2backsetup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back SETUP Packets Received Mask."]
    #[inline(always)]
    pub const fn set_back2backsetup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OUT Packet Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn outpkterrmsk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Packet Error Mask."]
    #[inline(always)]
    pub const fn set_outpkterrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Babble Error interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn bbleerrmsk(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error interrupt Mask."]
    #[inline(always)]
    pub const fn set_bbleerrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "NAK interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn nakmsk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "NAK interrupt Mask."]
    #[inline(always)]
    pub const fn set_nakmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Doepmsk {
    #[inline(always)]
    fn default() -> Doepmsk {
        Doepmsk(0)
    }
}
impl core::fmt::Debug for Doepmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doepmsk")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("epdisbldmsk", &self.epdisbldmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("setupmsk", &self.setupmsk())
            .field("outtknepdismsk", &self.outtknepdismsk())
            .field("stsphsercvdmsk", &self.stsphsercvdmsk())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterrmsk", &self.outpkterrmsk())
            .field("bbleerrmsk", &self.bbleerrmsk())
            .field("nakmsk", &self.nakmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doepmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Doepmsk {{ xfercomplmsk: {=bool:?}, epdisbldmsk: {=bool:?}, ahberrmsk: {=bool:?}, setupmsk: {=bool:?}, outtknepdismsk: {=bool:?}, stsphsercvdmsk: {=bool:?}, back2backsetup: {=bool:?}, outpkterrmsk: {=bool:?}, bbleerrmsk: {=bool:?}, nakmsk: {=bool:?} }}" , self . xfercomplmsk () , self . epdisbldmsk () , self . ahberrmsk () , self . setupmsk () , self . outtknepdismsk () , self . stsphsercvdmsk () , self . back2backsetup () , self . outpkterrmsk () , self . bbleerrmsk () , self . nakmsk ())
    }
}
#[doc = "Device Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsts(pub u32);
impl Dsts {
    #[doc = "Suspend Status."]
    #[must_use]
    #[inline(always)]
    pub const fn suspsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Status."]
    #[inline(always)]
    pub const fn set_suspsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enumerated Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn enumspd(&self) -> super::vals::Enumspd {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Enumspd::from_bits(val as u8)
    }
    #[doc = "Enumerated Speed."]
    #[inline(always)]
    pub const fn set_enumspd(&mut self, val: super::vals::Enumspd) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Erratic Error."]
    #[must_use]
    #[inline(always)]
    pub const fn errticerr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Erratic Error."]
    #[inline(always)]
    pub const fn set_errticerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Frame Number of the Received SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn soffn(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x3fff;
        val as u16
    }
    #[doc = "Frame Number of the Received SOF."]
    #[inline(always)]
    pub const fn set_soffn(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
    }
    #[doc = "Device Line Status."]
    #[must_use]
    #[inline(always)]
    pub const fn devlnsts(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Device Line Status."]
    #[inline(always)]
    pub const fn set_devlnsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
}
impl Default for Dsts {
    #[inline(always)]
    fn default() -> Dsts {
        Dsts(0)
    }
}
impl core::fmt::Debug for Dsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dsts")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("errticerr", &self.errticerr())
            .field("soffn", &self.soffn())
            .field("devlnsts", &self.devlnsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dsts {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dsts {{ suspsts: {=bool:?}, enumspd: {:?}, errticerr: {=bool:?}, soffn: {=u16:?}, devlnsts: {=u8:?} }}" , self . suspsts () , self . enumspd () , self . errticerr () , self . soffn () , self . devlnsts ())
    }
}
#[doc = "Device Threshold Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dthrctl(pub u32);
impl Dthrctl {
    #[doc = "Non-ISO IN Endpoints Threshold Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nonisothren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Non-ISO IN Endpoints Threshold Enable."]
    #[inline(always)]
    pub const fn set_nonisothren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ISO IN Endpoints Threshold Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn isothren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ISO IN Endpoints Threshold Enable."]
    #[inline(always)]
    pub const fn set_isothren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Threshold Length."]
    #[must_use]
    #[inline(always)]
    pub const fn txthrlen(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit Threshold Length."]
    #[inline(always)]
    pub const fn set_txthrlen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
    }
    #[doc = "AHB Threshold Ratio."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbthrratio(&self) -> super::vals::Ahbthrratio {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Ahbthrratio::from_bits(val as u8)
    }
    #[doc = "AHB Threshold Ratio."]
    #[inline(always)]
    pub const fn set_ahbthrratio(&mut self, val: super::vals::Ahbthrratio) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "Receive Threshold Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxthren(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Threshold Enable."]
    #[inline(always)]
    pub const fn set_rxthren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Threshold Length."]
    #[must_use]
    #[inline(always)]
    pub const fn rxthrlen(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive Threshold Length."]
    #[inline(always)]
    pub const fn set_rxthrlen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 17usize)) | (((val as u32) & 0x01ff) << 17usize);
    }
    #[doc = "Arbiter Parking Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn arbprken(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Arbiter Parking Enable."]
    #[inline(always)]
    pub const fn set_arbprken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Dthrctl {
    #[inline(always)]
    fn default() -> Dthrctl {
        Dthrctl(0)
    }
}
impl core::fmt::Debug for Dthrctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dthrctl")
            .field("nonisothren", &self.nonisothren())
            .field("isothren", &self.isothren())
            .field("txthrlen", &self.txthrlen())
            .field("ahbthrratio", &self.ahbthrratio())
            .field("rxthren", &self.rxthren())
            .field("rxthrlen", &self.rxthrlen())
            .field("arbprken", &self.arbprken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dthrctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dthrctl {{ nonisothren: {=bool:?}, isothren: {=bool:?}, txthrlen: {=u16:?}, ahbthrratio: {:?}, rxthren: {=bool:?}, rxthrlen: {=u16:?}, arbprken: {=bool:?} }}" , self . nonisothren () , self . isothren () , self . txthrlen () , self . ahbthrratio () , self . rxthren () , self . rxthrlen () , self . arbprken ())
    }
}
#[doc = "Device VBUS Discharge Time Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbusdis(pub u32);
impl Dvbusdis {
    #[doc = "Device VBUS Discharge Time."]
    #[must_use]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device VBUS Discharge Time."]
    #[inline(always)]
    pub const fn set_dvbusdis(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dvbusdis {
    #[inline(always)]
    fn default() -> Dvbusdis {
        Dvbusdis(0)
    }
}
impl core::fmt::Debug for Dvbusdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dvbusdis")
            .field("dvbusdis", &self.dvbusdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dvbusdis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dvbusdis {{ dvbusdis: {=u16:?} }}", self.dvbusdis())
    }
}
#[doc = "Device VBUS Pulsing Time Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbuspulse(pub u32);
impl Dvbuspulse {
    #[doc = "Device VBUS Pulsing Time."]
    #[must_use]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Device VBUS Pulsing Time."]
    #[inline(always)]
    pub const fn set_dvbuspulse(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Dvbuspulse {
    #[inline(always)]
    fn default() -> Dvbuspulse {
        Dvbuspulse(0)
    }
}
impl core::fmt::Debug for Dvbuspulse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dvbuspulse")
            .field("dvbuspulse", &self.dvbuspulse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dvbuspulse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dvbuspulse {{ dvbuspulse: {=u16:?} }}",
            self.dvbuspulse()
        )
    }
}
#[doc = "AHB Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gahbcfg(pub u32);
impl Gahbcfg {
    #[doc = "Global Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn glblintrmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt Mask."]
    #[inline(always)]
    pub const fn set_glblintrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Burst Length/Type."]
    #[must_use]
    #[inline(always)]
    pub const fn hbstlen(&self) -> super::vals::Hbstlen {
        let val = (self.0 >> 1usize) & 0x0f;
        super::vals::Hbstlen::from_bits(val as u8)
    }
    #[doc = "Burst Length/Type."]
    #[inline(always)]
    pub const fn set_hbstlen(&mut self, val: super::vals::Hbstlen) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val.to_bits() as u32) & 0x0f) << 1usize);
    }
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-Periodic TxFIFO Empty Level."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfemplvl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Periodic TxFIFO Empty Level."]
    #[inline(always)]
    pub const fn set_nptxfemplvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Periodic TxFIFO Empty Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfemplvl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic TxFIFO Empty Level."]
    #[inline(always)]
    pub const fn set_ptxfemplvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Remote Memory Support."]
    #[must_use]
    #[inline(always)]
    pub const fn remmemsupp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Memory Support."]
    #[inline(always)]
    pub const fn set_remmemsupp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Notify All Dma Write Transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn notialldmawrit(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Notify All Dma Write Transactions."]
    #[inline(always)]
    pub const fn set_notialldmawrit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "AHB Single Support."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbsingle(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Single Support."]
    #[inline(always)]
    pub const fn set_ahbsingle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gahbcfg {
    #[inline(always)]
    fn default() -> Gahbcfg {
        Gahbcfg(0)
    }
}
impl core::fmt::Debug for Gahbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gahbcfg")
            .field("glblintrmsk", &self.glblintrmsk())
            .field("hbstlen", &self.hbstlen())
            .field("dmaen", &self.dmaen())
            .field("nptxfemplvl", &self.nptxfemplvl())
            .field("ptxfemplvl", &self.ptxfemplvl())
            .field("remmemsupp", &self.remmemsupp())
            .field("notialldmawrit", &self.notialldmawrit())
            .field("ahbsingle", &self.ahbsingle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gahbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gahbcfg {{ glblintrmsk: {=bool:?}, hbstlen: {:?}, dmaen: {=bool:?}, nptxfemplvl: {=bool:?}, ptxfemplvl: {=bool:?}, remmemsupp: {=bool:?}, notialldmawrit: {=bool:?}, ahbsingle: {=bool:?} }}" , self . glblintrmsk () , self . hbstlen () , self . dmaen () , self . nptxfemplvl () , self . ptxfemplvl () , self . remmemsupp () , self . notialldmawrit () , self . ahbsingle ())
    }
}
#[doc = "Global DFIFO Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdfifocfg(pub u32);
impl Gdfifocfg {
    #[must_use]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_gdfifocfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn epinfobaseaddr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_epinfobaseaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gdfifocfg {
    #[inline(always)]
    fn default() -> Gdfifocfg {
        Gdfifocfg(0)
    }
}
impl core::fmt::Debug for Gdfifocfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gdfifocfg")
            .field("gdfifocfg", &self.gdfifocfg())
            .field("epinfobaseaddr", &self.epinfobaseaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gdfifocfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gdfifocfg {{ gdfifocfg: {=u16:?}, epinfobaseaddr: {=u16:?} }}",
            self.gdfifocfg(),
            self.epinfobaseaddr()
        )
    }
}
#[doc = "Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintmsk(pub u32);
impl Gintmsk {
    #[doc = "Mode Mismatch Interrupt Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn modemismsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Mismatch Interrupt Mask (host and device)."]
    #[inline(always)]
    pub const fn set_modemismsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OTG Interrupt Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn otgintmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Interrupt Mask (host and device)."]
    #[inline(always)]
    pub const fn set_otgintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start of Frame Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn sofmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame Mask (host and device)."]
    #[inline(always)]
    pub const fn set_sofmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO Non-Empty Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxflvlmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Non-Empty Mask (host and device)."]
    #[inline(always)]
    pub const fn set_rxflvlmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Non-Periodic TxFIFO Empty Mask (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfempmsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Periodic TxFIFO Empty Mask (host only)."]
    #[inline(always)]
    pub const fn set_nptxfempmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Non-periodic IN NAK Effective Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn ginnakeffmsk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Non-periodic IN NAK Effective Mask (device only)."]
    #[inline(always)]
    pub const fn set_ginnakeffmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global OUT NAK Effective Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn goutnakeffmsk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global OUT NAK Effective Mask (device only)."]
    #[inline(always)]
    pub const fn set_goutnakeffmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Early Suspend Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn erlysuspmsk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Suspend Mask (device only)."]
    #[inline(always)]
    pub const fn set_erlysuspmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "USB Suspend Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn usbsuspmsk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "USB Suspend Mask (device only)."]
    #[inline(always)]
    pub const fn set_usbsuspmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "USB Reset Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn usbrstmsk(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Mask (device only)."]
    #[inline(always)]
    pub const fn set_usbrstmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enumeration Done Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn enumdonemsk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enumeration Done Mask (device only)."]
    #[inline(always)]
    pub const fn set_enumdonemsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Isochronous OUT Packet Dropped Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn isooutdropmsk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Isochronous OUT Packet Dropped Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_isooutdropmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End of Periodic Frame Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn eopfmsk(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End of Periodic Frame Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_eopfmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Mismatch Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn epmismsk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Mismatch Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_epmismsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "IN Endpoints Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn iepintmsk(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoints Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_iepintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OUT Endpoints Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn oepintmsk(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoints Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_oepintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Incomplete Isochronous IN Transfer Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn incompisoinmsk(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Incomplete Isochronous IN Transfer Mask (device only)."]
    #[inline(always)]
    pub const fn set_incompisoinmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Incomplete Periodic Transfer Mask (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn incomplpmsk(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Incomplete Periodic Transfer Mask (host only)."]
    #[inline(always)]
    pub const fn set_incomplpmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Fetch Suspended Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn fetsuspmsk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Fetch Suspended Mask (device only)."]
    #[inline(always)]
    pub const fn set_fetsuspmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reset detected Interrupt Mask (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn resetdetmsk(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reset detected Interrupt Mask (device only)."]
    #[inline(always)]
    pub const fn set_resetdetmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Host Port Interrupt Mask (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn prtintmsk(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Host Port Interrupt Mask (host only)."]
    #[inline(always)]
    pub const fn set_prtintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Host Channels Interrupt Mask (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn hchintmsk(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Host Channels Interrupt Mask (host only)."]
    #[inline(always)]
    pub const fn set_hchintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Periodic TxFIFO Empty Mask (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfempmsk(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic TxFIFO Empty Mask (host only)."]
    #[inline(always)]
    pub const fn set_ptxfempmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Connector ID Status Change Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn conidstschngmsk(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Connector ID Status Change Mask (host and device)."]
    #[inline(always)]
    pub const fn set_conidstschngmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Disconnect Detected Interrupt Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn disconnintmsk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Disconnect Detected Interrupt Mask (host and device)."]
    #[inline(always)]
    pub const fn set_disconnintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Session Request/New Session Detected Interrupt Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn sessreqintmsk(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Session Request/New Session Detected Interrupt Mask (host and device)."]
    #[inline(always)]
    pub const fn set_sessreqintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Resume/Remote Wakeup Detected Interrupt Mask (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupintmsk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Resume/Remote Wakeup Detected Interrupt Mask (host and device)."]
    #[inline(always)]
    pub const fn set_wkupintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gintmsk {
    #[inline(always)]
    fn default() -> Gintmsk {
        Gintmsk(0)
    }
}
impl core::fmt::Debug for Gintmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gintmsk")
            .field("modemismsk", &self.modemismsk())
            .field("otgintmsk", &self.otgintmsk())
            .field("sofmsk", &self.sofmsk())
            .field("rxflvlmsk", &self.rxflvlmsk())
            .field("nptxfempmsk", &self.nptxfempmsk())
            .field("ginnakeffmsk", &self.ginnakeffmsk())
            .field("goutnakeffmsk", &self.goutnakeffmsk())
            .field("erlysuspmsk", &self.erlysuspmsk())
            .field("usbsuspmsk", &self.usbsuspmsk())
            .field("usbrstmsk", &self.usbrstmsk())
            .field("enumdonemsk", &self.enumdonemsk())
            .field("isooutdropmsk", &self.isooutdropmsk())
            .field("eopfmsk", &self.eopfmsk())
            .field("epmismsk", &self.epmismsk())
            .field("iepintmsk", &self.iepintmsk())
            .field("oepintmsk", &self.oepintmsk())
            .field("incompisoinmsk", &self.incompisoinmsk())
            .field("incomplpmsk", &self.incomplpmsk())
            .field("fetsuspmsk", &self.fetsuspmsk())
            .field("resetdetmsk", &self.resetdetmsk())
            .field("prtintmsk", &self.prtintmsk())
            .field("hchintmsk", &self.hchintmsk())
            .field("ptxfempmsk", &self.ptxfempmsk())
            .field("conidstschngmsk", &self.conidstschngmsk())
            .field("disconnintmsk", &self.disconnintmsk())
            .field("sessreqintmsk", &self.sessreqintmsk())
            .field("wkupintmsk", &self.wkupintmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gintmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gintmsk {{ modemismsk: {=bool:?}, otgintmsk: {=bool:?}, sofmsk: {=bool:?}, rxflvlmsk: {=bool:?}, nptxfempmsk: {=bool:?}, ginnakeffmsk: {=bool:?}, goutnakeffmsk: {=bool:?}, erlysuspmsk: {=bool:?}, usbsuspmsk: {=bool:?}, usbrstmsk: {=bool:?}, enumdonemsk: {=bool:?}, isooutdropmsk: {=bool:?}, eopfmsk: {=bool:?}, epmismsk: {=bool:?}, iepintmsk: {=bool:?}, oepintmsk: {=bool:?}, incompisoinmsk: {=bool:?}, incomplpmsk: {=bool:?}, fetsuspmsk: {=bool:?}, resetdetmsk: {=bool:?}, prtintmsk: {=bool:?}, hchintmsk: {=bool:?}, ptxfempmsk: {=bool:?}, conidstschngmsk: {=bool:?}, disconnintmsk: {=bool:?}, sessreqintmsk: {=bool:?}, wkupintmsk: {=bool:?} }}" , self . modemismsk () , self . otgintmsk () , self . sofmsk () , self . rxflvlmsk () , self . nptxfempmsk () , self . ginnakeffmsk () , self . goutnakeffmsk () , self . erlysuspmsk () , self . usbsuspmsk () , self . usbrstmsk () , self . enumdonemsk () , self . isooutdropmsk () , self . eopfmsk () , self . epmismsk () , self . iepintmsk () , self . oepintmsk () , self . incompisoinmsk () , self . incomplpmsk () , self . fetsuspmsk () , self . resetdetmsk () , self . prtintmsk () , self . hchintmsk () , self . ptxfempmsk () , self . conidstschngmsk () , self . disconnintmsk () , self . sessreqintmsk () , self . wkupintmsk ())
    }
}
#[doc = "Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintsts(pub u32);
impl Gintsts {
    #[doc = "Current Mode of Operation (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn curmod(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Mode of Operation (host and device)."]
    #[inline(always)]
    pub const fn set_curmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode Mismatch Interrupt (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn modemis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Mismatch Interrupt (host and device)."]
    #[inline(always)]
    pub const fn set_modemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OTG Interrupt (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn otgint(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Interrupt (host and device)."]
    #[inline(always)]
    pub const fn set_otgint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start of Frame (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame (host and device)."]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RxFIFO Non-Empty (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxflvl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RxFIFO Non-Empty (host and device)."]
    #[inline(always)]
    pub const fn set_rxflvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Non-Periodic TxFIFO Empty (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfemp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Periodic TxFIFO Empty (host only)."]
    #[inline(always)]
    pub const fn set_nptxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Global IN Non-periodic NAK Effective (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn ginnakeff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global IN Non-periodic NAK Effective (device only)."]
    #[inline(always)]
    pub const fn set_ginnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global OUT NAK Effective (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn goutnakeff(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global OUT NAK Effective (device only)."]
    #[inline(always)]
    pub const fn set_goutnakeff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Early Suspend (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn erlysusp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Suspend (device only)."]
    #[inline(always)]
    pub const fn set_erlysusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "USB Suspend (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn usbsusp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "USB Suspend (device only)."]
    #[inline(always)]
    pub const fn set_usbsusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "USB Reset (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn usbrst(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset (device only)."]
    #[inline(always)]
    pub const fn set_usbrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enumeration Done (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn enumdone(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enumeration Done (device only)."]
    #[inline(always)]
    pub const fn set_enumdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Isochronous OUT Packet Dropped Interrupt (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn isooutdrop(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Isochronous OUT Packet Dropped Interrupt (device only)."]
    #[inline(always)]
    pub const fn set_isooutdrop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "End of Periodic Frame Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eopf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "End of Periodic Frame Interrupt."]
    #[inline(always)]
    pub const fn set_eopf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Endpoint Mismatch Interrupt (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn epmis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Mismatch Interrupt (device only)."]
    #[inline(always)]
    pub const fn set_epmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "IN Endpoints Interrupt (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn iepint(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoints Interrupt (device only)."]
    #[inline(always)]
    pub const fn set_iepint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OUT Endpoints Interrupt (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn oepint(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OUT Endpoints Interrupt (device only)."]
    #[inline(always)]
    pub const fn set_oepint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Incomplete Isochronous IN Transfer (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn incompisoin(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Incomplete Isochronous IN Transfer (device only)."]
    #[inline(always)]
    pub const fn set_incompisoin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Incomplete Periodic Transfer (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn incomplp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Incomplete Periodic Transfer (device only)."]
    #[inline(always)]
    pub const fn set_incomplp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Fetch Suspended (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn fetsusp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Fetch Suspended (device only)."]
    #[inline(always)]
    pub const fn set_fetsusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reset detected Interrupt (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn resetdet(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reset detected Interrupt (device only)."]
    #[inline(always)]
    pub const fn set_resetdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Host Port Interrupt (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn prtint(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Host Port Interrupt (host only)."]
    #[inline(always)]
    pub const fn set_prtint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Host Channels Interrupt (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn hchint(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Host Channels Interrupt (host only)."]
    #[inline(always)]
    pub const fn set_hchint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Periodic TxFIFO Empty (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfemp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic TxFIFO Empty (host only)."]
    #[inline(always)]
    pub const fn set_ptxfemp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Connector ID Status Change (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn conidstschng(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Connector ID Status Change (host and device)."]
    #[inline(always)]
    pub const fn set_conidstschng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Disconnect Detected Interrupt (host only)."]
    #[must_use]
    #[inline(always)]
    pub const fn disconnint(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Disconnect Detected Interrupt (host only)."]
    #[inline(always)]
    pub const fn set_disconnint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Session Request/New Session Detected Interrupt (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn sessreqint(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Session Request/New Session Detected Interrupt (host and device)."]
    #[inline(always)]
    pub const fn set_sessreqint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Resume/Remote Wakeup Detected Interrupt (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupint(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Resume/Remote Wakeup Detected Interrupt (host and device)."]
    #[inline(always)]
    pub const fn set_wkupint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gintsts {
    #[inline(always)]
    fn default() -> Gintsts {
        Gintsts(0)
    }
}
impl core::fmt::Debug for Gintsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gintsts")
            .field("curmod", &self.curmod())
            .field("modemis", &self.modemis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfemp", &self.nptxfemp())
            .field("ginnakeff", &self.ginnakeff())
            .field("goutnakeff", &self.goutnakeff())
            .field("erlysusp", &self.erlysusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdone", &self.enumdone())
            .field("isooutdrop", &self.isooutdrop())
            .field("eopf", &self.eopf())
            .field("epmis", &self.epmis())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("incompisoin", &self.incompisoin())
            .field("incomplp", &self.incomplp())
            .field("fetsusp", &self.fetsusp())
            .field("resetdet", &self.resetdet())
            .field("prtint", &self.prtint())
            .field("hchint", &self.hchint())
            .field("ptxfemp", &self.ptxfemp())
            .field("conidstschng", &self.conidstschng())
            .field("disconnint", &self.disconnint())
            .field("sessreqint", &self.sessreqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gintsts {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gintsts {{ curmod: {=bool:?}, modemis: {=bool:?}, otgint: {=bool:?}, sof: {=bool:?}, rxflvl: {=bool:?}, nptxfemp: {=bool:?}, ginnakeff: {=bool:?}, goutnakeff: {=bool:?}, erlysusp: {=bool:?}, usbsusp: {=bool:?}, usbrst: {=bool:?}, enumdone: {=bool:?}, isooutdrop: {=bool:?}, eopf: {=bool:?}, epmis: {=bool:?}, iepint: {=bool:?}, oepint: {=bool:?}, incompisoin: {=bool:?}, incomplp: {=bool:?}, fetsusp: {=bool:?}, resetdet: {=bool:?}, prtint: {=bool:?}, hchint: {=bool:?}, ptxfemp: {=bool:?}, conidstschng: {=bool:?}, disconnint: {=bool:?}, sessreqint: {=bool:?}, wkupint: {=bool:?} }}" , self . curmod () , self . modemis () , self . otgint () , self . sof () , self . rxflvl () , self . nptxfemp () , self . ginnakeff () , self . goutnakeff () , self . erlysusp () , self . usbsusp () , self . usbrst () , self . enumdone () , self . isooutdrop () , self . eopf () , self . epmis () , self . iepint () , self . oepint () , self . incompisoin () , self . incomplp () , self . fetsusp () , self . resetdet () , self . prtint () , self . hchint () , self . ptxfemp () , self . conidstschng () , self . disconnint () , self . sessreqint () , self . wkupint ())
    }
}
#[doc = "Non-periodic Transmit FIFO Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gnptxfsiz(pub u32);
impl Gnptxfsiz {
    #[doc = "Non-periodic Transmit RAM Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Non-periodic Transmit RAM Start Address."]
    #[inline(always)]
    pub const fn set_nptxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Non-periodic TxFIFO Depth (host only) / IN Endpoint TxFIFO 0 Depth (device only)."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfineptxf0dep(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Non-periodic TxFIFO Depth (host only) / IN Endpoint TxFIFO 0 Depth (device only)."]
    #[inline(always)]
    pub const fn set_nptxfineptxf0dep(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gnptxfsiz {
    #[inline(always)]
    fn default() -> Gnptxfsiz {
        Gnptxfsiz(0)
    }
}
impl core::fmt::Debug for Gnptxfsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gnptxfsiz")
            .field("nptxfstaddr", &self.nptxfstaddr())
            .field("nptxfineptxf0dep", &self.nptxfineptxf0dep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gnptxfsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gnptxfsiz {{ nptxfstaddr: {=u16:?}, nptxfineptxf0dep: {=u16:?} }}",
            self.nptxfstaddr(),
            self.nptxfineptxf0dep()
        )
    }
}
#[doc = "Non-periodic Transmit FIFO/Queue Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gnptxsts(pub u32);
impl Gnptxsts {
    #[doc = "Non-periodic TxFIFO Space Avail."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxfspcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Non-periodic TxFIFO Space Avail."]
    #[inline(always)]
    pub const fn set_nptxfspcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Non-periodic Transmit Request Queue Space Available."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxqspcavail(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Non-periodic Transmit Request Queue Space Available."]
    #[inline(always)]
    pub const fn set_nptxqspcavail(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Top of the Non-periodic Transmit Request Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn nptxqtop(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Top of the Non-periodic Transmit Request Queue."]
    #[inline(always)]
    pub const fn set_nptxqtop(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Gnptxsts {
    #[inline(always)]
    fn default() -> Gnptxsts {
        Gnptxsts(0)
    }
}
impl core::fmt::Debug for Gnptxsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gnptxsts")
            .field("nptxfspcavail", &self.nptxfspcavail())
            .field("nptxqspcavail", &self.nptxqspcavail())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gnptxsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gnptxsts {{ nptxfspcavail: {=u16:?}, nptxqspcavail: {=u8:?}, nptxqtop: {=u8:?} }}",
            self.nptxfspcavail(),
            self.nptxqspcavail(),
            self.nptxqtop()
        )
    }
}
#[doc = "OTG Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgctl(pub u32);
impl Gotgctl {
    #[doc = "Session Request Success."]
    #[must_use]
    #[inline(always)]
    pub const fn sesreqscs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session Request Success."]
    #[inline(always)]
    pub const fn set_sesreqscs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Session Request."]
    #[must_use]
    #[inline(always)]
    pub const fn sesreq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Session Request."]
    #[inline(always)]
    pub const fn set_sesreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VBUS Valid Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbvalidoven(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Valid Override Enable."]
    #[inline(always)]
    pub const fn set_vbvalidoven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Valid OverrideValue."]
    #[must_use]
    #[inline(always)]
    pub const fn vbvalidovval(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Valid OverrideValue."]
    #[inline(always)]
    pub const fn set_vbvalidovval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "A-Peripheral Session Valid Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn avalidoven(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "A-Peripheral Session Valid Override Enable."]
    #[inline(always)]
    pub const fn set_avalidoven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "A-Peripheral Session Valid OverrideValue."]
    #[must_use]
    #[inline(always)]
    pub const fn avalidovval(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "A-Peripheral Session Valid OverrideValue."]
    #[inline(always)]
    pub const fn set_avalidovval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "B-Peripheral Session Valid Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalidoven(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "B-Peripheral Session Valid Override Enable."]
    #[inline(always)]
    pub const fn set_bvalidoven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "B-Peripheral Session Valid OverrideValue."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalidovval(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "B-Peripheral Session Valid OverrideValue."]
    #[inline(always)]
    pub const fn set_bvalidovval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Host Negotiation Success."]
    #[must_use]
    #[inline(always)]
    pub const fn hstnegscs(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Host Negotiation Success."]
    #[inline(always)]
    pub const fn set_hstnegscs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HNP Request."]
    #[must_use]
    #[inline(always)]
    pub const fn hnpreq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HNP Request."]
    #[inline(always)]
    pub const fn set_hnpreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Host Set HNP Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hstsethnpen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Host Set HNP Enable."]
    #[inline(always)]
    pub const fn set_hstsethnpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Device HNP Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn devhnpen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Device HNP Enabled."]
    #[inline(always)]
    pub const fn set_devhnpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Embedded Host Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ehen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Embedded Host Enable."]
    #[inline(always)]
    pub const fn set_ehen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Debounce Filter Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn dbncefltrbypass(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debounce Filter Bypass."]
    #[inline(always)]
    pub const fn set_dbncefltrbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Connector ID Status."]
    #[must_use]
    #[inline(always)]
    pub const fn conidsts(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Connector ID Status."]
    #[inline(always)]
    pub const fn set_conidsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Long/Short Debounce Time."]
    #[must_use]
    #[inline(always)]
    pub const fn dbnctime(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Long/Short Debounce Time."]
    #[inline(always)]
    pub const fn set_dbnctime(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A-Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn asesvld(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A-Session Valid."]
    #[inline(always)]
    pub const fn set_asesvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "B-Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bsesvld(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "B-Session Valid."]
    #[inline(always)]
    pub const fn set_bsesvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OTG Version."]
    #[must_use]
    #[inline(always)]
    pub const fn otgver(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Version."]
    #[inline(always)]
    pub const fn set_otgver(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Current Mode of Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn curmod(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Current Mode of Operation."]
    #[inline(always)]
    pub const fn set_curmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Gotgctl {
    #[inline(always)]
    fn default() -> Gotgctl {
        Gotgctl(0)
    }
}
impl core::fmt::Debug for Gotgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gotgctl")
            .field("sesreqscs", &self.sesreqscs())
            .field("sesreq", &self.sesreq())
            .field("vbvalidoven", &self.vbvalidoven())
            .field("vbvalidovval", &self.vbvalidovval())
            .field("avalidoven", &self.avalidoven())
            .field("avalidovval", &self.avalidovval())
            .field("bvalidoven", &self.bvalidoven())
            .field("bvalidovval", &self.bvalidovval())
            .field("hstnegscs", &self.hstnegscs())
            .field("hnpreq", &self.hnpreq())
            .field("hstsethnpen", &self.hstsethnpen())
            .field("devhnpen", &self.devhnpen())
            .field("ehen", &self.ehen())
            .field("dbncefltrbypass", &self.dbncefltrbypass())
            .field("conidsts", &self.conidsts())
            .field("dbnctime", &self.dbnctime())
            .field("asesvld", &self.asesvld())
            .field("bsesvld", &self.bsesvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gotgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gotgctl {{ sesreqscs: {=bool:?}, sesreq: {=bool:?}, vbvalidoven: {=bool:?}, vbvalidovval: {=bool:?}, avalidoven: {=bool:?}, avalidovval: {=bool:?}, bvalidoven: {=bool:?}, bvalidovval: {=bool:?}, hstnegscs: {=bool:?}, hnpreq: {=bool:?}, hstsethnpen: {=bool:?}, devhnpen: {=bool:?}, ehen: {=bool:?}, dbncefltrbypass: {=bool:?}, conidsts: {=bool:?}, dbnctime: {=bool:?}, asesvld: {=bool:?}, bsesvld: {=bool:?}, otgver: {=bool:?}, curmod: {=bool:?} }}" , self . sesreqscs () , self . sesreq () , self . vbvalidoven () , self . vbvalidovval () , self . avalidoven () , self . avalidovval () , self . bvalidoven () , self . bvalidovval () , self . hstnegscs () , self . hnpreq () , self . hstsethnpen () , self . devhnpen () , self . ehen () , self . dbncefltrbypass () , self . conidsts () , self . dbnctime () , self . asesvld () , self . bsesvld () , self . otgver () , self . curmod ())
    }
}
#[doc = "OTG Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgint(pub u32);
impl Gotgint {
    #[doc = "Session End Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sesenddet(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Detected."]
    #[inline(always)]
    pub const fn set_sesenddet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Session Request Success Status Change."]
    #[must_use]
    #[inline(always)]
    pub const fn sesreqsucstschng(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Session Request Success Status Change."]
    #[inline(always)]
    pub const fn set_sesreqsucstschng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Host Negotiation Success Status Change."]
    #[must_use]
    #[inline(always)]
    pub const fn hstnegsucstschng(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Host Negotiation Success Status Change."]
    #[inline(always)]
    pub const fn set_hstnegsucstschng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Host Negotiation Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn hstnegdet(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Host Negotiation Detected."]
    #[inline(always)]
    pub const fn set_hstnegdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A-Device Timeout Change."]
    #[must_use]
    #[inline(always)]
    pub const fn adevtoutchg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Timeout Change."]
    #[inline(always)]
    pub const fn set_adevtoutchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Debounce Done."]
    #[must_use]
    #[inline(always)]
    pub const fn dbncedone(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Debounce Done."]
    #[inline(always)]
    pub const fn set_dbncedone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Gotgint {
    #[inline(always)]
    fn default() -> Gotgint {
        Gotgint(0)
    }
}
impl core::fmt::Debug for Gotgint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gotgint")
            .field("sesenddet", &self.sesenddet())
            .field("sesreqsucstschng", &self.sesreqsucstschng())
            .field("hstnegsucstschng", &self.hstnegsucstschng())
            .field("hstnegdet", &self.hstnegdet())
            .field("adevtoutchg", &self.adevtoutchg())
            .field("dbncedone", &self.dbncedone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gotgint {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gotgint {{ sesenddet: {=bool:?}, sesreqsucstschng: {=bool:?}, hstnegsucstschng: {=bool:?}, hstnegdet: {=bool:?}, adevtoutchg: {=bool:?}, dbncedone: {=bool:?} }}" , self . sesenddet () , self . sesreqsucstschng () , self . hstnegsucstschng () , self . hstnegdet () , self . adevtoutchg () , self . dbncedone ())
    }
}
#[doc = "Reset Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grstctl(pub u32);
impl Grstctl {
    #[doc = "Core Soft Reset (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn csftrst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Core Soft Reset (host and device)."]
    #[inline(always)]
    pub const fn set_csftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PIU FS Dedicated Controller Soft Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn piufssftrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PIU FS Dedicated Controller Soft Reset."]
    #[inline(always)]
    pub const fn set_piufssftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Frame Counter Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn frmcntrrst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host Frame Counter Reset."]
    #[inline(always)]
    pub const fn set_frmcntrrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RxFIFO Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfflsh(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RxFIFO Flush."]
    #[inline(always)]
    pub const fn set_rxfflsh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TxFIFO Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn txfflsh(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TxFIFO Flush."]
    #[inline(always)]
    pub const fn set_txfflsh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TxFIFO Number (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn txfnum(&self) -> super::vals::Txfnum {
        let val = (self.0 >> 6usize) & 0x1f;
        super::vals::Txfnum::from_bits(val as u8)
    }
    #[doc = "TxFIFO Number (host and device)."]
    #[inline(always)]
    pub const fn set_txfnum(&mut self, val: super::vals::Txfnum) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
    #[doc = "DMA Request Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn dmareq(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Signal."]
    #[inline(always)]
    pub const fn set_dmareq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Master Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbidle(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Master Idle."]
    #[inline(always)]
    pub const fn set_ahbidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Grstctl {
    #[inline(always)]
    fn default() -> Grstctl {
        Grstctl(0)
    }
}
impl core::fmt::Debug for Grstctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Grstctl")
            .field("csftrst", &self.csftrst())
            .field("piufssftrst", &self.piufssftrst())
            .field("frmcntrrst", &self.frmcntrrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("dmareq", &self.dmareq())
            .field("ahbidle", &self.ahbidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Grstctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Grstctl {{ csftrst: {=bool:?}, piufssftrst: {=bool:?}, frmcntrrst: {=bool:?}, rxfflsh: {=bool:?}, txfflsh: {=bool:?}, txfnum: {:?}, dmareq: {=bool:?}, ahbidle: {=bool:?} }}" , self . csftrst () , self . piufssftrst () , self . frmcntrrst () , self . rxfflsh () , self . txfflsh () , self . txfnum () , self . dmareq () , self . ahbidle ())
    }
}
#[doc = "Receive FIFO Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grxfsiz(pub u32);
impl Grxfsiz {
    #[doc = "RxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfdep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "RxFIFO Depth."]
    #[inline(always)]
    pub const fn set_rxfdep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Grxfsiz {
    #[inline(always)]
    fn default() -> Grxfsiz {
        Grxfsiz(0)
    }
}
impl core::fmt::Debug for Grxfsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Grxfsiz")
            .field("rxfdep", &self.rxfdep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Grxfsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Grxfsiz {{ rxfdep: {=u16:?} }}", self.rxfdep())
    }
}
#[doc = "Receive Status Read /Pop Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grxstsp(pub u32);
impl Grxstsp {
    #[doc = "Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn chnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub const fn set_chnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Byte Count."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte Count."]
    #[inline(always)]
    pub const fn set_bcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Data PID (host or device)."]
    #[must_use]
    #[inline(always)]
    pub const fn dpid(&self) -> super::vals::GrxstspDpid {
        let val = (self.0 >> 15usize) & 0x03;
        super::vals::GrxstspDpid::from_bits(val as u8)
    }
    #[doc = "Data PID (host or device)."]
    #[inline(always)]
    pub const fn set_dpid(&mut self, val: super::vals::GrxstspDpid) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
    }
    #[doc = "Packet Status (host or device)."]
    #[must_use]
    #[inline(always)]
    pub const fn pktsts(&self) -> super::vals::GrxstspPktsts {
        let val = (self.0 >> 17usize) & 0x0f;
        super::vals::GrxstspPktsts::from_bits(val as u8)
    }
    #[doc = "Packet Status (host or device)."]
    #[inline(always)]
    pub const fn set_pktsts(&mut self, val: super::vals::GrxstspPktsts) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Frame Number."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "Frame Number."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for Grxstsp {
    #[inline(always)]
    fn default() -> Grxstsp {
        Grxstsp(0)
    }
}
impl core::fmt::Debug for Grxstsp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Grxstsp")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("fn_", &self.fn_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Grxstsp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Grxstsp {{ chnum: {=u8:?}, bcnt: {=u16:?}, dpid: {:?}, pktsts: {:?}, fn_: {=u8:?} }}",
            self.chnum(),
            self.bcnt(),
            self.dpid(),
            self.pktsts(),
            self.fn_()
        )
    }
}
#[doc = "Receive Status Debug Read Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grxstsr(pub u32);
impl Grxstsr {
    #[doc = "Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn chnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub const fn set_chnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Byte Count."]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte Count."]
    #[inline(always)]
    pub const fn set_bcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Data PID."]
    #[must_use]
    #[inline(always)]
    pub const fn dpid(&self) -> super::vals::GrxstsrDpid {
        let val = (self.0 >> 15usize) & 0x03;
        super::vals::GrxstsrDpid::from_bits(val as u8)
    }
    #[doc = "Data PID."]
    #[inline(always)]
    pub const fn set_dpid(&mut self, val: super::vals::GrxstsrDpid) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
    }
    #[doc = "Packet Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pktsts(&self) -> super::vals::GrxstsrPktsts {
        let val = (self.0 >> 17usize) & 0x0f;
        super::vals::GrxstsrPktsts::from_bits(val as u8)
    }
    #[doc = "Packet Status."]
    #[inline(always)]
    pub const fn set_pktsts(&mut self, val: super::vals::GrxstsrPktsts) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Frame Number."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "Frame Number."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for Grxstsr {
    #[inline(always)]
    fn default() -> Grxstsr {
        Grxstsr(0)
    }
}
impl core::fmt::Debug for Grxstsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Grxstsr")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("fn_", &self.fn_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Grxstsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Grxstsr {{ chnum: {=u8:?}, bcnt: {=u16:?}, dpid: {:?}, pktsts: {:?}, fn_: {=u8:?} }}",
            self.chnum(),
            self.bcnt(),
            self.dpid(),
            self.pktsts(),
            self.fn_()
        )
    }
}
#[doc = "USB Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gusbcfg(pub u32);
impl Gusbcfg {
    #[doc = "Timeout Calibration (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn toutcal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Timeout Calibration (host and device)."]
    #[inline(always)]
    pub const fn set_toutcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Full-Speed Serial Interface Select."]
    #[must_use]
    #[inline(always)]
    pub const fn fsintf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Full-Speed Serial Interface Select."]
    #[inline(always)]
    pub const fn set_fsintf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SRP-Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn srpcap(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SRP-Capable."]
    #[inline(always)]
    pub const fn set_srpcap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HNP-Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn hnpcap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HNP-Capable."]
    #[inline(always)]
    pub const fn set_hnpcap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "USB Turnaround Time."]
    #[must_use]
    #[inline(always)]
    pub const fn usbtrdtim(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "USB Turnaround Time."]
    #[inline(always)]
    pub const fn set_usbtrdtim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "TermSel DLine Pulsing Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn termseldlpulse(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TermSel DLine Pulsing Selection."]
    #[inline(always)]
    pub const fn set_termseldlpulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Tx End Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn txenddelay(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Tx End Delay."]
    #[inline(always)]
    pub const fn set_txenddelay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Force Host Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcehstmode(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Force Host Mode."]
    #[inline(always)]
    pub const fn set_forcehstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Force Device Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcedevmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Force Device Mode."]
    #[inline(always)]
    pub const fn set_forcedevmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Corrupt Tx packet (host and device)."]
    #[must_use]
    #[inline(always)]
    pub const fn corrupttxpkt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Corrupt Tx packet (host and device)."]
    #[inline(always)]
    pub const fn set_corrupttxpkt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gusbcfg {
    #[inline(always)]
    fn default() -> Gusbcfg {
        Gusbcfg(0)
    }
}
impl core::fmt::Debug for Gusbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gusbcfg")
            .field("toutcal", &self.toutcal())
            .field("fsintf", &self.fsintf())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("termseldlpulse", &self.termseldlpulse())
            .field("txenddelay", &self.txenddelay())
            .field("forcehstmode", &self.forcehstmode())
            .field("forcedevmode", &self.forcedevmode())
            .field("corrupttxpkt", &self.corrupttxpkt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gusbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Gusbcfg {{ toutcal: {=u8:?}, fsintf: {=bool:?}, srpcap: {=bool:?}, hnpcap: {=bool:?}, usbtrdtim: {=u8:?}, termseldlpulse: {=bool:?}, txenddelay: {=bool:?}, forcehstmode: {=bool:?}, forcedevmode: {=bool:?}, corrupttxpkt: {=bool:?} }}" , self . toutcal () , self . fsintf () , self . srpcap () , self . hnpcap () , self . usbtrdtim () , self . termseldlpulse () , self . txenddelay () , self . forcehstmode () , self . forcedevmode () , self . corrupttxpkt ())
    }
}
#[doc = "Host All Channels Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haint(pub u32);
impl Haint {
    #[doc = "Channel Interrupt for channel 0 - 13."]
    #[must_use]
    #[inline(always)]
    pub const fn haint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Channel Interrupt for channel 0 - 13."]
    #[inline(always)]
    pub const fn set_haint(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Haint {
    #[inline(always)]
    fn default() -> Haint {
        Haint(0)
    }
}
impl core::fmt::Debug for Haint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haint")
            .field("haint", &self.haint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Haint {{ haint: {=u16:?} }}", self.haint())
    }
}
#[doc = "Host All Channels Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haintmsk(pub u32);
impl Haintmsk {
    #[doc = "Channel Interrupt Mask for channel 0 - 13."]
    #[must_use]
    #[inline(always)]
    pub const fn haintmsk(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Channel Interrupt Mask for channel 0 - 13."]
    #[inline(always)]
    pub const fn set_haintmsk(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Haintmsk {
    #[inline(always)]
    fn default() -> Haintmsk {
        Haintmsk(0)
    }
}
impl core::fmt::Debug for Haintmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haintmsk")
            .field("haintmsk", &self.haintmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haintmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Haintmsk {{ haintmsk: {=u16:?} }}", self.haintmsk())
    }
}
#[doc = "Host Channel x Characteristics Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0Char(pub u32);
impl Hc0Char {
    #[doc = "Maximum Packet Size."]
    #[must_use]
    #[inline(always)]
    pub const fn mps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Maximum Packet Size."]
    #[inline(always)]
    pub const fn set_mps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Endpoint Number."]
    #[must_use]
    #[inline(always)]
    pub const fn epnum(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint Number."]
    #[inline(always)]
    pub const fn set_epnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Endpoint Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn epdir(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Direction."]
    #[inline(always)]
    pub const fn set_epdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Low-Speed Device."]
    #[must_use]
    #[inline(always)]
    pub const fn lspddev(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Speed Device."]
    #[inline(always)]
    pub const fn set_lspddev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn eptype(&self) -> super::vals::Hc0CharEptype {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Hc0CharEptype::from_bits(val as u8)
    }
    #[doc = "Endpoint Type."]
    #[inline(always)]
    pub const fn set_eptype(&mut self, val: super::vals::Hc0CharEptype) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Multi Count (MC) / Error Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Multi Count (MC) / Error Count."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn devaddr(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn set_devaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
    }
    #[doc = "Odd Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn oddfrm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Odd Frame."]
    #[inline(always)]
    pub const fn set_oddfrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Channel Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn chdis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Disable."]
    #[inline(always)]
    pub const fn set_chdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Enable."]
    #[inline(always)]
    pub const fn set_chena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hc0Char {
    #[inline(always)]
    fn default() -> Hc0Char {
        Hc0Char(0)
    }
}
impl core::fmt::Debug for Hc0Char {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0Char")
            .field("mps", &self.mps())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lspddev", &self.lspddev())
            .field("eptype", &self.eptype())
            .field("mc", &self.mc())
            .field("devaddr", &self.devaddr())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0Char {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hc0Char {{ mps: {=u16:?}, epnum: {=u8:?}, epdir: {=bool:?}, lspddev: {=bool:?}, eptype: {:?}, mc: {=u8:?}, devaddr: {=u8:?}, oddfrm: {=bool:?}, chdis: {=bool:?}, chena: {=bool:?} }}" , self . mps () , self . epnum () , self . epdir () , self . lspddev () , self . eptype () , self . mc () , self . devaddr () , self . oddfrm () , self . chdis () , self . chena ())
    }
}
#[doc = "Host Channel x Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0Int(pub u32);
impl Hc0Int {
    #[doc = "Transfer Completed."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercompl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed."]
    #[inline(always)]
    pub const fn set_xfercompl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel Halted."]
    #[must_use]
    #[inline(always)]
    pub const fn chhltd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Halted."]
    #[inline(always)]
    pub const fn set_chhltd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error."]
    #[inline(always)]
    pub const fn set_ahberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STALL Response Received Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Response Received Interrupt."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NAK Response Received Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn nak(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Response Received Interrupt."]
    #[inline(always)]
    pub const fn set_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ACK Response Received/Transmitted Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Response Received/Transmitted Interrupt."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transaction Error."]
    #[must_use]
    #[inline(always)]
    pub const fn xacterr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction Error."]
    #[inline(always)]
    pub const fn set_xacterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Babble Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bblerr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error."]
    #[inline(always)]
    pub const fn set_bblerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Overrun."]
    #[must_use]
    #[inline(always)]
    pub const fn frmovrun(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Overrun."]
    #[inline(always)]
    pub const fn set_frmovrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data Toggle Error."]
    #[must_use]
    #[inline(always)]
    pub const fn datatglerr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data Toggle Error."]
    #[inline(always)]
    pub const fn set_datatglerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Hc0Int {
    #[inline(always)]
    fn default() -> Hc0Int {
        Hc0Int(0)
    }
}
impl core::fmt::Debug for Hc0Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0Int")
            .field("xfercompl", &self.xfercompl())
            .field("chhltd", &self.chhltd())
            .field("ahberr", &self.ahberr())
            .field("stall", &self.stall())
            .field("nak", &self.nak())
            .field("ack", &self.ack())
            .field("xacterr", &self.xacterr())
            .field("bblerr", &self.bblerr())
            .field("frmovrun", &self.frmovrun())
            .field("datatglerr", &self.datatglerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hc0Int {{ xfercompl: {=bool:?}, chhltd: {=bool:?}, ahberr: {=bool:?}, stall: {=bool:?}, nak: {=bool:?}, ack: {=bool:?}, xacterr: {=bool:?}, bblerr: {=bool:?}, frmovrun: {=bool:?}, datatglerr: {=bool:?} }}" , self . xfercompl () , self . chhltd () , self . ahberr () , self . stall () , self . nak () , self . ack () , self . xacterr () , self . bblerr () , self . frmovrun () , self . datatglerr ())
    }
}
#[doc = "Host Channel x Interrupt Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0Intmsk(pub u32);
impl Hc0Intmsk {
    #[doc = "Transfer Completed Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercomplmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Completed Mask."]
    #[inline(always)]
    pub const fn set_xfercomplmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel Halted Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn chhltdmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Halted Mask."]
    #[inline(always)]
    pub const fn set_chhltdmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ahberrmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error Mask."]
    #[inline(always)]
    pub const fn set_ahberrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STALL Response Received Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn stallmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "STALL Response Received Interrupt Mask."]
    #[inline(always)]
    pub const fn set_stallmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NAK Response Received Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn nakmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Response Received Interrupt Mask."]
    #[inline(always)]
    pub const fn set_nakmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ACK Response Received/Transmitted Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ackmsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ACK Response Received/Transmitted Interrupt Mask."]
    #[inline(always)]
    pub const fn set_ackmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transaction Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn xacterrmsk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction Error Mask."]
    #[inline(always)]
    pub const fn set_xacterrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Babble Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn bblerrmsk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Babble Error Mask."]
    #[inline(always)]
    pub const fn set_bblerrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Overrun Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn frmovrunmsk(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Overrun Mask."]
    #[inline(always)]
    pub const fn set_frmovrunmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data Toggle Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn datatglerrmsk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data Toggle Error Mask."]
    #[inline(always)]
    pub const fn set_datatglerrmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Hc0Intmsk {
    #[inline(always)]
    fn default() -> Hc0Intmsk {
        Hc0Intmsk(0)
    }
}
impl core::fmt::Debug for Hc0Intmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0Intmsk")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("chhltdmsk", &self.chhltdmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("stallmsk", &self.stallmsk())
            .field("nakmsk", &self.nakmsk())
            .field("ackmsk", &self.ackmsk())
            .field("xacterrmsk", &self.xacterrmsk())
            .field("bblerrmsk", &self.bblerrmsk())
            .field("frmovrunmsk", &self.frmovrunmsk())
            .field("datatglerrmsk", &self.datatglerrmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0Intmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hc0Intmsk {{ xfercomplmsk: {=bool:?}, chhltdmsk: {=bool:?}, ahberrmsk: {=bool:?}, stallmsk: {=bool:?}, nakmsk: {=bool:?}, ackmsk: {=bool:?}, xacterrmsk: {=bool:?}, bblerrmsk: {=bool:?}, frmovrunmsk: {=bool:?}, datatglerrmsk: {=bool:?} }}" , self . xfercomplmsk () , self . chhltdmsk () , self . ahberrmsk () , self . stallmsk () , self . nakmsk () , self . ackmsk () , self . xacterrmsk () , self . bblerrmsk () , self . frmovrunmsk () , self . datatglerrmsk ())
    }
}
#[doc = "Host Channel x Split Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0Splt(pub u32);
impl Hc0Splt {
    #[doc = "Port Address."]
    #[must_use]
    #[inline(always)]
    pub const fn prtaddr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Port Address."]
    #[inline(always)]
    pub const fn set_prtaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Hub Address."]
    #[must_use]
    #[inline(always)]
    pub const fn hubaddr(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "Hub Address."]
    #[inline(always)]
    pub const fn set_hubaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "Transaction Position."]
    #[must_use]
    #[inline(always)]
    pub const fn xactpos(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Transaction Position."]
    #[inline(always)]
    pub const fn set_xactpos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Do Complete Split."]
    #[must_use]
    #[inline(always)]
    pub const fn compsplt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Do Complete Split."]
    #[inline(always)]
    pub const fn set_compsplt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Split Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spltena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Split Enable."]
    #[inline(always)]
    pub const fn set_spltena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hc0Splt {
    #[inline(always)]
    fn default() -> Hc0Splt {
        Hc0Splt(0)
    }
}
impl core::fmt::Debug for Hc0Splt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0Splt")
            .field("prtaddr", &self.prtaddr())
            .field("hubaddr", &self.hubaddr())
            .field("xactpos", &self.xactpos())
            .field("compsplt", &self.compsplt())
            .field("spltena", &self.spltena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0Splt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hc0Splt {{ prtaddr: {=u8:?}, hubaddr: {=u8:?}, xactpos: {=u8:?}, compsplt: {=bool:?}, spltena: {=bool:?} }}" , self . prtaddr () , self . hubaddr () , self . xactpos () , self . compsplt () , self . spltena ())
    }
}
#[doc = "Host Channel x Transfer Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0Tsiz(pub u32);
impl Hc0Tsiz {
    #[doc = "Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn xfersize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Transfer Size."]
    #[inline(always)]
    pub const fn set_xfersize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "Packet Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pktcnt(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[doc = "Packet Count."]
    #[inline(always)]
    pub const fn set_pktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[doc = "The Application Programs This Field With the Type of."]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> super::vals::Hc0TsizPid {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Hc0TsizPid::from_bits(val as u8)
    }
    #[doc = "The Application Programs This Field With the Type of."]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: super::vals::Hc0TsizPid) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Hc0Tsiz {
    #[inline(always)]
    fn default() -> Hc0Tsiz {
        Hc0Tsiz(0)
    }
}
impl core::fmt::Debug for Hc0Tsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0Tsiz")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("pid", &self.pid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0Tsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hc0Tsiz {{ xfersize: {=u32:?}, pktcnt: {=u16:?}, pid: {:?} }}",
            self.xfersize(),
            self.pktcnt(),
            self.pid()
        )
    }
}
#[doc = "Host Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfg(pub u32);
impl Hcfg {
    #[doc = "FS/LS PHY Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn fslspclksel(&self) -> super::vals::Fslspclksel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Fslspclksel::from_bits(val as u8)
    }
    #[doc = "FS/LS PHY Clock Select."]
    #[inline(always)]
    pub const fn set_fslspclksel(&mut self, val: super::vals::Fslspclksel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FS- and LS-Only Support."]
    #[must_use]
    #[inline(always)]
    pub const fn fslssupp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FS- and LS-Only Support."]
    #[inline(always)]
    pub const fn set_fslssupp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable 32 kHz Suspend Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ena32khzs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 32 kHz Suspend Mode."]
    #[inline(always)]
    pub const fn set_ena32khzs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Validation Period."]
    #[must_use]
    #[inline(always)]
    pub const fn resvalid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Resume Validation Period."]
    #[inline(always)]
    pub const fn set_resvalid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Mode Change Time."]
    #[must_use]
    #[inline(always)]
    pub const fn modechtimen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Change Time."]
    #[inline(always)]
    pub const fn set_modechtimen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcfg {
    #[inline(always)]
    fn default() -> Hcfg {
        Hcfg(0)
    }
}
impl core::fmt::Debug for Hcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcfg")
            .field("fslspclksel", &self.fslspclksel())
            .field("fslssupp", &self.fslssupp())
            .field("ena32khzs", &self.ena32khzs())
            .field("resvalid", &self.resvalid())
            .field("modechtimen", &self.modechtimen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hcfg {{ fslspclksel: {:?}, fslssupp: {=bool:?}, ena32khzs: {=bool:?}, resvalid: {=u8:?}, modechtimen: {=bool:?} }}" , self . fslspclksel () , self . fslssupp () , self . ena32khzs () , self . resvalid () , self . modechtimen ())
    }
}
#[doc = "Host Frame Interval Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfir(pub u32);
impl Hfir {
    #[doc = "Frame Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn frint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Interval."]
    #[inline(always)]
    pub const fn set_frint(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload Control."]
    #[must_use]
    #[inline(always)]
    pub const fn hfirrldctrl(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Control."]
    #[inline(always)]
    pub const fn set_hfirrldctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Hfir {
    #[inline(always)]
    fn default() -> Hfir {
        Hfir(0)
    }
}
impl core::fmt::Debug for Hfir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfir")
            .field("frint", &self.frint())
            .field("hfirrldctrl", &self.hfirrldctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfir {{ frint: {=u16:?}, hfirrldctrl: {=bool:?} }}",
            self.frint(),
            self.hfirrldctrl()
        )
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfnum(pub u32);
impl Hfnum {
    #[doc = "Frame Number."]
    #[must_use]
    #[inline(always)]
    pub const fn frnum(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Number."]
    #[inline(always)]
    pub const fn set_frnum(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Frame Time Remaining."]
    #[must_use]
    #[inline(always)]
    pub const fn frrem(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Time Remaining."]
    #[inline(always)]
    pub const fn set_frrem(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hfnum {
    #[inline(always)]
    fn default() -> Hfnum {
        Hfnum(0)
    }
}
impl core::fmt::Debug for Hfnum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfnum")
            .field("frnum", &self.frnum())
            .field("frrem", &self.frrem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfnum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hfnum {{ frnum: {=u16:?}, frrem: {=u16:?} }}",
            self.frnum(),
            self.frrem()
        )
    }
}
#[doc = "Host Port Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hprt(pub u32);
impl Hprt {
    #[doc = "Port Connect Status."]
    #[must_use]
    #[inline(always)]
    pub const fn prtconnsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Connect Status."]
    #[inline(always)]
    pub const fn set_prtconnsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Connect Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn prtconndet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Connect Detected."]
    #[inline(always)]
    pub const fn set_prtconndet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prtena(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enable."]
    #[inline(always)]
    pub const fn set_prtena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable/Disable Change."]
    #[must_use]
    #[inline(always)]
    pub const fn prtenchng(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enable/Disable Change."]
    #[inline(always)]
    pub const fn set_prtenchng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Overcurrent Active."]
    #[must_use]
    #[inline(always)]
    pub const fn prtovrcurract(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Overcurrent Active."]
    #[inline(always)]
    pub const fn set_prtovrcurract(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Overcurrent Change."]
    #[must_use]
    #[inline(always)]
    pub const fn prtovrcurrchng(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port Overcurrent Change."]
    #[inline(always)]
    pub const fn set_prtovrcurrchng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn prtres(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port Resume."]
    #[inline(always)]
    pub const fn set_prtres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn prtsusp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port Suspend."]
    #[inline(always)]
    pub const fn set_prtsusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn prtrst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset."]
    #[inline(always)]
    pub const fn set_prtrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Line Status."]
    #[must_use]
    #[inline(always)]
    pub const fn prtlnsts(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port Line Status."]
    #[inline(always)]
    pub const fn set_prtlnsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power."]
    #[must_use]
    #[inline(always)]
    pub const fn prtpwr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power."]
    #[inline(always)]
    pub const fn set_prtpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Test Control."]
    #[must_use]
    #[inline(always)]
    pub const fn prttstctl(&self) -> super::vals::Prttstctl {
        let val = (self.0 >> 13usize) & 0x0f;
        super::vals::Prttstctl::from_bits(val as u8)
    }
    #[doc = "Port Test Control."]
    #[inline(always)]
    pub const fn set_prttstctl(&mut self, val: super::vals::Prttstctl) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val.to_bits() as u32) & 0x0f) << 13usize);
    }
    #[doc = "Port Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn prtspd(&self) -> super::vals::Prtspd {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Prtspd::from_bits(val as u8)
    }
    #[doc = "Port Speed."]
    #[inline(always)]
    pub const fn set_prtspd(&mut self, val: super::vals::Prtspd) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
}
impl Default for Hprt {
    #[inline(always)]
    fn default() -> Hprt {
        Hprt(0)
    }
}
impl core::fmt::Debug for Hprt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hprt")
            .field("prtconnsts", &self.prtconnsts())
            .field("prtconndet", &self.prtconndet())
            .field("prtena", &self.prtena())
            .field("prtenchng", &self.prtenchng())
            .field("prtovrcurract", &self.prtovrcurract())
            .field("prtovrcurrchng", &self.prtovrcurrchng())
            .field("prtres", &self.prtres())
            .field("prtsusp", &self.prtsusp())
            .field("prtrst", &self.prtrst())
            .field("prtlnsts", &self.prtlnsts())
            .field("prtpwr", &self.prtpwr())
            .field("prttstctl", &self.prttstctl())
            .field("prtspd", &self.prtspd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hprt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hprt {{ prtconnsts: {=bool:?}, prtconndet: {=bool:?}, prtena: {=bool:?}, prtenchng: {=bool:?}, prtovrcurract: {=bool:?}, prtovrcurrchng: {=bool:?}, prtres: {=bool:?}, prtsusp: {=bool:?}, prtrst: {=bool:?}, prtlnsts: {=u8:?}, prtpwr: {=bool:?}, prttstctl: {:?}, prtspd: {:?} }}" , self . prtconnsts () , self . prtconndet () , self . prtena () , self . prtenchng () , self . prtovrcurract () , self . prtovrcurrchng () , self . prtres () , self . prtsusp () , self . prtrst () , self . prtlnsts () , self . prtpwr () , self . prttstctl () , self . prtspd ())
    }
}
#[doc = "Host Periodic Transmit FIFO Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxfsiz(pub u32);
impl Hptxfsiz {
    #[doc = "Host Periodic TxFIFO Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfstaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Host Periodic TxFIFO Start Address."]
    #[inline(always)]
    pub const fn set_ptxfstaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Host Periodic TxFIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfsize(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Host Periodic TxFIFO Depth."]
    #[inline(always)]
    pub const fn set_ptxfsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Hptxfsiz {
    #[inline(always)]
    fn default() -> Hptxfsiz {
        Hptxfsiz(0)
    }
}
impl core::fmt::Debug for Hptxfsiz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hptxfsiz")
            .field("ptxfstaddr", &self.ptxfstaddr())
            .field("ptxfsize", &self.ptxfsize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hptxfsiz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hptxfsiz {{ ptxfstaddr: {=u16:?}, ptxfsize: {=u16:?} }}",
            self.ptxfstaddr(),
            self.ptxfsize()
        )
    }
}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxsts(pub u32);
impl Hptxsts {
    #[doc = "Periodic Transmit Data FIFO Space Available."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxfspcavail(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Periodic Transmit Data FIFO Space Available."]
    #[inline(always)]
    pub const fn set_ptxfspcavail(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Periodic Transmit Request Queue Space Available."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxqspcavail(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Periodic Transmit Request Queue Space Available."]
    #[inline(always)]
    pub const fn set_ptxqspcavail(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Top of the Periodic Transmit Request Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxqtop(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Top of the Periodic Transmit Request Queue."]
    #[inline(always)]
    pub const fn set_ptxqtop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hptxsts {
    #[inline(always)]
    fn default() -> Hptxsts {
        Hptxsts(0)
    }
}
impl core::fmt::Debug for Hptxsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hptxsts")
            .field("ptxfspcavail", &self.ptxfspcavail())
            .field("ptxqspcavail", &self.ptxqspcavail())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hptxsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hptxsts {{ ptxfspcavail: {=u16:?}, ptxqspcavail: {=u8:?}, ptxqtop: {=u8:?} }}",
            self.ptxfspcavail(),
            self.ptxqspcavail(),
            self.ptxqtop()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "VBUSDETH Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdeth(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUSDETH Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vbusdeth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUSDETL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdetl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUSDETL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vbusdetl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DCD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("vbusdeth", &self.vbusdeth())
            .field("vbusdetl", &self.vbusdetl())
            .field("err", &self.err())
            .field("dcd", &self.dcd())
            .field("pd", &self.pd())
            .field("sd", &self.sd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ vbusdeth: {=bool:?}, vbusdetl: {=bool:?}, err: {=bool:?}, dcd: {=bool:?}, pd: {=bool:?}, sd: {=bool:?} }}" , self . vbusdeth () , self . vbusdetl () , self . err () , self . dcd () , self . pd () , self . sd ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "VBUS Detect High Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdeth(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect High Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vbusdeth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Detect Low Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdetl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Low Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vbusdetl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Detection Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Detection Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data Contact Detection Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data Contact Detection Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Primary Detection Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Primary Detection Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Secondary Detection Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("vbusdeth", &self.vbusdeth())
            .field("vbusdetl", &self.vbusdetl())
            .field("err", &self.err())
            .field("dcd", &self.dcd())
            .field("pd", &self.pd())
            .field("sd", &self.sd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ vbusdeth: {=bool:?}, vbusdetl: {=bool:?}, err: {=bool:?}, dcd: {=bool:?}, pd: {=bool:?}, sd: {=bool:?} }}" , self . vbusdeth () , self . vbusdetl () , self . err () , self . dcd () , self . pd () , self . sd ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set VBUSDETH Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdeth(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set VBUSDETH Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vbusdeth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set VBUSDETL Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdetl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set VBUSDETL Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vbusdetl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set ERR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set ERR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set DCD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set DCD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set PD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set PD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set SD Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set SD Interrupt Flag."]
    #[inline(always)]
    pub const fn set_sd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("vbusdeth", &self.vbusdeth())
            .field("vbusdetl", &self.vbusdetl())
            .field("err", &self.err())
            .field("dcd", &self.dcd())
            .field("pd", &self.pd())
            .field("sd", &self.sd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ vbusdeth: {=bool:?}, vbusdetl: {=bool:?}, err: {=bool:?}, dcd: {=bool:?}, pd: {=bool:?}, sd: {=bool:?} }}" , self . vbusdeth () , self . vbusdetl () , self . err () , self . dcd () , self . pd () , self . sd ())
    }
}
#[doc = "USB LEM Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lemctrl(pub u32);
impl Lemctrl {
    #[doc = "Set the Number of LFC Clk Counts to Form 3ms."]
    #[must_use]
    #[inline(always)]
    pub const fn timebase(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Set the Number of LFC Clk Counts to Form 3ms."]
    #[inline(always)]
    pub const fn set_timebase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Lemctrl {
    #[inline(always)]
    fn default() -> Lemctrl {
        Lemctrl(0)
    }
}
impl core::fmt::Debug for Lemctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lemctrl")
            .field("timebase", &self.timebase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lemctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lemctrl {{ timebase: {=u16:?} }}", self.timebase())
    }
}
#[doc = "Power and Clock Gating Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcgcctl(pub u32);
impl Pcgcctl {
    #[doc = "Stop PHY clock."]
    #[must_use]
    #[inline(always)]
    pub const fn stoppclk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop PHY clock."]
    #[inline(always)]
    pub const fn set_stoppclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gate HCLK."]
    #[must_use]
    #[inline(always)]
    pub const fn gatehclk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gate HCLK."]
    #[inline(always)]
    pub const fn set_gatehclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power Clamp."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrclmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power Clamp."]
    #[inline(always)]
    pub const fn set_pwrclmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset Power-Down Modules."]
    #[must_use]
    #[inline(always)]
    pub const fn rstpdwnmodule(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Power-Down Modules."]
    #[inline(always)]
    pub const fn set_rstpdwnmodule(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PHY In Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn physleep(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PHY In Sleep."]
    #[inline(always)]
    pub const fn set_physleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset after suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn resetaftersusp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reset after suspend."]
    #[inline(always)]
    pub const fn set_resetaftersusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Pcgcctl {
    #[inline(always)]
    fn default() -> Pcgcctl {
        Pcgcctl(0)
    }
}
impl core::fmt::Debug for Pcgcctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcgcctl")
            .field("stoppclk", &self.stoppclk())
            .field("gatehclk", &self.gatehclk())
            .field("pwrclmp", &self.pwrclmp())
            .field("rstpdwnmodule", &self.rstpdwnmodule())
            .field("physleep", &self.physleep())
            .field("resetaftersusp", &self.resetaftersusp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcgcctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pcgcctl {{ stoppclk: {=bool:?}, gatehclk: {=bool:?}, pwrclmp: {=bool:?}, rstpdwnmodule: {=bool:?}, physleep: {=bool:?}, resetaftersusp: {=bool:?} }}" , self . stoppclk () , self . gatehclk () , self . pwrclmp () , self . rstpdwnmodule () , self . physleep () , self . resetaftersusp ())
    }
}
#[doc = "I/O Routing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Route(pub u32);
impl Route {
    #[doc = "USB PHY Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn phypen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB PHY Pin Enable."]
    #[inline(always)]
    pub const fn set_phypen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUSEN Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusenpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUSEN Pin Enable."]
    #[inline(always)]
    pub const fn set_vbusenpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Route {
    #[inline(always)]
    fn default() -> Route {
        Route(0)
    }
}
impl core::fmt::Debug for Route {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Route")
            .field("phypen", &self.phypen())
            .field("vbusenpen", &self.vbusenpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Route {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Route {{ phypen: {=bool:?}, vbusenpen: {=bool:?} }}",
            self.phypen(),
            self.vbusenpen()
        )
    }
}
#[doc = "System Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "VBUS Detect High."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusdeth(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect High."]
    #[inline(always)]
    pub const fn set_vbusdeth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Energy Mode Active."]
    #[must_use]
    #[inline(always)]
    pub const fn lemactive(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Low Energy Mode Active."]
    #[inline(always)]
    pub const fn set_lemactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data Contact Detection Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdto(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Data Contact Detection Timeout."]
    #[inline(always)]
    pub const fn set_dcdto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Standard Downstream Port Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sdp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Standard Downstream Port Detected."]
    #[inline(always)]
    pub const fn set_sdp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Charging Downstream Port Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn cdp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Charging Downstream Port Detected."]
    #[inline(always)]
    pub const fn set_cdp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Dedicated Charging Port Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn dcp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Dedicated Charging Port Detected."]
    #[inline(always)]
    pub const fn set_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ACA Full Speed TypeB Device."]
    #[must_use]
    #[inline(always)]
    pub const fn acafs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ACA Full Speed TypeB Device."]
    #[inline(always)]
    pub const fn set_acafs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ACA Low Speed TypeB Device."]
    #[must_use]
    #[inline(always)]
    pub const fn acals(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ACA Low Speed TypeB Device."]
    #[inline(always)]
    pub const fn set_acals(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "USB Charger Detect Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn usbcdbusy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB Charger Detect Busy."]
    #[inline(always)]
    pub const fn set_usbcdbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("vbusdeth", &self.vbusdeth())
            .field("lemactive", &self.lemactive())
            .field("dcdto", &self.dcdto())
            .field("sdp", &self.sdp())
            .field("cdp", &self.cdp())
            .field("dcp", &self.dcp())
            .field("acafs", &self.acafs())
            .field("acals", &self.acals())
            .field("usbcdbusy", &self.usbcdbusy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ vbusdeth: {=bool:?}, lemactive: {=bool:?}, dcdto: {=bool:?}, sdp: {=bool:?}, cdp: {=bool:?}, dcp: {=bool:?}, acafs: {=bool:?}, acals: {=bool:?}, usbcdbusy: {=bool:?} }}" , self . vbusdeth () , self . lemactive () , self . dcdto () , self . sdp () , self . cdp () , self . dcp () , self . acafs () , self . acals () , self . usbcdbusy ())
    }
}
