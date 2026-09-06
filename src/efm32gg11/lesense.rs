#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Scan Configuration."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Ch0Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Scan Configuration."]
    #[inline(always)]
    pub const fn interact(self) -> crate::common::Reg<regs::Ch0Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Scan Configuration."]
    #[inline(always)]
    pub const fn eval(self) -> crate::common::Reg<regs::Ch0Eval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "LESENSE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lesense {
    ptr: *mut u8,
}
unsafe impl Send for Lesense {}
unsafe impl Sync for Lesense {}
impl Lesense {
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
    #[doc = "Timing Control Register."]
    #[inline(always)]
    pub const fn timctrl(self) -> crate::common::Reg<regs::Timctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Control Register."]
    #[inline(always)]
    pub const fn perctrl(self) -> crate::common::Reg<regs::Perctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Decoder Control Register."]
    #[inline(always)]
    pub const fn decctrl(self) -> crate::common::Reg<regs::Decctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Bias Control Register."]
    #[inline(always)]
    pub const fn biasctrl(self) -> crate::common::Reg<regs::Biasctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LESENSE Evaluation Control."]
    #[inline(always)]
    pub const fn evalctrl(self) -> crate::common::Reg<regs::Evalctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "PRS Control Register."]
    #[inline(always)]
    pub const fn prsctrl(self) -> crate::common::Reg<regs::Prsctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel Enable Register."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Scan Result Register."]
    #[inline(always)]
    pub const fn scanres(self) -> crate::common::Reg<regs::Scanres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Result Buffer Pointers."]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<regs::Ptr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Result Buffer Data Register."]
    #[inline(always)]
    pub const fn bufdata(self) -> crate::common::Reg<regs::Bufdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Current Channel Index."]
    #[inline(always)]
    pub const fn curch(self) -> crate::common::Reg<regs::Curch, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Current Decoder State."]
    #[inline(always)]
    pub const fn decstate(self) -> crate::common::Reg<regs::Decstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Decoder Input Register."]
    #[inline(always)]
    pub const fn sensorstate(self) -> crate::common::Reg<regs::Sensorstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GPIO Idle Phase Configuration."]
    #[inline(always)]
    pub const fn idleconf(self) -> crate::common::Reg<regs::Idleconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Alternative Excite Pin Configuration."]
    #[inline(always)]
    pub const fn altexconf(self) -> crate::common::Reg<regs::Altexconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Interrupt Flag Register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Synchronization Busy Register."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "I/O Routing Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn st0(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn st1(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn st2(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn st3(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn st4(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn st5(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn st6(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn st7(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn st8(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn st9(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn st10(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn st11(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn st12(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn st13(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[inline(always)]
    pub const fn st14(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn st15(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn st16(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn st17(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn st18(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn st19(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn st20(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn st21(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn st22(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn st23(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[inline(always)]
    pub const fn st24(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn st25(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[inline(always)]
    pub const fn st26(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[inline(always)]
    pub const fn st27(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[inline(always)]
    pub const fn st28(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[inline(always)]
    pub const fn st29(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[inline(always)]
    pub const fn st30(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[inline(always)]
    pub const fn st31(self) -> St {
        unsafe { St::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf0_data(self) -> crate::common::Reg<regs::Buf0Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf1_data(self) -> crate::common::Reg<regs::Buf1Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf2_data(self) -> crate::common::Reg<regs::Buf2Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf3_data(self) -> crate::common::Reg<regs::Buf3Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf4_data(self) -> crate::common::Reg<regs::Buf4Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf5_data(self) -> crate::common::Reg<regs::Buf5Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf6_data(self) -> crate::common::Reg<regs::Buf6Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf7_data(self) -> crate::common::Reg<regs::Buf7Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf8_data(self) -> crate::common::Reg<regs::Buf8Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf9_data(self) -> crate::common::Reg<regs::Buf9Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf10_data(self) -> crate::common::Reg<regs::Buf10Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf11_data(self) -> crate::common::Reg<regs::Buf11Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf12_data(self) -> crate::common::Reg<regs::Buf12Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf13_data(self) -> crate::common::Reg<regs::Buf13Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf14_data(self) -> crate::common::Reg<regs::Buf14Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Scan Results."]
    #[inline(always)]
    pub const fn buf15_data(self) -> crate::common::Reg<regs::Buf15Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn ch0(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn ch1(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn ch2(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn ch3(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[inline(always)]
    pub const fn ch4(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn ch5(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[inline(always)]
    pub const fn ch6(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch7(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch8(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch9(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch10(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch11(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch12(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn ch13(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[inline(always)]
    pub const fn ch14(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[inline(always)]
    pub const fn ch15(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St {
    ptr: *mut u8,
}
unsafe impl Send for St {}
unsafe impl Sync for St {}
impl St {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "State Transition Configuration a."]
    #[inline(always)]
    pub const fn tconfa(self) -> crate::common::Reg<regs::St0Tconfa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "State Transition Configuration B."]
    #[inline(always)]
    pub const fn tconfb(self) -> crate::common::Reg<regs::St0Tconfb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
