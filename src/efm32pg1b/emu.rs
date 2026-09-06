#[doc = "EMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emu {
    ptr: *mut u8,
}
unsafe impl Send for Emu {}
unsafe impl Sync for Emu {}
impl Emu {
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
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Configuration Lock Register."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Memory Control Register."]
    #[inline(always)]
    pub const fn ram0ctrl(self) -> crate::common::Reg<regs::Ram0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "EM4 Control Register."]
    #[inline(always)]
    pub const fn em4ctrl(self) -> crate::common::Reg<regs::Em4ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Temperature Limits for Interrupt Generation."]
    #[inline(always)]
    pub const fn templimits(self) -> crate::common::Reg<regs::Templimits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Value of Last Temperature Measurement."]
    #[inline(always)]
    pub const fn temp(self) -> crate::common::Reg<regs::Temp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Regulator and Supply Lock Register."]
    #[inline(always)]
    pub const fn pwrlock(self) -> crate::common::Reg<regs::Pwrlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Power Configuration Register."]
    #[inline(always)]
    pub const fn pwrcfg(self) -> crate::common::Reg<regs::Pwrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Power Control Register."]
    #[inline(always)]
    pub const fn pwrctrl(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "DCDC Control."]
    #[inline(always)]
    pub const fn dcdcctrl(self) -> crate::common::Reg<regs::Dcdcctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "DCDC Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn dcdcmiscctrl(self) -> crate::common::Reg<regs::Dcdcmiscctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "DCDC Power Train NFET Zero Current Detector Control Register."]
    #[inline(always)]
    pub const fn dcdczdetctrl(self) -> crate::common::Reg<regs::Dcdczdetctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "DCDC Power Train PFET Current Limiter Control Register."]
    #[inline(always)]
    pub const fn dcdcclimctrl(self) -> crate::common::Reg<regs::Dcdcclimctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "DCDC Low Noise Compensator Control Register."]
    #[inline(always)]
    pub const fn dcdclncompctrl(
        self,
    ) -> crate::common::Reg<regs::Dcdclncompctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DCDC Low Noise Voltage Register."]
    #[inline(always)]
    pub const fn dcdclnvctrl(self) -> crate::common::Reg<regs::Dcdclnvctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "DCDC Controller Timing Value Register."]
    #[inline(always)]
    pub const fn dcdctiming(self) -> crate::common::Reg<regs::Dcdctiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DCDC Low Power Voltage Register."]
    #[inline(always)]
    pub const fn dcdclpvctrl(self) -> crate::common::Reg<regs::Dcdclpvctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "DCDC Low Power Control Register."]
    #[inline(always)]
    pub const fn dcdclpctrl(self) -> crate::common::Reg<regs::Dcdclpctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "DCDC Low Noise Controller Frequency Control."]
    #[inline(always)]
    pub const fn dcdclnfreqctrl(
        self,
    ) -> crate::common::Reg<regs::Dcdclnfreqctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "DCDC Read Status Register."]
    #[inline(always)]
    pub const fn dcdcsync(self) -> crate::common::Reg<regs::Dcdcsync, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "VMON AVDD Channel Control."]
    #[inline(always)]
    pub const fn vmonavddctrl(self) -> crate::common::Reg<regs::Vmonavddctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Alternate VMON AVDD Channel Control."]
    #[inline(always)]
    pub const fn vmonaltavddctrl(
        self,
    ) -> crate::common::Reg<regs::Vmonaltavddctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "VMON DVDD Channel Control."]
    #[inline(always)]
    pub const fn vmondvddctrl(self) -> crate::common::Reg<regs::Vmondvddctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "VMON IOVDD0 Channel Control."]
    #[inline(always)]
    pub const fn vmonio0ctrl(self) -> crate::common::Reg<regs::Vmonio0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Configurations Related to the Bias."]
    #[inline(always)]
    pub const fn biasconf(self) -> crate::common::Reg<regs::Biasconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Test Lock Register."]
    #[inline(always)]
    pub const fn testlock(self) -> crate::common::Reg<regs::Testlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Test Control Register for Regulator and BIAS."]
    #[inline(always)]
    pub const fn biastestctrl(self) -> crate::common::Reg<regs::Biastestctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
