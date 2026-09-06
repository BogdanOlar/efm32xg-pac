#[doc = "CAN0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
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
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Error Count Register."]
    #[inline(always)]
    pub const fn errcnt(self) -> crate::common::Reg<regs::Errcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Bit Timing Register."]
    #[inline(always)]
    pub const fn bittiming(self) -> crate::common::Reg<regs::Bittiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Identification Register."]
    #[inline(always)]
    pub const fn intid(self) -> crate::common::Reg<regs::Intid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Test Register."]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "BRP Extension Register."]
    #[inline(always)]
    pub const fn brpe(self) -> crate::common::Reg<regs::Brpe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Transmission Request Register."]
    #[inline(always)]
    pub const fn transreq(self) -> crate::common::Reg<regs::Transreq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "New Data Register."]
    #[inline(always)]
    pub const fn messagedata(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Message Valid Register."]
    #[inline(always)]
    pub const fn messagestate(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Message Object Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if0if(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Message Object Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn if0ifs(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Message Object Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn if0ifc(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Message Object Interrupt Enable Register."]
    #[inline(always)]
    pub const fn if0ien(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Status Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if1if(self) -> crate::common::Reg<regs::If1if, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Message Object Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn if1ifs(self) -> crate::common::Reg<regs::If1ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Message Object Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn if1ifc(self) -> crate::common::Reg<regs::If1ifc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Status Interrupt Enable Register."]
    #[inline(always)]
    pub const fn if1ien(self) -> crate::common::Reg<regs::If1ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "I/O Routing Register."]
    #[inline(always)]
    pub const fn route(self) -> crate::common::Reg<regs::Route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interface Command Mask Register."]
    #[inline(always)]
    pub const fn mir0_cmdmask(self) -> crate::common::Reg<regs::Mir0Cmdmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Interface Mask Register."]
    #[inline(always)]
    pub const fn mir0_mask(self) -> crate::common::Reg<regs::Mir0Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Interface Arbitration Register."]
    #[inline(always)]
    pub const fn mir0_arb(self) -> crate::common::Reg<regs::Mir0Arb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Interface Message Control Register."]
    #[inline(always)]
    pub const fn mir0_ctrl(self) -> crate::common::Reg<regs::Mir0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Interface Data a Register."]
    #[inline(always)]
    pub const fn mir0_datal(self) -> crate::common::Reg<regs::Mir0Datal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Interface Data B Register."]
    #[inline(always)]
    pub const fn mir0_datah(self) -> crate::common::Reg<regs::Mir0Datah, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Interface Command Request Register."]
    #[inline(always)]
    pub const fn mir0_cmdreq(self) -> crate::common::Reg<regs::Mir0Cmdreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interface Command Mask Register."]
    #[inline(always)]
    pub const fn mir1_cmdmask(self) -> crate::common::Reg<regs::Mir0Cmdmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Interface Mask Register."]
    #[inline(always)]
    pub const fn mir1_mask(self) -> crate::common::Reg<regs::Mir0Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interface Arbitration Register."]
    #[inline(always)]
    pub const fn mir1_arb(self) -> crate::common::Reg<regs::Mir0Arb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Interface Message Control Register."]
    #[inline(always)]
    pub const fn mir1_ctrl(self) -> crate::common::Reg<regs::Mir0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Interface Data a Register."]
    #[inline(always)]
    pub const fn mir1_datal(self) -> crate::common::Reg<regs::Mir0Datal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Interface Data B Register."]
    #[inline(always)]
    pub const fn mir1_datah(self) -> crate::common::Reg<regs::Mir0Datah, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Interface Command Request Register."]
    #[inline(always)]
    pub const fn mir1_cmdreq(self) -> crate::common::Reg<regs::Mir0Cmdreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
}
pub mod regs;
pub mod vals;
