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
    #[doc = "VMON IOVDD1 Channel Control."]
    #[inline(always)]
    pub const fn vmonio1ctrl(self) -> crate::common::Reg<regs::Vmonio1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "VMON BUVDD Channel Control."]
    #[inline(always)]
    pub const fn vmonbuvddctrl(self) -> crate::common::Reg<regs::Vmonbuvddctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Memory Control Register."]
    #[inline(always)]
    pub const fn ram1ctrl(self) -> crate::common::Reg<regs::Ram1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Memory Control Register."]
    #[inline(always)]
    pub const fn ram2ctrl(self) -> crate::common::Reg<regs::Ram2ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Backup Power Configuration Register."]
    #[inline(always)]
    pub const fn buctrl(self) -> crate::common::Reg<regs::Buctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "5V Regulator Control."]
    #[inline(always)]
    pub const fn r5vctrl(self) -> crate::common::Reg<regs::R5vctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "5V Regulator Control."]
    #[inline(always)]
    pub const fn r5vadcctrl(self) -> crate::common::Reg<regs::R5vadcctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "5V Regulator Voltage Select."]
    #[inline(always)]
    pub const fn r5voutlevel(self) -> crate::common::Reg<regs::R5voutlevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "5V Detector Enables."]
    #[inline(always)]
    pub const fn r5vdetctrl(self) -> crate::common::Reg<regs::R5vdetctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01."]
    #[inline(always)]
    pub const fn dcdclpem01cfg(self) -> crate::common::Reg<regs::Dcdclpem01cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "5V Detector Status Register."]
    #[inline(always)]
    pub const fn r5vstatus(self) -> crate::common::Reg<regs::R5vstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "5V Read Status Register."]
    #[inline(always)]
    pub const fn r5vsync(self) -> crate::common::Reg<regs::R5vsync, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral."]
    #[inline(always)]
    pub const fn em23pernoretaincmd(
        self,
    ) -> crate::common::Reg<regs::Em23pernoretaincmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It."]
    #[inline(always)]
    pub const fn em23pernoretainstatus(
        self,
    ) -> crate::common::Reg<regs::Em23pernoretainstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23."]
    #[inline(always)]
    pub const fn em23pernoretainctrl(
        self,
    ) -> crate::common::Reg<regs::Em23pernoretainctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
}
pub mod regs;
pub mod vals;
