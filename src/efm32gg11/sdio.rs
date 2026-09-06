#[doc = "SDIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio {
    ptr: *mut u8,
}
unsafe impl Send for Sdio {}
unsafe impl Sync for Sdio {}
impl Sdio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDMA System Address Register."]
    #[inline(always)]
    pub const fn sdmasysaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Block Size and Block Count Register."]
    #[inline(always)]
    pub const fn blksize(self) -> crate::common::Reg<regs::Blksize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SD Command Argument Register."]
    #[inline(always)]
    pub const fn cmdarg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Transfer Mode and Command Register."]
    #[inline(always)]
    pub const fn tfrmode(self) -> crate::common::Reg<regs::Tfrmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Response0 and Response1 Register."]
    #[inline(always)]
    pub const fn resp0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Response2 and Response3 Register."]
    #[inline(always)]
    pub const fn resp2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Response4 and Response5 Register."]
    #[inline(always)]
    pub const fn resp4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Response6 and Response7 Register."]
    #[inline(always)]
    pub const fn resp6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Buffer Data Register."]
    #[inline(always)]
    pub const fn bufdatport(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Present State Register."]
    #[inline(always)]
    pub const fn prsstat(self) -> crate::common::Reg<regs::Prsstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register."]
    #[inline(always)]
    pub const fn hostctrl1(self) -> crate::common::Reg<regs::Hostctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Clock Control, Timeout Control and Software Register."]
    #[inline(always)]
    pub const fn clockctrl(self) -> crate::common::Reg<regs::Clockctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Normal and Error Interrupt Status Register."]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Normal and Error Interrupt Status Enable Register."]
    #[inline(always)]
    pub const fn ifenc(self) -> crate::common::Reg<regs::Ifenc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Normal and Error Interrupt Signal Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "AUTO CMD12 Error Status and Host Control2 Register."]
    #[inline(always)]
    pub const fn ac12errstat(self) -> crate::common::Reg<regs::Ac12errstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capabilities Register to Hold Bits 31~0."]
    #[inline(always)]
    pub const fn capab0(self) -> crate::common::Reg<regs::Capab0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capabilities Register to Hold Bits 63~32."]
    #[inline(always)]
    pub const fn capab2(self) -> crate::common::Reg<regs::Capab2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Maximum Current Capabilities Register."]
    #[inline(always)]
    pub const fn maxcurcapab(self) -> crate::common::Reg<regs::Maxcurcapab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Force Event Register for Auto CMD Error Status."]
    #[inline(always)]
    pub const fn fevterrstat(self) -> crate::common::Reg<regs::Fevterrstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ADMA Error Status Register."]
    #[inline(always)]
    pub const fn admaes(self) -> crate::common::Reg<regs::Admaes, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ADMA System Address Register."]
    #[inline(always)]
    pub const fn adsaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Preset Value for Initialization and Default Speed Mode."]
    #[inline(always)]
    pub const fn prstval0(self) -> crate::common::Reg<regs::Prstval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Preset Value for High Speed and SDR12 Modes."]
    #[inline(always)]
    pub const fn prstval2(self) -> crate::common::Reg<regs::Prstval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Preset Value for SDR25 and SDR50 Modes."]
    #[inline(always)]
    pub const fn prstval4(self) -> crate::common::Reg<regs::Prstval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Preset Value for SDR104 and DDR50 Modes."]
    #[inline(always)]
    pub const fn prstval6(self) -> crate::common::Reg<regs::Prstval6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Boot Timeout Control Register."]
    #[inline(always)]
    pub const fn boottoctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Slot Interrupt Status Register."]
    #[inline(always)]
    pub const fn slotintstat(self) -> crate::common::Reg<regs::Slotintstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Core Control Signals."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Core Configuration 0."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Core Configuration 1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Core Configuration Preset Value 0."]
    #[inline(always)]
    pub const fn cfgpresetval0(self) -> crate::common::Reg<regs::Cfgpresetval0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Core Configuration Preset Value 1."]
    #[inline(always)]
    pub const fn cfgpresetval1(self) -> crate::common::Reg<regs::Cfgpresetval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "Core Configuration Preset Value 2."]
    #[inline(always)]
    pub const fn cfgpresetval2(self) -> crate::common::Reg<regs::Cfgpresetval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "Core Configuration Preset Value 3."]
    #[inline(always)]
    pub const fn cfgpresetval3(self) -> crate::common::Reg<regs::Cfgpresetval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "I/O LOCATION Register."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "I/O LOCATION Register."]
    #[inline(always)]
    pub const fn routeloc1(self) -> crate::common::Reg<regs::Routeloc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "I/O LOCATION Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
}
pub mod regs;
pub mod vals;
