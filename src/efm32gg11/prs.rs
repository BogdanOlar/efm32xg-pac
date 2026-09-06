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
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc3(self) -> crate::common::Reg<regs::Routeloc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc4(self) -> crate::common::Reg<regs::Routeloc4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc5(self) -> crate::common::Reg<regs::Routeloc5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DMA Request 0 Register."]
    #[inline(always)]
    pub const fn dmareq0(self) -> crate::common::Reg<regs::Dmareq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "DMA Request 1 Register."]
    #[inline(always)]
    pub const fn dmareq1(self) -> crate::common::Reg<regs::Dmareq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "PRS Channel Values."]
    #[inline(always)]
    pub const fn peek(self) -> crate::common::Reg<regs::Peek, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch0_ctrl(self) -> crate::common::Reg<regs::Ch0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch1_ctrl(self) -> crate::common::Reg<regs::Ch1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch2_ctrl(self) -> crate::common::Reg<regs::Ch2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch3_ctrl(self) -> crate::common::Reg<regs::Ch3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch4_ctrl(self) -> crate::common::Reg<regs::Ch4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch5_ctrl(self) -> crate::common::Reg<regs::Ch5Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch6_ctrl(self) -> crate::common::Reg<regs::Ch6Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch7_ctrl(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch8_ctrl(self) -> crate::common::Reg<regs::Ch8Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch9_ctrl(self) -> crate::common::Reg<regs::Ch9Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch10_ctrl(self) -> crate::common::Reg<regs::Ch10Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch11_ctrl(self) -> crate::common::Reg<regs::Ch11Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch12_ctrl(self) -> crate::common::Reg<regs::Ch12Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch13_ctrl(self) -> crate::common::Reg<regs::Ch13Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch14_ctrl(self) -> crate::common::Reg<regs::Ch14Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch15_ctrl(self) -> crate::common::Reg<regs::Ch15Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch16_ctrl(self) -> crate::common::Reg<regs::Ch16Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch17_ctrl(self) -> crate::common::Reg<regs::Ch17Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch18_ctrl(self) -> crate::common::Reg<regs::Ch18Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch19_ctrl(self) -> crate::common::Reg<regs::Ch19Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch20_ctrl(self) -> crate::common::Reg<regs::Ch20Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch21_ctrl(self) -> crate::common::Reg<regs::Ch21Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch22_ctrl(self) -> crate::common::Reg<regs::Ch22Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch23_ctrl(self) -> crate::common::Reg<regs::Ch23Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
}
pub mod regs;
pub mod vals;
