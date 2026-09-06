#[doc = "CRYPTO0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crypto {
    ptr: *mut u8,
}
unsafe impl Send for Crypto {}
unsafe impl Sync for Crypto {}
impl Crypto {
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
    #[doc = "Wide Arithmetic Configuration."]
    #[inline(always)]
    pub const fn wac(self) -> crate::common::Reg<regs::Wac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data Status Register."]
    #[inline(always)]
    pub const fn dstatus(self) -> crate::common::Reg<regs::Dstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control Status Register."]
    #[inline(always)]
    pub const fn cstatus(self) -> crate::common::Reg<regs::Cstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "KEY Register Access."]
    #[inline(always)]
    pub const fn key(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "KEY Buffer Register Access."]
    #[inline(always)]
    pub const fn keybuf(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Sequence Control."]
    #[inline(always)]
    pub const fn seqctrl(self) -> crate::common::Reg<regs::Seqctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Sequence Control B."]
    #[inline(always)]
    pub const fn seqctrlb(self) -> crate::common::Reg<regs::Seqctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "AES Interrupt Flags."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn ifs(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn ifc(self) -> crate::common::Reg<regs::Ifs, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Sequence Register 0."]
    #[inline(always)]
    pub const fn seq0(self) -> crate::common::Reg<regs::Seq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Sequence Register 1."]
    #[inline(always)]
    pub const fn seq1(self) -> crate::common::Reg<regs::Seq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Sequence Register 2."]
    #[inline(always)]
    pub const fn seq2(self) -> crate::common::Reg<regs::Seq2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Sequence Register 3."]
    #[inline(always)]
    pub const fn seq3(self) -> crate::common::Reg<regs::Seq3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Sequence Register 4."]
    #[inline(always)]
    pub const fn seq4(self) -> crate::common::Reg<regs::Seq4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DATA0 Register Access."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "DATA1 Register Access."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "DATA2 Register Access."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "DATA3 Register Access."]
    #[inline(always)]
    pub const fn data3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "DATA0XOR Register Access."]
    #[inline(always)]
    pub const fn data0xor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "DATA0 Register Byte Access."]
    #[inline(always)]
    pub const fn data0byte(self) -> crate::common::Reg<regs::Data0byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "DATA1 Register Byte Access."]
    #[inline(always)]
    pub const fn data1byte(self) -> crate::common::Reg<regs::Data1byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "DATA0 Register Byte XOR Access."]
    #[inline(always)]
    pub const fn data0xorbyte(self) -> crate::common::Reg<regs::Data0xorbyte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "DATA0 Register Byte 12 Access."]
    #[inline(always)]
    pub const fn data0byte12(self) -> crate::common::Reg<regs::Data0byte12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "DATA0 Register Byte 13 Access."]
    #[inline(always)]
    pub const fn data0byte13(self) -> crate::common::Reg<regs::Data0byte13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "DATA0 Register Byte 14 Access."]
    #[inline(always)]
    pub const fn data0byte14(self) -> crate::common::Reg<regs::Data0byte14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "DATA0 Register Byte 15 Access."]
    #[inline(always)]
    pub const fn data0byte15(self) -> crate::common::Reg<regs::Data0byte15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "DDATA0 Register Access."]
    #[inline(always)]
    pub const fn ddata0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "DDATA1 Register Access."]
    #[inline(always)]
    pub const fn ddata1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "DDATA2 Register Access."]
    #[inline(always)]
    pub const fn ddata2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "DDATA3 Register Access."]
    #[inline(always)]
    pub const fn ddata3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "DDATA4 Register Access."]
    #[inline(always)]
    pub const fn ddata4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "DDATA0 Register Big Endian Access."]
    #[inline(always)]
    pub const fn ddata0big(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "DDATA0 Register Byte Access."]
    #[inline(always)]
    pub const fn ddata0byte(self) -> crate::common::Reg<regs::Ddata0byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "DDATA1 Register Byte Access."]
    #[inline(always)]
    pub const fn ddata1byte(self) -> crate::common::Reg<regs::Ddata1byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "DDATA0 Register Byte 32 Access."]
    #[inline(always)]
    pub const fn ddata0byte32(self) -> crate::common::Reg<regs::Ddata0byte32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "QDATA0 Register Access."]
    #[inline(always)]
    pub const fn qdata0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "QDATA1 Register Access."]
    #[inline(always)]
    pub const fn qdata1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "QDATA1 Register Big Endian Access."]
    #[inline(always)]
    pub const fn qdata1big(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "QDATA0 Register Byte Access."]
    #[inline(always)]
    pub const fn qdata0byte(self) -> crate::common::Reg<regs::Qdata0byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "QDATA1 Register Byte Access."]
    #[inline(always)]
    pub const fn qdata1byte(self) -> crate::common::Reg<regs::Qdata1byte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
