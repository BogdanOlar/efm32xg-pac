#[doc = "QSPI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi {
    ptr: *mut u8,
}
unsafe impl Send for Qspi {}
unsafe impl Sync for Qspi {}
impl Qspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Octal-SPI Configuration Register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Device Read Instruction Configuration Register."]
    #[inline(always)]
    pub const fn devinstrrdconfig(
        self,
    ) -> crate::common::Reg<regs::Devinstrrdconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Device Write Instruction Configuration Register."]
    #[inline(always)]
    pub const fn devinstrwrconfig(
        self,
    ) -> crate::common::Reg<regs::Devinstrwrconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Device Delay Register."]
    #[inline(always)]
    pub const fn devdelay(self) -> crate::common::Reg<regs::Devdelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Read Data Capture Register."]
    #[inline(always)]
    pub const fn rddatacapture(self) -> crate::common::Reg<regs::Rddatacapture, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Device Size Configuration Register."]
    #[inline(always)]
    pub const fn devsizeconfig(self) -> crate::common::Reg<regs::Devsizeconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SRAM Partition Configuration Register."]
    #[inline(always)]
    pub const fn srampartitioncfg(
        self,
    ) -> crate::common::Reg<regs::Srampartitioncfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Indirect Address Trigger Register."]
    #[inline(always)]
    pub const fn indahbaddrtrigger(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Remap Address Register."]
    #[inline(always)]
    pub const fn remapaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Mode Bit Configuration Register."]
    #[inline(always)]
    pub const fn modebitconfig(self) -> crate::common::Reg<regs::Modebitconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SRAM Fill Register."]
    #[inline(always)]
    pub const fn sramfill(self) -> crate::common::Reg<regs::Sramfill, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "TX Threshold Register."]
    #[inline(always)]
    pub const fn txthresh(self) -> crate::common::Reg<regs::Txthresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "RX Threshold Register."]
    #[inline(always)]
    pub const fn rxthresh(self) -> crate::common::Reg<regs::Rxthresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Write Completion Control Register."]
    #[inline(always)]
    pub const fn writecompletionctrl(
        self,
    ) -> crate::common::Reg<regs::Writecompletionctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Polling Expiration Register."]
    #[inline(always)]
    pub const fn noofpollsbefexp(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn irqstatus(self) -> crate::common::Reg<regs::Irqstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Mask."]
    #[inline(always)]
    pub const fn irqmask(self) -> crate::common::Reg<regs::Irqmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Lower Write Protection Register."]
    #[inline(always)]
    pub const fn lowerwrprot(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Upper Write Protection Register."]
    #[inline(always)]
    pub const fn upperwrprot(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Write Protection Control Register."]
    #[inline(always)]
    pub const fn wrprotctrl(self) -> crate::common::Reg<regs::Wrprotctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Indirect Read Transfer Control Register."]
    #[inline(always)]
    pub const fn indirectreadxferctrl(
        self,
    ) -> crate::common::Reg<regs::Indirectreadxferctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Indirect Read Transfer Watermark Register."]
    #[inline(always)]
    pub const fn indirectreadxferwatermark(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Indirect Read Transfer Start Address Register."]
    #[inline(always)]
    pub const fn indirectreadxferstart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Indirect Read Transfer Number Bytes Register."]
    #[inline(always)]
    pub const fn indirectreadxfernumbytes(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Indirect Write Transfer Control Register."]
    #[inline(always)]
    pub const fn indirectwritexferctrl(
        self,
    ) -> crate::common::Reg<regs::Indirectwritexferctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Indirect Write Transfer Watermark Register."]
    #[inline(always)]
    pub const fn indirectwritexferwatermark(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Indirect Write Transfer Start Address Register."]
    #[inline(always)]
    pub const fn indirectwritexferstart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Indirect Write Transfer Number Bytes Register."]
    #[inline(always)]
    pub const fn indirectwritexfernumbytes(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Indirect Trigger Address Range Register."]
    #[inline(always)]
    pub const fn indirecttriggeraddrrange(
        self,
    ) -> crate::common::Reg<regs::Indirecttriggeraddrrange, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Flash Command Control Memory Register (STIG)."]
    #[inline(always)]
    pub const fn flashcommandctrlmem(
        self,
    ) -> crate::common::Reg<regs::Flashcommandctrlmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Flash Command Control Register (STIG)."]
    #[inline(always)]
    pub const fn flashcmdctrl(self) -> crate::common::Reg<regs::Flashcmdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Flash Command Address Register (STIG)."]
    #[inline(always)]
    pub const fn flashcmdaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Flash Command Read Data Register (Lower) (STIG)."]
    #[inline(always)]
    pub const fn flashrddatalower(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Flash Command Read Data Register (Upper) (STIG)."]
    #[inline(always)]
    pub const fn flashrddataupper(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Flash Command Write Data Register (Lower) (STIG)."]
    #[inline(always)]
    pub const fn flashwrdatalower(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Flash Command Write Data Register (Upper) (STIG)."]
    #[inline(always)]
    pub const fn flashwrdataupper(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Polling Flash Status Register."]
    #[inline(always)]
    pub const fn pollingflashstatus(
        self,
    ) -> crate::common::Reg<regs::Pollingflashstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "PHY Configuration Register."]
    #[inline(always)]
    pub const fn phyconfiguration(
        self,
    ) -> crate::common::Reg<regs::Phyconfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Opcode Extension Register (Lower)."]
    #[inline(always)]
    pub const fn opcodeextlower(
        self,
    ) -> crate::common::Reg<regs::Opcodeextlower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Opcode Extension Register (Upper)."]
    #[inline(always)]
    pub const fn opcodeextupper(
        self,
    ) -> crate::common::Reg<regs::Opcodeextupper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Module ID Register."]
    #[inline(always)]
    pub const fn moduleid(self) -> crate::common::Reg<regs::Moduleid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "I/O Routing Pin Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "I/O Route Location Register 0."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
}
pub mod regs;
pub mod vals;
