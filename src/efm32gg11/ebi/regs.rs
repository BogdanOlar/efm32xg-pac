#[doc = "Address Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrtiming(pub u32);
impl Addrtiming {
    #[doc = "Address Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address Setup Time."]
    #[inline(always)]
    pub const fn set_addrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrhold(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Address Hold Time."]
    #[inline(always)]
    pub const fn set_addrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfale(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Addrtiming {
    #[inline(always)]
    fn default() -> Addrtiming {
        Addrtiming(0)
    }
}
impl core::fmt::Debug for Addrtiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addrtiming")
            .field("addrsetup", &self.addrsetup())
            .field("addrhold", &self.addrhold())
            .field("halfale", &self.halfale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addrtiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addrtiming {{ addrsetup: {=u8:?}, addrhold: {=u8:?}, halfale: {=bool:?} }}",
            self.addrsetup(),
            self.addrhold(),
            self.halfale()
        )
    }
}
#[doc = "Address Timing Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrtiming1(pub u32);
impl Addrtiming1 {
    #[doc = "Address Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address Setup Time."]
    #[inline(always)]
    pub const fn set_addrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrhold(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Address Hold Time."]
    #[inline(always)]
    pub const fn set_addrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfale(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Addrtiming1 {
    #[inline(always)]
    fn default() -> Addrtiming1 {
        Addrtiming1(0)
    }
}
impl core::fmt::Debug for Addrtiming1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addrtiming1")
            .field("addrsetup", &self.addrsetup())
            .field("addrhold", &self.addrhold())
            .field("halfale", &self.halfale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addrtiming1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addrtiming1 {{ addrsetup: {=u8:?}, addrhold: {=u8:?}, halfale: {=bool:?} }}",
            self.addrsetup(),
            self.addrhold(),
            self.halfale()
        )
    }
}
#[doc = "Address Timing Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrtiming2(pub u32);
impl Addrtiming2 {
    #[doc = "Address Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address Setup Time."]
    #[inline(always)]
    pub const fn set_addrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrhold(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Address Hold Time."]
    #[inline(always)]
    pub const fn set_addrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfale(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Addrtiming2 {
    #[inline(always)]
    fn default() -> Addrtiming2 {
        Addrtiming2(0)
    }
}
impl core::fmt::Debug for Addrtiming2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addrtiming2")
            .field("addrsetup", &self.addrsetup())
            .field("addrhold", &self.addrhold())
            .field("halfale", &self.halfale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addrtiming2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addrtiming2 {{ addrsetup: {=u8:?}, addrhold: {=u8:?}, halfale: {=bool:?} }}",
            self.addrsetup(),
            self.addrhold(),
            self.halfale()
        )
    }
}
#[doc = "Address Timing Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrtiming3(pub u32);
impl Addrtiming3 {
    #[doc = "Address Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address Setup Time."]
    #[inline(always)]
    pub const fn set_addrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addrhold(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Address Hold Time."]
    #[inline(always)]
    pub const fn set_addrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfale(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle ALE Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Addrtiming3 {
    #[inline(always)]
    fn default() -> Addrtiming3 {
        Addrtiming3(0)
    }
}
impl core::fmt::Debug for Addrtiming3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addrtiming3")
            .field("addrsetup", &self.addrsetup())
            .field("addrhold", &self.addrhold())
            .field("halfale", &self.halfale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addrtiming3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addrtiming3 {{ addrsetup: {=u8:?}, addrhold: {=u8:?}, halfale: {=bool:?} }}",
            self.addrsetup(),
            self.addrhold(),
            self.halfale()
        )
    }
}
#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Error Correction Code Generation Start."]
    #[must_use]
    #[inline(always)]
    pub const fn eccstart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error Correction Code Generation Start."]
    #[inline(always)]
    pub const fn set_eccstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error Correction Code Generation Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn eccstop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error Correction Code Generation Stop."]
    #[inline(always)]
    pub const fn set_eccstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error Correction Code Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn eccclear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error Correction Code Clear."]
    #[inline(always)]
    pub const fn set_eccclear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("eccstart", &self.eccstart())
            .field("eccstop", &self.eccstop())
            .field("eccclear", &self.eccclear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ eccstart: {=bool:?}, eccstop: {=bool:?}, eccclear: {=bool:?} }}",
            self.eccstart(),
            self.eccstop(),
            self.eccclear()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Mode 1."]
    #[must_use]
    #[inline(always)]
    pub const fn mode1(&self) -> super::vals::Mode1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Mode1::from_bits(val as u8)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub const fn set_mode1(&mut self, val: super::vals::Mode1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Mode 2."]
    #[must_use]
    #[inline(always)]
    pub const fn mode2(&self) -> super::vals::Mode2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Mode2::from_bits(val as u8)
    }
    #[doc = "Mode 2."]
    #[inline(always)]
    pub const fn set_mode2(&mut self, val: super::vals::Mode2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Mode 3."]
    #[must_use]
    #[inline(always)]
    pub const fn mode3(&self) -> super::vals::Mode3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Mode3::from_bits(val as u8)
    }
    #[doc = "Mode 3."]
    #[inline(always)]
    pub const fn set_mode3(&mut self, val: super::vals::Mode3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Bank 0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bank0en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bank 0 Enable."]
    #[inline(always)]
    pub const fn set_bank0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bank 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bank1en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bank 1 Enable."]
    #[inline(always)]
    pub const fn set_bank1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bank 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bank2en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bank 2 Enable."]
    #[inline(always)]
    pub const fn set_bank2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bank 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bank3en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bank 3 Enable."]
    #[inline(always)]
    pub const fn set_bank3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "No Idle Cycle Insertion on Bank 0."]
    #[must_use]
    #[inline(always)]
    pub const fn noidle(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "No Idle Cycle Insertion on Bank 0."]
    #[inline(always)]
    pub const fn set_noidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "No Idle Cycle Insertion on Bank 1."]
    #[must_use]
    #[inline(always)]
    pub const fn noidle1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "No Idle Cycle Insertion on Bank 1."]
    #[inline(always)]
    pub const fn set_noidle1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "No Idle Cycle Insertion on Bank 2."]
    #[must_use]
    #[inline(always)]
    pub const fn noidle2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "No Idle Cycle Insertion on Bank 2."]
    #[inline(always)]
    pub const fn set_noidle2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "No Idle Cycle Insertion on Bank 3."]
    #[must_use]
    #[inline(always)]
    pub const fn noidle3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "No Idle Cycle Insertion on Bank 3."]
    #[inline(always)]
    pub const fn set_noidle3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ARDY Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ardyen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Enable."]
    #[inline(always)]
    pub const fn set_ardyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ARDY Timeout Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ardytodis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Timeout Disable."]
    #[inline(always)]
    pub const fn set_ardytodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ARDY Enable for Bank 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ardy1en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Enable for Bank 1."]
    #[inline(always)]
    pub const fn set_ardy1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ARDY Timeout Disable for Bank 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ardyto1dis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Timeout Disable for Bank 1."]
    #[inline(always)]
    pub const fn set_ardyto1dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ARDY Enable for Bank 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ardy2en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Enable for Bank 2."]
    #[inline(always)]
    pub const fn set_ardy2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ARDY Timeout Disable for Bank 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ardyto2dis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Timeout Disable for Bank 2."]
    #[inline(always)]
    pub const fn set_ardyto2dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ARDY Enable for Bank 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ardy3en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Enable for Bank 3."]
    #[inline(always)]
    pub const fn set_ardy3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ARDY Timeout Disable for Bank 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ardyto3dis(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Timeout Disable for Bank 3."]
    #[inline(always)]
    pub const fn set_ardyto3dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Byte Lane Enable for Bank 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Lane Enable for Bank 0."]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Byte Lane Enable for Bank 1."]
    #[must_use]
    #[inline(always)]
    pub const fn bl1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Lane Enable for Bank 1."]
    #[inline(always)]
    pub const fn set_bl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Byte Lane Enable for Bank 2."]
    #[must_use]
    #[inline(always)]
    pub const fn bl2(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Lane Enable for Bank 2."]
    #[inline(always)]
    pub const fn set_bl2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Byte Lane Enable for Bank 3."]
    #[must_use]
    #[inline(always)]
    pub const fn bl3(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Lane Enable for Bank 3."]
    #[inline(always)]
    pub const fn set_bl3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Individual Timing Set, Line Polarity and Mode Definition Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn its(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Individual Timing Set, Line Polarity and Mode Definition Enable."]
    #[inline(always)]
    pub const fn set_its(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Alternative Address Map Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altmap(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Address Map Enable."]
    #[inline(always)]
    pub const fn set_altmap(&mut self, val: bool) {
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
            .field("mode", &self.mode())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("bank0en", &self.bank0en())
            .field("bank1en", &self.bank1en())
            .field("bank2en", &self.bank2en())
            .field("bank3en", &self.bank3en())
            .field("noidle", &self.noidle())
            .field("noidle1", &self.noidle1())
            .field("noidle2", &self.noidle2())
            .field("noidle3", &self.noidle3())
            .field("ardyen", &self.ardyen())
            .field("ardytodis", &self.ardytodis())
            .field("ardy1en", &self.ardy1en())
            .field("ardyto1dis", &self.ardyto1dis())
            .field("ardy2en", &self.ardy2en())
            .field("ardyto2dis", &self.ardyto2dis())
            .field("ardy3en", &self.ardy3en())
            .field("ardyto3dis", &self.ardyto3dis())
            .field("bl", &self.bl())
            .field("bl1", &self.bl1())
            .field("bl2", &self.bl2())
            .field("bl3", &self.bl3())
            .field("its", &self.its())
            .field("altmap", &self.altmap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ mode: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, bank0en: {=bool:?}, bank1en: {=bool:?}, bank2en: {=bool:?}, bank3en: {=bool:?}, noidle: {=bool:?}, noidle1: {=bool:?}, noidle2: {=bool:?}, noidle3: {=bool:?}, ardyen: {=bool:?}, ardytodis: {=bool:?}, ardy1en: {=bool:?}, ardyto1dis: {=bool:?}, ardy2en: {=bool:?}, ardyto2dis: {=bool:?}, ardy3en: {=bool:?}, ardyto3dis: {=bool:?}, bl: {=bool:?}, bl1: {=bool:?}, bl2: {=bool:?}, bl3: {=bool:?}, its: {=bool:?}, altmap: {=bool:?} }}" , self . mode () , self . mode1 () , self . mode2 () , self . mode3 () , self . bank0en () , self . bank1en () , self . bank2en () , self . bank3en () , self . noidle () , self . noidle1 () , self . noidle2 () , self . noidle3 () , self . ardyen () , self . ardytodis () , self . ardy1en () , self . ardyto1dis () , self . ardy2en () , self . ardyto2dis () , self . ardy3en () , self . ardyto3dis () , self . bl () , self . bl1 () , self . bl2 () , self . bl3 () , self . its () , self . altmap ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "Vertical Sync Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vsync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Sync Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Horizontal Sync Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsync(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Horizontal Sync Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Vertical Back Porch Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbporch(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Back Porch Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vbporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Vertical Front Porch Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vfporch(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Front Porch Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vfporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direct Drive Data Empty Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ddempty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Data Empty Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ddempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direct Drive Jitter Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ddjit(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Jitter Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ddjit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EBI_TFTPIXEL0 Empty Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel0empty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL0 Empty Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tftpixel0empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EBI_TFTPIXEL1 Empty Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel1empty(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL1 Empty Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tftpixel1empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EBI_TFTPIXEL Full Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelfull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL Full Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tftpixelfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EBI_TFTPIXEL Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tftpixelof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("vsync", &self.vsync())
            .field("hsync", &self.hsync())
            .field("vbporch", &self.vbporch())
            .field("vfporch", &self.vfporch())
            .field("ddempty", &self.ddempty())
            .field("ddjit", &self.ddjit())
            .field("tftpixel0empty", &self.tftpixel0empty())
            .field("tftpixel1empty", &self.tftpixel1empty())
            .field("tftpixelfull", &self.tftpixelfull())
            .field("tftpixelof", &self.tftpixelof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ vsync: {=bool:?}, hsync: {=bool:?}, vbporch: {=bool:?}, vfporch: {=bool:?}, ddempty: {=bool:?}, ddjit: {=bool:?}, tftpixel0empty: {=bool:?}, tftpixel1empty: {=bool:?}, tftpixelfull: {=bool:?}, tftpixelof: {=bool:?} }}" , self . vsync () , self . hsync () , self . vbporch () , self . vfporch () , self . ddempty () , self . ddjit () , self . tftpixel0empty () , self . tftpixel1empty () , self . tftpixelfull () , self . tftpixelof ())
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Vertical Sync Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vsync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Sync Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Horizontal Sync Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hsync(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Horizontal Sync Interrupt Flag."]
    #[inline(always)]
    pub const fn set_hsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Vertical Back Porch Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vbporch(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Back Porch Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vbporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Vertical Front Porch Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn vfporch(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Front Porch Interrupt Flag."]
    #[inline(always)]
    pub const fn set_vfporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direct Drive Data Empty Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ddempty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Data Empty Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ddempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direct Drive Jitter Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ddjit(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Jitter Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ddjit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EBI_TFTPIXEL0 is Empty Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel0empty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL0 is Empty Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tftpixel0empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EBI_TFTPIXEL1 is Empty Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel1empty(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL1 is Empty Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tftpixel1empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EBI_TFTPIXEL is Full Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelfull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL is Full Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tftpixelfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EBI_TFTPIXEL Register Overflow Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL Register Overflow Interrupt Flag."]
    #[inline(always)]
    pub const fn set_tftpixelof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("vsync", &self.vsync())
            .field("hsync", &self.hsync())
            .field("vbporch", &self.vbporch())
            .field("vfporch", &self.vfporch())
            .field("ddempty", &self.ddempty())
            .field("ddjit", &self.ddjit())
            .field("tftpixel0empty", &self.tftpixel0empty())
            .field("tftpixel1empty", &self.tftpixel1empty())
            .field("tftpixelfull", &self.tftpixelfull())
            .field("tftpixelof", &self.tftpixelof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "If {{ vsync: {=bool:?}, hsync: {=bool:?}, vbporch: {=bool:?}, vfporch: {=bool:?}, ddempty: {=bool:?}, ddjit: {=bool:?}, tftpixel0empty: {=bool:?}, tftpixel1empty: {=bool:?}, tftpixelfull: {=bool:?}, tftpixelof: {=bool:?} }}" , self . vsync () , self . hsync () , self . vbporch () , self . vfporch () , self . ddempty () , self . ddjit () , self . tftpixel0empty () , self . tftpixel1empty () , self . tftpixelfull () , self . tftpixelof ())
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Vertical Sync Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn vsync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Sync Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_vsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Horizontal Sync Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn hsync(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Horizontal Sync Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_hsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Vertical Back Porch Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn vbporch(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Back Porch Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_vbporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Vertical Front Porch Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn vfporch(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Vertical Front Porch Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_vfporch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direct Drive Data Empty Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn ddempty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Data Empty Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_ddempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direct Drive Jitter Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn ddjit(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direct Drive Jitter Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_ddjit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EBI_TFTPIXEL0 Empty Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel0empty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL0 Empty Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_tftpixel0empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EBI_TFTPIXEL1 Empty Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel1empty(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL1 Empty Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_tftpixel1empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EBI_TFTPIXEL Full Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelfull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL Full Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_tftpixelfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EBI_TFTPIXEL Overflow Interrupt Flag Set."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL Overflow Interrupt Flag Set."]
    #[inline(always)]
    pub const fn set_tftpixelof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("vsync", &self.vsync())
            .field("hsync", &self.hsync())
            .field("vbporch", &self.vbporch())
            .field("vfporch", &self.vfporch())
            .field("ddempty", &self.ddempty())
            .field("ddjit", &self.ddjit())
            .field("tftpixel0empty", &self.tftpixel0empty())
            .field("tftpixel1empty", &self.tftpixel1empty())
            .field("tftpixelfull", &self.tftpixelfull())
            .field("tftpixelof", &self.tftpixelof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifs {{ vsync: {=bool:?}, hsync: {=bool:?}, vbporch: {=bool:?}, vfporch: {=bool:?}, ddempty: {=bool:?}, ddjit: {=bool:?}, tftpixel0empty: {=bool:?}, tftpixel1empty: {=bool:?}, tftpixelfull: {=bool:?}, tftpixelof: {=bool:?} }}" , self . vsync () , self . hsync () , self . vbporch () , self . vfporch () , self . ddempty () , self . ddjit () , self . tftpixel0empty () , self . tftpixel1empty () , self . tftpixelfull () , self . tftpixelof ())
    }
}
#[doc = "NAND Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandctrl(pub u32);
impl Nandctrl {
    #[doc = "NAND Flash Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NAND Flash Control Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NAND Flash Bank."]
    #[must_use]
    #[inline(always)]
    pub const fn banksel(&self) -> super::vals::NandctrlBanksel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::NandctrlBanksel::from_bits(val as u8)
    }
    #[doc = "NAND Flash Bank."]
    #[inline(always)]
    pub const fn set_banksel(&mut self, val: super::vals::NandctrlBanksel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Nandctrl {
    #[inline(always)]
    fn default() -> Nandctrl {
        Nandctrl(0)
    }
}
impl core::fmt::Debug for Nandctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandctrl")
            .field("en", &self.en())
            .field("banksel", &self.banksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nandctrl {{ en: {=bool:?}, banksel: {:?} }}",
            self.en(),
            self.banksel()
        )
    }
}
#[doc = "Page Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pagectrl(pub u32);
impl Pagectrl {
    #[doc = "Page Length."]
    #[must_use]
    #[inline(always)]
    pub const fn pagelen(&self) -> super::vals::Pagelen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pagelen::from_bits(val as u8)
    }
    #[doc = "Page Length."]
    #[inline(always)]
    pub const fn set_pagelen(&mut self, val: super::vals::Pagelen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Intrapage Hit Only on Incremental Addresses."]
    #[must_use]
    #[inline(always)]
    pub const fn inchit(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Intrapage Hit Only on Incremental Addresses."]
    #[inline(always)]
    pub const fn set_inchit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Page Read Access Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdpa(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Page Read Access Time."]
    #[inline(always)]
    pub const fn set_rdpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Maximum Page Open Time."]
    #[must_use]
    #[inline(always)]
    pub const fn keepopen(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x7f;
        val as u8
    }
    #[doc = "Maximum Page Open Time."]
    #[inline(always)]
    pub const fn set_keepopen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 20usize)) | (((val as u32) & 0x7f) << 20usize);
    }
}
impl Default for Pagectrl {
    #[inline(always)]
    fn default() -> Pagectrl {
        Pagectrl(0)
    }
}
impl core::fmt::Debug for Pagectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pagectrl")
            .field("pagelen", &self.pagelen())
            .field("inchit", &self.inchit())
            .field("rdpa", &self.rdpa())
            .field("keepopen", &self.keepopen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pagectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pagectrl {{ pagelen: {:?}, inchit: {=bool:?}, rdpa: {=u8:?}, keepopen: {=u8:?} }}",
            self.pagelen(),
            self.inchit(),
            self.rdpa(),
            self.keepopen()
        )
    }
}
#[doc = "Polarity Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polarity(pub u32);
impl Polarity {
    #[doc = "Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cspol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_cspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn repol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Enable Polarity."]
    #[inline(always)]
    pub const fn set_repol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn wepol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write Enable Polarity."]
    #[inline(always)]
    pub const fn set_wepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address Latch Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn alepol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Latch Polarity."]
    #[inline(always)]
    pub const fn set_alepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ARDY Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ardypol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Polarity."]
    #[inline(always)]
    pub const fn set_ardypol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BL Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn blpol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BL Polarity."]
    #[inline(always)]
    pub const fn set_blpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Polarity {
    #[inline(always)]
    fn default() -> Polarity {
        Polarity(0)
    }
}
impl core::fmt::Debug for Polarity {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polarity")
            .field("cspol", &self.cspol())
            .field("repol", &self.repol())
            .field("wepol", &self.wepol())
            .field("alepol", &self.alepol())
            .field("ardypol", &self.ardypol())
            .field("blpol", &self.blpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polarity {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Polarity {{ cspol: {=bool:?}, repol: {=bool:?}, wepol: {=bool:?}, alepol: {=bool:?}, ardypol: {=bool:?}, blpol: {=bool:?} }}" , self . cspol () , self . repol () , self . wepol () , self . alepol () , self . ardypol () , self . blpol ())
    }
}
#[doc = "Polarity Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polarity1(pub u32);
impl Polarity1 {
    #[doc = "Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cspol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_cspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn repol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Enable Polarity."]
    #[inline(always)]
    pub const fn set_repol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn wepol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write Enable Polarity."]
    #[inline(always)]
    pub const fn set_wepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address Latch Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn alepol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Latch Polarity."]
    #[inline(always)]
    pub const fn set_alepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ARDY Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ardypol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Polarity."]
    #[inline(always)]
    pub const fn set_ardypol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BL Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn blpol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BL Polarity."]
    #[inline(always)]
    pub const fn set_blpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Polarity1 {
    #[inline(always)]
    fn default() -> Polarity1 {
        Polarity1(0)
    }
}
impl core::fmt::Debug for Polarity1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polarity1")
            .field("cspol", &self.cspol())
            .field("repol", &self.repol())
            .field("wepol", &self.wepol())
            .field("alepol", &self.alepol())
            .field("ardypol", &self.ardypol())
            .field("blpol", &self.blpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polarity1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Polarity1 {{ cspol: {=bool:?}, repol: {=bool:?}, wepol: {=bool:?}, alepol: {=bool:?}, ardypol: {=bool:?}, blpol: {=bool:?} }}" , self . cspol () , self . repol () , self . wepol () , self . alepol () , self . ardypol () , self . blpol ())
    }
}
#[doc = "Polarity Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polarity2(pub u32);
impl Polarity2 {
    #[doc = "Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cspol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_cspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn repol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Enable Polarity."]
    #[inline(always)]
    pub const fn set_repol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn wepol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write Enable Polarity."]
    #[inline(always)]
    pub const fn set_wepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address Latch Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn alepol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Latch Polarity."]
    #[inline(always)]
    pub const fn set_alepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ARDY Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ardypol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Polarity."]
    #[inline(always)]
    pub const fn set_ardypol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BL Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn blpol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BL Polarity."]
    #[inline(always)]
    pub const fn set_blpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Polarity2 {
    #[inline(always)]
    fn default() -> Polarity2 {
        Polarity2(0)
    }
}
impl core::fmt::Debug for Polarity2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polarity2")
            .field("cspol", &self.cspol())
            .field("repol", &self.repol())
            .field("wepol", &self.wepol())
            .field("alepol", &self.alepol())
            .field("ardypol", &self.ardypol())
            .field("blpol", &self.blpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polarity2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Polarity2 {{ cspol: {=bool:?}, repol: {=bool:?}, wepol: {=bool:?}, alepol: {=bool:?}, ardypol: {=bool:?}, blpol: {=bool:?} }}" , self . cspol () , self . repol () , self . wepol () , self . alepol () , self . ardypol () , self . blpol ())
    }
}
#[doc = "Polarity Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polarity3(pub u32);
impl Polarity3 {
    #[doc = "Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cspol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_cspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn repol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Enable Polarity."]
    #[inline(always)]
    pub const fn set_repol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Enable Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn wepol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write Enable Polarity."]
    #[inline(always)]
    pub const fn set_wepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address Latch Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn alepol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Latch Polarity."]
    #[inline(always)]
    pub const fn set_alepol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ARDY Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ardypol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ARDY Polarity."]
    #[inline(always)]
    pub const fn set_ardypol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BL Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn blpol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BL Polarity."]
    #[inline(always)]
    pub const fn set_blpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Polarity3 {
    #[inline(always)]
    fn default() -> Polarity3 {
        Polarity3(0)
    }
}
impl core::fmt::Debug for Polarity3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polarity3")
            .field("cspol", &self.cspol())
            .field("repol", &self.repol())
            .field("wepol", &self.wepol())
            .field("alepol", &self.alepol())
            .field("ardypol", &self.ardypol())
            .field("blpol", &self.blpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polarity3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Polarity3 {{ cspol: {=bool:?}, repol: {=bool:?}, wepol: {=bool:?}, alepol: {=bool:?}, ardypol: {=bool:?}, blpol: {=bool:?} }}" , self . cspol () , self . repol () , self . wepol () , self . alepol () , self . ardypol () , self . blpol ())
    }
}
#[doc = "Read Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdtiming(pub u32);
impl Rdtiming {
    #[doc = "Read Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Read Setup Time."]
    #[inline(always)]
    pub const fn set_rdsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Read Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Read Strobe Time."]
    #[inline(always)]
    pub const fn set_rdstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Read Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Read Hold Time."]
    #[inline(always)]
    pub const fn set_rdhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfre(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Page Mode Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pagemode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Page Mode Access Enable."]
    #[inline(always)]
    pub const fn set_pagemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Rdtiming {
    #[inline(always)]
    fn default() -> Rdtiming {
        Rdtiming(0)
    }
}
impl core::fmt::Debug for Rdtiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdtiming")
            .field("rdsetup", &self.rdsetup())
            .field("rdstrb", &self.rdstrb())
            .field("rdhold", &self.rdhold())
            .field("halfre", &self.halfre())
            .field("prefetch", &self.prefetch())
            .field("pagemode", &self.pagemode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdtiming {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rdtiming {{ rdsetup: {=u8:?}, rdstrb: {=u8:?}, rdhold: {=u8:?}, halfre: {=bool:?}, prefetch: {=bool:?}, pagemode: {=bool:?} }}" , self . rdsetup () , self . rdstrb () , self . rdhold () , self . halfre () , self . prefetch () , self . pagemode ())
    }
}
#[doc = "Read Timing Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdtiming1(pub u32);
impl Rdtiming1 {
    #[doc = "Read Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Read Setup Time."]
    #[inline(always)]
    pub const fn set_rdsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Read Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Read Strobe Time."]
    #[inline(always)]
    pub const fn set_rdstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Read Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Read Hold Time."]
    #[inline(always)]
    pub const fn set_rdhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfre(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Page Mode Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pagemode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Page Mode Access Enable."]
    #[inline(always)]
    pub const fn set_pagemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Rdtiming1 {
    #[inline(always)]
    fn default() -> Rdtiming1 {
        Rdtiming1(0)
    }
}
impl core::fmt::Debug for Rdtiming1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdtiming1")
            .field("rdsetup", &self.rdsetup())
            .field("rdstrb", &self.rdstrb())
            .field("rdhold", &self.rdhold())
            .field("halfre", &self.halfre())
            .field("prefetch", &self.prefetch())
            .field("pagemode", &self.pagemode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdtiming1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rdtiming1 {{ rdsetup: {=u8:?}, rdstrb: {=u8:?}, rdhold: {=u8:?}, halfre: {=bool:?}, prefetch: {=bool:?}, pagemode: {=bool:?} }}" , self . rdsetup () , self . rdstrb () , self . rdhold () , self . halfre () , self . prefetch () , self . pagemode ())
    }
}
#[doc = "Read Timing Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdtiming2(pub u32);
impl Rdtiming2 {
    #[doc = "Read Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Read Setup Time."]
    #[inline(always)]
    pub const fn set_rdsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Read Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Read Strobe Time."]
    #[inline(always)]
    pub const fn set_rdstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Read Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Read Hold Time."]
    #[inline(always)]
    pub const fn set_rdhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfre(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Page Mode Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pagemode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Page Mode Access Enable."]
    #[inline(always)]
    pub const fn set_pagemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Rdtiming2 {
    #[inline(always)]
    fn default() -> Rdtiming2 {
        Rdtiming2(0)
    }
}
impl core::fmt::Debug for Rdtiming2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdtiming2")
            .field("rdsetup", &self.rdsetup())
            .field("rdstrb", &self.rdstrb())
            .field("rdhold", &self.rdhold())
            .field("halfre", &self.halfre())
            .field("prefetch", &self.prefetch())
            .field("pagemode", &self.pagemode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdtiming2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rdtiming2 {{ rdsetup: {=u8:?}, rdstrb: {=u8:?}, rdhold: {=u8:?}, halfre: {=bool:?}, prefetch: {=bool:?}, pagemode: {=bool:?} }}" , self . rdsetup () , self . rdstrb () , self . rdhold () , self . halfre () , self . prefetch () , self . pagemode ())
    }
}
#[doc = "Read Timing Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdtiming3(pub u32);
impl Rdtiming3 {
    #[doc = "Read Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Read Setup Time."]
    #[inline(always)]
    pub const fn set_rdsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Read Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Read Strobe Time."]
    #[inline(always)]
    pub const fn set_rdstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Read Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn rdhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Read Hold Time."]
    #[inline(always)]
    pub const fn set_rdhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfre(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle REn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Page Mode Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pagemode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Page Mode Access Enable."]
    #[inline(always)]
    pub const fn set_pagemode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Rdtiming3 {
    #[inline(always)]
    fn default() -> Rdtiming3 {
        Rdtiming3(0)
    }
}
impl core::fmt::Debug for Rdtiming3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdtiming3")
            .field("rdsetup", &self.rdsetup())
            .field("rdstrb", &self.rdstrb())
            .field("rdhold", &self.rdhold())
            .field("halfre", &self.halfre())
            .field("prefetch", &self.prefetch())
            .field("pagemode", &self.pagemode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdtiming3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rdtiming3 {{ rdsetup: {=u8:?}, rdstrb: {=u8:?}, rdhold: {=u8:?}, halfre: {=bool:?}, prefetch: {=bool:?}, pagemode: {=bool:?} }}" , self . rdsetup () , self . rdstrb () , self . rdhold () , self . halfre () , self . prefetch () , self . pagemode ())
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
    pub const fn ebiloc(&self) -> super::vals::Ebiloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ebiloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ebiloc(&mut self, val: super::vals::Ebiloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn csloc(&self) -> super::vals::Csloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Csloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_csloc(&mut self, val: super::vals::Csloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn nandloc(&self) -> super::vals::Nandloc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Nandloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_nandloc(&mut self, val: super::vals::Nandloc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn tftloc(&self) -> super::vals::Tftloc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Tftloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_tftloc(&mut self, val: super::vals::Tftloc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
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
            .field("ebiloc", &self.ebiloc())
            .field("csloc", &self.csloc())
            .field("nandloc", &self.nandloc())
            .field("tftloc", &self.tftloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ ebiloc: {:?}, csloc: {:?}, nandloc: {:?}, tftloc: {:?} }}",
            self.ebiloc(),
            self.csloc(),
            self.nandloc(),
            self.tftloc()
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
    pub const fn adloc(&self) -> super::vals::Adloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Adloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_adloc(&mut self, val: super::vals::Adloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn aloc(&self) -> super::vals::Aloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Aloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_aloc(&mut self, val: super::vals::Aloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn rdyloc(&self) -> super::vals::Rdyloc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Rdyloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_rdyloc(&mut self, val: super::vals::Rdyloc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
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
            .field("adloc", &self.adloc())
            .field("aloc", &self.aloc())
            .field("rdyloc", &self.rdyloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc1 {{ adloc: {:?}, aloc: {:?}, rdyloc: {:?} }}",
            self.adloc(),
            self.aloc(),
            self.rdyloc()
        )
    }
}
#[doc = "I/O Routing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "EBI Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ebipen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EBI Pin Enable."]
    #[inline(always)]
    pub const fn set_ebipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EBI_CS0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs0pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_CS0 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EBI_CS1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs1pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_CS1 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EBI_CS2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs2pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_CS2 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EBI_CS3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs3pen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_CS3 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EBI_ALE Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn alepen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_ALE Pin Enable."]
    #[inline(always)]
    pub const fn set_alepen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EBI_ARDY Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ardypen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_ARDY Pin Enable."]
    #[inline(always)]
    pub const fn set_ardypen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EBI_BL\\[1:0\\] Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn blpen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_BL\\[1:0\\] Pin Enable."]
    #[inline(always)]
    pub const fn set_blpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "NANDRE and NANDWE Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nandpen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NANDRE and NANDWE Pin Enable."]
    #[inline(always)]
    pub const fn set_nandpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Sets the Lower Bound for EBI_A Enabling."]
    #[must_use]
    #[inline(always)]
    pub const fn alb(&self) -> super::vals::Alb {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Alb::from_bits(val as u8)
    }
    #[doc = "Sets the Lower Bound for EBI_A Enabling."]
    #[inline(always)]
    pub const fn set_alb(&mut self, val: super::vals::Alb) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "EBI_A Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apen(&self) -> super::vals::Apen {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::Apen::from_bits(val as u8)
    }
    #[doc = "EBI_A Pin Enable."]
    #[inline(always)]
    pub const fn set_apen(&mut self, val: super::vals::Apen) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "EBI_TFT Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFT Pin Enable."]
    #[inline(always)]
    pub const fn set_tftpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "EBI_DATA Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dataenpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_DATA Pin Enable."]
    #[inline(always)]
    pub const fn set_dataenpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "EBI_CSTFT Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cstftpen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_CSTFT Pin Enable."]
    #[inline(always)]
    pub const fn set_cstftpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("ebipen", &self.ebipen())
            .field("cs0pen", &self.cs0pen())
            .field("cs1pen", &self.cs1pen())
            .field("cs2pen", &self.cs2pen())
            .field("cs3pen", &self.cs3pen())
            .field("alepen", &self.alepen())
            .field("ardypen", &self.ardypen())
            .field("blpen", &self.blpen())
            .field("nandpen", &self.nandpen())
            .field("alb", &self.alb())
            .field("apen", &self.apen())
            .field("tftpen", &self.tftpen())
            .field("dataenpen", &self.dataenpen())
            .field("cstftpen", &self.cstftpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ ebipen: {=bool:?}, cs0pen: {=bool:?}, cs1pen: {=bool:?}, cs2pen: {=bool:?}, cs3pen: {=bool:?}, alepen: {=bool:?}, ardypen: {=bool:?}, blpen: {=bool:?}, nandpen: {=bool:?}, alb: {:?}, apen: {:?}, tftpen: {=bool:?}, dataenpen: {=bool:?}, cstftpen: {=bool:?} }}" , self . ebipen () , self . cs0pen () , self . cs1pen () , self . cs2pen () , self . cs3pen () , self . alepen () , self . ardypen () , self . blpen () , self . nandpen () , self . alb () , self . apen () , self . tftpen () , self . dataenpen () , self . cstftpen ())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "EBI Busy With AHB Transaction."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EBI Busy With AHB Transaction."]
    #[inline(always)]
    pub const fn set_ahbact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EBI ECC Generation Active."]
    #[must_use]
    #[inline(always)]
    pub const fn eccact(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EBI ECC Generation Active."]
    #[inline(always)]
    pub const fn set_eccact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EBI_TFTPIXEL0 is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel0empty(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL0 is Empty."]
    #[inline(always)]
    pub const fn set_tftpixel0empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EBI_TFTPIXEL1 is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixel1empty(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL1 is Empty."]
    #[inline(always)]
    pub const fn set_tftpixel1empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EBI_TFTPIXEL0 is Full."]
    #[must_use]
    #[inline(always)]
    pub const fn tftpixelfull(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTPIXEL0 is Full."]
    #[inline(always)]
    pub const fn set_tftpixelfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "EBI Busy With Direct Drive Transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn ddact(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "EBI Busy With Direct Drive Transactions."]
    #[inline(always)]
    pub const fn set_ddact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EBI_TFTDD Register is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn tftddempty(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EBI_TFTDD Register is Empty."]
    #[inline(always)]
    pub const fn set_tftddempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("ahbact", &self.ahbact())
            .field("eccact", &self.eccact())
            .field("tftpixel0empty", &self.tftpixel0empty())
            .field("tftpixel1empty", &self.tftpixel1empty())
            .field("tftpixelfull", &self.tftpixelfull())
            .field("ddact", &self.ddact())
            .field("tftddempty", &self.tftddempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ ahbact: {=bool:?}, eccact: {=bool:?}, tftpixel0empty: {=bool:?}, tftpixel1empty: {=bool:?}, tftpixelfull: {=bool:?}, ddact: {=bool:?}, tftddempty: {=bool:?} }}" , self . ahbact () , self . eccact () , self . tftpixel0empty () , self . tftpixel1empty () , self . tftpixelfull () , self . ddact () , self . tftddempty ())
    }
}
#[doc = "TFT Alpha Blending Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftalpha(pub u32);
impl Tftalpha {
    #[doc = "TFT Alpha Blending Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "TFT Alpha Blending Factor."]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Tftalpha {
    #[inline(always)]
    fn default() -> Tftalpha {
        Tftalpha(0)
    }
}
impl core::fmt::Debug for Tftalpha {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftalpha")
            .field("alpha", &self.alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftalpha {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftalpha {{ alpha: {=u16:?} }}", self.alpha())
    }
}
#[doc = "Color Format Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftcolorformat(pub u32);
impl Tftcolorformat {
    #[doc = "Sprite Pixel Color Format."]
    #[must_use]
    #[inline(always)]
    pub const fn pixel0format(&self) -> super::vals::Pixel0format {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pixel0format::from_bits(val as u8)
    }
    #[doc = "Sprite Pixel Color Format."]
    #[inline(always)]
    pub const fn set_pixel0format(&mut self, val: super::vals::Pixel0format) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Source and Destination Pixel Color Format."]
    #[must_use]
    #[inline(always)]
    pub const fn pixel1format(&self) -> super::vals::Pixel1format {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pixel1format::from_bits(val as u8)
    }
    #[doc = "Source and Destination Pixel Color Format."]
    #[inline(always)]
    pub const fn set_pixel1format(&mut self, val: super::vals::Pixel1format) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Tftcolorformat {
    #[inline(always)]
    fn default() -> Tftcolorformat {
        Tftcolorformat(0)
    }
}
impl core::fmt::Debug for Tftcolorformat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftcolorformat")
            .field("pixel0format", &self.pixel0format())
            .field("pixel1format", &self.pixel1format())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftcolorformat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tftcolorformat {{ pixel0format: {:?}, pixel1format: {:?} }}",
            self.pixel0format(),
            self.pixel1format()
        )
    }
}
#[doc = "TFT Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftctrl(pub u32);
impl Tftctrl {
    #[doc = "TFT Direct Drive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dd(&self) -> super::vals::Dd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dd::from_bits(val as u8)
    }
    #[doc = "TFT Direct Drive Mode."]
    #[inline(always)]
    pub const fn set_dd(&mut self, val: super::vals::Dd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TFT Mask and Blend Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn maskblend(&self) -> super::vals::Maskblend {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Maskblend::from_bits(val as u8)
    }
    #[doc = "TFT Mask and Blend Mode."]
    #[inline(always)]
    pub const fn set_maskblend(&mut self, val: super::vals::Maskblend) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "TFT EBI_DCLK Shift Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftdclken(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TFT EBI_DCLK Shift Enable."]
    #[inline(always)]
    pub const fn set_shiftdclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TFT Frame Base Copy Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn fbctrig(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TFT Frame Base Copy Trigger."]
    #[inline(always)]
    pub const fn set_fbctrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interleave Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> super::vals::Interleave {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Interleave::from_bits(val as u8)
    }
    #[doc = "Interleave Mode."]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: super::vals::Interleave) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Masking/Alpha Blending Color1 Source."]
    #[must_use]
    #[inline(always)]
    pub const fn color1src(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Masking/Alpha Blending Color1 Source."]
    #[inline(always)]
    pub const fn set_color1src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TFT Transaction Width."]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> super::vals::Width {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Width::from_bits(val as u8)
    }
    #[doc = "TFT Transaction Width."]
    #[inline(always)]
    pub const fn set_width(&mut self, val: super::vals::Width) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Alias to Graphics Bank Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aliasbanken(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Alias to Graphics Bank Enable."]
    #[inline(always)]
    pub const fn set_aliasbanken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Graphics Bank."]
    #[must_use]
    #[inline(always)]
    pub const fn banksel(&self) -> super::vals::TftctrlBanksel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::TftctrlBanksel::from_bits(val as u8)
    }
    #[doc = "Graphics Bank."]
    #[inline(always)]
    pub const fn set_banksel(&mut self, val: super::vals::TftctrlBanksel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Graphic Bank Select Aliasing."]
    #[must_use]
    #[inline(always)]
    pub const fn aliasbank(&self) -> super::vals::Aliasbank {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Aliasbank::from_bits(val as u8)
    }
    #[doc = "Graphic Bank Select Aliasing."]
    #[inline(always)]
    pub const fn set_aliasbank(&mut self, val: super::vals::Aliasbank) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
}
impl Default for Tftctrl {
    #[inline(always)]
    fn default() -> Tftctrl {
        Tftctrl(0)
    }
}
impl core::fmt::Debug for Tftctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftctrl")
            .field("dd", &self.dd())
            .field("maskblend", &self.maskblend())
            .field("shiftdclken", &self.shiftdclken())
            .field("fbctrig", &self.fbctrig())
            .field("interleave", &self.interleave())
            .field("color1src", &self.color1src())
            .field("width", &self.width())
            .field("aliasbanken", &self.aliasbanken())
            .field("banksel", &self.banksel())
            .field("aliasbank", &self.aliasbank())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tftctrl {{ dd: {:?}, maskblend: {:?}, shiftdclken: {=bool:?}, fbctrig: {=bool:?}, interleave: {:?}, color1src: {=bool:?}, width: {:?}, aliasbanken: {=bool:?}, banksel: {:?}, aliasbank: {:?} }}" , self . dd () , self . maskblend () , self . shiftdclken () , self . fbctrig () , self . interleave () , self . color1src () , self . width () , self . aliasbanken () , self . banksel () , self . aliasbank ())
    }
}
#[doc = "TFT Direct Drive Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftdd(pub u32);
impl Tftdd {
    #[doc = "TFT Direct Drive Data From Internal Memory."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "TFT Direct Drive Data From Internal Memory."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tftdd {
    #[inline(always)]
    fn default() -> Tftdd {
        Tftdd(0)
    }
}
impl core::fmt::Debug for Tftdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftdd").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftdd {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "TFT Frame Base Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftframebase(pub u32);
impl Tftframebase {
    #[doc = "Frame Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn framebase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Frame Base Address."]
    #[inline(always)]
    pub const fn set_framebase(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Tftframebase {
    #[inline(always)]
    fn default() -> Tftframebase {
        Tftframebase(0)
    }
}
impl core::fmt::Debug for Tftframebase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftframebase")
            .field("framebase", &self.framebase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftframebase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tftframebase {{ framebase: {=u32:?} }}",
            self.framebase()
        )
    }
}
#[doc = "TFT Horizontal Porch Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfthporch(pub u32);
impl Tfthporch {
    #[doc = "Horizontal Synchronization Pulse Width."]
    #[must_use]
    #[inline(always)]
    pub const fn hsync(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Horizontal Synchronization Pulse Width."]
    #[inline(always)]
    pub const fn set_hsync(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Horizontal Front Porch Size."]
    #[must_use]
    #[inline(always)]
    pub const fn hfporch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Horizontal Front Porch Size."]
    #[inline(always)]
    pub const fn set_hfporch(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Horizontal Back Porch Size."]
    #[must_use]
    #[inline(always)]
    pub const fn hbporch(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0xff;
        val as u8
    }
    #[doc = "Horizontal Back Porch Size."]
    #[inline(always)]
    pub const fn set_hbporch(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
    }
    #[doc = "HSYNC Start Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn hsyncstart(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "HSYNC Start Delay."]
    #[inline(always)]
    pub const fn set_hsyncstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for Tfthporch {
    #[inline(always)]
    fn default() -> Tfthporch {
        Tfthporch(0)
    }
}
impl core::fmt::Debug for Tfthporch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfthporch")
            .field("hsync", &self.hsync())
            .field("hfporch", &self.hfporch())
            .field("hbporch", &self.hbporch())
            .field("hsyncstart", &self.hsyncstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfthporch {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tfthporch {{ hsync: {=u8:?}, hfporch: {=u8:?}, hbporch: {=u8:?}, hsyncstart: {=u8:?} }}" , self . hsync () , self . hfporch () , self . hbporch () , self . hsyncstart ())
    }
}
#[doc = "TFT Masking Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftmask(pub u32);
impl Tftmask {
    #[doc = "TFT Mask Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tftmask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "TFT Mask Value."]
    #[inline(always)]
    pub const fn set_tftmask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tftmask {
    #[inline(always)]
    fn default() -> Tftmask {
        Tftmask(0)
    }
}
impl core::fmt::Debug for Tftmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftmask")
            .field("tftmask", &self.tftmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftmask {{ tftmask: {=u32:?} }}", self.tftmask())
    }
}
#[doc = "TFT Alpha Blending Result Pixel Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftpixel(pub u32);
impl Tftpixel {
    #[doc = "Alpha Blending Result."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Alpha Blending Result."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tftpixel {
    #[inline(always)]
    fn default() -> Tftpixel {
        Tftpixel(0)
    }
}
impl core::fmt::Debug for Tftpixel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftpixel")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftpixel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftpixel {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "TFT Pixel 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftpixel0(pub u32);
impl Tftpixel0 {
    #[doc = "RGB Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "RGB Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tftpixel0 {
    #[inline(always)]
    fn default() -> Tftpixel0 {
        Tftpixel0(0)
    }
}
impl core::fmt::Debug for Tftpixel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftpixel0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftpixel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftpixel0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "TFT Pixel 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftpixel1(pub u32);
impl Tftpixel1 {
    #[doc = "RGB Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "RGB Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tftpixel1 {
    #[inline(always)]
    fn default() -> Tftpixel1 {
        Tftpixel1(0)
    }
}
impl core::fmt::Debug for Tftpixel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftpixel1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftpixel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftpixel1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "TFT Polarity Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftpolarity(pub u32);
impl Tftpolarity {
    #[doc = "TFT Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cspol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TFT Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_cspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TFT DCLK Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn dclkpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TFT DCLK Polarity."]
    #[inline(always)]
    pub const fn set_dclkpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TFT DATAEN Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn dataenpol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TFT DATAEN Polarity."]
    #[inline(always)]
    pub const fn set_dataenpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address Latch Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn hsyncpol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Latch Polarity."]
    #[inline(always)]
    pub const fn set_hsyncpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VSYNC Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn vsyncpol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VSYNC Polarity."]
    #[inline(always)]
    pub const fn set_vsyncpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Tftpolarity {
    #[inline(always)]
    fn default() -> Tftpolarity {
        Tftpolarity(0)
    }
}
impl core::fmt::Debug for Tftpolarity {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftpolarity")
            .field("cspol", &self.cspol())
            .field("dclkpol", &self.dclkpol())
            .field("dataenpol", &self.dataenpol())
            .field("hsyncpol", &self.hsyncpol())
            .field("vsyncpol", &self.vsyncpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftpolarity {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tftpolarity {{ cspol: {=bool:?}, dclkpol: {=bool:?}, dataenpol: {=bool:?}, hsyncpol: {=bool:?}, vsyncpol: {=bool:?} }}" , self . cspol () , self . dclkpol () , self . dataenpol () , self . hsyncpol () , self . vsyncpol ())
    }
}
#[doc = "TFT Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftsize(pub u32);
impl Tftsize {
    #[doc = "Horizontal Size (excluding Porches)."]
    #[must_use]
    #[inline(always)]
    pub const fn hsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Horizontal Size (excluding Porches)."]
    #[inline(always)]
    pub const fn set_hsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Vertical Size (excluding Porches)."]
    #[must_use]
    #[inline(always)]
    pub const fn vsz(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Vertical Size (excluding Porches)."]
    #[inline(always)]
    pub const fn set_vsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Tftsize {
    #[inline(always)]
    fn default() -> Tftsize {
        Tftsize(0)
    }
}
impl core::fmt::Debug for Tftsize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftsize")
            .field("hsz", &self.hsz())
            .field("vsz", &self.vsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftsize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tftsize {{ hsz: {=u16:?}, vsz: {=u16:?} }}",
            self.hsz(),
            self.vsz()
        )
    }
}
#[doc = "TFT Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftstatus(pub u32);
impl Tftstatus {
    #[doc = "Horizontal Count."]
    #[must_use]
    #[inline(always)]
    pub const fn hcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Horizontal Count."]
    #[inline(always)]
    pub const fn set_hcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Vertical Count."]
    #[must_use]
    #[inline(always)]
    pub const fn vcnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Vertical Count."]
    #[inline(always)]
    pub const fn set_vcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Tftstatus {
    #[inline(always)]
    fn default() -> Tftstatus {
        Tftstatus(0)
    }
}
impl core::fmt::Debug for Tftstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftstatus")
            .field("hcnt", &self.hcnt())
            .field("vcnt", &self.vcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tftstatus {{ hcnt: {=u16:?}, vcnt: {=u16:?} }}",
            self.hcnt(),
            self.vcnt()
        )
    }
}
#[doc = "TFT Stride Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftstride(pub u32);
impl Tftstride {
    #[doc = "Horizontal Stride."]
    #[must_use]
    #[inline(always)]
    pub const fn hstride(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Horizontal Stride."]
    #[inline(always)]
    pub const fn set_hstride(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Tftstride {
    #[inline(always)]
    fn default() -> Tftstride {
        Tftstride(0)
    }
}
impl core::fmt::Debug for Tftstride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftstride")
            .field("hstride", &self.hstride())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftstride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tftstride {{ hstride: {=u16:?} }}", self.hstride())
    }
}
#[doc = "TFT Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfttiming(pub u32);
impl Tfttiming {
    #[doc = "TFT Direct Drive Transaction (EBI_DCLK) Period."]
    #[must_use]
    #[inline(always)]
    pub const fn dclkperiod(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "TFT Direct Drive Transaction (EBI_DCLK) Period."]
    #[inline(always)]
    pub const fn set_dclkperiod(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "TFT Direct Drive Transaction Start."]
    #[must_use]
    #[inline(always)]
    pub const fn tftstart(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "TFT Direct Drive Transaction Start."]
    #[inline(always)]
    pub const fn set_tftstart(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "TFT Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn tftsetup(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "TFT Setup Time."]
    #[inline(always)]
    pub const fn set_tftsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "TFT Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn tfthold(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "TFT Hold Time."]
    #[inline(always)]
    pub const fn set_tfthold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Tfttiming {
    #[inline(always)]
    fn default() -> Tfttiming {
        Tfttiming(0)
    }
}
impl core::fmt::Debug for Tfttiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfttiming")
            .field("dclkperiod", &self.dclkperiod())
            .field("tftstart", &self.tftstart())
            .field("tftsetup", &self.tftsetup())
            .field("tfthold", &self.tfthold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfttiming {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tfttiming {{ dclkperiod: {=u16:?}, tftstart: {=u16:?}, tftsetup: {=u8:?}, tfthold: {=u8:?} }}" , self . dclkperiod () , self . tftstart () , self . tftsetup () , self . tfthold ())
    }
}
#[doc = "TFT Vertical Porch Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftvporch(pub u32);
impl Tftvporch {
    #[doc = "Vertical Synchronization Pulse Width."]
    #[must_use]
    #[inline(always)]
    pub const fn vsync(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Vertical Synchronization Pulse Width."]
    #[inline(always)]
    pub const fn set_vsync(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Vertical Front Porch Size."]
    #[must_use]
    #[inline(always)]
    pub const fn vfporch(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Vertical Front Porch Size."]
    #[inline(always)]
    pub const fn set_vfporch(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Vertical Back Porch Size."]
    #[must_use]
    #[inline(always)]
    pub const fn vbporch(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Vertical Back Porch Size."]
    #[inline(always)]
    pub const fn set_vbporch(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tftvporch {
    #[inline(always)]
    fn default() -> Tftvporch {
        Tftvporch(0)
    }
}
impl core::fmt::Debug for Tftvporch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tftvporch")
            .field("vsync", &self.vsync())
            .field("vfporch", &self.vfporch())
            .field("vbporch", &self.vbporch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tftvporch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tftvporch {{ vsync: {=u8:?}, vfporch: {=u16:?}, vbporch: {=u16:?} }}",
            self.vsync(),
            self.vfporch(),
            self.vbporch()
        )
    }
}
#[doc = "Write Timing Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrtiming(pub u32);
impl Wrtiming {
    #[doc = "Write Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Write Setup Time."]
    #[inline(always)]
    pub const fn set_wrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Write Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Write Strobe Time."]
    #[inline(always)]
    pub const fn set_wrstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Write Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Write Hold Time."]
    #[inline(always)]
    pub const fn set_wrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfwe(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write Buffer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wbufdis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write Buffer Disable."]
    #[inline(always)]
    pub const fn set_wbufdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Wrtiming {
    #[inline(always)]
    fn default() -> Wrtiming {
        Wrtiming(0)
    }
}
impl core::fmt::Debug for Wrtiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrtiming")
            .field("wrsetup", &self.wrsetup())
            .field("wrstrb", &self.wrstrb())
            .field("wrhold", &self.wrhold())
            .field("halfwe", &self.halfwe())
            .field("wbufdis", &self.wbufdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrtiming {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wrtiming {{ wrsetup: {=u8:?}, wrstrb: {=u8:?}, wrhold: {=u8:?}, halfwe: {=bool:?}, wbufdis: {=bool:?} }}" , self . wrsetup () , self . wrstrb () , self . wrhold () , self . halfwe () , self . wbufdis ())
    }
}
#[doc = "Write Timing Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrtiming1(pub u32);
impl Wrtiming1 {
    #[doc = "Write Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Write Setup Time."]
    #[inline(always)]
    pub const fn set_wrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Write Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Write Strobe Time."]
    #[inline(always)]
    pub const fn set_wrstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Write Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Write Hold Time."]
    #[inline(always)]
    pub const fn set_wrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfwe(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write Buffer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wbufdis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write Buffer Disable."]
    #[inline(always)]
    pub const fn set_wbufdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Wrtiming1 {
    #[inline(always)]
    fn default() -> Wrtiming1 {
        Wrtiming1(0)
    }
}
impl core::fmt::Debug for Wrtiming1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrtiming1")
            .field("wrsetup", &self.wrsetup())
            .field("wrstrb", &self.wrstrb())
            .field("wrhold", &self.wrhold())
            .field("halfwe", &self.halfwe())
            .field("wbufdis", &self.wbufdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrtiming1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wrtiming1 {{ wrsetup: {=u8:?}, wrstrb: {=u8:?}, wrhold: {=u8:?}, halfwe: {=bool:?}, wbufdis: {=bool:?} }}" , self . wrsetup () , self . wrstrb () , self . wrhold () , self . halfwe () , self . wbufdis ())
    }
}
#[doc = "Write Timing Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrtiming2(pub u32);
impl Wrtiming2 {
    #[doc = "Write Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Write Setup Time."]
    #[inline(always)]
    pub const fn set_wrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Write Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Write Strobe Time."]
    #[inline(always)]
    pub const fn set_wrstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Write Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Write Hold Time."]
    #[inline(always)]
    pub const fn set_wrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfwe(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write Buffer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wbufdis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write Buffer Disable."]
    #[inline(always)]
    pub const fn set_wbufdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Wrtiming2 {
    #[inline(always)]
    fn default() -> Wrtiming2 {
        Wrtiming2(0)
    }
}
impl core::fmt::Debug for Wrtiming2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrtiming2")
            .field("wrsetup", &self.wrsetup())
            .field("wrstrb", &self.wrstrb())
            .field("wrhold", &self.wrhold())
            .field("halfwe", &self.halfwe())
            .field("wbufdis", &self.wbufdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrtiming2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wrtiming2 {{ wrsetup: {=u8:?}, wrstrb: {=u8:?}, wrhold: {=u8:?}, halfwe: {=bool:?}, wbufdis: {=bool:?} }}" , self . wrsetup () , self . wrstrb () , self . wrhold () , self . halfwe () , self . wbufdis ())
    }
}
#[doc = "Write Timing Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrtiming3(pub u32);
impl Wrtiming3 {
    #[doc = "Write Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrsetup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Write Setup Time."]
    #[inline(always)]
    pub const fn set_wrsetup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Write Strobe Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrstrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Write Strobe Time."]
    #[inline(always)]
    pub const fn set_wrstrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Write Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn wrhold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Write Hold Time."]
    #[inline(always)]
    pub const fn set_wrhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn halfwe(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle WEn Strobe Duration Enable."]
    #[inline(always)]
    pub const fn set_halfwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write Buffer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wbufdis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write Buffer Disable."]
    #[inline(always)]
    pub const fn set_wbufdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Wrtiming3 {
    #[inline(always)]
    fn default() -> Wrtiming3 {
        Wrtiming3(0)
    }
}
impl core::fmt::Debug for Wrtiming3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrtiming3")
            .field("wrsetup", &self.wrsetup())
            .field("wrstrb", &self.wrstrb())
            .field("wrhold", &self.wrhold())
            .field("halfwe", &self.halfwe())
            .field("wbufdis", &self.wbufdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrtiming3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wrtiming3 {{ wrsetup: {=u8:?}, wrstrb: {=u8:?}, wrhold: {=u8:?}, halfwe: {=bool:?}, wbufdis: {=bool:?} }}" , self . wrsetup () , self . wrstrb () , self . wrhold () , self . halfwe () , self . wbufdis ())
    }
}
