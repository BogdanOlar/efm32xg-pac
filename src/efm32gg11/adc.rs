#[doc = "ADC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Single Channel Control Register."]
    #[inline(always)]
    pub const fn singlectrl(self) -> crate::common::Reg<regs::Singlectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Single Channel Control Register Continued."]
    #[inline(always)]
    pub const fn singlectrlx(self) -> crate::common::Reg<regs::Singlectrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Scan Control Register."]
    #[inline(always)]
    pub const fn scanctrl(self) -> crate::common::Reg<regs::Scanctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Scan Control Register Continued."]
    #[inline(always)]
    pub const fn scanctrlx(self) -> crate::common::Reg<regs::Scanctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Scan Sequence Input Mask Register."]
    #[inline(always)]
    pub const fn scanmask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Input Selection Register for Scan Mode."]
    #[inline(always)]
    pub const fn scaninputsel(self) -> crate::common::Reg<regs::Scaninputsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Negative Input Select Register for Scan."]
    #[inline(always)]
    pub const fn scannegsel(self) -> crate::common::Reg<regs::Scannegsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Compare Threshold Register."]
    #[inline(always)]
    pub const fn cmpthr(self) -> crate::common::Reg<regs::Cmpthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation."]
    #[inline(always)]
    pub const fn biasprog(self) -> crate::common::Reg<regs::Biasprog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Calibration Register."]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Single Conversion Result Data."]
    #[inline(always)]
    pub const fn singledata(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Scan Conversion Result Data."]
    #[inline(always)]
    pub const fn scandata(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Single Conversion Result Data Peek Register."]
    #[inline(always)]
    pub const fn singledatap(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Scan Sequence Result Data Peek Register."]
    #[inline(always)]
    pub const fn scandatap(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Scan Sequence Result Data + Data Source Register."]
    #[inline(always)]
    pub const fn scandatax(self) -> crate::common::Reg<regs::Scandatax, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Scan Sequence Result Data + Data Source Peek Register."]
    #[inline(always)]
    pub const fn scandataxp(self) -> crate::common::Reg<regs::Scandataxp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "APORT Request Status Register."]
    #[inline(always)]
    pub const fn aportreq(self) -> crate::common::Reg<regs::Aportreq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "APORT Conflict Status Register."]
    #[inline(always)]
    pub const fn aportconflict(self) -> crate::common::Reg<regs::Aportconflict, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Single FIFO Count Register."]
    #[inline(always)]
    pub const fn singlefifocount(
        self,
    ) -> crate::common::Reg<regs::Singlefifocount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Scan FIFO Count Register."]
    #[inline(always)]
    pub const fn scanfifocount(self) -> crate::common::Reg<regs::Scanfifocount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Single FIFO Clear Register."]
    #[inline(always)]
    pub const fn singlefifoclear(
        self,
    ) -> crate::common::Reg<regs::Singlefifoclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Scan FIFO Clear Register."]
    #[inline(always)]
    pub const fn scanfifoclear(self) -> crate::common::Reg<regs::Scanfifoclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "APORT Bus Master Disable Register."]
    #[inline(always)]
    pub const fn aportmasterdis(
        self,
    ) -> crate::common::Reg<regs::Aportmasterdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
}
pub mod regs;
pub mod vals;
