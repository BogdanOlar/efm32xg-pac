#[doc = "GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn port_a(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn port_b(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn port_c(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn port_d(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn port_e(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn port_f(self) -> Port {
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "External Interrupt Port Select Low Register."]
    #[inline(always)]
    pub const fn extipsell(self) -> crate::common::Reg<regs::Extipsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "External Interrupt Port Select High Register."]
    #[inline(always)]
    pub const fn extipselh(self) -> crate::common::Reg<regs::Extipselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low Register."]
    #[inline(always)]
    pub const fn extipinsell(self) -> crate::common::Reg<regs::Extipinsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High Register."]
    #[inline(always)]
    pub const fn extipinselh(self) -> crate::common::Reg<regs::Extipinselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger Register."]
    #[inline(always)]
    pub const fn extirise(self) -> crate::common::Reg<regs::Extirise, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger Register."]
    #[inline(always)]
    pub const fn extifall(self) -> crate::common::Reg<regs::Extifall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "External Interrupt Level Register."]
    #[inline(always)]
    pub const fn extilevel(self) -> crate::common::Reg<regs::Extilevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "EM4 Wake Up Enable Register."]
    #[inline(always)]
    pub const fn em4wuen(self) -> crate::common::Reg<regs::Em4wuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "I/O Routing Pin Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Input Sense Register."]
    #[inline(always)]
    pub const fn insense(self) -> crate::common::Reg<regs::Insense, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Configuration Lock Register."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port {
    ptr: *mut u8,
}
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Port Pin Mode Low Register."]
    #[inline(always)]
    pub const fn model(self) -> crate::common::Reg<regs::Model, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Port Pin Mode High Register."]
    #[inline(always)]
    pub const fn modeh(self) -> crate::common::Reg<regs::Modeh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Port Data Out Register."]
    #[inline(always)]
    pub const fn dout(self) -> crate::common::Reg<regs::Dout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Port Data Out Toggle Register."]
    #[inline(always)]
    pub const fn douttgl(self) -> crate::common::Reg<regs::Douttgl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Port Data in Register."]
    #[inline(always)]
    pub const fn din(self) -> crate::common::Reg<regs::Din, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Port Unlocked Pins Register."]
    #[inline(always)]
    pub const fn pinlockn(self) -> crate::common::Reg<regs::Pinlockn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Over Voltage Disable for All Modes."]
    #[inline(always)]
    pub const fn ovtdis(self) -> crate::common::Reg<regs::Ovtdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs;
pub mod vals;
