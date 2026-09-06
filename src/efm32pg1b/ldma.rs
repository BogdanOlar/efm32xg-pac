#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel Peripheral Request Select Register."]
    #[inline(always)]
    pub const fn reqsel(self) -> crate::common::Reg<regs::Ch7Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Ch7Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel Loop Counter Register."]
    #[inline(always)]
    pub const fn loop_(self) -> crate::common::Reg<regs::Ch7Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Descriptor Source Data Address Register."]
    #[inline(always)]
    pub const fn src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel Descriptor Destination Data Address Register."]
    #[inline(always)]
    pub const fn dst(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel Descriptor Link Structure Address Register."]
    #[inline(always)]
    pub const fn link(self) -> crate::common::Reg<regs::Ch7Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
#[doc = "LDMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldma {
    ptr: *mut u8,
}
unsafe impl Send for Ldma {}
unsafe impl Sync for Ldma {}
impl Ldma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)."]
    #[inline(always)]
    pub const fn sync(self) -> crate::common::Reg<regs::Sync, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA Channel Enable Register (Single-Cycle RMW)."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA Channel Busy Register."]
    #[inline(always)]
    pub const fn chbusy(self) -> crate::common::Reg<regs::Chbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)."]
    #[inline(always)]
    pub const fn chdone(self) -> crate::common::Reg<regs::Chdone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DMA Channel Debug Halt Register."]
    #[inline(always)]
    pub const fn dbghalt(self) -> crate::common::Reg<regs::Dbghalt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "DMA Channel Software Transfer Request Register."]
    #[inline(always)]
    pub const fn swreq(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DMA Channel Request Disable Register."]
    #[inline(always)]
    pub const fn reqdis(self) -> crate::common::Reg<regs::Reqdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "DMA Channel Requests Pending Register."]
    #[inline(always)]
    pub const fn reqpend(self) -> crate::common::Reg<regs::Reqpend, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "DMA Channel Link Load Register."]
    #[inline(always)]
    pub const fn linkload(self) -> crate::common::Reg<regs::Linkload, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "DMA Channel Request Clear Register."]
    #[inline(always)]
    pub const fn reqclear(self) -> crate::common::Reg<regs::Reqclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn ch0(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn ch1(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch2(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch3(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn ch4(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn ch5(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn ch6(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch7(self) -> Channel {
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
