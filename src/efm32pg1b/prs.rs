#[doc = "PRS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs {
    ptr: *mut u8,
}
unsafe impl Send for Prs {}
unsafe impl Sync for Prs {}
impl Prs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software Pulse Register."]
    #[inline(always)]
    pub const fn swpulse(self) -> crate::common::Reg<regs::Swpulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Software Level Register."]
    #[inline(always)]
    pub const fn swlevel(self) -> crate::common::Reg<regs::Swlevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "I/O Routing Pin Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc1(self) -> crate::common::Reg<regs::Routeloc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc2(self) -> crate::common::Reg<regs::Routeloc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA Request 0 Register."]
    #[inline(always)]
    pub const fn dmareq0(self) -> crate::common::Reg<regs::DmaReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Request 1 Register."]
    #[inline(always)]
    pub const fn dmareq1(self) -> crate::common::Reg<regs::DmaReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PRS Channel Values."]
    #[inline(always)]
    pub const fn peek(self) -> crate::common::Reg<regs::Peek, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch_ctrl(self, n: usize) -> crate::common::Reg<regs::ChCtrl, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
