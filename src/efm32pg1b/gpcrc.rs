#[doc = "GPCRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcrc {
    ptr: *mut u8,
}
unsafe impl Send for Gpcrc {}
unsafe impl Sync for Gpcrc {}
impl Gpcrc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CRC Init Value."]
    #[inline(always)]
    pub const fn init(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC Polynomial Value."]
    #[inline(always)]
    pub const fn poly(self) -> crate::common::Reg<regs::Poly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Input 32-bit Data Register."]
    #[inline(always)]
    pub const fn inputdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Input 16-bit Data Register."]
    #[inline(always)]
    pub const fn inputdatahword(
        self,
    ) -> crate::common::Reg<regs::Inputdatahword, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Input 8-bit Data Register."]
    #[inline(always)]
    pub const fn inputdatabyte(self) -> crate::common::Reg<regs::Inputdatabyte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CRC Data Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CRC Data Reverse Register."]
    #[inline(always)]
    pub const fn datarev(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CRC Data Byte Reverse Register."]
    #[inline(always)]
    pub const fn databyterev(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;
