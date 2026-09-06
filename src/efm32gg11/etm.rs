#[doc = "ETM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etm {
    ptr: *mut u8,
}
unsafe impl Send for Etm {}
unsafe impl Sync for Etm {}
impl Etm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Main Control Register."]
    #[inline(always)]
    pub const fn etmcr(self) -> crate::common::Reg<regs::Etmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configuration Code Register."]
    #[inline(always)]
    pub const fn etmccr(self) -> crate::common::Reg<regs::Etmccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ETM Trigger Event Register."]
    #[inline(always)]
    pub const fn etmtrigger(self) -> crate::common::Reg<regs::Etmtrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ETM Status Register."]
    #[inline(always)]
    pub const fn etmsr(self) -> crate::common::Reg<regs::Etmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ETM System Configuration Register."]
    #[inline(always)]
    pub const fn etmscr(self) -> crate::common::Reg<regs::Etmscr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ETM TraceEnable Event Register."]
    #[inline(always)]
    pub const fn etmteevr(self) -> crate::common::Reg<regs::Etmteevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "ETM Trace control Register."]
    #[inline(always)]
    pub const fn etmtecr1(self) -> crate::common::Reg<regs::Etmtecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "ETM Fifo Full Level Register."]
    #[inline(always)]
    pub const fn etmfflr(self) -> crate::common::Reg<regs::Etmfflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Counter Reload Value."]
    #[inline(always)]
    pub const fn etmcntrldvr1(self) -> crate::common::Reg<regs::Etmcntrldvr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Synchronisation Frequency Register."]
    #[inline(always)]
    pub const fn etmsyncfr(self) -> crate::common::Reg<regs::Etmsyncfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "ID Register."]
    #[inline(always)]
    pub const fn etmidr(self) -> crate::common::Reg<regs::Etmidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Configuration Code Extension Register."]
    #[inline(always)]
    pub const fn etmccer(self) -> crate::common::Reg<regs::Etmccer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "TraceEnable Start/Stop EmbeddedICE Control Register."]
    #[inline(always)]
    pub const fn etmtesseicr(self) -> crate::common::Reg<regs::Etmtesseicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Timestamp Event Register."]
    #[inline(always)]
    pub const fn etmtsevr(self) -> crate::common::Reg<regs::Etmtsevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "CoreSight Trace ID Register."]
    #[inline(always)]
    pub const fn etmtraceidr(self) -> crate::common::Reg<regs::Etmtraceidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "ETM ID Register 2."]
    #[inline(always)]
    pub const fn etmidr2(self) -> crate::common::Reg<regs::Etmidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Device Power-down Status Register."]
    #[inline(always)]
    pub const fn etmpdsr(self) -> crate::common::Reg<regs::Etmpdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Integration Test Miscellaneous Inputs Register."]
    #[inline(always)]
    pub const fn etmiscin(self) -> crate::common::Reg<regs::Etmiscin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ee0usize) as _) }
    }
    #[doc = "Integration Test Trigger Out Register."]
    #[inline(always)]
    pub const fn ittrigout(self) -> crate::common::Reg<regs::Ittrigout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ee8usize) as _) }
    }
    #[doc = "ETM Integration Test ATB Control 2 Register."]
    #[inline(always)]
    pub const fn etmitatbctr2(self) -> crate::common::Reg<regs::Etmitatbctr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ef0usize) as _) }
    }
    #[doc = "ETM Integration Test ATB Control 0 Register."]
    #[inline(always)]
    pub const fn etmitatbctr0(self) -> crate::common::Reg<regs::Etmitatbctr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ef8usize) as _) }
    }
    #[doc = "ETM Integration Control Register."]
    #[inline(always)]
    pub const fn etmitctrl(self) -> crate::common::Reg<regs::Etmitctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "ETM Claim Tag Set Register."]
    #[inline(always)]
    pub const fn etmclaimset(self) -> crate::common::Reg<regs::Etmclaimset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "ETM Claim Tag Clear Register."]
    #[inline(always)]
    pub const fn etmclaimclr(self) -> crate::common::Reg<regs::Etmclaimclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "ETM Lock Access Register."]
    #[inline(always)]
    pub const fn etmlar(self) -> crate::common::Reg<regs::Etmlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb0usize) as _) }
    }
    #[doc = "Lock Status Register."]
    #[inline(always)]
    pub const fn etmlsr(self) -> crate::common::Reg<regs::Etmlsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "ETM Authentication Status Register."]
    #[inline(always)]
    pub const fn etmauthstatus(self) -> crate::common::Reg<regs::Etmauthstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb8usize) as _) }
    }
    #[doc = "CoreSight Device Type Register."]
    #[inline(always)]
    pub const fn etmdevtype(self) -> crate::common::Reg<regs::Etmdevtype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
    }
    #[doc = "Peripheral ID4 Register."]
    #[inline(always)]
    pub const fn etmpidr4(self) -> crate::common::Reg<regs::Etmpidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Peripheral ID5 Register."]
    #[inline(always)]
    pub const fn etmpidr5(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Peripheral ID6 Register."]
    #[inline(always)]
    pub const fn etmpidr6(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "Peripheral ID7 Register."]
    #[inline(always)]
    pub const fn etmpidr7(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "Peripheral ID0 Register."]
    #[inline(always)]
    pub const fn etmpidr0(self) -> crate::common::Reg<regs::Etmpidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Peripheral ID1 Register."]
    #[inline(always)]
    pub const fn etmpidr1(self) -> crate::common::Reg<regs::Etmpidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Peripheral ID2 Register."]
    #[inline(always)]
    pub const fn etmpidr2(self) -> crate::common::Reg<regs::Etmpidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Peripheral ID3 Register."]
    #[inline(always)]
    pub const fn etmpidr3(self) -> crate::common::Reg<regs::Etmpidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Component ID0 Register."]
    #[inline(always)]
    pub const fn etmcidr0(self) -> crate::common::Reg<regs::Etmcidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Component ID1 Register."]
    #[inline(always)]
    pub const fn etmcidr1(self) -> crate::common::Reg<regs::Etmcidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Component ID2 Register."]
    #[inline(always)]
    pub const fn etmcidr2(self) -> crate::common::Reg<regs::Etmcidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Component ID3 Register."]
    #[inline(always)]
    pub const fn etmcidr3(self) -> crate::common::Reg<regs::Etmcidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
