#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Ctrl(pub u32);
impl Ch0Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch0CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch0CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch0CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch0CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch0CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch0CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch0Ctrl {
    #[inline(always)]
    fn default() -> Ch0Ctrl {
        Ch0Ctrl(0)
    }
}
impl core::fmt::Debug for Ch0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch0Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch10Ctrl(pub u32);
impl Ch10Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch10CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch10CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch10CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch10CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch10CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch10CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch10Ctrl {
    #[inline(always)]
    fn default() -> Ch10Ctrl {
        Ch10Ctrl(0)
    }
}
impl core::fmt::Debug for Ch10Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch10Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch10Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch10Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch11Ctrl(pub u32);
impl Ch11Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch11CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch11CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch11CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch11CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch11CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch11CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch11Ctrl {
    #[inline(always)]
    fn default() -> Ch11Ctrl {
        Ch11Ctrl(0)
    }
}
impl core::fmt::Debug for Ch11Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch11Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch11Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch11Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch12Ctrl(pub u32);
impl Ch12Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch12CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch12CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch12CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch12CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch12CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch12CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch12Ctrl {
    #[inline(always)]
    fn default() -> Ch12Ctrl {
        Ch12Ctrl(0)
    }
}
impl core::fmt::Debug for Ch12Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch12Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch12Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch12Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch13Ctrl(pub u32);
impl Ch13Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch13CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch13CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch13CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch13CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch13CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch13CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch13Ctrl {
    #[inline(always)]
    fn default() -> Ch13Ctrl {
        Ch13Ctrl(0)
    }
}
impl core::fmt::Debug for Ch13Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch13Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch13Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch13Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch14Ctrl(pub u32);
impl Ch14Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch14CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch14CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch14CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch14CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch14CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch14CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch14Ctrl {
    #[inline(always)]
    fn default() -> Ch14Ctrl {
        Ch14Ctrl(0)
    }
}
impl core::fmt::Debug for Ch14Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch14Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch14Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch14Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch15Ctrl(pub u32);
impl Ch15Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch15CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch15CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch15CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch15CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch15CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch15CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch15Ctrl {
    #[inline(always)]
    fn default() -> Ch15Ctrl {
        Ch15Ctrl(0)
    }
}
impl core::fmt::Debug for Ch15Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch15Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch15Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch15Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch16Ctrl(pub u32);
impl Ch16Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch16CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch16CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch16CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch16CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch16CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch16CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch16Ctrl {
    #[inline(always)]
    fn default() -> Ch16Ctrl {
        Ch16Ctrl(0)
    }
}
impl core::fmt::Debug for Ch16Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch16Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch16Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch16Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch17Ctrl(pub u32);
impl Ch17Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch17CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch17CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch17CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch17CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch17CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch17CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch17Ctrl {
    #[inline(always)]
    fn default() -> Ch17Ctrl {
        Ch17Ctrl(0)
    }
}
impl core::fmt::Debug for Ch17Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch17Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch17Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch17Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch18Ctrl(pub u32);
impl Ch18Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch18CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch18CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch18CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch18CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch18CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch18CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch18Ctrl {
    #[inline(always)]
    fn default() -> Ch18Ctrl {
        Ch18Ctrl(0)
    }
}
impl core::fmt::Debug for Ch18Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch18Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch18Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch18Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch19Ctrl(pub u32);
impl Ch19Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch19CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch19CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch19CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch19CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch19CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch19CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch19Ctrl {
    #[inline(always)]
    fn default() -> Ch19Ctrl {
        Ch19Ctrl(0)
    }
}
impl core::fmt::Debug for Ch19Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch19Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch19Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch19Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Ctrl(pub u32);
impl Ch1Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch1CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch1CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch1CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch1CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch1CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch1CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch1Ctrl {
    #[inline(always)]
    fn default() -> Ch1Ctrl {
        Ch1Ctrl(0)
    }
}
impl core::fmt::Debug for Ch1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch1Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch20Ctrl(pub u32);
impl Ch20Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch20CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch20CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch20CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch20CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch20CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch20CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch20Ctrl {
    #[inline(always)]
    fn default() -> Ch20Ctrl {
        Ch20Ctrl(0)
    }
}
impl core::fmt::Debug for Ch20Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch20Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch20Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch20Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch21Ctrl(pub u32);
impl Ch21Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch21CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch21CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch21CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch21CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch21CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch21CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch21Ctrl {
    #[inline(always)]
    fn default() -> Ch21Ctrl {
        Ch21Ctrl(0)
    }
}
impl core::fmt::Debug for Ch21Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch21Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch21Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch21Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch22Ctrl(pub u32);
impl Ch22Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch22CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch22CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch22CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch22CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch22CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch22CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch22Ctrl {
    #[inline(always)]
    fn default() -> Ch22Ctrl {
        Ch22Ctrl(0)
    }
}
impl core::fmt::Debug for Ch22Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch22Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch22Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch22Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch23Ctrl(pub u32);
impl Ch23Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch23CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch23CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch23CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch23CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch23CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch23CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch23Ctrl {
    #[inline(always)]
    fn default() -> Ch23Ctrl {
        Ch23Ctrl(0)
    }
}
impl core::fmt::Debug for Ch23Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch23Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch23Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch23Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Ctrl(pub u32);
impl Ch2Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch2CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch2CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch2CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch2CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch2CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch2CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch2Ctrl {
    #[inline(always)]
    fn default() -> Ch2Ctrl {
        Ch2Ctrl(0)
    }
}
impl core::fmt::Debug for Ch2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch2Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Ctrl(pub u32);
impl Ch3Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch3CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch3CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch3CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch3CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch3CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch3CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch3Ctrl {
    #[inline(always)]
    fn default() -> Ch3Ctrl {
        Ch3Ctrl(0)
    }
}
impl core::fmt::Debug for Ch3Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch3Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Ctrl(pub u32);
impl Ch4Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch4CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch4CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch4CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch4CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch4CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch4CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch4Ctrl {
    #[inline(always)]
    fn default() -> Ch4Ctrl {
        Ch4Ctrl(0)
    }
}
impl core::fmt::Debug for Ch4Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch4Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Ctrl(pub u32);
impl Ch5Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch5CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch5CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch5CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch5CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch5CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch5CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch5Ctrl {
    #[inline(always)]
    fn default() -> Ch5Ctrl {
        Ch5Ctrl(0)
    }
}
impl core::fmt::Debug for Ch5Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch5Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Ctrl(pub u32);
impl Ch6Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch6CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch6CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch6CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch6CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch6CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch6CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch6Ctrl {
    #[inline(always)]
    fn default() -> Ch6Ctrl {
        Ch6Ctrl(0)
    }
}
impl core::fmt::Debug for Ch6Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch6Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Ctrl(pub u32);
impl Ch7Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch7CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch7CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch7CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch7CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch7CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch7CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch7Ctrl {
    #[inline(always)]
    fn default() -> Ch7Ctrl {
        Ch7Ctrl(0)
    }
}
impl core::fmt::Debug for Ch7Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch7Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch8Ctrl(pub u32);
impl Ch8Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch8CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch8CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch8CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch8CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch8CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch8CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch8Ctrl {
    #[inline(always)]
    fn default() -> Ch8Ctrl {
        Ch8Ctrl(0)
    }
}
impl core::fmt::Debug for Ch8Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch8Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch8Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch8Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Channel Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch9Ctrl(pub u32);
impl Ch9Ctrl {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch9CtrlSourcesel {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Ch9CtrlSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch9CtrlSourcesel) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Edge Detect Select."]
    #[must_use]
    #[inline(always)]
    pub const fn edsel(&self) -> super::vals::Ch9CtrlEdsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ch9CtrlEdsel::from_bits(val as u8)
    }
    #[doc = "Edge Detect Select."]
    #[inline(always)]
    pub const fn set_edsel(&mut self, val: super::vals::Ch9CtrlEdsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Stretch Channel Output."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Stretch Channel Output."]
    #[inline(always)]
    pub const fn set_stretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Invert Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Channel."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Or Previous."]
    #[must_use]
    #[inline(always)]
    pub const fn orprev(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Or Previous."]
    #[inline(always)]
    pub const fn set_orprev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "And Next."]
    #[must_use]
    #[inline(always)]
    pub const fn andnext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "And Next."]
    #[inline(always)]
    pub const fn set_andnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Reflex."]
    #[must_use]
    #[inline(always)]
    pub const fn async_(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Reflex."]
    #[inline(always)]
    pub const fn set_async_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ch9Ctrl {
    #[inline(always)]
    fn default() -> Ch9Ctrl {
        Ch9Ctrl(0)
    }
}
impl core::fmt::Debug for Ch9Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch9Ctrl")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .field("edsel", &self.edsel())
            .field("stretch", &self.stretch())
            .field("inv", &self.inv())
            .field("orprev", &self.orprev())
            .field("andnext", &self.andnext())
            .field("async_", &self.async_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch9Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch9Ctrl {{ sigsel: {=u8:?}, sourcesel: {:?}, edsel: {:?}, stretch: {=bool:?}, inv: {=bool:?}, orprev: {=bool:?}, andnext: {=bool:?}, async_: {=bool:?} }}" , self . sigsel () , self . sourcesel () , self . edsel () , self . stretch () , self . inv () , self . orprev () , self . andnext () , self . async_ ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Set Event on PRS."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonprs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set Event on PRS."]
    #[inline(always)]
    pub const fn set_sevonprs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SEVONPRS PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonprssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "SEVONPRS PRS Channel Select."]
    #[inline(always)]
    pub const fn set_sevonprssel(&mut self, val: super::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
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
            .field("sevonprs", &self.sevonprs())
            .field("sevonprssel", &self.sevonprssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ sevonprs: {=bool:?}, sevonprssel: {:?} }}",
            self.sevonprs(),
            self.sevonprssel()
        )
    }
}
#[doc = "DMA Request 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq0(pub u32);
impl Dmareq0 {
    #[doc = "DMA Request 0 PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x1f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DMA Request 0 PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
}
impl Default for Dmareq0 {
    #[inline(always)]
    fn default() -> Dmareq0 {
        Dmareq0(0)
    }
}
impl core::fmt::Debug for Dmareq0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq0")
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmareq0 {{ prssel: {:?} }}", self.prssel())
    }
}
#[doc = "DMA Request 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq1(pub u32);
impl Dmareq1 {
    #[doc = "DMA Request 1 PRS Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn prssel(&self) -> super::vals::Prssel {
        let val = (self.0 >> 6usize) & 0x1f;
        super::vals::Prssel::from_bits(val as u8)
    }
    #[doc = "DMA Request 1 PRS Channel Select."]
    #[inline(always)]
    pub const fn set_prssel(&mut self, val: super::vals::Prssel) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
}
impl Default for Dmareq1 {
    #[inline(always)]
    fn default() -> Dmareq1 {
        Dmareq1(0)
    }
}
impl core::fmt::Debug for Dmareq1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq1")
            .field("prssel", &self.prssel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmareq1 {{ prssel: {:?} }}", self.prssel())
    }
}
#[doc = "PRS Channel Values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek(pub u32);
impl Peek {
    #[doc = "Channel 0 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Current Value."]
    #[inline(always)]
    pub const fn set_ch0val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1val(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Current Value."]
    #[inline(always)]
    pub const fn set_ch1val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2val(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Current Value."]
    #[inline(always)]
    pub const fn set_ch2val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Current Value."]
    #[inline(always)]
    pub const fn set_ch3val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4val(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Current Value."]
    #[inline(always)]
    pub const fn set_ch4val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5val(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Current Value."]
    #[inline(always)]
    pub const fn set_ch5val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6val(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Current Value."]
    #[inline(always)]
    pub const fn set_ch6val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7val(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Current Value."]
    #[inline(always)]
    pub const fn set_ch7val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8val(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Current Value."]
    #[inline(always)]
    pub const fn set_ch8val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Current Value."]
    #[inline(always)]
    pub const fn set_ch9val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10val(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Current Value."]
    #[inline(always)]
    pub const fn set_ch10val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11val(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Current Value."]
    #[inline(always)]
    pub const fn set_ch11val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 12 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12val(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 12 Current Value."]
    #[inline(always)]
    pub const fn set_ch12val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 13 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13val(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 13 Current Value."]
    #[inline(always)]
    pub const fn set_ch13val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 14 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14val(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 14 Current Value."]
    #[inline(always)]
    pub const fn set_ch14val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Channel 15 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15val(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 15 Current Value."]
    #[inline(always)]
    pub const fn set_ch15val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Channel 16 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch16val(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 16 Current Value."]
    #[inline(always)]
    pub const fn set_ch16val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 17 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch17val(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 17 Current Value."]
    #[inline(always)]
    pub const fn set_ch17val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 18 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch18val(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 18 Current Value."]
    #[inline(always)]
    pub const fn set_ch18val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 19 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch19val(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 19 Current Value."]
    #[inline(always)]
    pub const fn set_ch19val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 20 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch20val(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 20 Current Value."]
    #[inline(always)]
    pub const fn set_ch20val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 21 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch21val(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 21 Current Value."]
    #[inline(always)]
    pub const fn set_ch21val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Channel 22 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch22val(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 22 Current Value."]
    #[inline(always)]
    pub const fn set_ch22val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Channel 23 Current Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ch23val(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 23 Current Value."]
    #[inline(always)]
    pub const fn set_ch23val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Peek {
    #[inline(always)]
    fn default() -> Peek {
        Peek(0)
    }
}
impl core::fmt::Debug for Peek {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Peek")
            .field("ch0val", &self.ch0val())
            .field("ch1val", &self.ch1val())
            .field("ch2val", &self.ch2val())
            .field("ch3val", &self.ch3val())
            .field("ch4val", &self.ch4val())
            .field("ch5val", &self.ch5val())
            .field("ch6val", &self.ch6val())
            .field("ch7val", &self.ch7val())
            .field("ch8val", &self.ch8val())
            .field("ch9val", &self.ch9val())
            .field("ch10val", &self.ch10val())
            .field("ch11val", &self.ch11val())
            .field("ch12val", &self.ch12val())
            .field("ch13val", &self.ch13val())
            .field("ch14val", &self.ch14val())
            .field("ch15val", &self.ch15val())
            .field("ch16val", &self.ch16val())
            .field("ch17val", &self.ch17val())
            .field("ch18val", &self.ch18val())
            .field("ch19val", &self.ch19val())
            .field("ch20val", &self.ch20val())
            .field("ch21val", &self.ch21val())
            .field("ch22val", &self.ch22val())
            .field("ch23val", &self.ch23val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Peek {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Peek {{ ch0val: {=bool:?}, ch1val: {=bool:?}, ch2val: {=bool:?}, ch3val: {=bool:?}, ch4val: {=bool:?}, ch5val: {=bool:?}, ch6val: {=bool:?}, ch7val: {=bool:?}, ch8val: {=bool:?}, ch9val: {=bool:?}, ch10val: {=bool:?}, ch11val: {=bool:?}, ch12val: {=bool:?}, ch13val: {=bool:?}, ch14val: {=bool:?}, ch15val: {=bool:?}, ch16val: {=bool:?}, ch17val: {=bool:?}, ch18val: {=bool:?}, ch19val: {=bool:?}, ch20val: {=bool:?}, ch21val: {=bool:?}, ch22val: {=bool:?}, ch23val: {=bool:?} }}" , self . ch0val () , self . ch1val () , self . ch2val () , self . ch3val () , self . ch4val () , self . ch5val () , self . ch6val () , self . ch7val () , self . ch8val () , self . ch9val () , self . ch10val () , self . ch11val () , self . ch12val () , self . ch13val () , self . ch14val () , self . ch15val () , self . ch16val () , self . ch17val () , self . ch18val () , self . ch19val () , self . ch20val () , self . ch21val () , self . ch22val () , self . ch23val ())
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
    pub const fn ch0loc(&self) -> super::vals::Ch0loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch0loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch0loc(&mut self, val: super::vals::Ch0loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1loc(&self) -> super::vals::Ch1loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch1loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch1loc(&mut self, val: super::vals::Ch1loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2loc(&self) -> super::vals::Ch2loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch2loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch2loc(&mut self, val: super::vals::Ch2loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3loc(&self) -> super::vals::Ch3loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch3loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch3loc(&mut self, val: super::vals::Ch3loc) {
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
            .field("ch0loc", &self.ch0loc())
            .field("ch1loc", &self.ch1loc())
            .field("ch2loc", &self.ch2loc())
            .field("ch3loc", &self.ch3loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ ch0loc: {:?}, ch1loc: {:?}, ch2loc: {:?}, ch3loc: {:?} }}",
            self.ch0loc(),
            self.ch1loc(),
            self.ch2loc(),
            self.ch3loc()
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
    pub const fn ch4loc(&self) -> super::vals::Ch4loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch4loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch4loc(&mut self, val: super::vals::Ch4loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5loc(&self) -> super::vals::Ch5loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch5loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch5loc(&mut self, val: super::vals::Ch5loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6loc(&self) -> super::vals::Ch6loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch6loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch6loc(&mut self, val: super::vals::Ch6loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7loc(&self) -> super::vals::Ch7loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch7loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch7loc(&mut self, val: super::vals::Ch7loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
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
            .field("ch4loc", &self.ch4loc())
            .field("ch5loc", &self.ch5loc())
            .field("ch6loc", &self.ch6loc())
            .field("ch7loc", &self.ch7loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc1 {{ ch4loc: {:?}, ch5loc: {:?}, ch6loc: {:?}, ch7loc: {:?} }}",
            self.ch4loc(),
            self.ch5loc(),
            self.ch6loc(),
            self.ch7loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc2(pub u32);
impl Routeloc2 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8loc(&self) -> super::vals::Ch8loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch8loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch8loc(&mut self, val: super::vals::Ch8loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9loc(&self) -> super::vals::Ch9loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch9loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch9loc(&mut self, val: super::vals::Ch9loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10loc(&self) -> super::vals::Ch10loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch10loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch10loc(&mut self, val: super::vals::Ch10loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11loc(&self) -> super::vals::Ch11loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch11loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch11loc(&mut self, val: super::vals::Ch11loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Routeloc2 {
    #[inline(always)]
    fn default() -> Routeloc2 {
        Routeloc2(0)
    }
}
impl core::fmt::Debug for Routeloc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc2")
            .field("ch8loc", &self.ch8loc())
            .field("ch9loc", &self.ch9loc())
            .field("ch10loc", &self.ch10loc())
            .field("ch11loc", &self.ch11loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc2 {{ ch8loc: {:?}, ch9loc: {:?}, ch10loc: {:?}, ch11loc: {:?} }}",
            self.ch8loc(),
            self.ch9loc(),
            self.ch10loc(),
            self.ch11loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc3(pub u32);
impl Routeloc3 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12loc(&self) -> super::vals::Ch12loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch12loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch12loc(&mut self, val: super::vals::Ch12loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13loc(&self) -> super::vals::Ch13loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch13loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch13loc(&mut self, val: super::vals::Ch13loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14loc(&self) -> super::vals::Ch14loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch14loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch14loc(&mut self, val: super::vals::Ch14loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15loc(&self) -> super::vals::Ch15loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch15loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch15loc(&mut self, val: super::vals::Ch15loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Routeloc3 {
    #[inline(always)]
    fn default() -> Routeloc3 {
        Routeloc3(0)
    }
}
impl core::fmt::Debug for Routeloc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc3")
            .field("ch12loc", &self.ch12loc())
            .field("ch13loc", &self.ch13loc())
            .field("ch14loc", &self.ch14loc())
            .field("ch15loc", &self.ch15loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc3 {{ ch12loc: {:?}, ch13loc: {:?}, ch14loc: {:?}, ch15loc: {:?} }}",
            self.ch12loc(),
            self.ch13loc(),
            self.ch14loc(),
            self.ch15loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc4(pub u32);
impl Routeloc4 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch16loc(&self) -> super::vals::Ch16loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch16loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch16loc(&mut self, val: super::vals::Ch16loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch17loc(&self) -> super::vals::Ch17loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch17loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch17loc(&mut self, val: super::vals::Ch17loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch18loc(&self) -> super::vals::Ch18loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch18loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch18loc(&mut self, val: super::vals::Ch18loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch19loc(&self) -> super::vals::Ch19loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch19loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch19loc(&mut self, val: super::vals::Ch19loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Routeloc4 {
    #[inline(always)]
    fn default() -> Routeloc4 {
        Routeloc4(0)
    }
}
impl core::fmt::Debug for Routeloc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc4")
            .field("ch16loc", &self.ch16loc())
            .field("ch17loc", &self.ch17loc())
            .field("ch18loc", &self.ch18loc())
            .field("ch19loc", &self.ch19loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc4 {{ ch16loc: {:?}, ch17loc: {:?}, ch18loc: {:?}, ch19loc: {:?} }}",
            self.ch16loc(),
            self.ch17loc(),
            self.ch18loc(),
            self.ch19loc()
        )
    }
}
#[doc = "I/O Routing Location Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc5(pub u32);
impl Routeloc5 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch20loc(&self) -> super::vals::Ch20loc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Ch20loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch20loc(&mut self, val: super::vals::Ch20loc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch21loc(&self) -> super::vals::Ch21loc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Ch21loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch21loc(&mut self, val: super::vals::Ch21loc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch22loc(&self) -> super::vals::Ch22loc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch22loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch22loc(&mut self, val: super::vals::Ch22loc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn ch23loc(&self) -> super::vals::Ch23loc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Ch23loc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_ch23loc(&mut self, val: super::vals::Ch23loc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Routeloc5 {
    #[inline(always)]
    fn default() -> Routeloc5 {
        Routeloc5(0)
    }
}
impl core::fmt::Debug for Routeloc5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc5")
            .field("ch20loc", &self.ch20loc())
            .field("ch21loc", &self.ch21loc())
            .field("ch22loc", &self.ch22loc())
            .field("ch23loc", &self.ch23loc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc5 {{ ch20loc: {:?}, ch21loc: {:?}, ch22loc: {:?}, ch23loc: {:?} }}",
            self.ch20loc(),
            self.ch21loc(),
            self.ch22loc(),
            self.ch23loc()
        )
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CH0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CH0 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CH1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CH1 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CH2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CH2 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CH3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CH3 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CH4 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4pen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CH4 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CH5 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CH5 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CH6 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6pen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CH6 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CH7 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7pen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CH7 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CH8 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CH8 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch8pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CH9 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CH9 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch9pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CH10 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10pen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CH10 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch10pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CH11 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11pen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CH11 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch11pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CH12 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12pen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CH12 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch12pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CH13 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13pen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CH13 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch13pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CH14 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14pen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CH14 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch14pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CH15 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15pen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CH15 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch15pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CH16 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch16pen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CH16 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch16pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CH17 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch17pen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CH17 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch17pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CH18 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch18pen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CH18 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch18pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CH19 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch19pen(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CH19 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch19pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CH20 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch20pen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CH20 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch20pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CH21 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch21pen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CH21 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch21pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "CH22 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch22pen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CH22 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch22pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CH23 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ch23pen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CH23 Pin Enable."]
    #[inline(always)]
    pub const fn set_ch23pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("ch0pen", &self.ch0pen())
            .field("ch1pen", &self.ch1pen())
            .field("ch2pen", &self.ch2pen())
            .field("ch3pen", &self.ch3pen())
            .field("ch4pen", &self.ch4pen())
            .field("ch5pen", &self.ch5pen())
            .field("ch6pen", &self.ch6pen())
            .field("ch7pen", &self.ch7pen())
            .field("ch8pen", &self.ch8pen())
            .field("ch9pen", &self.ch9pen())
            .field("ch10pen", &self.ch10pen())
            .field("ch11pen", &self.ch11pen())
            .field("ch12pen", &self.ch12pen())
            .field("ch13pen", &self.ch13pen())
            .field("ch14pen", &self.ch14pen())
            .field("ch15pen", &self.ch15pen())
            .field("ch16pen", &self.ch16pen())
            .field("ch17pen", &self.ch17pen())
            .field("ch18pen", &self.ch18pen())
            .field("ch19pen", &self.ch19pen())
            .field("ch20pen", &self.ch20pen())
            .field("ch21pen", &self.ch21pen())
            .field("ch22pen", &self.ch22pen())
            .field("ch23pen", &self.ch23pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ ch0pen: {=bool:?}, ch1pen: {=bool:?}, ch2pen: {=bool:?}, ch3pen: {=bool:?}, ch4pen: {=bool:?}, ch5pen: {=bool:?}, ch6pen: {=bool:?}, ch7pen: {=bool:?}, ch8pen: {=bool:?}, ch9pen: {=bool:?}, ch10pen: {=bool:?}, ch11pen: {=bool:?}, ch12pen: {=bool:?}, ch13pen: {=bool:?}, ch14pen: {=bool:?}, ch15pen: {=bool:?}, ch16pen: {=bool:?}, ch17pen: {=bool:?}, ch18pen: {=bool:?}, ch19pen: {=bool:?}, ch20pen: {=bool:?}, ch21pen: {=bool:?}, ch22pen: {=bool:?}, ch23pen: {=bool:?} }}" , self . ch0pen () , self . ch1pen () , self . ch2pen () , self . ch3pen () , self . ch4pen () , self . ch5pen () , self . ch6pen () , self . ch7pen () , self . ch8pen () , self . ch9pen () , self . ch10pen () , self . ch11pen () , self . ch12pen () , self . ch13pen () , self . ch14pen () , self . ch15pen () , self . ch16pen () , self . ch17pen () , self . ch18pen () , self . ch19pen () , self . ch20pen () , self . ch21pen () , self . ch22pen () , self . ch23pen ())
    }
}
#[doc = "Software Level Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swlevel(pub u32);
impl Swlevel {
    #[doc = "Channel 0 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0level(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Software Level."]
    #[inline(always)]
    pub const fn set_ch0level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1level(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Software Level."]
    #[inline(always)]
    pub const fn set_ch1level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2level(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Software Level."]
    #[inline(always)]
    pub const fn set_ch2level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3level(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Software Level."]
    #[inline(always)]
    pub const fn set_ch3level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4level(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Software Level."]
    #[inline(always)]
    pub const fn set_ch4level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5level(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Software Level."]
    #[inline(always)]
    pub const fn set_ch5level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6level(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Software Level."]
    #[inline(always)]
    pub const fn set_ch6level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7level(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Software Level."]
    #[inline(always)]
    pub const fn set_ch7level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8level(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Software Level."]
    #[inline(always)]
    pub const fn set_ch8level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9level(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Software Level."]
    #[inline(always)]
    pub const fn set_ch9level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10level(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Software Level."]
    #[inline(always)]
    pub const fn set_ch10level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11level(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Software Level."]
    #[inline(always)]
    pub const fn set_ch11level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 12 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12level(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 12 Software Level."]
    #[inline(always)]
    pub const fn set_ch12level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 13 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13level(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 13 Software Level."]
    #[inline(always)]
    pub const fn set_ch13level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 14 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14level(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 14 Software Level."]
    #[inline(always)]
    pub const fn set_ch14level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Channel 15 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15level(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 15 Software Level."]
    #[inline(always)]
    pub const fn set_ch15level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Channel 16 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch16level(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 16 Software Level."]
    #[inline(always)]
    pub const fn set_ch16level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 17 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch17level(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 17 Software Level."]
    #[inline(always)]
    pub const fn set_ch17level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 18 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch18level(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 18 Software Level."]
    #[inline(always)]
    pub const fn set_ch18level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 19 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch19level(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 19 Software Level."]
    #[inline(always)]
    pub const fn set_ch19level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 20 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch20level(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 20 Software Level."]
    #[inline(always)]
    pub const fn set_ch20level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 21 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch21level(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 21 Software Level."]
    #[inline(always)]
    pub const fn set_ch21level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Channel 22 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch22level(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 22 Software Level."]
    #[inline(always)]
    pub const fn set_ch22level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Channel 23 Software Level."]
    #[must_use]
    #[inline(always)]
    pub const fn ch23level(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 23 Software Level."]
    #[inline(always)]
    pub const fn set_ch23level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Swlevel {
    #[inline(always)]
    fn default() -> Swlevel {
        Swlevel(0)
    }
}
impl core::fmt::Debug for Swlevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swlevel")
            .field("ch0level", &self.ch0level())
            .field("ch1level", &self.ch1level())
            .field("ch2level", &self.ch2level())
            .field("ch3level", &self.ch3level())
            .field("ch4level", &self.ch4level())
            .field("ch5level", &self.ch5level())
            .field("ch6level", &self.ch6level())
            .field("ch7level", &self.ch7level())
            .field("ch8level", &self.ch8level())
            .field("ch9level", &self.ch9level())
            .field("ch10level", &self.ch10level())
            .field("ch11level", &self.ch11level())
            .field("ch12level", &self.ch12level())
            .field("ch13level", &self.ch13level())
            .field("ch14level", &self.ch14level())
            .field("ch15level", &self.ch15level())
            .field("ch16level", &self.ch16level())
            .field("ch17level", &self.ch17level())
            .field("ch18level", &self.ch18level())
            .field("ch19level", &self.ch19level())
            .field("ch20level", &self.ch20level())
            .field("ch21level", &self.ch21level())
            .field("ch22level", &self.ch22level())
            .field("ch23level", &self.ch23level())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swlevel {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swlevel {{ ch0level: {=bool:?}, ch1level: {=bool:?}, ch2level: {=bool:?}, ch3level: {=bool:?}, ch4level: {=bool:?}, ch5level: {=bool:?}, ch6level: {=bool:?}, ch7level: {=bool:?}, ch8level: {=bool:?}, ch9level: {=bool:?}, ch10level: {=bool:?}, ch11level: {=bool:?}, ch12level: {=bool:?}, ch13level: {=bool:?}, ch14level: {=bool:?}, ch15level: {=bool:?}, ch16level: {=bool:?}, ch17level: {=bool:?}, ch18level: {=bool:?}, ch19level: {=bool:?}, ch20level: {=bool:?}, ch21level: {=bool:?}, ch22level: {=bool:?}, ch23level: {=bool:?} }}" , self . ch0level () , self . ch1level () , self . ch2level () , self . ch3level () , self . ch4level () , self . ch5level () , self . ch6level () , self . ch7level () , self . ch8level () , self . ch9level () , self . ch10level () , self . ch11level () , self . ch12level () , self . ch13level () , self . ch14level () , self . ch15level () , self . ch16level () , self . ch17level () , self . ch18level () , self . ch19level () , self . ch20level () , self . ch21level () , self . ch22level () , self . ch23level ())
    }
}
#[doc = "Software Pulse Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swpulse(pub u32);
impl Swpulse {
    #[doc = "Channel 0 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch0pulse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch0pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch1pulse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch1pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch2pulse(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch2pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch3pulse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch3pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch4pulse(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch4pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch5pulse(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch5pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch6pulse(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch6pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch7pulse(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch7pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch8pulse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 8 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch8pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch9pulse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 9 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch9pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch10pulse(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 10 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch10pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch11pulse(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 11 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch11pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 12 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch12pulse(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 12 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch12pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 13 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch13pulse(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 13 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch13pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 14 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch14pulse(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 14 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch14pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Channel 15 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch15pulse(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 15 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch15pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Channel 16 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch16pulse(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 16 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch16pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 17 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch17pulse(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 17 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch17pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 18 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch18pulse(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 18 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch18pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 19 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch19pulse(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 19 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch19pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 20 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch20pulse(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 20 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch20pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 21 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch21pulse(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 21 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch21pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Channel 22 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch22pulse(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 22 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch22pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Channel 23 Pulse Generation."]
    #[must_use]
    #[inline(always)]
    pub const fn ch23pulse(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 23 Pulse Generation."]
    #[inline(always)]
    pub const fn set_ch23pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Swpulse {
    #[inline(always)]
    fn default() -> Swpulse {
        Swpulse(0)
    }
}
impl core::fmt::Debug for Swpulse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swpulse")
            .field("ch0pulse", &self.ch0pulse())
            .field("ch1pulse", &self.ch1pulse())
            .field("ch2pulse", &self.ch2pulse())
            .field("ch3pulse", &self.ch3pulse())
            .field("ch4pulse", &self.ch4pulse())
            .field("ch5pulse", &self.ch5pulse())
            .field("ch6pulse", &self.ch6pulse())
            .field("ch7pulse", &self.ch7pulse())
            .field("ch8pulse", &self.ch8pulse())
            .field("ch9pulse", &self.ch9pulse())
            .field("ch10pulse", &self.ch10pulse())
            .field("ch11pulse", &self.ch11pulse())
            .field("ch12pulse", &self.ch12pulse())
            .field("ch13pulse", &self.ch13pulse())
            .field("ch14pulse", &self.ch14pulse())
            .field("ch15pulse", &self.ch15pulse())
            .field("ch16pulse", &self.ch16pulse())
            .field("ch17pulse", &self.ch17pulse())
            .field("ch18pulse", &self.ch18pulse())
            .field("ch19pulse", &self.ch19pulse())
            .field("ch20pulse", &self.ch20pulse())
            .field("ch21pulse", &self.ch21pulse())
            .field("ch22pulse", &self.ch22pulse())
            .field("ch23pulse", &self.ch23pulse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpulse {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Swpulse {{ ch0pulse: {=bool:?}, ch1pulse: {=bool:?}, ch2pulse: {=bool:?}, ch3pulse: {=bool:?}, ch4pulse: {=bool:?}, ch5pulse: {=bool:?}, ch6pulse: {=bool:?}, ch7pulse: {=bool:?}, ch8pulse: {=bool:?}, ch9pulse: {=bool:?}, ch10pulse: {=bool:?}, ch11pulse: {=bool:?}, ch12pulse: {=bool:?}, ch13pulse: {=bool:?}, ch14pulse: {=bool:?}, ch15pulse: {=bool:?}, ch16pulse: {=bool:?}, ch17pulse: {=bool:?}, ch18pulse: {=bool:?}, ch19pulse: {=bool:?}, ch20pulse: {=bool:?}, ch21pulse: {=bool:?}, ch22pulse: {=bool:?}, ch23pulse: {=bool:?} }}" , self . ch0pulse () , self . ch1pulse () , self . ch2pulse () , self . ch3pulse () , self . ch4pulse () , self . ch5pulse () , self . ch6pulse () , self . ch7pulse () , self . ch8pulse () , self . ch9pulse () , self . ch10pulse () , self . ch11pulse () , self . ch12pulse () , self . ch13pulse () , self . ch14pulse () , self . ch15pulse () , self . ch16pulse () , self . ch17pulse () , self . ch18pulse () , self . ch19pulse () , self . ch20pulse () , self . ch21pulse () , self . ch22pulse () , self . ch23pulse ())
    }
}
