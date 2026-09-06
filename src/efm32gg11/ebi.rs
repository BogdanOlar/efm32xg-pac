#[doc = "EBI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ebi {
    ptr: *mut u8,
}
unsafe impl Send for Ebi {}
unsafe impl Sync for Ebi {}
impl Ebi {
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
    #[doc = "Address Timing Register."]
    #[inline(always)]
    pub const fn addrtiming(self) -> crate::common::Reg<regs::Addrtiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Read Timing Register."]
    #[inline(always)]
    pub const fn rdtiming(self) -> crate::common::Reg<regs::Rdtiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Write Timing Register."]
    #[inline(always)]
    pub const fn wrtiming(self) -> crate::common::Reg<regs::Wrtiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Polarity Register."]
    #[inline(always)]
    pub const fn polarity(self) -> crate::common::Reg<regs::Polarity, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Address Timing Register 1."]
    #[inline(always)]
    pub const fn addrtiming1(self) -> crate::common::Reg<regs::Addrtiming1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Read Timing Register 1."]
    #[inline(always)]
    pub const fn rdtiming1(self) -> crate::common::Reg<regs::Rdtiming1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Write Timing Register 1."]
    #[inline(always)]
    pub const fn wrtiming1(self) -> crate::common::Reg<regs::Wrtiming1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Polarity Register 1."]
    #[inline(always)]
    pub const fn polarity1(self) -> crate::common::Reg<regs::Polarity1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Address Timing Register 2."]
    #[inline(always)]
    pub const fn addrtiming2(self) -> crate::common::Reg<regs::Addrtiming2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Read Timing Register 2."]
    #[inline(always)]
    pub const fn rdtiming2(self) -> crate::common::Reg<regs::Rdtiming2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Write Timing Register 2."]
    #[inline(always)]
    pub const fn wrtiming2(self) -> crate::common::Reg<regs::Wrtiming2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Polarity Register 2."]
    #[inline(always)]
    pub const fn polarity2(self) -> crate::common::Reg<regs::Polarity2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Address Timing Register 3."]
    #[inline(always)]
    pub const fn addrtiming3(self) -> crate::common::Reg<regs::Addrtiming3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Read Timing Register 3."]
    #[inline(always)]
    pub const fn rdtiming3(self) -> crate::common::Reg<regs::Rdtiming3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Write Timing Register 3."]
    #[inline(always)]
    pub const fn wrtiming3(self) -> crate::common::Reg<regs::Wrtiming3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Polarity Register 3."]
    #[inline(always)]
    pub const fn polarity3(self) -> crate::common::Reg<regs::Polarity3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Page Control Register."]
    #[inline(always)]
    pub const fn pagectrl(self) -> crate::common::Reg<regs::Pagectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "NAND Control Register."]
    #[inline(always)]
    pub const fn nandctrl(self) -> crate::common::Reg<regs::Nandctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ECC Parity Register."]
    #[inline(always)]
    pub const fn eccparity(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "TFT Control Register."]
    #[inline(always)]
    pub const fn tftctrl(self) -> crate::common::Reg<regs::Tftctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "TFT Status Register."]
    #[inline(always)]
    pub const fn tftstatus(self) -> crate::common::Reg<regs::Tftstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Color Format Register."]
    #[inline(always)]
    pub const fn tftcolorformat(
        self,
    ) -> crate::common::Reg<regs::Tftcolorformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "TFT Frame Base Register."]
    #[inline(always)]
    pub const fn tftframebase(self) -> crate::common::Reg<regs::Tftframebase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "TFT Stride Register."]
    #[inline(always)]
    pub const fn tftstride(self) -> crate::common::Reg<regs::Tftstride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "TFT Size Register."]
    #[inline(always)]
    pub const fn tftsize(self) -> crate::common::Reg<regs::Tftsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "TFT Horizontal Porch Register."]
    #[inline(always)]
    pub const fn tfthporch(self) -> crate::common::Reg<regs::Tfthporch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "TFT Vertical Porch Register."]
    #[inline(always)]
    pub const fn tftvporch(self) -> crate::common::Reg<regs::Tftvporch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "TFT Timing Register."]
    #[inline(always)]
    pub const fn tfttiming(self) -> crate::common::Reg<regs::Tfttiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "TFT Polarity Register."]
    #[inline(always)]
    pub const fn tftpolarity(self) -> crate::common::Reg<regs::Tftpolarity, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "TFT Direct Drive Data Register."]
    #[inline(always)]
    pub const fn tftdd(self) -> crate::common::Reg<regs::Tftdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "TFT Alpha Blending Register."]
    #[inline(always)]
    pub const fn tftalpha(self) -> crate::common::Reg<regs::Tftalpha, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "TFT Pixel 0 Register."]
    #[inline(always)]
    pub const fn tftpixel0(self) -> crate::common::Reg<regs::Tftpixel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "TFT Pixel 1 Register."]
    #[inline(always)]
    pub const fn tftpixel1(self) -> crate::common::Reg<regs::Tftpixel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "TFT Alpha Blending Result Pixel Register."]
    #[inline(always)]
    pub const fn tftpixel(self) -> crate::common::Reg<regs::Tftpixel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "TFT Masking Register."]
    #[inline(always)]
    pub const fn tftmask(self) -> crate::common::Reg<regs::Tftmask, crate::common::RW> {
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
    #[doc = "I/O Routing Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "I/O Routing Location Register."]
    #[inline(always)]
    pub const fn routeloc1(self) -> crate::common::Reg<regs::Routeloc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
