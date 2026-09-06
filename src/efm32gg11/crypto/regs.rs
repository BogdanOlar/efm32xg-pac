#[doc = "Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Execute Instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn instr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Execute Instruction."]
    #[inline(always)]
    pub const fn set_instr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Encryption/Decryption SEQUENCE Start."]
    #[must_use]
    #[inline(always)]
    pub const fn seqstart(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Encryption/Decryption SEQUENCE Start."]
    #[inline(always)]
    pub const fn set_seqstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Sequence Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn seqstop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Stop."]
    #[inline(always)]
    pub const fn set_seqstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence Step."]
    #[must_use]
    #[inline(always)]
    pub const fn seqstep(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Step."]
    #[inline(always)]
    pub const fn set_seqstep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd")
            .field("instr", &self.instr())
            .field("seqstart", &self.seqstart())
            .field("seqstop", &self.seqstop())
            .field("seqstep", &self.seqstep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmd {{ instr: {=u8:?}, seqstart: {=bool:?}, seqstop: {=bool:?}, seqstep: {=bool:?} }}",
            self.instr(),
            self.seqstart(),
            self.seqstop(),
            self.seqstep()
        )
    }
}
#[doc = "Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cstatus(pub u32);
impl Cstatus {
    #[doc = "Selected ALU Operand 0."]
    #[must_use]
    #[inline(always)]
    pub const fn v0(&self) -> super::vals::V0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::V0::from_bits(val as u8)
    }
    #[doc = "Selected ALU Operand 0."]
    #[inline(always)]
    pub const fn set_v0(&mut self, val: super::vals::V0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Selected ALU Operand 1."]
    #[must_use]
    #[inline(always)]
    pub const fn v1(&self) -> super::vals::V1 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::V1::from_bits(val as u8)
    }
    #[doc = "Selected ALU Operand 1."]
    #[inline(always)]
    pub const fn set_v1(&mut self, val: super::vals::V1) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Sequence Part."]
    #[must_use]
    #[inline(always)]
    pub const fn seqpart(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Part."]
    #[inline(always)]
    pub const fn set_seqpart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Sequence Skip Next Instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn seqskip(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Skip Next Instruction."]
    #[inline(always)]
    pub const fn set_seqskip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Sequence Next Instruction Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn seqip(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Next Instruction Pointer."]
    #[inline(always)]
    pub const fn set_seqip(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
}
impl Default for Cstatus {
    #[inline(always)]
    fn default() -> Cstatus {
        Cstatus(0)
    }
}
impl core::fmt::Debug for Cstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cstatus")
            .field("v0", &self.v0())
            .field("v1", &self.v1())
            .field("seqpart", &self.seqpart())
            .field("seqskip", &self.seqskip())
            .field("seqip", &self.seqip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cstatus {{ v0: {:?}, v1: {:?}, seqpart: {=bool:?}, seqskip: {=bool:?}, seqip: {=u8:?} }}" , self . v0 () , self . v1 () , self . seqpart () , self . seqskip () , self . seqip ())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "AES Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES Mode."]
    #[inline(always)]
    pub const fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Key Buffer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn keybufdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Key Buffer Disable."]
    #[inline(always)]
    pub const fn set_keybufdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SHA Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sha(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SHA Mode."]
    #[inline(always)]
    pub const fn set_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "No Stalling of Bus When Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn nobusystall(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "No Stalling of Bus When Busy."]
    #[inline(always)]
    pub const fn set_nobusystall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Increment Width."]
    #[must_use]
    #[inline(always)]
    pub const fn incwidth(&self) -> super::vals::Incwidth {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Incwidth::from_bits(val as u8)
    }
    #[doc = "Increment Width."]
    #[inline(always)]
    pub const fn set_incwidth(&mut self, val: super::vals::Incwidth) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "DMA0 Read Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0mode(&self) -> super::vals::Dma0mode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dma0mode::from_bits(val as u8)
    }
    #[doc = "DMA0 Read Mode."]
    #[inline(always)]
    pub const fn set_dma0mode(&mut self, val: super::vals::Dma0mode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA0 Read Register Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0rsel(&self) -> super::vals::Dma0rsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dma0rsel::from_bits(val as u8)
    }
    #[doc = "DMA0 Read Register Select."]
    #[inline(always)]
    pub const fn set_dma0rsel(&mut self, val: super::vals::Dma0rsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA1 Read Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1mode(&self) -> super::vals::Dma1mode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Dma1mode::from_bits(val as u8)
    }
    #[doc = "DMA1 Read Mode."]
    #[inline(always)]
    pub const fn set_dma1mode(&mut self, val: super::vals::Dma1mode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DATA0 DMA Unaligned Read Register Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1rsel(&self) -> super::vals::Dma1rsel {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dma1rsel::from_bits(val as u8)
    }
    #[doc = "DATA0 DMA Unaligned Read Register Select."]
    #[inline(always)]
    pub const fn set_dma1rsel(&mut self, val: super::vals::Dma1rsel) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Combined Data0 Write DMA Request."]
    #[must_use]
    #[inline(always)]
    pub const fn combdma0wereq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Combined Data0 Write DMA Request."]
    #[inline(always)]
    pub const fn set_combdma0wereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("aes", &self.aes())
            .field("keybufdis", &self.keybufdis())
            .field("sha", &self.sha())
            .field("nobusystall", &self.nobusystall())
            .field("incwidth", &self.incwidth())
            .field("dma0mode", &self.dma0mode())
            .field("dma0rsel", &self.dma0rsel())
            .field("dma1mode", &self.dma1mode())
            .field("dma1rsel", &self.dma1rsel())
            .field("combdma0wereq", &self.combdma0wereq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ aes: {=bool:?}, keybufdis: {=bool:?}, sha: {=bool:?}, nobusystall: {=bool:?}, incwidth: {:?}, dma0mode: {:?}, dma0rsel: {:?}, dma1mode: {:?}, dma1rsel: {:?}, combdma0wereq: {=bool:?} }}" , self . aes () , self . keybufdis () , self . sha () , self . nobusystall () , self . incwidth () , self . dma0mode () , self . dma0rsel () , self . dma1mode () , self . dma1rsel () , self . combdma0wereq ())
    }
}
#[doc = "DATA0 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0byte(pub u32);
impl Data0byte {
    #[doc = "Data 0 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 Byte Access."]
    #[inline(always)]
    pub const fn set_data0byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0byte {
    #[inline(always)]
    fn default() -> Data0byte {
        Data0byte(0)
    }
}
impl core::fmt::Debug for Data0byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0byte")
            .field("data0byte", &self.data0byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data0byte {{ data0byte: {=u8:?} }}", self.data0byte())
    }
}
#[doc = "DATA0 Register Byte 12 Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0byte12(pub u32);
impl Data0byte12 {
    #[doc = "Data 0 Byte 12 Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0byte12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 Byte 12 Access."]
    #[inline(always)]
    pub const fn set_data0byte12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0byte12 {
    #[inline(always)]
    fn default() -> Data0byte12 {
        Data0byte12(0)
    }
}
impl core::fmt::Debug for Data0byte12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0byte12")
            .field("data0byte12", &self.data0byte12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0byte12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data0byte12 {{ data0byte12: {=u8:?} }}",
            self.data0byte12()
        )
    }
}
#[doc = "DATA0 Register Byte 13 Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0byte13(pub u32);
impl Data0byte13 {
    #[doc = "Data 0 Byte 13 Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0byte13(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 Byte 13 Access."]
    #[inline(always)]
    pub const fn set_data0byte13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0byte13 {
    #[inline(always)]
    fn default() -> Data0byte13 {
        Data0byte13(0)
    }
}
impl core::fmt::Debug for Data0byte13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0byte13")
            .field("data0byte13", &self.data0byte13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0byte13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data0byte13 {{ data0byte13: {=u8:?} }}",
            self.data0byte13()
        )
    }
}
#[doc = "DATA0 Register Byte 14 Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0byte14(pub u32);
impl Data0byte14 {
    #[doc = "Data 0 Byte 14 Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0byte14(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 Byte 14 Access."]
    #[inline(always)]
    pub const fn set_data0byte14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0byte14 {
    #[inline(always)]
    fn default() -> Data0byte14 {
        Data0byte14(0)
    }
}
impl core::fmt::Debug for Data0byte14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0byte14")
            .field("data0byte14", &self.data0byte14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0byte14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data0byte14 {{ data0byte14: {=u8:?} }}",
            self.data0byte14()
        )
    }
}
#[doc = "DATA0 Register Byte 15 Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0byte15(pub u32);
impl Data0byte15 {
    #[doc = "Data 0 Byte 15 Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0byte15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 Byte 15 Access."]
    #[inline(always)]
    pub const fn set_data0byte15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0byte15 {
    #[inline(always)]
    fn default() -> Data0byte15 {
        Data0byte15(0)
    }
}
impl core::fmt::Debug for Data0byte15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0byte15")
            .field("data0byte15", &self.data0byte15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0byte15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data0byte15 {{ data0byte15: {=u8:?} }}",
            self.data0byte15()
        )
    }
}
#[doc = "DATA0 Register Byte XOR Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0xorbyte(pub u32);
impl Data0xorbyte {
    #[doc = "Data 0 XOR Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data0xorbyte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0 XOR Byte Access."]
    #[inline(always)]
    pub const fn set_data0xorbyte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data0xorbyte {
    #[inline(always)]
    fn default() -> Data0xorbyte {
        Data0xorbyte(0)
    }
}
impl core::fmt::Debug for Data0xorbyte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data0xorbyte")
            .field("data0xorbyte", &self.data0xorbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data0xorbyte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data0xorbyte {{ data0xorbyte: {=u8:?} }}",
            self.data0xorbyte()
        )
    }
}
#[doc = "DATA1 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data1byte(pub u32);
impl Data1byte {
    #[doc = "Data 1 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn data1byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 1 Byte Access."]
    #[inline(always)]
    pub const fn set_data1byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data1byte {
    #[inline(always)]
    fn default() -> Data1byte {
        Data1byte(0)
    }
}
impl core::fmt::Debug for Data1byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data1byte")
            .field("data1byte", &self.data1byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data1byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data1byte {{ data1byte: {=u8:?} }}", self.data1byte())
    }
}
#[doc = "DDATA0 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddata0byte(pub u32);
impl Ddata0byte {
    #[doc = "Ddata 0 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata0byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ddata 0 Byte Access."]
    #[inline(always)]
    pub const fn set_ddata0byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ddata0byte {
    #[inline(always)]
    fn default() -> Ddata0byte {
        Ddata0byte(0)
    }
}
impl core::fmt::Debug for Ddata0byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ddata0byte")
            .field("ddata0byte", &self.ddata0byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ddata0byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ddata0byte {{ ddata0byte: {=u8:?} }}", self.ddata0byte())
    }
}
#[doc = "DDATA0 Register Byte 32 Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddata0byte32(pub u32);
impl Ddata0byte32 {
    #[doc = "Ddata 0 Byte 32 Access."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata0byte32(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Ddata 0 Byte 32 Access."]
    #[inline(always)]
    pub const fn set_ddata0byte32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ddata0byte32 {
    #[inline(always)]
    fn default() -> Ddata0byte32 {
        Ddata0byte32(0)
    }
}
impl core::fmt::Debug for Ddata0byte32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ddata0byte32")
            .field("ddata0byte32", &self.ddata0byte32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ddata0byte32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ddata0byte32 {{ ddata0byte32: {=u8:?} }}",
            self.ddata0byte32()
        )
    }
}
#[doc = "DDATA1 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddata1byte(pub u32);
impl Ddata1byte {
    #[doc = "Ddata 1 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata1byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ddata 1 Byte Access."]
    #[inline(always)]
    pub const fn set_ddata1byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ddata1byte {
    #[inline(always)]
    fn default() -> Ddata1byte {
        Ddata1byte(0)
    }
}
impl core::fmt::Debug for Ddata1byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ddata1byte")
            .field("ddata1byte", &self.ddata1byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ddata1byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ddata1byte {{ ddata1byte: {=u8:?} }}", self.ddata1byte())
    }
}
#[doc = "Data Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dstatus(pub u32);
impl Dstatus {
    #[doc = "Data 0 Zero."]
    #[must_use]
    #[inline(always)]
    pub const fn data0zero(&self) -> super::vals::Data0zero {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Data0zero::from_bits(val as u8)
    }
    #[doc = "Data 0 Zero."]
    #[inline(always)]
    pub const fn set_data0zero(&mut self, val: super::vals::Data0zero) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "LSBs in DDATA0."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata0lsbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "LSBs in DDATA0."]
    #[inline(always)]
    pub const fn set_ddata0lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "MSB in DDATA0."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata0msbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "MSB in DDATA0."]
    #[inline(always)]
    pub const fn set_ddata0msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "MSB in DDATA1."]
    #[must_use]
    #[inline(always)]
    pub const fn ddata1msb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "MSB in DDATA1."]
    #[inline(always)]
    pub const fn set_ddata1msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Carry From Arithmetic Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn carry(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Carry From Arithmetic Operation."]
    #[inline(always)]
    pub const fn set_carry(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Dstatus {
    #[inline(always)]
    fn default() -> Dstatus {
        Dstatus(0)
    }
}
impl core::fmt::Debug for Dstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dstatus")
            .field("data0zero", &self.data0zero())
            .field("ddata0lsbs", &self.ddata0lsbs())
            .field("ddata0msbs", &self.ddata0msbs())
            .field("ddata1msb", &self.ddata1msb())
            .field("carry", &self.carry())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dstatus {{ data0zero: {:?}, ddata0lsbs: {=u8:?}, ddata0msbs: {=u8:?}, ddata1msb: {=bool:?}, carry: {=bool:?} }}" , self . data0zero () , self . ddata0lsbs () , self . ddata0msbs () , self . ddata1msb () , self . carry ())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "INSTRDONE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn instrdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INSTRDONE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_instrdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SEQDONE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn seqdone(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SEQDONE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_seqdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("instrdone", &self.instrdone())
            .field("seqdone", &self.seqdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ien {{ instrdone: {=bool:?}, seqdone: {=bool:?} }}",
            self.instrdone(),
            self.seqdone()
        )
    }
}
#[doc = "AES Interrupt Flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If(pub u32);
impl If {
    #[doc = "Instruction Done."]
    #[must_use]
    #[inline(always)]
    pub const fn instrdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction Done."]
    #[inline(always)]
    pub const fn set_instrdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sequence Done."]
    #[must_use]
    #[inline(always)]
    pub const fn seqdone(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Done."]
    #[inline(always)]
    pub const fn set_seqdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("instrdone", &self.instrdone())
            .field("seqdone", &self.seqdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for If {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "If {{ instrdone: {=bool:?}, seqdone: {=bool:?} }}",
            self.instrdone(),
            self.seqdone()
        )
    }
}
#[doc = "Interrupt Flag Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc = "Set INSTRDONE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn instrdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set INSTRDONE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_instrdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set SEQDONE Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn seqdone(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set SEQDONE Interrupt Flag."]
    #[inline(always)]
    pub const fn set_seqdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("instrdone", &self.instrdone())
            .field("seqdone", &self.seqdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifs {{ instrdone: {=bool:?}, seqdone: {=bool:?} }}",
            self.instrdone(),
            self.seqdone()
        )
    }
}
#[doc = "QDATA0 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdata0byte(pub u32);
impl Qdata0byte {
    #[doc = "Qdata 0 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn qdata0byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Qdata 0 Byte Access."]
    #[inline(always)]
    pub const fn set_qdata0byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Qdata0byte {
    #[inline(always)]
    fn default() -> Qdata0byte {
        Qdata0byte(0)
    }
}
impl core::fmt::Debug for Qdata0byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdata0byte")
            .field("qdata0byte", &self.qdata0byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdata0byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdata0byte {{ qdata0byte: {=u8:?} }}", self.qdata0byte())
    }
}
#[doc = "QDATA1 Register Byte Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdata1byte(pub u32);
impl Qdata1byte {
    #[doc = "Qdata 1 Byte Access."]
    #[must_use]
    #[inline(always)]
    pub const fn qdata1byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Qdata 1 Byte Access."]
    #[inline(always)]
    pub const fn set_qdata1byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Qdata1byte {
    #[inline(always)]
    fn default() -> Qdata1byte {
        Qdata1byte(0)
    }
}
impl core::fmt::Debug for Qdata1byte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdata1byte")
            .field("qdata1byte", &self.qdata1byte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdata1byte {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdata1byte {{ qdata1byte: {=u8:?} }}", self.qdata1byte())
    }
}
#[doc = "Sequence Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq0(pub u32);
impl Seq0 {
    #[doc = "Sequence Instruction 0."]
    #[must_use]
    #[inline(always)]
    pub const fn instr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 0."]
    #[inline(always)]
    pub const fn set_instr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sequence Instruction 1."]
    #[must_use]
    #[inline(always)]
    pub const fn instr1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 1."]
    #[inline(always)]
    pub const fn set_instr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sequence Instruction 2."]
    #[must_use]
    #[inline(always)]
    pub const fn instr2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 2."]
    #[inline(always)]
    pub const fn set_instr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sequence Instruction 3."]
    #[must_use]
    #[inline(always)]
    pub const fn instr3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 3."]
    #[inline(always)]
    pub const fn set_instr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Seq0 {
    #[inline(always)]
    fn default() -> Seq0 {
        Seq0(0)
    }
}
impl core::fmt::Debug for Seq0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seq0")
            .field("instr0", &self.instr0())
            .field("instr1", &self.instr1())
            .field("instr2", &self.instr2())
            .field("instr3", &self.instr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seq0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seq0 {{ instr0: {=u8:?}, instr1: {=u8:?}, instr2: {=u8:?}, instr3: {=u8:?} }}",
            self.instr0(),
            self.instr1(),
            self.instr2(),
            self.instr3()
        )
    }
}
#[doc = "Sequence Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq1(pub u32);
impl Seq1 {
    #[doc = "Sequence Instruction 4."]
    #[must_use]
    #[inline(always)]
    pub const fn instr4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 4."]
    #[inline(always)]
    pub const fn set_instr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sequence Instruction 5."]
    #[must_use]
    #[inline(always)]
    pub const fn instr5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 5."]
    #[inline(always)]
    pub const fn set_instr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sequence Instruction 6."]
    #[must_use]
    #[inline(always)]
    pub const fn instr6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 6."]
    #[inline(always)]
    pub const fn set_instr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sequence Instruction 7."]
    #[must_use]
    #[inline(always)]
    pub const fn instr7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 7."]
    #[inline(always)]
    pub const fn set_instr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Seq1 {
    #[inline(always)]
    fn default() -> Seq1 {
        Seq1(0)
    }
}
impl core::fmt::Debug for Seq1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seq1")
            .field("instr4", &self.instr4())
            .field("instr5", &self.instr5())
            .field("instr6", &self.instr6())
            .field("instr7", &self.instr7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seq1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seq1 {{ instr4: {=u8:?}, instr5: {=u8:?}, instr6: {=u8:?}, instr7: {=u8:?} }}",
            self.instr4(),
            self.instr5(),
            self.instr6(),
            self.instr7()
        )
    }
}
#[doc = "Sequence Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq2(pub u32);
impl Seq2 {
    #[doc = "Sequence Instruction 8."]
    #[must_use]
    #[inline(always)]
    pub const fn instr8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 8."]
    #[inline(always)]
    pub const fn set_instr8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sequence Instruction 9."]
    #[must_use]
    #[inline(always)]
    pub const fn instr9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 9."]
    #[inline(always)]
    pub const fn set_instr9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sequence Instruction 10."]
    #[must_use]
    #[inline(always)]
    pub const fn instr10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 10."]
    #[inline(always)]
    pub const fn set_instr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sequence Instruction 11."]
    #[must_use]
    #[inline(always)]
    pub const fn instr11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 11."]
    #[inline(always)]
    pub const fn set_instr11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Seq2 {
    #[inline(always)]
    fn default() -> Seq2 {
        Seq2(0)
    }
}
impl core::fmt::Debug for Seq2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seq2")
            .field("instr8", &self.instr8())
            .field("instr9", &self.instr9())
            .field("instr10", &self.instr10())
            .field("instr11", &self.instr11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seq2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seq2 {{ instr8: {=u8:?}, instr9: {=u8:?}, instr10: {=u8:?}, instr11: {=u8:?} }}",
            self.instr8(),
            self.instr9(),
            self.instr10(),
            self.instr11()
        )
    }
}
#[doc = "Sequence Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq3(pub u32);
impl Seq3 {
    #[doc = "Sequence Instruction 12."]
    #[must_use]
    #[inline(always)]
    pub const fn instr12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 12."]
    #[inline(always)]
    pub const fn set_instr12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sequence Instruction 13."]
    #[must_use]
    #[inline(always)]
    pub const fn instr13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 13."]
    #[inline(always)]
    pub const fn set_instr13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sequence Instruction 14."]
    #[must_use]
    #[inline(always)]
    pub const fn instr14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 14."]
    #[inline(always)]
    pub const fn set_instr14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sequence Instruction 15."]
    #[must_use]
    #[inline(always)]
    pub const fn instr15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 15."]
    #[inline(always)]
    pub const fn set_instr15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Seq3 {
    #[inline(always)]
    fn default() -> Seq3 {
        Seq3(0)
    }
}
impl core::fmt::Debug for Seq3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seq3")
            .field("instr12", &self.instr12())
            .field("instr13", &self.instr13())
            .field("instr14", &self.instr14())
            .field("instr15", &self.instr15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seq3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seq3 {{ instr12: {=u8:?}, instr13: {=u8:?}, instr14: {=u8:?}, instr15: {=u8:?} }}",
            self.instr12(),
            self.instr13(),
            self.instr14(),
            self.instr15()
        )
    }
}
#[doc = "Sequence Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq4(pub u32);
impl Seq4 {
    #[doc = "Sequence Instruction 16."]
    #[must_use]
    #[inline(always)]
    pub const fn instr16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 16."]
    #[inline(always)]
    pub const fn set_instr16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sequence Instruction 17."]
    #[must_use]
    #[inline(always)]
    pub const fn instr17(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 17."]
    #[inline(always)]
    pub const fn set_instr17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sequence Instruction 18."]
    #[must_use]
    #[inline(always)]
    pub const fn instr18(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 18."]
    #[inline(always)]
    pub const fn set_instr18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sequence Instruction 19."]
    #[must_use]
    #[inline(always)]
    pub const fn instr19(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sequence Instruction 19."]
    #[inline(always)]
    pub const fn set_instr19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Seq4 {
    #[inline(always)]
    fn default() -> Seq4 {
        Seq4(0)
    }
}
impl core::fmt::Debug for Seq4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seq4")
            .field("instr16", &self.instr16())
            .field("instr17", &self.instr17())
            .field("instr18", &self.instr18())
            .field("instr19", &self.instr19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seq4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seq4 {{ instr16: {=u8:?}, instr17: {=u8:?}, instr18: {=u8:?}, instr19: {=u8:?} }}",
            self.instr16(),
            self.instr17(),
            self.instr18(),
            self.instr19()
        )
    }
}
#[doc = "Sequence Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seqctrl(pub u32);
impl Seqctrl {
    #[doc = "Buffer Length a in Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn lengtha(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Buffer Length a in Bytes."]
    #[inline(always)]
    pub const fn set_lengtha(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Size of Data Blocks."]
    #[must_use]
    #[inline(always)]
    pub const fn blocksize(&self) -> super::vals::Blocksize {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Blocksize::from_bits(val as u8)
    }
    #[doc = "Size of Data Blocks."]
    #[inline(always)]
    pub const fn set_blocksize(&mut self, val: super::vals::Blocksize) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA0 Skip."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0skip(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "DMA0 Skip."]
    #[inline(always)]
    pub const fn set_dma0skip(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA1 Skip."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1skip(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "DMA1 Skip."]
    #[inline(always)]
    pub const fn set_dma1skip(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "DMA0 Preserve a."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0presa(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 Preserve a."]
    #[inline(always)]
    pub const fn set_dma0presa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 Preserve a."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1presa(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 Preserve a."]
    #[inline(always)]
    pub const fn set_dma1presa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt Sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Halt Sequence."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Seqctrl {
    #[inline(always)]
    fn default() -> Seqctrl {
        Seqctrl(0)
    }
}
impl core::fmt::Debug for Seqctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seqctrl")
            .field("lengtha", &self.lengtha())
            .field("blocksize", &self.blocksize())
            .field("dma0skip", &self.dma0skip())
            .field("dma1skip", &self.dma1skip())
            .field("dma0presa", &self.dma0presa())
            .field("dma1presa", &self.dma1presa())
            .field("halt", &self.halt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seqctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Seqctrl {{ lengtha: {=u16:?}, blocksize: {:?}, dma0skip: {=u8:?}, dma1skip: {=u8:?}, dma0presa: {=bool:?}, dma1presa: {=bool:?}, halt: {=bool:?} }}" , self . lengtha () , self . blocksize () , self . dma0skip () , self . dma1skip () , self . dma0presa () , self . dma1presa () , self . halt ())
    }
}
#[doc = "Sequence Control B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seqctrlb(pub u32);
impl Seqctrlb {
    #[doc = "Buffer Length B in Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn lengthb(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Buffer Length B in Bytes."]
    #[inline(always)]
    pub const fn set_lengthb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "DMA0 Preserve B."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0presb(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 Preserve B."]
    #[inline(always)]
    pub const fn set_dma0presb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 Preserve B."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1presb(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 Preserve B."]
    #[inline(always)]
    pub const fn set_dma1presb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Seqctrlb {
    #[inline(always)]
    fn default() -> Seqctrlb {
        Seqctrlb(0)
    }
}
impl core::fmt::Debug for Seqctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seqctrlb")
            .field("lengthb", &self.lengthb())
            .field("dma0presb", &self.dma0presb())
            .field("dma1presb", &self.dma1presb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seqctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seqctrlb {{ lengthb: {=u16:?}, dma0presb: {=bool:?}, dma1presb: {=bool:?} }}",
            self.lengthb(),
            self.dma0presb(),
            self.dma1presb()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "AES SEQUENCE Running."]
    #[must_use]
    #[inline(always)]
    pub const fn seqrunning(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES SEQUENCE Running."]
    #[inline(always)]
    pub const fn set_seqrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Action is Active."]
    #[must_use]
    #[inline(always)]
    pub const fn instrrunning(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Action is Active."]
    #[inline(always)]
    pub const fn set_instrrunning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Action is Active."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaactive(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Action is Active."]
    #[inline(always)]
    pub const fn set_dmaactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("seqrunning", &self.seqrunning())
            .field("instrrunning", &self.instrrunning())
            .field("dmaactive", &self.dmaactive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ seqrunning: {=bool:?}, instrrunning: {=bool:?}, dmaactive: {=bool:?} }}",
            self.seqrunning(),
            self.instrrunning(),
            self.dmaactive()
        )
    }
}
#[doc = "Wide Arithmetic Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wac(pub u32);
impl Wac {
    #[doc = "Modular Operation Modulus."]
    #[must_use]
    #[inline(always)]
    pub const fn modulus(&self) -> super::vals::Modulus {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Modulus::from_bits(val as u8)
    }
    #[doc = "Modular Operation Modulus."]
    #[inline(always)]
    pub const fn set_modulus(&mut self, val: super::vals::Modulus) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Modular Operation Field Type."]
    #[must_use]
    #[inline(always)]
    pub const fn modop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Modular Operation Field Type."]
    #[inline(always)]
    pub const fn set_modop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multiply Width."]
    #[must_use]
    #[inline(always)]
    pub const fn mulwidth(&self) -> super::vals::Mulwidth {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Mulwidth::from_bits(val as u8)
    }
    #[doc = "Multiply Width."]
    #[inline(always)]
    pub const fn set_mulwidth(&mut self, val: super::vals::Mulwidth) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Result Width."]
    #[must_use]
    #[inline(always)]
    pub const fn resultwidth(&self) -> super::vals::Resultwidth {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Resultwidth::from_bits(val as u8)
    }
    #[doc = "Result Width."]
    #[inline(always)]
    pub const fn set_resultwidth(&mut self, val: super::vals::Resultwidth) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Wac {
    #[inline(always)]
    fn default() -> Wac {
        Wac(0)
    }
}
impl core::fmt::Debug for Wac {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wac")
            .field("modulus", &self.modulus())
            .field("modop", &self.modop())
            .field("mulwidth", &self.mulwidth())
            .field("resultwidth", &self.resultwidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wac {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wac {{ modulus: {:?}, modop: {=bool:?}, mulwidth: {:?}, resultwidth: {:?} }}",
            self.modulus(),
            self.modop(),
            self.mulwidth(),
            self.resultwidth()
        )
    }
}
