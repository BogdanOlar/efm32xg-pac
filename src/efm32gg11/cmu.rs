#[doc = "CMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmu {
    ptr: *mut u8,
}
unsafe impl Send for Cmu {}
unsafe impl Sync for Cmu {}
impl Cmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CMU Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USHFRCO Control Register."]
    #[inline(always)]
    pub const fn ushfrcoctrl(self) -> crate::common::Reg<regs::Ushfrcoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "HFRCO Control Register."]
    #[inline(always)]
    pub const fn hfrcoctrl(self) -> crate::common::Reg<regs::Hfrcoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "AUXHFRCO Control Register."]
    #[inline(always)]
    pub const fn auxhfrcoctrl(self) -> crate::common::Reg<regs::Auxhfrcoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LFRCO Control Register."]
    #[inline(always)]
    pub const fn lfrcoctrl(self) -> crate::common::Reg<regs::Lfrcoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "HFXO Control Register."]
    #[inline(always)]
    pub const fn hfxoctrl(self) -> crate::common::Reg<regs::Hfxoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "HFXO Control 1."]
    #[inline(always)]
    pub const fn hfxoctrl1(self) -> crate::common::Reg<regs::Hfxoctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "HFXO Startup Control."]
    #[inline(always)]
    pub const fn hfxostartupctrl(
        self,
    ) -> crate::common::Reg<regs::Hfxostartupctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "HFXO Steady State Control."]
    #[inline(always)]
    pub const fn hfxosteadystatectrl(
        self,
    ) -> crate::common::Reg<regs::Hfxosteadystatectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "HFXO Timeout Control."]
    #[inline(always)]
    pub const fn hfxotimeoutctrl(
        self,
    ) -> crate::common::Reg<regs::Hfxotimeoutctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "LFXO Control Register."]
    #[inline(always)]
    pub const fn lfxoctrl(self) -> crate::common::Reg<regs::Lfxoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "DPLL Control Register."]
    #[inline(always)]
    pub const fn dpllctrl(self) -> crate::common::Reg<regs::Dpllctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "DPLL Control Register."]
    #[inline(always)]
    pub const fn dpllctrl1(self) -> crate::common::Reg<regs::Dpllctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Calibration Control Register."]
    #[inline(always)]
    pub const fn calctrl(self) -> crate::common::Reg<regs::Calctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Calibration Counter Register."]
    #[inline(always)]
    pub const fn calcnt(self) -> crate::common::Reg<regs::Calcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Oscillator Enable/Disable Command Register."]
    #[inline(always)]
    pub const fn oscencmd(self) -> crate::common::Reg<regs::Oscencmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Debug Trace Clock Select."]
    #[inline(always)]
    pub const fn dbgclksel(self) -> crate::common::Reg<regs::Dbgclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "High Frequency Clock Select Command Register."]
    #[inline(always)]
    pub const fn hfclksel(self) -> crate::common::Reg<regs::Hfclksel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Low Frequency A Clock Select Register."]
    #[inline(always)]
    pub const fn lfaclksel(self) -> crate::common::Reg<regs::Lfaclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Low Frequency B Clock Select Register."]
    #[inline(always)]
    pub const fn lfbclksel(self) -> crate::common::Reg<regs::Lfbclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Low Frequency E Clock Select Register."]
    #[inline(always)]
    pub const fn lfeclksel(self) -> crate::common::Reg<regs::Lfeclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Low Frequency C Clock Select Register."]
    #[inline(always)]
    pub const fn lfcclksel(self) -> crate::common::Reg<regs::Lfcclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "HFCLK Status Register."]
    #[inline(always)]
    pub const fn hfclkstatus(self) -> crate::common::Reg<regs::Hfclkstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "HFXO Trim Status."]
    #[inline(always)]
    pub const fn hfxotrimstatus(
        self,
    ) -> crate::common::Reg<regs::Hfxotrimstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "High Frequency Bus Clock Enable Register 0."]
    #[inline(always)]
    pub const fn hfbusclken0(self) -> crate::common::Reg<regs::Hfbusclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "High Frequency Peripheral Clock Enable Register 0."]
    #[inline(always)]
    pub const fn hfperclken0(self) -> crate::common::Reg<regs::Hfperclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "High Frequency Peripheral Clock Enable Register 1."]
    #[inline(always)]
    pub const fn hfperclken1(self) -> crate::common::Reg<regs::Hfperclken1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfaclken0(self) -> crate::common::Reg<regs::Lfaclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfbclken0(self) -> crate::common::Reg<regs::Lfbclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfcclken0(self) -> crate::common::Reg<regs::Lfcclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfeclken0(self) -> crate::common::Reg<regs::Lfeclken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "High Frequency Clock Prescaler Register."]
    #[inline(always)]
    pub const fn hfpresc(self) -> crate::common::Reg<regs::Hfpresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "High Frequency Bus Clock Prescaler Register."]
    #[inline(always)]
    pub const fn hfbuspresc(self) -> crate::common::Reg<regs::Hfbuspresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "High Frequency Core Clock Prescaler Register."]
    #[inline(always)]
    pub const fn hfcorepresc(self) -> crate::common::Reg<regs::Hfcorepresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "High Frequency Peripheral Clock Prescaler Register."]
    #[inline(always)]
    pub const fn hfperpresc(self) -> crate::common::Reg<regs::Hfperpresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "High Frequency Export Clock Prescaler Register."]
    #[inline(always)]
    pub const fn hfexppresc(self) -> crate::common::Reg<regs::Hfexppresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "High Frequency Peripheral Clock Prescaler B Register."]
    #[inline(always)]
    pub const fn hfperprescb(self) -> crate::common::Reg<regs::Hfperprescb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "High Frequency Peripheral Clock Prescaler C Register."]
    #[inline(always)]
    pub const fn hfperprescc(self) -> crate::common::Reg<regs::Hfperprescc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Low Frequency a Prescaler Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfapresc0(self) -> crate::common::Reg<regs::Lfapresc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Low Frequency B Prescaler Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfbpresc0(self) -> crate::common::Reg<regs::Lfbpresc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Low Frequency E Prescaler Register 0 (Async Reg)."]
    #[inline(always)]
    pub const fn lfepresc0(self) -> crate::common::Reg<regs::Lfepresc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Synchronization Busy Register."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Freeze Register."]
    #[inline(always)]
    pub const fn freeze(self) -> crate::common::Reg<regs::Freeze, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "PCNT Control Register."]
    #[inline(always)]
    pub const fn pcntctrl(self) -> crate::common::Reg<regs::Pcntctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "ADC Control Register."]
    #[inline(always)]
    pub const fn adcctrl(self) -> crate::common::Reg<regs::Adcctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "SDIO Control Register."]
    #[inline(always)]
    pub const fn sdioctrl(self) -> crate::common::Reg<regs::Sdioctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "QSPI Control Register."]
    #[inline(always)]
    pub const fn qspictrl(self) -> crate::common::Reg<regs::Qspictrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "I/O Routing Pin Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc1(self) -> crate::common::Reg<regs::Routeloc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Configuration Lock Register."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "HFRCO Spread Spectrum Register."]
    #[inline(always)]
    pub const fn hfrcoss(self) -> crate::common::Reg<regs::Hfrcoss, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "USB Control Register."]
    #[inline(always)]
    pub const fn usbctrl(self) -> crate::common::Reg<regs::Usbctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "USB Clock Recovery Control."]
    #[inline(always)]
    pub const fn usbcrctrl(self) -> crate::common::Reg<regs::Usbcrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
