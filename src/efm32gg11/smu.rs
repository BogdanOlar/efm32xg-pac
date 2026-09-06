#[doc = "SMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu {
    ptr: *mut u8,
}
unsafe impl Send for Smu {}
unsafe impl Sync for Smu {}
impl Smu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "PPU Control Register."]
    #[inline(always)]
    pub const fn ppuctrl(self) -> crate::common::Reg<regs::Ppuctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PPU Privilege Access Type Descriptor 0."]
    #[inline(always)]
    pub const fn ppupatd0(self) -> crate::common::Reg<regs::Ppupatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PPU Privilege Access Type Descriptor 1."]
    #[inline(always)]
    pub const fn ppupatd1(self) -> crate::common::Reg<regs::Ppupatd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "PPU Privilege Access Type Descriptor 2."]
    #[inline(always)]
    pub const fn ppupatd2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "PPU Fault Status."]
    #[inline(always)]
    pub const fn ppufs(self) -> crate::common::Reg<regs::Ppufs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
}
pub mod regs;
pub mod vals;
