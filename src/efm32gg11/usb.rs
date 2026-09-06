#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc {
    ptr: *mut u8,
}
unsafe impl Send for Hc {}
unsafe impl Sync for Hc {}
impl Hc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Host Channel x Characteristics Register."]
    #[inline(always)]
    pub const fn char(self) -> crate::common::Reg<regs::Hc0Char, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Host Channel x Split Control Register."]
    #[inline(always)]
    pub const fn splt(self) -> crate::common::Reg<regs::Hc0Splt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Host Channel x Interrupt Register."]
    #[inline(always)]
    pub const fn int(self) -> crate::common::Reg<regs::Hc0Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Host Channel x Interrupt Mask Register."]
    #[inline(always)]
    pub const fn intmsk(self) -> crate::common::Reg<regs::Hc0Intmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Host Channel x Transfer Size Register."]
    #[inline(always)]
    pub const fn tsiz(self) -> crate::common::Reg<regs::Hc0Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Host Channel x DMA Address Register."]
    #[inline(always)]
    pub const fn dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[doc = "USB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "System Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "I/O Routing Register."]
    #[inline(always)]
    pub const fn route(self) -> crate::common::Reg<regs::Route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Charger Detect Configuration Register."]
    #[inline(always)]
    pub const fn cdconf(self) -> crate::common::Reg<regs::Cdconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Data TRIM 1 Values for USB DP and DM."]
    #[inline(always)]
    pub const fn dattrim1(self) -> crate::common::Reg<regs::Dattrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "USB LEM Control Register."]
    #[inline(always)]
    pub const fn lemctrl(self) -> crate::common::Reg<regs::Lemctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "OTG Control and Status Register."]
    #[inline(always)]
    pub const fn gotgctl(self) -> crate::common::Reg<regs::Gotgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e000usize) as _) }
    }
    #[doc = "OTG Interrupt Register."]
    #[inline(always)]
    pub const fn gotgint(self) -> crate::common::Reg<regs::Gotgint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e004usize) as _) }
    }
    #[doc = "AHB Configuration Register."]
    #[inline(always)]
    pub const fn gahbcfg(self) -> crate::common::Reg<regs::Gahbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e008usize) as _) }
    }
    #[doc = "USB Configuration Register."]
    #[inline(always)]
    pub const fn gusbcfg(self) -> crate::common::Reg<regs::Gusbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e00cusize) as _) }
    }
    #[doc = "Reset Register."]
    #[inline(always)]
    pub const fn grstctl(self) -> crate::common::Reg<regs::Grstctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e010usize) as _) }
    }
    #[doc = "Interrupt Register."]
    #[inline(always)]
    pub const fn gintsts(self) -> crate::common::Reg<regs::Gintsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e014usize) as _) }
    }
    #[doc = "Interrupt Mask Register."]
    #[inline(always)]
    pub const fn gintmsk(self) -> crate::common::Reg<regs::Gintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e018usize) as _) }
    }
    #[doc = "Receive Status Debug Read Register."]
    #[inline(always)]
    pub const fn grxstsr(self) -> crate::common::Reg<regs::Grxstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e01cusize) as _) }
    }
    #[doc = "Receive Status Read /Pop Register."]
    #[inline(always)]
    pub const fn grxstsp(self) -> crate::common::Reg<regs::Grxstsp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e020usize) as _) }
    }
    #[doc = "Receive FIFO Size Register."]
    #[inline(always)]
    pub const fn grxfsiz(self) -> crate::common::Reg<regs::Grxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e024usize) as _) }
    }
    #[doc = "Non-periodic Transmit FIFO Size Register."]
    #[inline(always)]
    pub const fn gnptxfsiz(self) -> crate::common::Reg<regs::Gnptxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e028usize) as _) }
    }
    #[doc = "Non-periodic Transmit FIFO/Queue Status Register."]
    #[inline(always)]
    pub const fn gnptxsts(self) -> crate::common::Reg<regs::Gnptxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e02cusize) as _) }
    }
    #[doc = "Synopsys ID Register."]
    #[inline(always)]
    pub const fn gsnpsid(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e040usize) as _) }
    }
    #[doc = "Global DFIFO Configuration Register."]
    #[inline(always)]
    pub const fn gdfifocfg(self) -> crate::common::Reg<regs::Gdfifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e05cusize) as _) }
    }
    #[doc = "Host Periodic Transmit FIFO Size Register."]
    #[inline(always)]
    pub const fn hptxfsiz(self) -> crate::common::Reg<regs::Hptxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e100usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 1."]
    #[inline(always)]
    pub const fn dieptxf1(self) -> crate::common::Reg<regs::Dieptxf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e104usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 2."]
    #[inline(always)]
    pub const fn dieptxf2(self) -> crate::common::Reg<regs::Dieptxf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e108usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 3."]
    #[inline(always)]
    pub const fn dieptxf3(self) -> crate::common::Reg<regs::Dieptxf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e10cusize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 4."]
    #[inline(always)]
    pub const fn dieptxf4(self) -> crate::common::Reg<regs::Dieptxf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e110usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 5."]
    #[inline(always)]
    pub const fn dieptxf5(self) -> crate::common::Reg<regs::Dieptxf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e114usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Size Register 6."]
    #[inline(always)]
    pub const fn dieptxf6(self) -> crate::common::Reg<regs::Dieptxf6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e118usize) as _) }
    }
    #[doc = "Host Configuration Register."]
    #[inline(always)]
    pub const fn hcfg(self) -> crate::common::Reg<regs::Hcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e400usize) as _) }
    }
    #[doc = "Host Frame Interval Register."]
    #[inline(always)]
    pub const fn hfir(self) -> crate::common::Reg<regs::Hfir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e404usize) as _) }
    }
    #[doc = "Host Frame Number/Frame Time Remaining Register."]
    #[inline(always)]
    pub const fn hfnum(self) -> crate::common::Reg<regs::Hfnum, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e408usize) as _) }
    }
    #[doc = "Host Periodic Transmit FIFO/Queue Status Register."]
    #[inline(always)]
    pub const fn hptxsts(self) -> crate::common::Reg<regs::Hptxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e410usize) as _) }
    }
    #[doc = "Host All Channels Interrupt Register."]
    #[inline(always)]
    pub const fn haint(self) -> crate::common::Reg<regs::Haint, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e414usize) as _) }
    }
    #[doc = "Host All Channels Interrupt Mask Register."]
    #[inline(always)]
    pub const fn haintmsk(self) -> crate::common::Reg<regs::Haintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e418usize) as _) }
    }
    #[doc = "Host Port Control and Status Register."]
    #[inline(always)]
    pub const fn hprt(self) -> crate::common::Reg<regs::Hprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e440usize) as _) }
    }
    #[inline(always)]
    pub const fn hc0(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e500usize) as _) }
    }
    #[inline(always)]
    pub const fn hc1(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e520usize) as _) }
    }
    #[inline(always)]
    pub const fn hc2(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e540usize) as _) }
    }
    #[inline(always)]
    pub const fn hc3(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e560usize) as _) }
    }
    #[inline(always)]
    pub const fn hc4(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e580usize) as _) }
    }
    #[inline(always)]
    pub const fn hc5(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e5a0usize) as _) }
    }
    #[inline(always)]
    pub const fn hc6(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e5c0usize) as _) }
    }
    #[inline(always)]
    pub const fn hc7(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e5e0usize) as _) }
    }
    #[inline(always)]
    pub const fn hc8(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e600usize) as _) }
    }
    #[inline(always)]
    pub const fn hc9(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e620usize) as _) }
    }
    #[inline(always)]
    pub const fn hc10(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e640usize) as _) }
    }
    #[inline(always)]
    pub const fn hc11(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e660usize) as _) }
    }
    #[inline(always)]
    pub const fn hc12(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e680usize) as _) }
    }
    #[inline(always)]
    pub const fn hc13(self) -> Hc {
        unsafe { Hc::from_ptr(self.ptr.wrapping_add(0x000d_e6a0usize) as _) }
    }
    #[doc = "Device Configuration Register."]
    #[inline(always)]
    pub const fn dcfg(self) -> crate::common::Reg<regs::Dcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e800usize) as _) }
    }
    #[doc = "Device Control Register."]
    #[inline(always)]
    pub const fn dctl(self) -> crate::common::Reg<regs::Dctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e804usize) as _) }
    }
    #[doc = "Device Status Register."]
    #[inline(always)]
    pub const fn dsts(self) -> crate::common::Reg<regs::Dsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e808usize) as _) }
    }
    #[doc = "Device IN Endpoint Common Interrupt Mask Register."]
    #[inline(always)]
    pub const fn diepmsk(self) -> crate::common::Reg<regs::Diepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e810usize) as _) }
    }
    #[doc = "Device OUT Endpoint Common Interrupt Mask Register."]
    #[inline(always)]
    pub const fn doepmsk(self) -> crate::common::Reg<regs::Doepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e814usize) as _) }
    }
    #[doc = "Device All Endpoints Interrupt Register."]
    #[inline(always)]
    pub const fn daint(self) -> crate::common::Reg<regs::Daint, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e818usize) as _) }
    }
    #[doc = "Device All Endpoints Interrupt Mask Register."]
    #[inline(always)]
    pub const fn daintmsk(self) -> crate::common::Reg<regs::Daintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e81cusize) as _) }
    }
    #[doc = "Device VBUS Discharge Time Register."]
    #[inline(always)]
    pub const fn dvbusdis(self) -> crate::common::Reg<regs::Dvbusdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e828usize) as _) }
    }
    #[doc = "Device VBUS Pulsing Time Register."]
    #[inline(always)]
    pub const fn dvbuspulse(self) -> crate::common::Reg<regs::Dvbuspulse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e82cusize) as _) }
    }
    #[doc = "Device Threshold Control Register."]
    #[inline(always)]
    pub const fn dthrctl(self) -> crate::common::Reg<regs::Dthrctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e830usize) as _) }
    }
    #[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register."]
    #[inline(always)]
    pub const fn diepempmsk(self) -> crate::common::Reg<regs::Diepempmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e834usize) as _) }
    }
    #[doc = "Device Control IN Endpoint 0 Control Register."]
    #[inline(always)]
    pub const fn diep0ctl(self) -> crate::common::Reg<regs::Diep0ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e900usize) as _) }
    }
    #[doc = "Device IN Endpoint 0 Interrupt Register."]
    #[inline(always)]
    pub const fn diep0int(self) -> crate::common::Reg<regs::Diep0int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e908usize) as _) }
    }
    #[doc = "Device IN Endpoint 0 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep0tsiz(self) -> crate::common::Reg<regs::Diep0tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e910usize) as _) }
    }
    #[doc = "Device IN Endpoint 0 DMA Address Register."]
    #[inline(always)]
    pub const fn diep0dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e914usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 0."]
    #[inline(always)]
    pub const fn diep0txfsts(self) -> crate::common::Reg<regs::Diep0txfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e918usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep0_ctl(self) -> crate::common::Reg<regs::Diep0Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e920usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep0_int(self) -> crate::common::Reg<regs::Diep0Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e928usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep0_tsiz(self) -> crate::common::Reg<regs::Diep0Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e930usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep0_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e934usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep0_dtxfsts(self) -> crate::common::Reg<regs::Diep0Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e938usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep1_ctl(self) -> crate::common::Reg<regs::Diep1Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e940usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep1_int(self) -> crate::common::Reg<regs::Diep1Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e948usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep1_tsiz(self) -> crate::common::Reg<regs::Diep1Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e950usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep1_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e954usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep1_dtxfsts(self) -> crate::common::Reg<regs::Diep1Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e958usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep2_ctl(self) -> crate::common::Reg<regs::Diep2Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e960usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep2_int(self) -> crate::common::Reg<regs::Diep2Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e968usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep2_tsiz(self) -> crate::common::Reg<regs::Diep2Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e970usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep2_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e974usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep2_dtxfsts(self) -> crate::common::Reg<regs::Diep2Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e978usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep3_ctl(self) -> crate::common::Reg<regs::Diep3Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e980usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep3_int(self) -> crate::common::Reg<regs::Diep3Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e988usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep3_tsiz(self) -> crate::common::Reg<regs::Diep3Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e990usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep3_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e994usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep3_dtxfsts(self) -> crate::common::Reg<regs::Diep3Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e998usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep4_ctl(self) -> crate::common::Reg<regs::Diep4Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9a0usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep4_int(self) -> crate::common::Reg<regs::Diep4Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9a8usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep4_tsiz(self) -> crate::common::Reg<regs::Diep4Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9b0usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep4_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9b4usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep4_dtxfsts(self) -> crate::common::Reg<regs::Diep4Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9b8usize) as _) }
    }
    #[doc = "Device Control IN Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn diep5_ctl(self) -> crate::common::Reg<regs::Diep5Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9c0usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn diep5_int(self) -> crate::common::Reg<regs::Diep5Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9c8usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn diep5_tsiz(self) -> crate::common::Reg<regs::Diep5Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9d0usize) as _) }
    }
    #[doc = "Device IN Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn diep5_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9d4usize) as _) }
    }
    #[doc = "Device IN Endpoint Transmit FIFO Status Register 1."]
    #[inline(always)]
    pub const fn diep5_dtxfsts(self) -> crate::common::Reg<regs::Diep5Dtxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_e9d8usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint 0 Control Register."]
    #[inline(always)]
    pub const fn doep0ctl(self) -> crate::common::Reg<regs::Doep0ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb00usize) as _) }
    }
    #[doc = "Device OUT Endpoint 0 Interrupt Register."]
    #[inline(always)]
    pub const fn doep0int(self) -> crate::common::Reg<regs::Doep0int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb08usize) as _) }
    }
    #[doc = "Device OUT Endpoint 0 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep0tsiz(self) -> crate::common::Reg<regs::Doep0tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb10usize) as _) }
    }
    #[doc = "Device OUT Endpoint 0 DMA Address Register."]
    #[inline(always)]
    pub const fn doep0dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb14usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep0_ctl(self) -> crate::common::Reg<regs::Doep0Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb20usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep0_int(self) -> crate::common::Reg<regs::Doep0Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb28usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep0_tsiz(self) -> crate::common::Reg<regs::Doep0Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb30usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep0_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb34usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep1_ctl(self) -> crate::common::Reg<regs::Doep1Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb40usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep1_int(self) -> crate::common::Reg<regs::Doep1Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb48usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep1_tsiz(self) -> crate::common::Reg<regs::Doep1Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb50usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep1_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb54usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep2_ctl(self) -> crate::common::Reg<regs::Doep2Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb60usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep2_int(self) -> crate::common::Reg<regs::Doep2Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb68usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep2_tsiz(self) -> crate::common::Reg<regs::Doep2Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb70usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep2_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb74usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep3_ctl(self) -> crate::common::Reg<regs::Doep3Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb80usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep3_int(self) -> crate::common::Reg<regs::Doep3Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb88usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep3_tsiz(self) -> crate::common::Reg<regs::Doep3Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb90usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep3_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eb94usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep4_ctl(self) -> crate::common::Reg<regs::Doep4Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eba0usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep4_int(self) -> crate::common::Reg<regs::Doep4Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_eba8usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep4_tsiz(self) -> crate::common::Reg<regs::Doep4Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebb0usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep4_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebb4usize) as _) }
    }
    #[doc = "Device Control OUT Endpoint x+1 Control Register."]
    #[inline(always)]
    pub const fn doep5_ctl(self) -> crate::common::Reg<regs::Doep5Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebc0usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Interrupt Register."]
    #[inline(always)]
    pub const fn doep5_int(self) -> crate::common::Reg<regs::Doep5Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebc8usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 Transfer Size Register."]
    #[inline(always)]
    pub const fn doep5_tsiz(self) -> crate::common::Reg<regs::Doep5Tsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebd0usize) as _) }
    }
    #[doc = "Device OUT Endpoint x+1 DMA Address Register."]
    #[inline(always)]
    pub const fn doep5_dmaaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ebd4usize) as _) }
    }
    #[doc = "Power and Clock Gating Control Register."]
    #[inline(always)]
    pub const fn pcgcctl(self) -> crate::common::Reg<regs::Pcgcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x000d_ee00usize) as _) }
    }
}
pub mod regs;
pub mod vals;
