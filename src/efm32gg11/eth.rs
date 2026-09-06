#[doc = "ETH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Network control register."]
    #[inline(always)]
    pub const fn networkctrl(self) -> crate::common::Reg<regs::Networkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Network configuration register."]
    #[inline(always)]
    pub const fn networkcfg(self) -> crate::common::Reg<regs::Networkcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Network status register."]
    #[inline(always)]
    pub const fn networkstatus(self) -> crate::common::Reg<regs::Networkstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Transmit status register."]
    #[inline(always)]
    pub const fn txstatus(self) -> crate::common::Reg<regs::Txstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Start address of the receive buffer queue."]
    #[inline(always)]
    pub const fn rxqptr(self) -> crate::common::Reg<regs::Rxqptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Start address of the transmit buffer queue."]
    #[inline(always)]
    pub const fn txqptr(self) -> crate::common::Reg<regs::Txqptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Receive status register."]
    #[inline(always)]
    pub const fn rxstatus(self) -> crate::common::Reg<regs::Rxstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn iens(self) -> crate::common::Reg<regs::Iens, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Disable Register."]
    #[inline(always)]
    pub const fn ienc(self) -> crate::common::Reg<regs::Ienc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt mask register."]
    #[inline(always)]
    pub const fn ienro(self) -> crate::common::Reg<regs::Ienro, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PHY management register."]
    #[inline(always)]
    pub const fn phymngmnt(self) -> crate::common::Reg<regs::Phymngmnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Received Pause Quantum Register."]
    #[inline(always)]
    pub const fn rxpausequant(self) -> crate::common::Reg<regs::Rxpausequant, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Transmit Pause Quantum Register."]
    #[inline(always)]
    pub const fn txpausequant(self) -> crate::common::Reg<regs::Txpausequant, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TX Partial Store and Forward."]
    #[inline(always)]
    pub const fn pbuftxcutthru(self) -> crate::common::Reg<regs::Pbuftxcutthru, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RX Partial Store and Forward."]
    #[inline(always)]
    pub const fn pbufrxcutthru(self) -> crate::common::Reg<regs::Pbufrxcutthru, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Maximum Jumbo Frame Size."]
    #[inline(always)]
    pub const fn jumbomaxlen(self) -> crate::common::Reg<regs::Jumbomaxlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt moderation register."]
    #[inline(always)]
    pub const fn imod(self) -> crate::common::Reg<regs::Imod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "System wake time."]
    #[inline(always)]
    pub const fn syswaketime(self) -> crate::common::Reg<regs::Syswaketime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Hash Register Bottom \\[31:0\\]."]
    #[inline(always)]
    pub const fn hashbottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Hash Register Top \\[63:32\\]."]
    #[inline(always)]
    pub const fn hashtop(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Specific Address 1 Bottom."]
    #[inline(always)]
    pub const fn specaddr1bottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Specific Address 1 Top."]
    #[inline(always)]
    pub const fn specaddr1top(self) -> crate::common::Reg<regs::Specaddr1top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Specific Address 2 Bottom."]
    #[inline(always)]
    pub const fn specaddr2bottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Specific Address 2 Top."]
    #[inline(always)]
    pub const fn specaddr2top(self) -> crate::common::Reg<regs::Specaddr2top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Specific Address 3 Bottom."]
    #[inline(always)]
    pub const fn specaddr3bottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Specific Address 3 Top."]
    #[inline(always)]
    pub const fn specaddr3top(self) -> crate::common::Reg<regs::Specaddr3top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Specific Address 4 Bottom."]
    #[inline(always)]
    pub const fn specaddr4bottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Specific Address 4 Top."]
    #[inline(always)]
    pub const fn specaddr4top(self) -> crate::common::Reg<regs::Specaddr4top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Type ID Match 1."]
    #[inline(always)]
    pub const fn spectype1(self) -> crate::common::Reg<regs::Spectype1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Type ID Match 2."]
    #[inline(always)]
    pub const fn spectype2(self) -> crate::common::Reg<regs::Spectype2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Type ID Match 3."]
    #[inline(always)]
    pub const fn spectype3(self) -> crate::common::Reg<regs::Spectype3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Type ID Match 4."]
    #[inline(always)]
    pub const fn spectype4(self) -> crate::common::Reg<regs::Spectype4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Wake on LAN Register."]
    #[inline(always)]
    pub const fn wolreg(self) -> crate::common::Reg<regs::Wolreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "IPG stretch register."]
    #[inline(always)]
    pub const fn stretchratio(self) -> crate::common::Reg<regs::Stretchratio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Stacked VLAN Register."]
    #[inline(always)]
    pub const fn stackedvlan(self) -> crate::common::Reg<regs::Stackedvlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Transmit PFC Pause Register."]
    #[inline(always)]
    pub const fn txpfcpause(self) -> crate::common::Reg<regs::Txpfcpause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Specific Address Mask 1 Bottom 31:0."]
    #[inline(always)]
    pub const fn maskadd1bottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Specific Address Mask 1 Top 47:32."]
    #[inline(always)]
    pub const fn maskadd1top(self) -> crate::common::Reg<regs::Maskadd1top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "PTP RX unicast IP destination address."]
    #[inline(always)]
    pub const fn rxptpunicast(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "PTP TX unicast IP destination address."]
    #[inline(always)]
    pub const fn txptpunicast(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "TSU timer comparison value nanoseconds."]
    #[inline(always)]
    pub const fn tsunseccmp(self) -> crate::common::Reg<regs::Tsunseccmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "TSU timer comparison value seconds \\[31:0\\]."]
    #[inline(always)]
    pub const fn tsuseccmp(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "TSU timer comparison value seconds \\[47:32\\]."]
    #[inline(always)]
    pub const fn tsumsbseccmp(self) -> crate::common::Reg<regs::Tsumsbseccmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "PTP Event Frame Transmitted Seconds Register 47:32."]
    #[inline(always)]
    pub const fn tsuptptxmsbsec(
        self,
    ) -> crate::common::Reg<regs::Tsuptptxmsbsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "PTP Event Frame Received Seconds Register 47:32."]
    #[inline(always)]
    pub const fn tsuptprxmsbsec(
        self,
    ) -> crate::common::Reg<regs::Tsuptprxmsbsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32."]
    #[inline(always)]
    pub const fn tsupeertxmsbsec(
        self,
    ) -> crate::common::Reg<regs::Tsupeertxmsbsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "PTP Peer Event Frame Received Seconds Register 47:32."]
    #[inline(always)]
    pub const fn tsupeerrxmsbsec(
        self,
    ) -> crate::common::Reg<regs::Tsupeerrxmsbsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Octets transmitted 31:0."]
    #[inline(always)]
    pub const fn octetstxedbottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Octets Transmitted 47:32."]
    #[inline(always)]
    pub const fn octetstxedtop(self) -> crate::common::Reg<regs::Octetstxedtop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxedok(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Broadcast Frames Transmitted."]
    #[inline(always)]
    pub const fn broadcasttxed(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Multicast Frames Transmitted."]
    #[inline(always)]
    pub const fn multicasttxed(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Pause Frames Transmitted."]
    #[inline(always)]
    pub const fn pframestxed(self) -> crate::common::Reg<regs::Pframestxed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "64 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed64(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "65 to 127 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed65(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "128 to 255 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed128(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "256 to 511 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed256(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "512 to 1023 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed512(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "1024 to 1518 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed1024(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Greater Than 1518 Byte Frames Transmitted."]
    #[inline(always)]
    pub const fn framestxed1519(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Transmit Under Runs."]
    #[inline(always)]
    pub const fn txunderruns(self) -> crate::common::Reg<regs::Txunderruns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Single Collision Frames."]
    #[inline(always)]
    pub const fn singlecols(self) -> crate::common::Reg<regs::Singlecols, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Multiple Collision Frames."]
    #[inline(always)]
    pub const fn multicols(self) -> crate::common::Reg<regs::Multicols, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Excessive Collisions."]
    #[inline(always)]
    pub const fn excesscols(self) -> crate::common::Reg<regs::Excesscols, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Late Collisions."]
    #[inline(always)]
    pub const fn latecols(self) -> crate::common::Reg<regs::Latecols, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Deferred Transmission Frames."]
    #[inline(always)]
    pub const fn deferredframes(
        self,
    ) -> crate::common::Reg<regs::Deferredframes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Carrier Sense Errors."]
    #[inline(always)]
    pub const fn crserrs(self) -> crate::common::Reg<regs::Crserrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Octets Received 31:0."]
    #[inline(always)]
    pub const fn octetsrxedbottom(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Octets Received 47:32."]
    #[inline(always)]
    pub const fn octetsrxedtop(self) -> crate::common::Reg<regs::Octetsrxedtop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Frames Received."]
    #[inline(always)]
    pub const fn framesrxedok(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Broadcast Frames Received."]
    #[inline(always)]
    pub const fn broadcastrxed(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Multicast Frames Received."]
    #[inline(always)]
    pub const fn multicastrxed(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Pause Frames Received."]
    #[inline(always)]
    pub const fn pframesrxed(self) -> crate::common::Reg<regs::Pframesrxed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "64 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed64(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "65 to 127 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed65(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "128 to 255 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed128(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "256 to 511 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed256(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "512 to 1023 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed512(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "1024 to 1518 Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed1024(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "1519 to maximum Byte Frames Received."]
    #[inline(always)]
    pub const fn framesrxed1519(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Undersized Frames Received."]
    #[inline(always)]
    pub const fn undersizeframes(
        self,
    ) -> crate::common::Reg<regs::Undersizeframes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Oversize Frames Received."]
    #[inline(always)]
    pub const fn excessiverxlen(
        self,
    ) -> crate::common::Reg<regs::Excessiverxlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Jabbers Received."]
    #[inline(always)]
    pub const fn rxjabbers(self) -> crate::common::Reg<regs::Rxjabbers, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Frame Check Sequence Errors."]
    #[inline(always)]
    pub const fn fcserrs(self) -> crate::common::Reg<regs::Fcserrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Length Field Frame Errors."]
    #[inline(always)]
    pub const fn rxlenerrs(self) -> crate::common::Reg<regs::Rxlenerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Receive Symbol Errors."]
    #[inline(always)]
    pub const fn rxsymbolerrs(self) -> crate::common::Reg<regs::Rxsymbolerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Alignment Errors."]
    #[inline(always)]
    pub const fn alignerrs(self) -> crate::common::Reg<regs::Alignerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Receive Resource Errors."]
    #[inline(always)]
    pub const fn rxresourceerrs(
        self,
    ) -> crate::common::Reg<regs::Rxresourceerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Receive Overruns."]
    #[inline(always)]
    pub const fn rxoverruns(self) -> crate::common::Reg<regs::Rxoverruns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "IP Header Checksum Errors."]
    #[inline(always)]
    pub const fn rxipckerrs(self) -> crate::common::Reg<regs::Rxipckerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "TCP Checksum Errors."]
    #[inline(always)]
    pub const fn rxtcpckerrs(self) -> crate::common::Reg<regs::Rxtcpckerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "UDP Checksum Errors."]
    #[inline(always)]
    pub const fn rxudpckerrs(self) -> crate::common::Reg<regs::Rxudpckerrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Receive DMA Flushed Packets."]
    #[inline(always)]
    pub const fn autoflushedpkts(
        self,
    ) -> crate::common::Reg<regs::Autoflushedpkts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "1588 Timer Increment Register subscript nsec."]
    #[inline(always)]
    pub const fn tsutimerincrsubnsec(
        self,
    ) -> crate::common::Reg<regs::Tsutimerincrsubnsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "1588 Timer Seconds Register 47:32."]
    #[inline(always)]
    pub const fn tsutimermsbsec(
        self,
    ) -> crate::common::Reg<regs::Tsutimermsbsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "1588 Timer Seconds Register 31:0."]
    #[inline(always)]
    pub const fn tsutimersec(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "1588 Timer Nanoseconds Register."]
    #[inline(always)]
    pub const fn tsutimernsec(self) -> crate::common::Reg<regs::Tsutimernsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "This register returns all zeroes when read."]
    #[inline(always)]
    pub const fn tsutimeradjust(
        self,
    ) -> crate::common::Reg<regs::Tsutimeradjust, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "1588 Timer Increment Register."]
    #[inline(always)]
    pub const fn tsutimerincr(self) -> crate::common::Reg<regs::Tsutimerincr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "PTP Event Frame Transmitted Seconds Register 31:0."]
    #[inline(always)]
    pub const fn tsuptptxsec(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "PTP Event Frame Transmitted Nanoseconds Register."]
    #[inline(always)]
    pub const fn tsuptptxnsec(self) -> crate::common::Reg<regs::Tsuptptxnsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "PTP Event Frame Received Seconds Register 31:0."]
    #[inline(always)]
    pub const fn tsuptprxsec(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "PTP Event Frame Received Nanoseconds Register."]
    #[inline(always)]
    pub const fn tsuptprxnsec(self) -> crate::common::Reg<regs::Tsuptprxnsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0."]
    #[inline(always)]
    pub const fn tsupeertxsec(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register."]
    #[inline(always)]
    pub const fn tsupeertxnsec(self) -> crate::common::Reg<regs::Tsupeertxnsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "PTP Peer Event Frame Received Seconds Register 31:0."]
    #[inline(always)]
    pub const fn tsupeerrxsec(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "PTP Peer Event Frame Received Nanoseconds Register."]
    #[inline(always)]
    pub const fn tsupeerrxnsec(self) -> crate::common::Reg<regs::Tsupeerrxnsec, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Transmit Pause Quantum Register 1."]
    #[inline(always)]
    pub const fn txpausequant1(self) -> crate::common::Reg<regs::Txpausequant1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Transmit Pause Quantum Register 2."]
    #[inline(always)]
    pub const fn txpausequant2(self) -> crate::common::Reg<regs::Txpausequant2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Transmit Pause Quantum Register 3."]
    #[inline(always)]
    pub const fn txpausequant3(self) -> crate::common::Reg<regs::Txpausequant3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Received LPI transitions."]
    #[inline(always)]
    pub const fn rxlpi(self) -> crate::common::Reg<regs::Rxlpi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Received LPI time."]
    #[inline(always)]
    pub const fn rxlpitime(self) -> crate::common::Reg<regs::Rxlpitime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Transmit LPI transitions."]
    #[inline(always)]
    pub const fn txlpi(self) -> crate::common::Reg<regs::Txlpi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Transmit LPI time."]
    #[inline(always)]
    pub const fn txlpitime(self) -> crate::common::Reg<regs::Txlpitime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "TX BD control register."]
    #[inline(always)]
    pub const fn txbdctrl(self) -> crate::common::Reg<regs::Txbdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "RX BD control register."]
    #[inline(always)]
    pub const fn rxbdctrl(self) -> crate::common::Reg<regs::Rxbdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "I/O Route Enable Register."]
    #[inline(always)]
    pub const fn routepen(self) -> crate::common::Reg<regs::Routepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "I/O Route Location Register 0."]
    #[inline(always)]
    pub const fn routeloc0(self) -> crate::common::Reg<regs::Routeloc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "I/O Route Location Register 1."]
    #[inline(always)]
    pub const fn routeloc1(self) -> crate::common::Reg<regs::Routeloc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c0cusize) as _) }
    }
    #[doc = "Ethernet control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c10usize) as _) }
    }
}
pub mod regs;
pub mod vals;
