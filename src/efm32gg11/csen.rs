#[doc = "CSEN."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csen {
    ptr: *mut u8,
}
unsafe impl Send for Csen {}
unsafe impl Sync for Csen {}
impl Csen {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timing Control."]
    #[inline(always)]
    pub const fn timctrl(self) -> crate::common::Reg<regs::Timctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PRS Select."]
    #[inline(always)]
    pub const fn prssel(self) -> crate::common::Reg<regs::Prssel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Output Data."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Scan Channel Mask 0."]
    #[inline(always)]
    pub const fn scanmask0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Scan Input Selection 0."]
    #[inline(always)]
    pub const fn scaninputsel0(self) -> crate::common::Reg<regs::Scaninputsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Scan Channel Mask 1."]
    #[inline(always)]
    pub const fn scanmask1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Scan Input Selection 1."]
    #[inline(always)]
    pub const fn scaninputsel1(self) -> crate::common::Reg<regs::Scaninputsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "APORT Request Status."]
    #[inline(always)]
    pub const fn aportreq(self) -> crate::common::Reg<regs::Aportreq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "APORT Request Conflict."]
    #[inline(always)]
    pub const fn aportconflict(self) -> crate::common::Reg<regs::Aportconflict, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Comparator Threshold."]
    #[inline(always)]
    pub const fn cmpthr(self) -> crate::common::Reg<regs::Cmpthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Exponential Moving Average."]
    #[inline(always)]
    pub const fn ema(self) -> crate::common::Reg<regs::Ema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Exponential Moving Average Control."]
    #[inline(always)]
    pub const fn emactrl(self) -> crate::common::Reg<regs::Emactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Single Conversion Control."]
    #[inline(always)]
    pub const fn singlectrl(self) -> crate::common::Reg<regs::Singlectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Delta Modulation Baseline."]
    #[inline(always)]
    pub const fn dmbaseline(self) -> crate::common::Reg<regs::Dmbaseline, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Delta Modulation Configuration."]
    #[inline(always)]
    pub const fn dmcfg(self) -> crate::common::Reg<regs::Dmcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn anactrl(self) -> crate::common::Reg<regs::Anactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Interrupt Flag Set."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt Flag Clear."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
}
pub mod regs;
pub mod vals;
