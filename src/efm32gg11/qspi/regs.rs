#[doc = "Octal-SPI Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "QSPI Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbspi(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "QSPI Enable."]
    #[inline(always)]
    pub const fn set_enbspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Polarity, CPOL."]
    #[must_use]
    #[inline(always)]
    pub const fn selclkpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Polarity, CPOL."]
    #[inline(always)]
    pub const fn set_selclkpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock Phase, CPHA."]
    #[must_use]
    #[inline(always)]
    pub const fn selclkphase(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Phase, CPHA."]
    #[inline(always)]
    pub const fn set_selclkphase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PHY Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn phymodeenable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Mode Enable."]
    #[inline(always)]
    pub const fn set_phymodeenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Direct Access Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn enbdiraccctlr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Direct Access Controller."]
    #[inline(always)]
    pub const fn set_enbdiraccctlr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Legacy IP Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enblegacyipmode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Legacy IP Mode Enable."]
    #[inline(always)]
    pub const fn set_enblegacyipmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Peripheral Select Decode."]
    #[must_use]
    #[inline(always)]
    pub const fn periphseldec(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral Select Decode."]
    #[inline(always)]
    pub const fn set_periphseldec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Peripheral Chip Select Lines."]
    #[must_use]
    #[inline(always)]
    pub const fn periphcslines(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral Chip Select Lines."]
    #[inline(always)]
    pub const fn set_periphcslines(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Write Protect Flash Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn wrprotflash(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protect Flash Pin."]
    #[inline(always)]
    pub const fn set_wrprotflash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Address Remapping."]
    #[must_use]
    #[inline(always)]
    pub const fn enbahbaddrremap(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Address Remapping."]
    #[inline(always)]
    pub const fn set_enbahbaddrremap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enter XIP Mode on Next READ."]
    #[must_use]
    #[inline(always)]
    pub const fn enterxipmode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enter XIP Mode on Next READ."]
    #[inline(always)]
    pub const fn set_enterxipmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enter XIP Mode Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn enterxipmodeimm(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enter XIP Mode Immediately."]
    #[inline(always)]
    pub const fn set_enterxipmodeimm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Master Mode Baud Rate Divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrbauddiv(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Mode Baud Rate Divisor."]
    #[inline(always)]
    pub const fn set_mstrbauddiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
    }
    #[doc = "Enable Address Decoder."]
    #[must_use]
    #[inline(always)]
    pub const fn enableahbdecoder(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Address Decoder."]
    #[inline(always)]
    pub const fn set_enableahbdecoder(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable DTR Protocol."]
    #[must_use]
    #[inline(always)]
    pub const fn enabledtrprotocol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DTR Protocol."]
    #[inline(always)]
    pub const fn set_enabledtrprotocol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pipeline PHY Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pipelinephy(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pipeline PHY Mode Enable."]
    #[inline(always)]
    pub const fn set_pipelinephy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CRC Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn crcenable(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Enable Bit."]
    #[inline(always)]
    pub const fn set_crcenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Dual-byte Opcode Mode Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dualbyteopcodeen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Dual-byte Opcode Mode Enable Bit."]
    #[inline(always)]
    pub const fn set_dualbyteopcodeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Serial Interface and Low Level SPI Pipeline is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Interface and Low Level SPI Pipeline is IDLE."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("enbspi", &self.enbspi())
            .field("selclkpol", &self.selclkpol())
            .field("selclkphase", &self.selclkphase())
            .field("phymodeenable", &self.phymodeenable())
            .field("enbdiraccctlr", &self.enbdiraccctlr())
            .field("enblegacyipmode", &self.enblegacyipmode())
            .field("periphseldec", &self.periphseldec())
            .field("periphcslines", &self.periphcslines())
            .field("wrprotflash", &self.wrprotflash())
            .field("enbahbaddrremap", &self.enbahbaddrremap())
            .field("enterxipmode", &self.enterxipmode())
            .field("enterxipmodeimm", &self.enterxipmodeimm())
            .field("mstrbauddiv", &self.mstrbauddiv())
            .field("enableahbdecoder", &self.enableahbdecoder())
            .field("enabledtrprotocol", &self.enabledtrprotocol())
            .field("pipelinephy", &self.pipelinephy())
            .field("crcenable", &self.crcenable())
            .field("dualbyteopcodeen", &self.dualbyteopcodeen())
            .field("idle", &self.idle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Config {{ enbspi: {=bool:?}, selclkpol: {=bool:?}, selclkphase: {=bool:?}, phymodeenable: {=bool:?}, enbdiraccctlr: {=bool:?}, enblegacyipmode: {=bool:?}, periphseldec: {=bool:?}, periphcslines: {=u8:?}, wrprotflash: {=bool:?}, enbahbaddrremap: {=bool:?}, enterxipmode: {=bool:?}, enterxipmodeimm: {=bool:?}, mstrbauddiv: {=u8:?}, enableahbdecoder: {=bool:?}, enabledtrprotocol: {=bool:?}, pipelinephy: {=bool:?}, crcenable: {=bool:?}, dualbyteopcodeen: {=bool:?}, idle: {=bool:?} }}" , self . enbspi () , self . selclkpol () , self . selclkphase () , self . phymodeenable () , self . enbdiraccctlr () , self . enblegacyipmode () , self . periphseldec () , self . periphcslines () , self . wrprotflash () , self . enbahbaddrremap () , self . enterxipmode () , self . enterxipmodeimm () , self . mstrbauddiv () , self . enableahbdecoder () , self . enabledtrprotocol () , self . pipelinephy () , self . crcenable () , self . dualbyteopcodeen () , self . idle ())
    }
}
#[doc = "Device Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devdelay(pub u32);
impl Devdelay {
    #[doc = "Clock Delay for CS."]
    #[must_use]
    #[inline(always)]
    pub const fn dinit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Delay for CS."]
    #[inline(always)]
    pub const fn set_dinit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Delay for Last Transaction Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dafter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Delay for Last Transaction Bit."]
    #[inline(always)]
    pub const fn set_dafter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Clock Delay Between Two Chip Selects."]
    #[must_use]
    #[inline(always)]
    pub const fn dbtwn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Delay Between Two Chip Selects."]
    #[inline(always)]
    pub const fn set_dbtwn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Clock Delay for Chip Select Deassert."]
    #[must_use]
    #[inline(always)]
    pub const fn dnss(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Delay for Chip Select Deassert."]
    #[inline(always)]
    pub const fn set_dnss(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Devdelay {
    #[inline(always)]
    fn default() -> Devdelay {
        Devdelay(0)
    }
}
impl core::fmt::Debug for Devdelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devdelay")
            .field("dinit", &self.dinit())
            .field("dafter", &self.dafter())
            .field("dbtwn", &self.dbtwn())
            .field("dnss", &self.dnss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devdelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Devdelay {{ dinit: {=u8:?}, dafter: {=u8:?}, dbtwn: {=u8:?}, dnss: {=u8:?} }}",
            self.dinit(),
            self.dafter(),
            self.dbtwn(),
            self.dnss()
        )
    }
}
#[doc = "Device Read Instruction Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devinstrrdconfig(pub u32);
impl Devinstrrdconfig {
    #[doc = "Read Opcode in Non-XIP Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rdopcodenonxip(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Opcode in Non-XIP Mode."]
    #[inline(always)]
    pub const fn set_rdopcodenonxip(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Instruction Type."]
    #[must_use]
    #[inline(always)]
    pub const fn instrtype(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Instruction Type."]
    #[inline(always)]
    pub const fn set_instrtype(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "DDR Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ddren(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DDR Enable."]
    #[inline(always)]
    pub const fn set_ddren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Address Transfer Type for Standard SPI Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn addrxfertypestdmode(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Address Transfer Type for Standard SPI Modes."]
    #[inline(always)]
    pub const fn set_addrxfertypestdmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Data Transfer Type for Standard SPI Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn dataxfertypeextmode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Data Transfer Type for Standard SPI Modes."]
    #[inline(always)]
    pub const fn set_dataxfertypeextmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Mode Bit Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn modebitenable(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Bit Enable."]
    #[inline(always)]
    pub const fn set_modebitenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Dummy Read Clock Cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn dummyrdclkcycles(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Dummy Read Clock Cycles."]
    #[inline(always)]
    pub const fn set_dummyrdclkcycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Devinstrrdconfig {
    #[inline(always)]
    fn default() -> Devinstrrdconfig {
        Devinstrrdconfig(0)
    }
}
impl core::fmt::Debug for Devinstrrdconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devinstrrdconfig")
            .field("rdopcodenonxip", &self.rdopcodenonxip())
            .field("instrtype", &self.instrtype())
            .field("ddren", &self.ddren())
            .field("addrxfertypestdmode", &self.addrxfertypestdmode())
            .field("dataxfertypeextmode", &self.dataxfertypeextmode())
            .field("modebitenable", &self.modebitenable())
            .field("dummyrdclkcycles", &self.dummyrdclkcycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devinstrrdconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devinstrrdconfig {{ rdopcodenonxip: {=u8:?}, instrtype: {=u8:?}, ddren: {=bool:?}, addrxfertypestdmode: {=u8:?}, dataxfertypeextmode: {=u8:?}, modebitenable: {=bool:?}, dummyrdclkcycles: {=u8:?} }}" , self . rdopcodenonxip () , self . instrtype () , self . ddren () , self . addrxfertypestdmode () , self . dataxfertypeextmode () , self . modebitenable () , self . dummyrdclkcycles ())
    }
}
#[doc = "Device Write Instruction Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devinstrwrconfig(pub u32);
impl Devinstrwrconfig {
    #[doc = "Write Opcode."]
    #[must_use]
    #[inline(always)]
    pub const fn wropcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Opcode."]
    #[inline(always)]
    pub const fn set_wropcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "WEL Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn weldis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WEL Disable."]
    #[inline(always)]
    pub const fn set_weldis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Address Transfer Type for Standard SPI Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn addrxfertypestdmode(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Address Transfer Type for Standard SPI Modes."]
    #[inline(always)]
    pub const fn set_addrxfertypestdmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Data Transfer Type for Standard SPI Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn dataxfertypeextmode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Data Transfer Type for Standard SPI Modes."]
    #[inline(always)]
    pub const fn set_dataxfertypeextmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Dummy Write Clock Cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn dummywrclkcycles(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Dummy Write Clock Cycles."]
    #[inline(always)]
    pub const fn set_dummywrclkcycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Devinstrwrconfig {
    #[inline(always)]
    fn default() -> Devinstrwrconfig {
        Devinstrwrconfig(0)
    }
}
impl core::fmt::Debug for Devinstrwrconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devinstrwrconfig")
            .field("wropcode", &self.wropcode())
            .field("weldis", &self.weldis())
            .field("addrxfertypestdmode", &self.addrxfertypestdmode())
            .field("dataxfertypeextmode", &self.dataxfertypeextmode())
            .field("dummywrclkcycles", &self.dummywrclkcycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devinstrwrconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devinstrwrconfig {{ wropcode: {=u8:?}, weldis: {=bool:?}, addrxfertypestdmode: {=u8:?}, dataxfertypeextmode: {=u8:?}, dummywrclkcycles: {=u8:?} }}" , self . wropcode () , self . weldis () , self . addrxfertypestdmode () , self . dataxfertypeextmode () , self . dummywrclkcycles ())
    }
}
#[doc = "Device Size Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devsizeconfig(pub u32);
impl Devsizeconfig {
    #[doc = "Number of Address Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn numaddrbytes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Address Bytes."]
    #[inline(always)]
    pub const fn set_numaddrbytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of Bytes Per Device Page."]
    #[must_use]
    #[inline(always)]
    pub const fn bytesperdevicepage(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of Bytes Per Device Page."]
    #[inline(always)]
    pub const fn set_bytesperdevicepage(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "Number of Bytes Per Block."]
    #[must_use]
    #[inline(always)]
    pub const fn bytespersubsector(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Bytes Per Block."]
    #[inline(always)]
    pub const fn set_bytespersubsector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Size of Flash Device Connected to CS\\[0\\] Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn memsizeoncs0(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Size of Flash Device Connected to CS\\[0\\] Pin."]
    #[inline(always)]
    pub const fn set_memsizeoncs0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "Size of Flash Device Connected to CS\\[1\\] Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn memsizeoncs1(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "Size of Flash Device Connected to CS\\[1\\] Pin."]
    #[inline(always)]
    pub const fn set_memsizeoncs1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
}
impl Default for Devsizeconfig {
    #[inline(always)]
    fn default() -> Devsizeconfig {
        Devsizeconfig(0)
    }
}
impl core::fmt::Debug for Devsizeconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devsizeconfig")
            .field("numaddrbytes", &self.numaddrbytes())
            .field("bytesperdevicepage", &self.bytesperdevicepage())
            .field("bytespersubsector", &self.bytespersubsector())
            .field("memsizeoncs0", &self.memsizeoncs0())
            .field("memsizeoncs1", &self.memsizeoncs1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devsizeconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devsizeconfig {{ numaddrbytes: {=u8:?}, bytesperdevicepage: {=u16:?}, bytespersubsector: {=u8:?}, memsizeoncs0: {=u8:?}, memsizeoncs1: {=u8:?} }}" , self . numaddrbytes () , self . bytesperdevicepage () , self . bytespersubsector () , self . memsizeoncs0 () , self . memsizeoncs1 ())
    }
}
#[doc = "Flash Command Control Register (STIG)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcmdctrl(pub u32);
impl Flashcmdctrl {
    #[doc = "Execute the Command."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdexec(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Execute the Command."]
    #[inline(always)]
    pub const fn set_cmdexec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Execution in Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdexecstatus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command Execution in Progress."]
    #[inline(always)]
    pub const fn set_cmdexecstatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "STIG Memory Bank Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn stigmembanken(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "STIG Memory Bank Enable Bit."]
    #[inline(always)]
    pub const fn set_stigmembanken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Number of Dummy Cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn numdummycycles(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Dummy Cycles."]
    #[inline(always)]
    pub const fn set_numdummycycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "Number of Write Data Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn numwrdatabytes(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Write Data Bytes."]
    #[inline(always)]
    pub const fn set_numwrdatabytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Write Data Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbwritedata(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write Data Enable."]
    #[inline(always)]
    pub const fn set_enbwritedata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Number of Address Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn numaddrbytes(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Address Bytes."]
    #[inline(always)]
    pub const fn set_numaddrbytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Mode Bit Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbmodebit(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Bit Enable."]
    #[inline(always)]
    pub const fn set_enbmodebit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Address Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbcomdaddr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Command Address Enable."]
    #[inline(always)]
    pub const fn set_enbcomdaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Number of Read Data Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn numrddatabytes(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Read Data Bytes."]
    #[inline(always)]
    pub const fn set_numrddatabytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Read Data Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enbreaddata(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Read Data Enable."]
    #[inline(always)]
    pub const fn set_enbreaddata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Command Opcode."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdopcode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Opcode."]
    #[inline(always)]
    pub const fn set_cmdopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Flashcmdctrl {
    #[inline(always)]
    fn default() -> Flashcmdctrl {
        Flashcmdctrl(0)
    }
}
impl core::fmt::Debug for Flashcmdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashcmdctrl")
            .field("cmdexec", &self.cmdexec())
            .field("cmdexecstatus", &self.cmdexecstatus())
            .field("stigmembanken", &self.stigmembanken())
            .field("numdummycycles", &self.numdummycycles())
            .field("numwrdatabytes", &self.numwrdatabytes())
            .field("enbwritedata", &self.enbwritedata())
            .field("numaddrbytes", &self.numaddrbytes())
            .field("enbmodebit", &self.enbmodebit())
            .field("enbcomdaddr", &self.enbcomdaddr())
            .field("numrddatabytes", &self.numrddatabytes())
            .field("enbreaddata", &self.enbreaddata())
            .field("cmdopcode", &self.cmdopcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcmdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Flashcmdctrl {{ cmdexec: {=bool:?}, cmdexecstatus: {=bool:?}, stigmembanken: {=bool:?}, numdummycycles: {=u8:?}, numwrdatabytes: {=u8:?}, enbwritedata: {=bool:?}, numaddrbytes: {=u8:?}, enbmodebit: {=bool:?}, enbcomdaddr: {=bool:?}, numrddatabytes: {=u8:?}, enbreaddata: {=bool:?}, cmdopcode: {=u8:?} }}" , self . cmdexec () , self . cmdexecstatus () , self . stigmembanken () , self . numdummycycles () , self . numwrdatabytes () , self . enbwritedata () , self . numaddrbytes () , self . enbmodebit () , self . enbcomdaddr () , self . numrddatabytes () , self . enbreaddata () , self . cmdopcode ())
    }
}
#[doc = "Flash Command Control Memory Register (STIG)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcommandctrlmem(pub u32);
impl Flashcommandctrlmem {
    #[doc = "Trigger the Memory Bank Data Request."]
    #[must_use]
    #[inline(always)]
    pub const fn triggermembankreq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger the Memory Bank Data Request."]
    #[inline(always)]
    pub const fn set_triggermembankreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory Bank Data Request in Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn membankreqinprogress(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Memory Bank Data Request in Progress."]
    #[inline(always)]
    pub const fn set_membankreqinprogress(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Last Requested Data From the STIG Memory Bank."]
    #[must_use]
    #[inline(always)]
    pub const fn membankreaddata(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Last Requested Data From the STIG Memory Bank."]
    #[inline(always)]
    pub const fn set_membankreaddata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of Read Bytes for the Extended STIG."]
    #[must_use]
    #[inline(always)]
    pub const fn nbofstigreadbytes(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Read Bytes for the Extended STIG."]
    #[inline(always)]
    pub const fn set_nbofstigreadbytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Memory Bank Address."]
    #[must_use]
    #[inline(always)]
    pub const fn membankaddr(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x01ff;
        val as u16
    }
    #[doc = "Memory Bank Address."]
    #[inline(always)]
    pub const fn set_membankaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
    }
}
impl Default for Flashcommandctrlmem {
    #[inline(always)]
    fn default() -> Flashcommandctrlmem {
        Flashcommandctrlmem(0)
    }
}
impl core::fmt::Debug for Flashcommandctrlmem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashcommandctrlmem")
            .field("triggermembankreq", &self.triggermembankreq())
            .field("membankreqinprogress", &self.membankreqinprogress())
            .field("membankreaddata", &self.membankreaddata())
            .field("nbofstigreadbytes", &self.nbofstigreadbytes())
            .field("membankaddr", &self.membankaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcommandctrlmem {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Flashcommandctrlmem {{ triggermembankreq: {=bool:?}, membankreqinprogress: {=bool:?}, membankreaddata: {=u8:?}, nbofstigreadbytes: {=u8:?}, membankaddr: {=u16:?} }}" , self . triggermembankreq () , self . membankreqinprogress () , self . membankreaddata () , self . nbofstigreadbytes () , self . membankaddr ())
    }
}
#[doc = "Indirect Read Transfer Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Indirectreadxferctrl(pub u32);
impl Indirectreadxferctrl {
    #[doc = "Start Indirect Read."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Indirect Read."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cancel Indirect Read."]
    #[must_use]
    #[inline(always)]
    pub const fn cancel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Cancel Indirect Read."]
    #[inline(always)]
    pub const fn set_cancel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indirect Read Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rdstatus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Read Status."]
    #[inline(always)]
    pub const fn set_rdstatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SRAM Full."]
    #[must_use]
    #[inline(always)]
    pub const fn sramfull(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Full."]
    #[inline(always)]
    pub const fn set_sramfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Two Indirect Read Operations Have Been Queued."]
    #[must_use]
    #[inline(always)]
    pub const fn rdqueued(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Two Indirect Read Operations Have Been Queued."]
    #[inline(always)]
    pub const fn set_rdqueued(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indirect Completion Status."]
    #[must_use]
    #[inline(always)]
    pub const fn indopsdonestatus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Completion Status."]
    #[inline(always)]
    pub const fn set_indopsdonestatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number Indirect Operations Done."]
    #[must_use]
    #[inline(always)]
    pub const fn numindopsdone(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Number Indirect Operations Done."]
    #[inline(always)]
    pub const fn set_numindopsdone(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Indirectreadxferctrl {
    #[inline(always)]
    fn default() -> Indirectreadxferctrl {
        Indirectreadxferctrl(0)
    }
}
impl core::fmt::Debug for Indirectreadxferctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Indirectreadxferctrl")
            .field("start", &self.start())
            .field("cancel", &self.cancel())
            .field("rdstatus", &self.rdstatus())
            .field("sramfull", &self.sramfull())
            .field("rdqueued", &self.rdqueued())
            .field("indopsdonestatus", &self.indopsdonestatus())
            .field("numindopsdone", &self.numindopsdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Indirectreadxferctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Indirectreadxferctrl {{ start: {=bool:?}, cancel: {=bool:?}, rdstatus: {=bool:?}, sramfull: {=bool:?}, rdqueued: {=bool:?}, indopsdonestatus: {=bool:?}, numindopsdone: {=u8:?} }}" , self . start () , self . cancel () , self . rdstatus () , self . sramfull () , self . rdqueued () , self . indopsdonestatus () , self . numindopsdone ())
    }
}
#[doc = "Indirect Trigger Address Range Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Indirecttriggeraddrrange(pub u32);
impl Indirecttriggeraddrrange {
    #[doc = "Indirect Trigger Address Width."]
    #[must_use]
    #[inline(always)]
    pub const fn indrangewidth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indirect Trigger Address Width."]
    #[inline(always)]
    pub const fn set_indrangewidth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Indirecttriggeraddrrange {
    #[inline(always)]
    fn default() -> Indirecttriggeraddrrange {
        Indirecttriggeraddrrange(0)
    }
}
impl core::fmt::Debug for Indirecttriggeraddrrange {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Indirecttriggeraddrrange")
            .field("indrangewidth", &self.indrangewidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Indirecttriggeraddrrange {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Indirecttriggeraddrrange {{ indrangewidth: {=u8:?} }}",
            self.indrangewidth()
        )
    }
}
#[doc = "Indirect Write Transfer Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Indirectwritexferctrl(pub u32);
impl Indirectwritexferctrl {
    #[doc = "Start Indirect Write."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Indirect Write."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cancel Indirect Write."]
    #[must_use]
    #[inline(always)]
    pub const fn cancel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Cancel Indirect Write."]
    #[inline(always)]
    pub const fn set_cancel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indirect Write Status."]
    #[must_use]
    #[inline(always)]
    pub const fn wrstatus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Write Status."]
    #[inline(always)]
    pub const fn set_wrstatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Two Indirect Write Operations Have Been Queued."]
    #[must_use]
    #[inline(always)]
    pub const fn wrqueued(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Two Indirect Write Operations Have Been Queued."]
    #[inline(always)]
    pub const fn set_wrqueued(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indirect Completion Status."]
    #[must_use]
    #[inline(always)]
    pub const fn indopsdonestatus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Completion Status."]
    #[inline(always)]
    pub const fn set_indopsdonestatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indirect Operations Done."]
    #[must_use]
    #[inline(always)]
    pub const fn numindopsdone(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Indirect Operations Done."]
    #[inline(always)]
    pub const fn set_numindopsdone(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Indirectwritexferctrl {
    #[inline(always)]
    fn default() -> Indirectwritexferctrl {
        Indirectwritexferctrl(0)
    }
}
impl core::fmt::Debug for Indirectwritexferctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Indirectwritexferctrl")
            .field("start", &self.start())
            .field("cancel", &self.cancel())
            .field("wrstatus", &self.wrstatus())
            .field("wrqueued", &self.wrqueued())
            .field("indopsdonestatus", &self.indopsdonestatus())
            .field("numindopsdone", &self.numindopsdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Indirectwritexferctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Indirectwritexferctrl {{ start: {=bool:?}, cancel: {=bool:?}, wrstatus: {=bool:?}, wrqueued: {=bool:?}, indopsdonestatus: {=bool:?}, numindopsdone: {=u8:?} }}" , self . start () , self . cancel () , self . wrstatus () , self . wrqueued () , self . indopsdonestatus () , self . numindopsdone ())
    }
}
#[doc = "Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqmask(pub u32);
impl Irqmask {
    #[doc = "Mode M Failure Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn modemfailmask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mode M Failure Mask."]
    #[inline(always)]
    pub const fn set_modemfailmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underflow Detected Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn underflowdetmask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Detected Mask."]
    #[inline(always)]
    pub const fn set_underflowdetmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indirect Complete Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectopdonemask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Complete Mask."]
    #[inline(always)]
    pub const fn set_indirectopdonemask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indirect Read Reject Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectreadrejectmask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Read Reject Mask."]
    #[inline(always)]
    pub const fn set_indirectreadrejectmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protected Area Write Attempt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn protwrattemptmask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protected Area Write Attempt Mask."]
    #[inline(always)]
    pub const fn set_protwrattemptmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Illegal Access Detected Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn illegalaccessdetmask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Illegal Access Detected Mask."]
    #[inline(always)]
    pub const fn set_illegalaccessdetmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer Watermark Breach Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectxferlevelbreachmask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Watermark Breach Mask."]
    #[inline(always)]
    pub const fn set_indirectxferlevelbreachmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Overflow Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn recvoverflowmask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overflow Mask."]
    #[inline(always)]
    pub const fn set_recvoverflowmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Small TX FIFO Not Full Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifonotfullmask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Small TX FIFO Not Full Mask."]
    #[inline(always)]
    pub const fn set_txfifonotfullmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Small TX FIFO Full Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifofullmask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Small TX FIFO Full Mask."]
    #[inline(always)]
    pub const fn set_txfifofullmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Small RX FIFO Not Empty Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifonotemptymask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Small RX FIFO Not Empty Mask."]
    #[inline(always)]
    pub const fn set_rxfifonotemptymask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Small RX FIFO Full Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifofullmask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Small RX FIFO Full Mask."]
    #[inline(always)]
    pub const fn set_rxfifofullmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indirect Read Partition Overflow Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn indrdsramfullmask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Read Partition Overflow Mask."]
    #[inline(always)]
    pub const fn set_indrdsramfullmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Polling Expiration Detected Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn pollexpintmask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Polling Expiration Detected Mask."]
    #[inline(always)]
    pub const fn set_pollexpintmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "STIG Request Completion Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn stigreqmask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "STIG Request Completion Mask."]
    #[inline(always)]
    pub const fn set_stigreqmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RX CRC Data Error Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdataerrmask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RX CRC Data Error Mask."]
    #[inline(always)]
    pub const fn set_rxcrcdataerrmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RX CRC Data Valid Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdatavalmask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RX CRC Data Valid Mask."]
    #[inline(always)]
    pub const fn set_rxcrcdatavalmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX CRC Chunk Was Broken Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txcrcchunkbrkmask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TX CRC Chunk Was Broken Mask."]
    #[inline(always)]
    pub const fn set_txcrcchunkbrkmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Irqmask {
    #[inline(always)]
    fn default() -> Irqmask {
        Irqmask(0)
    }
}
impl core::fmt::Debug for Irqmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqmask")
            .field("modemfailmask", &self.modemfailmask())
            .field("underflowdetmask", &self.underflowdetmask())
            .field("indirectopdonemask", &self.indirectopdonemask())
            .field("indirectreadrejectmask", &self.indirectreadrejectmask())
            .field("protwrattemptmask", &self.protwrattemptmask())
            .field("illegalaccessdetmask", &self.illegalaccessdetmask())
            .field(
                "indirectxferlevelbreachmask",
                &self.indirectxferlevelbreachmask(),
            )
            .field("recvoverflowmask", &self.recvoverflowmask())
            .field("txfifonotfullmask", &self.txfifonotfullmask())
            .field("txfifofullmask", &self.txfifofullmask())
            .field("rxfifonotemptymask", &self.rxfifonotemptymask())
            .field("rxfifofullmask", &self.rxfifofullmask())
            .field("indrdsramfullmask", &self.indrdsramfullmask())
            .field("pollexpintmask", &self.pollexpintmask())
            .field("stigreqmask", &self.stigreqmask())
            .field("rxcrcdataerrmask", &self.rxcrcdataerrmask())
            .field("rxcrcdatavalmask", &self.rxcrcdatavalmask())
            .field("txcrcchunkbrkmask", &self.txcrcchunkbrkmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqmask {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irqmask {{ modemfailmask: {=bool:?}, underflowdetmask: {=bool:?}, indirectopdonemask: {=bool:?}, indirectreadrejectmask: {=bool:?}, protwrattemptmask: {=bool:?}, illegalaccessdetmask: {=bool:?}, indirectxferlevelbreachmask: {=bool:?}, recvoverflowmask: {=bool:?}, txfifonotfullmask: {=bool:?}, txfifofullmask: {=bool:?}, rxfifonotemptymask: {=bool:?}, rxfifofullmask: {=bool:?}, indrdsramfullmask: {=bool:?}, pollexpintmask: {=bool:?}, stigreqmask: {=bool:?}, rxcrcdataerrmask: {=bool:?}, rxcrcdatavalmask: {=bool:?}, txcrcchunkbrkmask: {=bool:?} }}" , self . modemfailmask () , self . underflowdetmask () , self . indirectopdonemask () , self . indirectreadrejectmask () , self . protwrattemptmask () , self . illegalaccessdetmask () , self . indirectxferlevelbreachmask () , self . recvoverflowmask () , self . txfifonotfullmask () , self . txfifofullmask () , self . rxfifonotemptymask () , self . rxfifofullmask () , self . indrdsramfullmask () , self . pollexpintmask () , self . stigreqmask () , self . rxcrcdataerrmask () , self . rxcrcdatavalmask () , self . txcrcchunkbrkmask ())
    }
}
#[doc = "Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqstatus(pub u32);
impl Irqstatus {
    #[doc = "Mode M Failure."]
    #[must_use]
    #[inline(always)]
    pub const fn modemfail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mode M Failure."]
    #[inline(always)]
    pub const fn set_modemfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underflow Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn underflowdet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow Detected."]
    #[inline(always)]
    pub const fn set_underflowdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indirect Operation Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectopdone(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Operation Complete."]
    #[inline(always)]
    pub const fn set_indirectopdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indirect Operation Was Requested but Could Not Be Accepted."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectreadreject(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Operation Was Requested but Could Not Be Accepted."]
    #[inline(always)]
    pub const fn set_indirectreadreject(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write to Protected Area Was Attempted and Rejected."]
    #[must_use]
    #[inline(always)]
    pub const fn protwrattempt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write to Protected Area Was Attempted and Rejected."]
    #[inline(always)]
    pub const fn set_protwrattempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Illegal Memory Access Has Been Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn illegalaccessdet(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Illegal Memory Access Has Been Detected."]
    #[inline(always)]
    pub const fn set_illegalaccessdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indirect Transfer Watermark Level Breached."]
    #[must_use]
    #[inline(always)]
    pub const fn indirectxferlevelbreach(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Transfer Watermark Level Breached."]
    #[inline(always)]
    pub const fn set_indirectxferlevelbreach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn recvoverflow(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Overflow."]
    #[inline(always)]
    pub const fn set_recvoverflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Small TX FIFO Not Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifonotfull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Small TX FIFO Not Full."]
    #[inline(always)]
    pub const fn set_txfifonotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Small TX FIFO Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifofull(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Small TX FIFO Full."]
    #[inline(always)]
    pub const fn set_txfifofull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Small RX FIFO Not Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifonotempty(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Small RX FIFO Not Empty."]
    #[inline(always)]
    pub const fn set_rxfifonotempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Small RX FIFO Full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifofull(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Small RX FIFO Full."]
    #[inline(always)]
    pub const fn set_rxfifofull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indirect Read Partition Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn indrdsramfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indirect Read Partition Overflow."]
    #[inline(always)]
    pub const fn set_indrdsramfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "The Maximum Number of Programmed Polls Cycles is Expired."]
    #[must_use]
    #[inline(always)]
    pub const fn pollexpint(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "The Maximum Number of Programmed Polls Cycles is Expired."]
    #[inline(always)]
    pub const fn set_pollexpint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The Controller is Ready for Getting Another STIG Request."]
    #[must_use]
    #[inline(always)]
    pub const fn stigreqint(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The Controller is Ready for Getting Another STIG Request."]
    #[inline(always)]
    pub const fn set_stigreqint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RX CRC Data Error."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdataerr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RX CRC Data Error."]
    #[inline(always)]
    pub const fn set_rxcrcdataerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RX CRC Data Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdataval(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RX CRC Data Valid."]
    #[inline(always)]
    pub const fn set_rxcrcdataval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX CRC Chunk Was Broken."]
    #[must_use]
    #[inline(always)]
    pub const fn txcrcchunkbrk(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TX CRC Chunk Was Broken."]
    #[inline(always)]
    pub const fn set_txcrcchunkbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Irqstatus {
    #[inline(always)]
    fn default() -> Irqstatus {
        Irqstatus(0)
    }
}
impl core::fmt::Debug for Irqstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqstatus")
            .field("modemfail", &self.modemfail())
            .field("underflowdet", &self.underflowdet())
            .field("indirectopdone", &self.indirectopdone())
            .field("indirectreadreject", &self.indirectreadreject())
            .field("protwrattempt", &self.protwrattempt())
            .field("illegalaccessdet", &self.illegalaccessdet())
            .field("indirectxferlevelbreach", &self.indirectxferlevelbreach())
            .field("recvoverflow", &self.recvoverflow())
            .field("txfifonotfull", &self.txfifonotfull())
            .field("txfifofull", &self.txfifofull())
            .field("rxfifonotempty", &self.rxfifonotempty())
            .field("rxfifofull", &self.rxfifofull())
            .field("indrdsramfull", &self.indrdsramfull())
            .field("pollexpint", &self.pollexpint())
            .field("stigreqint", &self.stigreqint())
            .field("rxcrcdataerr", &self.rxcrcdataerr())
            .field("rxcrcdataval", &self.rxcrcdataval())
            .field("txcrcchunkbrk", &self.txcrcchunkbrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irqstatus {{ modemfail: {=bool:?}, underflowdet: {=bool:?}, indirectopdone: {=bool:?}, indirectreadreject: {=bool:?}, protwrattempt: {=bool:?}, illegalaccessdet: {=bool:?}, indirectxferlevelbreach: {=bool:?}, recvoverflow: {=bool:?}, txfifonotfull: {=bool:?}, txfifofull: {=bool:?}, rxfifonotempty: {=bool:?}, rxfifofull: {=bool:?}, indrdsramfull: {=bool:?}, pollexpint: {=bool:?}, stigreqint: {=bool:?}, rxcrcdataerr: {=bool:?}, rxcrcdataval: {=bool:?}, txcrcchunkbrk: {=bool:?} }}" , self . modemfail () , self . underflowdet () , self . indirectopdone () , self . indirectreadreject () , self . protwrattempt () , self . illegalaccessdet () , self . indirectxferlevelbreach () , self . recvoverflow () , self . txfifonotfull () , self . txfifofull () , self . rxfifonotempty () , self . rxfifofull () , self . indrdsramfull () , self . pollexpint () , self . stigreqint () , self . rxcrcdataerr () , self . rxcrcdataval () , self . txcrcchunkbrk ())
    }
}
#[doc = "Mode Bit Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modebitconfig(pub u32);
impl Modebitconfig {
    #[doc = "Mode Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Mode Bits."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Chunk Size."]
    #[must_use]
    #[inline(always)]
    pub const fn chunksize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Chunk Size."]
    #[inline(always)]
    pub const fn set_chunksize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "CRC# Output Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn crcoutenable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CRC# Output Enable Bit."]
    #[inline(always)]
    pub const fn set_crcoutenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RX CRC Data (upper)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdataup(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RX CRC Data (upper)."]
    #[inline(always)]
    pub const fn set_rxcrcdataup(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "RX CRC Data (lower)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcrcdatalow(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "RX CRC Data (lower)."]
    #[inline(always)]
    pub const fn set_rxcrcdatalow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Modebitconfig {
    #[inline(always)]
    fn default() -> Modebitconfig {
        Modebitconfig(0)
    }
}
impl core::fmt::Debug for Modebitconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modebitconfig")
            .field("mode", &self.mode())
            .field("chunksize", &self.chunksize())
            .field("crcoutenable", &self.crcoutenable())
            .field("rxcrcdataup", &self.rxcrcdataup())
            .field("rxcrcdatalow", &self.rxcrcdatalow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modebitconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Modebitconfig {{ mode: {=u8:?}, chunksize: {=u8:?}, crcoutenable: {=bool:?}, rxcrcdataup: {=u8:?}, rxcrcdatalow: {=u8:?} }}" , self . mode () , self . chunksize () , self . crcoutenable () , self . rxcrcdataup () , self . rxcrcdatalow ())
    }
}
#[doc = "Module ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moduleid(pub u32);
impl Moduleid {
    #[doc = "Configuration ID Number."]
    #[must_use]
    #[inline(always)]
    pub const fn conf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Configuration ID Number."]
    #[inline(always)]
    pub const fn set_conf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Module/Revision ID Number."]
    #[must_use]
    #[inline(always)]
    pub const fn moduleid(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Module/Revision ID Number."]
    #[inline(always)]
    pub const fn set_moduleid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "Fix/patch Number."]
    #[must_use]
    #[inline(always)]
    pub const fn fixpatch(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fix/patch Number."]
    #[inline(always)]
    pub const fn set_fixpatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Moduleid {
    #[inline(always)]
    fn default() -> Moduleid {
        Moduleid(0)
    }
}
impl core::fmt::Debug for Moduleid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moduleid")
            .field("conf", &self.conf())
            .field("moduleid", &self.moduleid())
            .field("fixpatch", &self.fixpatch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moduleid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moduleid {{ conf: {=u8:?}, moduleid: {=u16:?}, fixpatch: {=u8:?} }}",
            self.conf(),
            self.moduleid(),
            self.fixpatch()
        )
    }
}
#[doc = "Opcode Extension Register (Lower)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opcodeextlower(pub u32);
impl Opcodeextlower {
    #[doc = "STIG Opcode Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn extstigopcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "STIG Opcode Extension."]
    #[inline(always)]
    pub const fn set_extstigopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Polling Opcode Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn extpollopcode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Polling Opcode Extension."]
    #[inline(always)]
    pub const fn set_extpollopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Write Opcode Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn extwriteopcode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write Opcode Extension."]
    #[inline(always)]
    pub const fn set_extwriteopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Read Opcode Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn extreadopcode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Read Opcode Extension."]
    #[inline(always)]
    pub const fn set_extreadopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Opcodeextlower {
    #[inline(always)]
    fn default() -> Opcodeextlower {
        Opcodeextlower(0)
    }
}
impl core::fmt::Debug for Opcodeextlower {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opcodeextlower")
            .field("extstigopcode", &self.extstigopcode())
            .field("extpollopcode", &self.extpollopcode())
            .field("extwriteopcode", &self.extwriteopcode())
            .field("extreadopcode", &self.extreadopcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opcodeextlower {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Opcodeextlower {{ extstigopcode: {=u8:?}, extpollopcode: {=u8:?}, extwriteopcode: {=u8:?}, extreadopcode: {=u8:?} }}" , self . extstigopcode () , self . extpollopcode () , self . extwriteopcode () , self . extreadopcode ())
    }
}
#[doc = "Opcode Extension Register (Upper)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opcodeextupper(pub u32);
impl Opcodeextupper {
    #[doc = "WEL Opcode Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn extwelopcode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "WEL Opcode Extension."]
    #[inline(always)]
    pub const fn set_extwelopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "WEL Opcode."]
    #[must_use]
    #[inline(always)]
    pub const fn welopcode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "WEL Opcode."]
    #[inline(always)]
    pub const fn set_welopcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Opcodeextupper {
    #[inline(always)]
    fn default() -> Opcodeextupper {
        Opcodeextupper(0)
    }
}
impl core::fmt::Debug for Opcodeextupper {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opcodeextupper")
            .field("extwelopcode", &self.extwelopcode())
            .field("welopcode", &self.welopcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opcodeextupper {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opcodeextupper {{ extwelopcode: {=u8:?}, welopcode: {=u8:?} }}",
            self.extwelopcode(),
            self.welopcode()
        )
    }
}
#[doc = "PHY Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phyconfiguration(pub u32);
impl Phyconfiguration {
    #[doc = "RX DLL Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn phyconfigrxdlldelay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "RX DLL Delay."]
    #[inline(always)]
    pub const fn set_phyconfigrxdlldelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "TX DLL Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn phyconfigtxdlldelay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "TX DLL Delay."]
    #[inline(always)]
    pub const fn set_phyconfigtxdlldelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "PHY Config Resync."]
    #[must_use]
    #[inline(always)]
    pub const fn phyconfigresync(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Config Resync."]
    #[inline(always)]
    pub const fn set_phyconfigresync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Phyconfiguration {
    #[inline(always)]
    fn default() -> Phyconfiguration {
        Phyconfiguration(0)
    }
}
impl core::fmt::Debug for Phyconfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phyconfiguration")
            .field("phyconfigrxdlldelay", &self.phyconfigrxdlldelay())
            .field("phyconfigtxdlldelay", &self.phyconfigtxdlldelay())
            .field("phyconfigresync", &self.phyconfigresync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phyconfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Phyconfiguration {{ phyconfigrxdlldelay: {=u8:?}, phyconfigtxdlldelay: {=u8:?}, phyconfigresync: {=bool:?} }}" , self . phyconfigrxdlldelay () , self . phyconfigtxdlldelay () , self . phyconfigresync ())
    }
}
#[doc = "Polling Flash Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pollingflashstatus(pub u32);
impl Pollingflashstatus {
    #[doc = "Device Status."]
    #[must_use]
    #[inline(always)]
    pub const fn devicestatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Device Status."]
    #[inline(always)]
    pub const fn set_devicestatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Device Status Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn devicestatusvalid(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Device Status Valid."]
    #[inline(always)]
    pub const fn set_devicestatusvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Auto-polling Dummy Cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn devicestatusnbdummy(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Auto-polling Dummy Cycles."]
    #[inline(always)]
    pub const fn set_devicestatusnbdummy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Pollingflashstatus {
    #[inline(always)]
    fn default() -> Pollingflashstatus {
        Pollingflashstatus(0)
    }
}
impl core::fmt::Debug for Pollingflashstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pollingflashstatus")
            .field("devicestatus", &self.devicestatus())
            .field("devicestatusvalid", &self.devicestatusvalid())
            .field("devicestatusnbdummy", &self.devicestatusnbdummy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pollingflashstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pollingflashstatus {{ devicestatus: {=u8:?}, devicestatusvalid: {=bool:?}, devicestatusnbdummy: {=u8:?} }}" , self . devicestatus () , self . devicestatusvalid () , self . devicestatusnbdummy ())
    }
}
#[doc = "Read Data Capture Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rddatacapture(pub u32);
impl Rddatacapture {
    #[doc = "Bypass the Adapted Loopback Clock Circuit."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the Adapted Loopback Clock Circuit."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn delay(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Delay."]
    #[inline(always)]
    pub const fn set_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "DQS Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dqsenable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DQS Enable Bit."]
    #[inline(always)]
    pub const fn set_dqsenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DDR Read Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn ddrreaddelay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DDR Read Delay."]
    #[inline(always)]
    pub const fn set_ddrreaddelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Rddatacapture {
    #[inline(always)]
    fn default() -> Rddatacapture {
        Rddatacapture(0)
    }
}
impl core::fmt::Debug for Rddatacapture {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rddatacapture")
            .field("bypass", &self.bypass())
            .field("delay", &self.delay())
            .field("dqsenable", &self.dqsenable())
            .field("ddrreaddelay", &self.ddrreaddelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rddatacapture {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rddatacapture {{ bypass: {=bool:?}, delay: {=u8:?}, dqsenable: {=bool:?}, ddrreaddelay: {=u8:?} }}" , self . bypass () , self . delay () , self . dqsenable () , self . ddrreaddelay ())
    }
}
#[doc = "I/O Route Location Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc = "I/O Location."]
    #[must_use]
    #[inline(always)]
    pub const fn qspiloc(&self) -> super::vals::Qspiloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Qspiloc::from_bits(val as u8)
    }
    #[doc = "I/O Location."]
    #[inline(always)]
    pub const fn set_qspiloc(&mut self, val: super::vals::Qspiloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Routeloc0 {
    #[inline(always)]
    fn default() -> Routeloc0 {
        Routeloc0(0)
    }
}
impl core::fmt::Debug for Routeloc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc0")
            .field("qspiloc", &self.qspiloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routeloc0 {{ qspiloc: {:?} }}", self.qspiloc())
    }
}
#[doc = "I/O Routing Pin Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "SCLK Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sclkpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Pin Enable."]
    #[inline(always)]
    pub const fn set_sclkpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CS0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs0pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CS0 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CS1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cs1pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CS1 Pin Enable."]
    #[inline(always)]
    pub const fn set_cs1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DQ0 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq0pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DQ0 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DQ1 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq1pen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DQ1 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DQ2 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq2pen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DQ2 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DQ3 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq3pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DQ3 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DQ4 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq4pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DQ4 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DQ5 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq5pen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DQ5 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DQ6 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq6pen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DQ6 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DQ7 Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dq7pen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DQ7 Pin Enable."]
    #[inline(always)]
    pub const fn set_dq7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DQS Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dqspen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DQS Pin Enable."]
    #[inline(always)]
    pub const fn set_dqspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SCLKIN Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sclkinpen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SCLKIN Pin Enable."]
    #[inline(always)]
    pub const fn set_sclkinpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Routepen {
    #[inline(always)]
    fn default() -> Routepen {
        Routepen(0)
    }
}
impl core::fmt::Debug for Routepen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routepen")
            .field("sclkpen", &self.sclkpen())
            .field("cs0pen", &self.cs0pen())
            .field("cs1pen", &self.cs1pen())
            .field("dq0pen", &self.dq0pen())
            .field("dq1pen", &self.dq1pen())
            .field("dq2pen", &self.dq2pen())
            .field("dq3pen", &self.dq3pen())
            .field("dq4pen", &self.dq4pen())
            .field("dq5pen", &self.dq5pen())
            .field("dq6pen", &self.dq6pen())
            .field("dq7pen", &self.dq7pen())
            .field("dqspen", &self.dqspen())
            .field("sclkinpen", &self.sclkinpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ sclkpen: {=bool:?}, cs0pen: {=bool:?}, cs1pen: {=bool:?}, dq0pen: {=bool:?}, dq1pen: {=bool:?}, dq2pen: {=bool:?}, dq3pen: {=bool:?}, dq4pen: {=bool:?}, dq5pen: {=bool:?}, dq6pen: {=bool:?}, dq7pen: {=bool:?}, dqspen: {=bool:?}, sclkinpen: {=bool:?} }}" , self . sclkpen () , self . cs0pen () , self . cs1pen () , self . dq0pen () , self . dq1pen () , self . dq2pen () , self . dq3pen () , self . dq4pen () , self . dq5pen () , self . dq6pen () , self . dq7pen () , self . dqspen () , self . sclkinpen ())
    }
}
#[doc = "RX Threshold Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxthresh(pub u32);
impl Rxthresh {
    #[doc = "Threshold Level."]
    #[must_use]
    #[inline(always)]
    pub const fn level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Threshold Level."]
    #[inline(always)]
    pub const fn set_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Rxthresh {
    #[inline(always)]
    fn default() -> Rxthresh {
        Rxthresh(0)
    }
}
impl core::fmt::Debug for Rxthresh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxthresh")
            .field("level", &self.level())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxthresh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxthresh {{ level: {=u8:?} }}", self.level())
    }
}
#[doc = "SRAM Fill Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramfill(pub u32);
impl Sramfill {
    #[doc = "SRAM Fill Level (Indirect Read Partition)."]
    #[must_use]
    #[inline(always)]
    pub const fn sramfillindacread(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SRAM Fill Level (Indirect Read Partition)."]
    #[inline(always)]
    pub const fn set_sramfillindacread(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "SRAM Fill Level (Indirect Write Partition)."]
    #[must_use]
    #[inline(always)]
    pub const fn sramfillindacwrite(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "SRAM Fill Level (Indirect Write Partition)."]
    #[inline(always)]
    pub const fn set_sramfillindacwrite(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sramfill {
    #[inline(always)]
    fn default() -> Sramfill {
        Sramfill(0)
    }
}
impl core::fmt::Debug for Sramfill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramfill")
            .field("sramfillindacread", &self.sramfillindacread())
            .field("sramfillindacwrite", &self.sramfillindacwrite())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramfill {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramfill {{ sramfillindacread: {=u16:?}, sramfillindacwrite: {=u16:?} }}",
            self.sramfillindacread(),
            self.sramfillindacwrite()
        )
    }
}
#[doc = "SRAM Partition Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srampartitioncfg(pub u32);
impl Srampartitioncfg {
    #[doc = "Indirect Read Partition Size."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indirect Read Partition Size."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Srampartitioncfg {
    #[inline(always)]
    fn default() -> Srampartitioncfg {
        Srampartitioncfg(0)
    }
}
impl core::fmt::Debug for Srampartitioncfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srampartitioncfg")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srampartitioncfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srampartitioncfg {{ addr: {=u8:?} }}", self.addr())
    }
}
#[doc = "TX Threshold Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txthresh(pub u32);
impl Txthresh {
    #[doc = "Threshold Level."]
    #[must_use]
    #[inline(always)]
    pub const fn level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Threshold Level."]
    #[inline(always)]
    pub const fn set_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Txthresh {
    #[inline(always)]
    fn default() -> Txthresh {
        Txthresh(0)
    }
}
impl core::fmt::Debug for Txthresh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txthresh")
            .field("level", &self.level())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txthresh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txthresh {{ level: {=u8:?} }}", self.level())
    }
}
#[doc = "Write Completion Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Writecompletionctrl(pub u32);
impl Writecompletionctrl {
    #[doc = "Opcode."]
    #[must_use]
    #[inline(always)]
    pub const fn opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Opcode."]
    #[inline(always)]
    pub const fn set_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Polling Bit Index."]
    #[must_use]
    #[inline(always)]
    pub const fn pollingbitindex(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Polling Bit Index."]
    #[inline(always)]
    pub const fn set_pollingbitindex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Polling Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pollingpolarity(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Polling Polarity."]
    #[inline(always)]
    pub const fn set_pollingpolarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Disable Polling."]
    #[must_use]
    #[inline(always)]
    pub const fn disablepolling(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Polling."]
    #[inline(always)]
    pub const fn set_disablepolling(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Polling Expiration."]
    #[must_use]
    #[inline(always)]
    pub const fn enablepollingexp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Polling Expiration."]
    #[inline(always)]
    pub const fn set_enablepollingexp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Poll Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pollcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Poll Count."]
    #[inline(always)]
    pub const fn set_pollcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Poll Repetition Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn pollrepdelay(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Poll Repetition Delay."]
    #[inline(always)]
    pub const fn set_pollrepdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Writecompletionctrl {
    #[inline(always)]
    fn default() -> Writecompletionctrl {
        Writecompletionctrl(0)
    }
}
impl core::fmt::Debug for Writecompletionctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Writecompletionctrl")
            .field("opcode", &self.opcode())
            .field("pollingbitindex", &self.pollingbitindex())
            .field("pollingpolarity", &self.pollingpolarity())
            .field("disablepolling", &self.disablepolling())
            .field("enablepollingexp", &self.enablepollingexp())
            .field("pollcount", &self.pollcount())
            .field("pollrepdelay", &self.pollrepdelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Writecompletionctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Writecompletionctrl {{ opcode: {=u8:?}, pollingbitindex: {=u8:?}, pollingpolarity: {=bool:?}, disablepolling: {=bool:?}, enablepollingexp: {=bool:?}, pollcount: {=u8:?}, pollrepdelay: {=u8:?} }}" , self . opcode () , self . pollingbitindex () , self . pollingpolarity () , self . disablepolling () , self . enablepollingexp () , self . pollcount () , self . pollrepdelay ())
    }
}
#[doc = "Write Protection Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrprotctrl(pub u32);
impl Wrprotctrl {
    #[doc = "Write Protection Inversion Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protection Inversion Bit."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write Protection Enable Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protection Enable Bit."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Wrprotctrl {
    #[inline(always)]
    fn default() -> Wrprotctrl {
        Wrprotctrl(0)
    }
}
impl core::fmt::Debug for Wrprotctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrprotctrl")
            .field("inv", &self.inv())
            .field("enb", &self.enb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrprotctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wrprotctrl {{ inv: {=bool:?}, enb: {=bool:?} }}",
            self.inv(),
            self.enb()
        )
    }
}
