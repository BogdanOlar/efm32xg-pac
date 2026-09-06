#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Cfg(pub u32);
impl Ch0Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch0CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch0CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch0CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch0Cfg {
    #[inline(always)]
    fn default() -> Ch0Cfg {
        Ch0Cfg(0)
    }
}
impl core::fmt::Debug for Ch0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Ctrl(pub u32);
impl Ch0Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch0CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch0CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch0CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch0CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch0CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch0CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch0CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch0CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch0CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch0CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch0CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch0CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch0CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch0CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch0CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch0Ctrl {
    #[inline(always)]
    fn default() -> Ch0Ctrl {
        Ch0Ctrl(0)
    }
}
impl core::fmt::Debug for Ch0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch0Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Link(pub u32);
impl Ch0Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch0Link {
    #[inline(always)]
    fn default() -> Ch0Link {
        Ch0Link(0)
    }
}
impl core::fmt::Debug for Ch0Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Loop(pub u32);
impl Ch0Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch0Loop {
    #[inline(always)]
    fn default() -> Ch0Loop {
        Ch0Loop(0)
    }
}
impl core::fmt::Debug for Ch0Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch0Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0Reqsel(pub u32);
impl Ch0Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch0ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch0ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch0ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch0Reqsel {
    #[inline(always)]
    fn default() -> Ch0Reqsel {
        Ch0Reqsel(0)
    }
}
impl core::fmt::Debug for Ch0Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch0Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch0Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Cfg(pub u32);
impl Ch1Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch1CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch1CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch1CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch1Cfg {
    #[inline(always)]
    fn default() -> Ch1Cfg {
        Ch1Cfg(0)
    }
}
impl core::fmt::Debug for Ch1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch1Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Ctrl(pub u32);
impl Ch1Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch1CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch1CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch1CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch1CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch1CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch1CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch1CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch1CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch1CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch1CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch1CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch1CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch1CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch1CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch1CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch1Ctrl {
    #[inline(always)]
    fn default() -> Ch1Ctrl {
        Ch1Ctrl(0)
    }
}
impl core::fmt::Debug for Ch1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch1Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Link(pub u32);
impl Ch1Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch1Link {
    #[inline(always)]
    fn default() -> Ch1Link {
        Ch1Link(0)
    }
}
impl core::fmt::Debug for Ch1Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch1Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Loop(pub u32);
impl Ch1Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch1Loop {
    #[inline(always)]
    fn default() -> Ch1Loop {
        Ch1Loop(0)
    }
}
impl core::fmt::Debug for Ch1Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch1Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1Reqsel(pub u32);
impl Ch1Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch1ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch1ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch1ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch1Reqsel {
    #[inline(always)]
    fn default() -> Ch1Reqsel {
        Ch1Reqsel(0)
    }
}
impl core::fmt::Debug for Ch1Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch1Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch1Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Cfg(pub u32);
impl Ch2Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch2CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch2CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch2CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch2Cfg {
    #[inline(always)]
    fn default() -> Ch2Cfg {
        Ch2Cfg(0)
    }
}
impl core::fmt::Debug for Ch2Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch2Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Ctrl(pub u32);
impl Ch2Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch2CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch2CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch2CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch2CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch2CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch2CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch2CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch2CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch2CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch2CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch2CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch2CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch2CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch2CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch2CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch2Ctrl {
    #[inline(always)]
    fn default() -> Ch2Ctrl {
        Ch2Ctrl(0)
    }
}
impl core::fmt::Debug for Ch2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch2Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Link(pub u32);
impl Ch2Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch2Link {
    #[inline(always)]
    fn default() -> Ch2Link {
        Ch2Link(0)
    }
}
impl core::fmt::Debug for Ch2Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch2Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Loop(pub u32);
impl Ch2Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch2Loop {
    #[inline(always)]
    fn default() -> Ch2Loop {
        Ch2Loop(0)
    }
}
impl core::fmt::Debug for Ch2Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch2Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2Reqsel(pub u32);
impl Ch2Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch2ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch2ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch2ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch2Reqsel {
    #[inline(always)]
    fn default() -> Ch2Reqsel {
        Ch2Reqsel(0)
    }
}
impl core::fmt::Debug for Ch2Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch2Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch2Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Cfg(pub u32);
impl Ch3Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch3CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch3CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch3CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch3Cfg {
    #[inline(always)]
    fn default() -> Ch3Cfg {
        Ch3Cfg(0)
    }
}
impl core::fmt::Debug for Ch3Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch3Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Ctrl(pub u32);
impl Ch3Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch3CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch3CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch3CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch3CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch3CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch3CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch3CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch3CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch3CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch3CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch3CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch3CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch3CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch3CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch3CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch3Ctrl {
    #[inline(always)]
    fn default() -> Ch3Ctrl {
        Ch3Ctrl(0)
    }
}
impl core::fmt::Debug for Ch3Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch3Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Link(pub u32);
impl Ch3Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch3Link {
    #[inline(always)]
    fn default() -> Ch3Link {
        Ch3Link(0)
    }
}
impl core::fmt::Debug for Ch3Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch3Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Loop(pub u32);
impl Ch3Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch3Loop {
    #[inline(always)]
    fn default() -> Ch3Loop {
        Ch3Loop(0)
    }
}
impl core::fmt::Debug for Ch3Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch3Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3Reqsel(pub u32);
impl Ch3Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch3ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch3ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch3ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch3Reqsel {
    #[inline(always)]
    fn default() -> Ch3Reqsel {
        Ch3Reqsel(0)
    }
}
impl core::fmt::Debug for Ch3Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch3Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch3Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Cfg(pub u32);
impl Ch4Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch4CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch4CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch4CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch4Cfg {
    #[inline(always)]
    fn default() -> Ch4Cfg {
        Ch4Cfg(0)
    }
}
impl core::fmt::Debug for Ch4Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch4Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Ctrl(pub u32);
impl Ch4Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch4CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch4CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch4CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch4CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch4CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch4CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch4CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch4CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch4CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch4CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch4CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch4CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch4CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch4CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch4CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch4Ctrl {
    #[inline(always)]
    fn default() -> Ch4Ctrl {
        Ch4Ctrl(0)
    }
}
impl core::fmt::Debug for Ch4Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch4Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Link(pub u32);
impl Ch4Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch4Link {
    #[inline(always)]
    fn default() -> Ch4Link {
        Ch4Link(0)
    }
}
impl core::fmt::Debug for Ch4Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch4Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Loop(pub u32);
impl Ch4Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch4Loop {
    #[inline(always)]
    fn default() -> Ch4Loop {
        Ch4Loop(0)
    }
}
impl core::fmt::Debug for Ch4Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch4Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4Reqsel(pub u32);
impl Ch4Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch4ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch4ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch4ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch4Reqsel {
    #[inline(always)]
    fn default() -> Ch4Reqsel {
        Ch4Reqsel(0)
    }
}
impl core::fmt::Debug for Ch4Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch4Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch4Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch4Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Cfg(pub u32);
impl Ch5Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch5CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch5CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch5CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch5Cfg {
    #[inline(always)]
    fn default() -> Ch5Cfg {
        Ch5Cfg(0)
    }
}
impl core::fmt::Debug for Ch5Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch5Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Ctrl(pub u32);
impl Ch5Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch5CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch5CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch5CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch5CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch5CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch5CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch5CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch5CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch5CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch5CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch5CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch5CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch5CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch5CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch5CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch5Ctrl {
    #[inline(always)]
    fn default() -> Ch5Ctrl {
        Ch5Ctrl(0)
    }
}
impl core::fmt::Debug for Ch5Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch5Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Link(pub u32);
impl Ch5Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch5Link {
    #[inline(always)]
    fn default() -> Ch5Link {
        Ch5Link(0)
    }
}
impl core::fmt::Debug for Ch5Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch5Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Loop(pub u32);
impl Ch5Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch5Loop {
    #[inline(always)]
    fn default() -> Ch5Loop {
        Ch5Loop(0)
    }
}
impl core::fmt::Debug for Ch5Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch5Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5Reqsel(pub u32);
impl Ch5Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch5ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch5ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch5ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch5Reqsel {
    #[inline(always)]
    fn default() -> Ch5Reqsel {
        Ch5Reqsel(0)
    }
}
impl core::fmt::Debug for Ch5Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch5Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch5Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch5Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Cfg(pub u32);
impl Ch6Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch6CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch6CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch6CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch6Cfg {
    #[inline(always)]
    fn default() -> Ch6Cfg {
        Ch6Cfg(0)
    }
}
impl core::fmt::Debug for Ch6Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch6Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Ctrl(pub u32);
impl Ch6Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch6CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch6CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch6CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch6CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch6CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch6CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch6CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch6CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch6CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch6CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch6CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch6CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch6CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch6CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch6CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch6Ctrl {
    #[inline(always)]
    fn default() -> Ch6Ctrl {
        Ch6Ctrl(0)
    }
}
impl core::fmt::Debug for Ch6Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch6Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Link(pub u32);
impl Ch6Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch6Link {
    #[inline(always)]
    fn default() -> Ch6Link {
        Ch6Link(0)
    }
}
impl core::fmt::Debug for Ch6Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch6Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Loop(pub u32);
impl Ch6Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch6Loop {
    #[inline(always)]
    fn default() -> Ch6Loop {
        Ch6Loop(0)
    }
}
impl core::fmt::Debug for Ch6Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch6Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6Reqsel(pub u32);
impl Ch6Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch6ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch6ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch6ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch6Reqsel {
    #[inline(always)]
    fn default() -> Ch6Reqsel {
        Ch6Reqsel(0)
    }
}
impl core::fmt::Debug for Ch6Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch6Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch6Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch6Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "Channel Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Cfg(pub u32);
impl Ch7Cfg {
    #[doc = "Arbitration Slot Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn arbslots(&self) -> super::vals::Ch7CfgArbslots {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ch7CfgArbslots::from_bits(val as u8)
    }
    #[doc = "Arbitration Slot Number Select."]
    #[inline(always)]
    pub const fn set_arbslots(&mut self, val: super::vals::Ch7CfgArbslots) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Source Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn srcincsign(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Increment Sign."]
    #[inline(always)]
    pub const fn set_srcincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Destination Address Increment Sign."]
    #[must_use]
    #[inline(always)]
    pub const fn dstincsign(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Increment Sign."]
    #[inline(always)]
    pub const fn set_dstincsign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ch7Cfg {
    #[inline(always)]
    fn default() -> Ch7Cfg {
        Ch7Cfg(0)
    }
}
impl core::fmt::Debug for Ch7Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Cfg")
            .field("arbslots", &self.arbslots())
            .field("srcincsign", &self.srcincsign())
            .field("dstincsign", &self.dstincsign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch7Cfg {{ arbslots: {:?}, srcincsign: {=bool:?}, dstincsign: {=bool:?} }}",
            self.arbslots(),
            self.srcincsign(),
            self.dstincsign()
        )
    }
}
#[doc = "Channel Descriptor Control Word Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Ctrl(pub u32);
impl Ch7Ctrl {
    #[doc = "DMA Structure Type."]
    #[must_use]
    #[inline(always)]
    pub const fn structtype(&self) -> super::vals::Ch7CtrlStructtype {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ch7CtrlStructtype::from_bits(val as u8)
    }
    #[doc = "DMA Structure Type."]
    #[inline(always)]
    pub const fn set_structtype(&mut self, val: super::vals::Ch7CtrlStructtype) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Structure DMA Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn structreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Structure DMA Transfer Request."]
    #[inline(always)]
    pub const fn set_structreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x07ff;
        val as u16
    }
    #[doc = "DMA Unit Data Transfer Count."]
    #[inline(always)]
    pub const fn set_xfercnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
    }
    #[doc = "Endian Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn byteswap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Endian Byte Swap."]
    #[inline(always)]
    pub const fn set_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Ch7CtrlBlocksize {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ch7CtrlBlocksize::from_bits(val as u8)
    }
    #[doc = "Block Transfer Size."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Ch7CtrlBlocksize) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doneifsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Operation Done Interrupt Flag Set Enable."]
    #[inline(always)]
    pub const fn set_doneifsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reqmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Transfer Mode Select."]
    #[inline(always)]
    pub const fn set_reqmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Decrement Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn decloopcnt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Loop Count."]
    #[inline(always)]
    pub const fn set_decloopcnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Ignore Sreq."]
    #[must_use]
    #[inline(always)]
    pub const fn ignoresreq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore Sreq."]
    #[inline(always)]
    pub const fn set_ignoresreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Source Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Ch7CtrlSrcinc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ch7CtrlSrcinc::from_bits(val as u8)
    }
    #[doc = "Source Address Increment Size."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Ch7CtrlSrcinc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Unit Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::Ch7CtrlSize {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ch7CtrlSize::from_bits(val as u8)
    }
    #[doc = "Unit Data Transfer Size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: super::vals::Ch7CtrlSize) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Destination Address Increment Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Ch7CtrlDstinc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ch7CtrlDstinc::from_bits(val as u8)
    }
    #[doc = "Destination Address Increment Size."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Ch7CtrlDstinc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Source Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn srcmode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Source Addressing Mode."]
    #[inline(always)]
    pub const fn set_srcmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Destination Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dstmode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Addressing Mode."]
    #[inline(always)]
    pub const fn set_dstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ch7Ctrl {
    #[inline(always)]
    fn default() -> Ch7Ctrl {
        Ch7Ctrl(0)
    }
}
impl core::fmt::Debug for Ch7Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Ctrl")
            .field("structtype", &self.structtype())
            .field("structreq", &self.structreq())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ch7Ctrl {{ structtype: {:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneifsen: {=bool:?}, reqmode: {=bool:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {=bool:?}, dstmode: {=bool:?} }}" , self . structtype () , self . structreq () , self . xfercnt () , self . byteswap () , self . blocksize () , self . doneifsen () , self . reqmode () , self . decloopcnt () , self . ignoresreq () , self . srcinc () , self . size () , self . dstinc () , self . srcmode () , self . dstmode ())
    }
}
#[doc = "Channel Descriptor Link Structure Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Link(pub u32);
impl Ch7Link {
    #[doc = "Link Structure Addressing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn linkmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Structure Addressing Mode."]
    #[inline(always)]
    pub const fn set_linkmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Next Structure."]
    #[must_use]
    #[inline(always)]
    pub const fn link(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Link Next Structure."]
    #[inline(always)]
    pub const fn set_link(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Link Structure Address."]
    #[must_use]
    #[inline(always)]
    pub const fn linkaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Link Structure Address."]
    #[inline(always)]
    pub const fn set_linkaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Ch7Link {
    #[inline(always)]
    fn default() -> Ch7Link {
        Ch7Link(0)
    }
}
impl core::fmt::Debug for Ch7Link {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Link")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Link {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch7Link {{ linkmode: {=bool:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
            self.linkmode(),
            self.link(),
            self.linkaddr()
        )
    }
}
#[doc = "Channel Loop Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Loop(pub u32);
impl Ch7Loop {
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Linked Structure Sequence Loop Counter."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ch7Loop {
    #[inline(always)]
    fn default() -> Ch7Loop {
        Ch7Loop(0)
    }
}
impl core::fmt::Debug for Ch7Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Loop")
            .field("loopcnt", &self.loopcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Loop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch7Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
    }
}
#[doc = "Channel Peripheral Request Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7Reqsel(pub u32);
impl Ch7Reqsel {
    #[doc = "Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Select."]
    #[inline(always)]
    pub const fn set_sigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sourcesel(&self) -> super::vals::Ch7ReqselSourcesel {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Ch7ReqselSourcesel::from_bits(val as u8)
    }
    #[doc = "Source Select."]
    #[inline(always)]
    pub const fn set_sourcesel(&mut self, val: super::vals::Ch7ReqselSourcesel) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ch7Reqsel {
    #[inline(always)]
    fn default() -> Ch7Reqsel {
        Ch7Reqsel(0)
    }
}
impl core::fmt::Debug for Ch7Reqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch7Reqsel")
            .field("sigsel", &self.sigsel())
            .field("sourcesel", &self.sourcesel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch7Reqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ch7Reqsel {{ sigsel: {=u8:?}, sourcesel: {:?} }}",
            self.sigsel(),
            self.sourcesel()
        )
    }
}
#[doc = "DMA Channel Busy Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chbusy(pub u32);
impl Chbusy {
    #[doc = "Channels Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Channels Busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Chbusy {
    #[inline(always)]
    fn default() -> Chbusy {
        Chbusy(0)
    }
}
impl core::fmt::Debug for Chbusy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chbusy")
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chbusy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chbusy {{ busy: {=u8:?} }}", self.busy())
    }
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chdone(pub u32);
impl Chdone {
    #[doc = "DMA Channel Linking or Done."]
    #[must_use]
    #[inline(always)]
    pub const fn chdone(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Channel Linking or Done."]
    #[inline(always)]
    pub const fn set_chdone(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Chdone {
    #[inline(always)]
    fn default() -> Chdone {
        Chdone(0)
    }
}
impl core::fmt::Debug for Chdone {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chdone")
            .field("chdone", &self.chdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chdone {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chdone {{ chdone: {=u8:?} }}", self.chdone())
    }
}
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chen(pub u32);
impl Chen {
    #[doc = "Channel Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn chen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Channel Enables."]
    #[inline(always)]
    pub const fn set_chen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Chen {
    #[inline(always)]
    fn default() -> Chen {
        Chen(0)
    }
}
impl core::fmt::Debug for Chen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chen").field("chen", &self.chen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chen {{ chen: {=u8:?} }}", self.chen())
    }
}
#[doc = "DMA Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Synchronization PRS Set Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn syncprsseten(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronization PRS Set Enable."]
    #[inline(always)]
    pub const fn set_syncprsseten(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Synchronization PRS Clear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn syncprsclren(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronization PRS Clear Enable."]
    #[inline(always)]
    pub const fn set_syncprsclren(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of Fixed Priority Channels."]
    #[must_use]
    #[inline(always)]
    pub const fn numfixed(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Fixed Priority Channels."]
    #[inline(always)]
    pub const fn set_numfixed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("syncprsseten", &self.syncprsseten())
            .field("syncprsclren", &self.syncprsclren())
            .field("numfixed", &self.numfixed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ syncprsseten: {=u8:?}, syncprsclren: {=u8:?}, numfixed: {=u8:?} }}",
            self.syncprsseten(),
            self.syncprsclren(),
            self.numfixed()
        )
    }
}
#[doc = "DMA Channel Debug Halt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbghalt(pub u32);
impl Dbghalt {
    #[doc = "DMA Debug Halt."]
    #[must_use]
    #[inline(always)]
    pub const fn dbghalt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Debug Halt."]
    #[inline(always)]
    pub const fn set_dbghalt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dbghalt {
    #[inline(always)]
    fn default() -> Dbghalt {
        Dbghalt(0)
    }
}
impl core::fmt::Debug for Dbghalt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbghalt")
            .field("dbghalt", &self.dbghalt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbghalt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbghalt {{ dbghalt: {=u8:?} }}", self.dbghalt())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "DONE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DONE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "ERROR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ERROR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ien {
    #[inline(always)]
    fn default() -> Ien {
        Ien(0)
    }
}
impl core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ien")
            .field("done", &self.done())
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ done: {=u8:?}, error: {=bool:?} }}",
            self.done(),
            self.error()
        )
    }
}
#[doc = "Interrupt Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "DMA Structure Operation Done Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Structure Operation Done Interrupt Flag."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transfer Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for If {
    #[inline(always)]
    fn default() -> If {
        If(0)
    }
}
impl core::fmt::Debug for If {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("If")
            .field("done", &self.done())
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "If {{ done: {=u8:?}, error: {=bool:?} }}",
            self.done(),
            self.error()
        )
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set DONE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Set DONE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Set ERROR Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set ERROR Interrupt Flag."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ifs {
    #[inline(always)]
    fn default() -> Ifs {
        Ifs(0)
    }
}
impl core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifs")
            .field("done", &self.done())
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ done: {=u8:?}, error: {=bool:?} }}",
            self.done(),
            self.error()
        )
    }
}
#[doc = "DMA Channel Link Load Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Linkload(pub u32);
impl Linkload {
    #[doc = "DMA Link Loads."]
    #[must_use]
    #[inline(always)]
    pub const fn linkload(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Link Loads."]
    #[inline(always)]
    pub const fn set_linkload(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Linkload {
    #[inline(always)]
    fn default() -> Linkload {
        Linkload(0)
    }
}
impl core::fmt::Debug for Linkload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Linkload")
            .field("linkload", &self.linkload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Linkload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Linkload {{ linkload: {=u8:?} }}", self.linkload())
    }
}
#[doc = "DMA Channel Request Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reqclear(pub u32);
impl Reqclear {
    #[doc = "DMA Request Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn reqclear(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Request Clear."]
    #[inline(always)]
    pub const fn set_reqclear(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Reqclear {
    #[inline(always)]
    fn default() -> Reqclear {
        Reqclear(0)
    }
}
impl core::fmt::Debug for Reqclear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reqclear")
            .field("reqclear", &self.reqclear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reqclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reqclear {{ reqclear: {=u8:?} }}", self.reqclear())
    }
}
#[doc = "DMA Channel Request Disable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reqdis(pub u32);
impl Reqdis {
    #[doc = "DMA Request Disables."]
    #[must_use]
    #[inline(always)]
    pub const fn reqdis(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Request Disables."]
    #[inline(always)]
    pub const fn set_reqdis(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Reqdis {
    #[inline(always)]
    fn default() -> Reqdis {
        Reqdis(0)
    }
}
impl core::fmt::Debug for Reqdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reqdis")
            .field("reqdis", &self.reqdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reqdis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reqdis {{ reqdis: {=u8:?} }}", self.reqdis())
    }
}
#[doc = "DMA Channel Requests Pending Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reqpend(pub u32);
impl Reqpend {
    #[doc = "DMA Requests Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn reqpend(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Requests Pending."]
    #[inline(always)]
    pub const fn set_reqpend(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Reqpend {
    #[inline(always)]
    fn default() -> Reqpend {
        Reqpend(0)
    }
}
impl core::fmt::Debug for Reqpend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reqpend")
            .field("reqpend", &self.reqpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reqpend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reqpend {{ reqpend: {=u8:?} }}", self.reqpend())
    }
}
#[doc = "DMA Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Any DMA Channel Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn anybusy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Any DMA Channel Busy."]
    #[inline(always)]
    pub const fn set_anybusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Any DMA Channel Request Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn anyreq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Any DMA Channel Request Pending."]
    #[inline(always)]
    pub const fn set_anyreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Granted Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn chgrant(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Granted Channel Number."]
    #[inline(always)]
    pub const fn set_chgrant(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Errant Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn cherror(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Errant Channel Number."]
    #[inline(always)]
    pub const fn set_cherror(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "FIFO Level."]
    #[must_use]
    #[inline(always)]
    pub const fn fifolevel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Level."]
    #[inline(always)]
    pub const fn set_fifolevel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Number of Channels."]
    #[must_use]
    #[inline(always)]
    pub const fn chnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Channels."]
    #[inline(always)]
    pub const fn set_chnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("anybusy", &self.anybusy())
            .field("anyreq", &self.anyreq())
            .field("chgrant", &self.chgrant())
            .field("cherror", &self.cherror())
            .field("fifolevel", &self.fifolevel())
            .field("chnum", &self.chnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ anybusy: {=bool:?}, anyreq: {=bool:?}, chgrant: {=u8:?}, cherror: {=u8:?}, fifolevel: {=u8:?}, chnum: {=u8:?} }}" , self . anybusy () , self . anyreq () , self . chgrant () , self . cherror () , self . fifolevel () , self . chnum ())
    }
}
#[doc = "DMA Channel Software Transfer Request Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swreq(pub u32);
impl Swreq {
    #[doc = "Software Transfer Requests."]
    #[must_use]
    #[inline(always)]
    pub const fn swreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Software Transfer Requests."]
    #[inline(always)]
    pub const fn set_swreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swreq {
    #[inline(always)]
    fn default() -> Swreq {
        Swreq(0)
    }
}
impl core::fmt::Debug for Swreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swreq")
            .field("swreq", &self.swreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swreq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swreq {{ swreq: {=u8:?} }}", self.swreq())
    }
}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sync(pub u32);
impl Sync {
    #[doc = "Synchronization Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn synctrig(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronization Trigger."]
    #[inline(always)]
    pub const fn set_synctrig(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sync {
    #[inline(always)]
    fn default() -> Sync {
        Sync(0)
    }
}
impl core::fmt::Debug for Sync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sync")
            .field("synctrig", &self.synctrig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sync {{ synctrig: {=u8:?} }}", self.synctrig())
    }
}
