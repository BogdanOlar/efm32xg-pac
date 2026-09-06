#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blocksize {
    #[doc = "A block is 16 bytes long."]
    _16bytes = 0x0,
    #[doc = "A block is 32 bytes long."]
    _32bytes = 0x01,
    #[doc = "A block is 64 bytes long."]
    _64bytes = 0x02,
    _RESERVED_3 = 0x03,
}
impl Blocksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blocksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blocksize {
    #[inline(always)]
    fn from(val: u8) -> Blocksize {
        Blocksize::from_bits(val)
    }
}
impl From<Blocksize> for u8 {
    #[inline(always)]
    fn from(val: Blocksize) -> u8 {
        Blocksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Data0zero {
    _RESERVED_0 = 0x0,
    #[doc = "In DATA0 bits 0 to 31 are all zero."]
    Zero0to31 = 0x01,
    #[doc = "In DATA0 bits 32 to 63 are all zero."]
    Zero32to63 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "In DATA0 bits 64 to 95 are all zero."]
    Zero64to95 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "In DATA0 bits 96 to 127 are all zero."]
    Zero96to127 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Data0zero {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Data0zero {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Data0zero {
    #[inline(always)]
    fn from(val: u8) -> Data0zero {
        Data0zero::from_bits(val)
    }
}
impl From<Data0zero> for u8 {
    #[inline(always)]
    fn from(val: Data0zero) -> u8 {
        Data0zero::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0mode {
    #[doc = "Target register is fully read/written during every DMA transaction."]
    Full = 0x0,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 0x01,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 0x02,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 0x03,
}
impl Dma0mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0mode {
    #[inline(always)]
    fn from(val: u8) -> Dma0mode {
        Dma0mode::from_bits(val)
    }
}
impl From<Dma0mode> for u8 {
    #[inline(always)]
    fn from(val: Dma0mode) -> u8 {
        Dma0mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0rsel {
    Data0 = 0x0,
    Ddata0 = 0x01,
    Ddata0big = 0x02,
    Qdata0 = 0x03,
}
impl Dma0rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0rsel {
    #[inline(always)]
    fn from(val: u8) -> Dma0rsel {
        Dma0rsel::from_bits(val)
    }
}
impl From<Dma0rsel> for u8 {
    #[inline(always)]
    fn from(val: Dma0rsel) -> u8 {
        Dma0rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1mode {
    #[doc = "Target register is fully read/written during every DMA transaction."]
    Full = 0x0,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 0x01,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 0x02,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 0x03,
}
impl Dma1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1mode {
    #[inline(always)]
    fn from(val: u8) -> Dma1mode {
        Dma1mode::from_bits(val)
    }
}
impl From<Dma1mode> for u8 {
    #[inline(always)]
    fn from(val: Dma1mode) -> u8 {
        Dma1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1rsel {
    Data1 = 0x0,
    Ddata1 = 0x01,
    Qdata1 = 0x02,
    Qdata1big = 0x03,
}
impl Dma1rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1rsel {
    #[inline(always)]
    fn from(val: u8) -> Dma1rsel {
        Dma1rsel::from_bits(val)
    }
}
impl From<Dma1rsel> for u8 {
    #[inline(always)]
    fn from(val: Dma1rsel) -> u8 {
        Dma1rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Incwidth {
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    Incwidth1 = 0x0,
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    Incwidth2 = 0x01,
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    Incwidth3 = 0x02,
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    Incwidth4 = 0x03,
}
impl Incwidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Incwidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Incwidth {
    #[inline(always)]
    fn from(val: u8) -> Incwidth {
        Incwidth::from_bits(val)
    }
}
impl From<Incwidth> for u8 {
    #[inline(always)]
    fn from(val: Incwidth) -> u8 {
        Incwidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modulus {
    #[doc = "Generic modulus. p = 2^256."]
    Bin256 = 0x0,
    #[doc = "Generic modulus. p = 2^128."]
    Bin128 = 0x01,
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1."]
    Eccbin233p = 0x02,
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1."]
    Eccbin163p = 0x03,
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1."]
    Gcmbin128 = 0x04,
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1."]
    Eccprime256p = 0x05,
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1."]
    Eccprime224p = 0x06,
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1."]
    Eccprime192p = 0x07,
    #[doc = "P modulus for B-233 ECC curve."]
    Eccbin233n = 0x08,
    #[doc = "P modulus for K-233 ECC curve."]
    Eccbin233kn = 0x09,
    #[doc = "P modulus for B-163 ECC curve."]
    Eccbin163n = 0x0a,
    #[doc = "P modulus for K-163 ECC curve."]
    Eccbin163kn = 0x0b,
    #[doc = "P modulus for P-256 ECC curve."]
    Eccprime256n = 0x0c,
    #[doc = "P modulus for P-224 ECC curve."]
    Eccprime224n = 0x0d,
    #[doc = "P modulus for P-192 ECC curve."]
    Eccprime192n = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Modulus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modulus {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modulus {
    #[inline(always)]
    fn from(val: u8) -> Modulus {
        Modulus::from_bits(val)
    }
}
impl From<Modulus> for u8 {
    #[inline(always)]
    fn from(val: Modulus) -> u8 {
        Modulus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mulwidth {
    #[doc = "Multiply 256 bits."]
    Mul256 = 0x0,
    #[doc = "Multiply 128 bits."]
    Mul128 = 0x01,
    #[doc = "Same number of bits as specified by MODULUS."]
    Mulmod = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mulwidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mulwidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mulwidth {
    #[inline(always)]
    fn from(val: u8) -> Mulwidth {
        Mulwidth::from_bits(val)
    }
}
impl From<Mulwidth> for u8 {
    #[inline(always)]
    fn from(val: Mulwidth) -> u8 {
        Mulwidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resultwidth {
    #[doc = "Results have 256 bits."]
    _256bit = 0x0,
    #[doc = "Results have 128 bits."]
    _128bit = 0x01,
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS."]
    _260bit = 0x02,
    _RESERVED_3 = 0x03,
}
impl Resultwidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resultwidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resultwidth {
    #[inline(always)]
    fn from(val: u8) -> Resultwidth {
        Resultwidth::from_bits(val)
    }
}
impl From<Resultwidth> for u8 {
    #[inline(always)]
    fn from(val: Resultwidth) -> u8 {
        Resultwidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V0 {
    Ddata0 = 0x0,
    Ddata1 = 0x01,
    Ddata2 = 0x02,
    Ddata3 = 0x03,
    Ddata4 = 0x04,
    Data0 = 0x05,
    Data1 = 0x06,
    Data2 = 0x07,
}
impl V0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V0 {
    #[inline(always)]
    fn from(val: u8) -> V0 {
        V0::from_bits(val)
    }
}
impl From<V0> for u8 {
    #[inline(always)]
    fn from(val: V0) -> u8 {
        V0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V1 {
    Ddata0 = 0x0,
    Ddata1 = 0x01,
    Ddata2 = 0x02,
    Ddata3 = 0x03,
    Ddata4 = 0x04,
    Data0 = 0x05,
    Data1 = 0x06,
    Data2 = 0x07,
}
impl V1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V1 {
    #[inline(always)]
    fn from(val: u8) -> V1 {
        V1::from_bits(val)
    }
}
impl From<V1> for u8 {
    #[inline(always)]
    fn from(val: V1) -> u8 {
        V1::to_bits(val)
    }
}
