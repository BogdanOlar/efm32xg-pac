#[doc = "AUTO CMD12 Error Status and Host Control2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ac12errstat(pub u32);
impl Ac12errstat {
    #[doc = "Auto CMD12 Not Executed."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12notexe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD12 Not Executed."]
    #[inline(always)]
    pub const fn set_ac12notexe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Auto CMD12 Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12toe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD12 Timeout Error."]
    #[inline(always)]
    pub const fn set_ac12toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12crcerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD CRC Error."]
    #[inline(always)]
    pub const fn set_ac12crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto CMD End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12endbiterr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD End Bit Error."]
    #[inline(always)]
    pub const fn set_ac12endbiterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Auto CMD Index Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12indexerr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD Index Error."]
    #[inline(always)]
    pub const fn set_ac12indexerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Command Not Issued By Auto CMD12 Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cnibac12err(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub const fn set_cnibac12err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UHS Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn uhsmodesel(&self) -> super::vals::Uhsmodesel {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Uhsmodesel::from_bits(val as u8)
    }
    #[doc = "UHS Mode Select."]
    #[inline(always)]
    pub const fn set_uhsmodesel(&mut self, val: super::vals::Uhsmodesel) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Voltage 1.8V Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sigen1p8v(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage 1.8V Signal Enable."]
    #[inline(always)]
    pub const fn set_sigen1p8v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Driver Strength Select."]
    #[must_use]
    #[inline(always)]
    pub const fn drvstnsel(&self) -> super::vals::Drvstnsel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Drvstnsel::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select."]
    #[inline(always)]
    pub const fn set_drvstnsel(&mut self, val: super::vals::Drvstnsel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Execute Tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn exetuning(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Execute Tuning."]
    #[inline(always)]
    pub const fn set_exetuning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Sampling Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sampclksel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Sampling Clock Select."]
    #[inline(always)]
    pub const fn set_sampclksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Asynchronous Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn asyncinten(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub const fn set_asyncinten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Preset Value Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prstvalen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Preset Value Enable."]
    #[inline(always)]
    pub const fn set_prstvalen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ac12errstat {
    #[inline(always)]
    fn default() -> Ac12errstat {
        Ac12errstat(0)
    }
}
impl core::fmt::Debug for Ac12errstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ac12errstat")
            .field("ac12notexe", &self.ac12notexe())
            .field("ac12toe", &self.ac12toe())
            .field("ac12crcerr", &self.ac12crcerr())
            .field("ac12endbiterr", &self.ac12endbiterr())
            .field("ac12indexerr", &self.ac12indexerr())
            .field("cnibac12err", &self.cnibac12err())
            .field("uhsmodesel", &self.uhsmodesel())
            .field("sigen1p8v", &self.sigen1p8v())
            .field("drvstnsel", &self.drvstnsel())
            .field("exetuning", &self.exetuning())
            .field("sampclksel", &self.sampclksel())
            .field("asyncinten", &self.asyncinten())
            .field("prstvalen", &self.prstvalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ac12errstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ac12errstat {{ ac12notexe: {=bool:?}, ac12toe: {=bool:?}, ac12crcerr: {=bool:?}, ac12endbiterr: {=bool:?}, ac12indexerr: {=bool:?}, cnibac12err: {=bool:?}, uhsmodesel: {:?}, sigen1p8v: {=bool:?}, drvstnsel: {:?}, exetuning: {=bool:?}, sampclksel: {=bool:?}, asyncinten: {=bool:?}, prstvalen: {=bool:?} }}" , self . ac12notexe () , self . ac12toe () , self . ac12crcerr () , self . ac12endbiterr () , self . ac12indexerr () , self . cnibac12err () , self . uhsmodesel () , self . sigen1p8v () , self . drvstnsel () , self . exetuning () , self . sampclksel () , self . asyncinten () , self . prstvalen ())
    }
}
#[doc = "ADMA Error Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Admaes(pub u32);
impl Admaes {
    #[doc = "ADMA Error State."]
    #[must_use]
    #[inline(always)]
    pub const fn admaes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADMA Error State."]
    #[inline(always)]
    pub const fn set_admaes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADMA Length Mismatch Error."]
    #[must_use]
    #[inline(always)]
    pub const fn admalme(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA Length Mismatch Error."]
    #[inline(always)]
    pub const fn set_admalme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Admaes {
    #[inline(always)]
    fn default() -> Admaes {
        Admaes(0)
    }
}
impl core::fmt::Debug for Admaes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Admaes")
            .field("admaes", &self.admaes())
            .field("admalme", &self.admalme())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Admaes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Admaes {{ admaes: {=u8:?}, admalme: {=bool:?} }}",
            self.admaes(),
            self.admalme()
        )
    }
}
#[doc = "Block Size and Block Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blksize(pub u32);
impl Blksize {
    #[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53."]
    #[must_use]
    #[inline(always)]
    pub const fn tfrblksize(&self) -> super::vals::Tfrblksize {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::Tfrblksize::from_bits(val as u16)
    }
    #[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53."]
    #[inline(always)]
    pub const fn set_tfrblksize(&mut self, val: super::vals::Tfrblksize) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Host SDMA Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn hstsdmabufsize(&self) -> super::vals::Hstsdmabufsize {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Hstsdmabufsize::from_bits(val as u8)
    }
    #[doc = "Host SDMA Buffer Size."]
    #[inline(always)]
    pub const fn set_hstsdmabufsize(&mut self, val: super::vals::Hstsdmabufsize) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Blocks Count for Current Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn blkscntforcurrtfr(&self) -> super::vals::Blkscntforcurrtfr {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Blkscntforcurrtfr::from_bits(val as u16)
    }
    #[doc = "Blocks Count for Current Transfer."]
    #[inline(always)]
    pub const fn set_blkscntforcurrtfr(&mut self, val: super::vals::Blkscntforcurrtfr) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Blksize {
    #[inline(always)]
    fn default() -> Blksize {
        Blksize(0)
    }
}
impl core::fmt::Debug for Blksize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blksize")
            .field("tfrblksize", &self.tfrblksize())
            .field("hstsdmabufsize", &self.hstsdmabufsize())
            .field("blkscntforcurrtfr", &self.blkscntforcurrtfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blksize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Blksize {{ tfrblksize: {:?}, hstsdmabufsize: {:?}, blkscntforcurrtfr: {:?} }}",
            self.tfrblksize(),
            self.hstsdmabufsize(),
            self.blkscntforcurrtfr()
        )
    }
}
#[doc = "Capabilities Register to Hold Bits 31~0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capab0(pub u32);
impl Capab0 {
    #[doc = "Timeout Clock Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn tmoutclkfreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Timeout Clock Frequency."]
    #[inline(always)]
    pub const fn set_tmoutclkfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Timeout Clock Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmoutclkunit(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Clock Unit."]
    #[inline(always)]
    pub const fn set_tmoutclkunit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Base Clock Frequency for SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn baseclkfreqsd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Base Clock Frequency for SD_CLK."]
    #[inline(always)]
    pub const fn set_baseclkfreqsd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Maximum Block Length."]
    #[must_use]
    #[inline(always)]
    pub const fn maxblocklen(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Maximum Block Length."]
    #[inline(always)]
    pub const fn set_maxblocklen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Extended Media Bus Support."]
    #[must_use]
    #[inline(always)]
    pub const fn extmediabussup(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Media Bus Support."]
    #[inline(always)]
    pub const fn set_extmediabussup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ADMA2 Support."]
    #[must_use]
    #[inline(always)]
    pub const fn adma2sup(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA2 Support."]
    #[inline(always)]
    pub const fn set_adma2sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "High Speed Support."]
    #[must_use]
    #[inline(always)]
    pub const fn hssup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Support."]
    #[inline(always)]
    pub const fn set_hssup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SDMA Support."]
    #[must_use]
    #[inline(always)]
    pub const fn sdmasup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SDMA Support."]
    #[inline(always)]
    pub const fn set_sdmasup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Suspend / Resume Support."]
    #[must_use]
    #[inline(always)]
    pub const fn susressup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend / Resume Support."]
    #[inline(always)]
    pub const fn set_susressup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Voltage Support 3.3V."]
    #[must_use]
    #[inline(always)]
    pub const fn voltsup3p3v(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Support 3.3V."]
    #[inline(always)]
    pub const fn set_voltsup3p3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Voltage Support 3.0V."]
    #[must_use]
    #[inline(always)]
    pub const fn voltsup3p0v(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Support 3.0V."]
    #[inline(always)]
    pub const fn set_voltsup3p0v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Voltage Support 1.8V."]
    #[must_use]
    #[inline(always)]
    pub const fn voltsup1p8v(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Support 1.8V."]
    #[inline(always)]
    pub const fn set_voltsup1p8v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "System Bus 64-bit Support."]
    #[must_use]
    #[inline(always)]
    pub const fn sysbus64bsup(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System Bus 64-bit Support."]
    #[inline(always)]
    pub const fn set_sysbus64bsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Asynchronous Interrupt Support."]
    #[must_use]
    #[inline(always)]
    pub const fn asyncintsup(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Interrupt Support."]
    #[inline(always)]
    pub const fn set_asyncintsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interface Card Slot Type."]
    #[must_use]
    #[inline(always)]
    pub const fn ifslottype(&self) -> super::vals::Ifslottype {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Ifslottype::from_bits(val as u8)
    }
    #[doc = "Interface Card Slot Type."]
    #[inline(always)]
    pub const fn set_ifslottype(&mut self, val: super::vals::Ifslottype) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Capab0 {
    #[inline(always)]
    fn default() -> Capab0 {
        Capab0(0)
    }
}
impl core::fmt::Debug for Capab0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capab0")
            .field("tmoutclkfreq", &self.tmoutclkfreq())
            .field("tmoutclkunit", &self.tmoutclkunit())
            .field("baseclkfreqsd", &self.baseclkfreqsd())
            .field("maxblocklen", &self.maxblocklen())
            .field("extmediabussup", &self.extmediabussup())
            .field("adma2sup", &self.adma2sup())
            .field("hssup", &self.hssup())
            .field("sdmasup", &self.sdmasup())
            .field("susressup", &self.susressup())
            .field("voltsup3p3v", &self.voltsup3p3v())
            .field("voltsup3p0v", &self.voltsup3p0v())
            .field("voltsup1p8v", &self.voltsup1p8v())
            .field("sysbus64bsup", &self.sysbus64bsup())
            .field("asyncintsup", &self.asyncintsup())
            .field("ifslottype", &self.ifslottype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capab0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Capab0 {{ tmoutclkfreq: {=u8:?}, tmoutclkunit: {=bool:?}, baseclkfreqsd: {=u8:?}, maxblocklen: {=u8:?}, extmediabussup: {=bool:?}, adma2sup: {=bool:?}, hssup: {=bool:?}, sdmasup: {=bool:?}, susressup: {=bool:?}, voltsup3p3v: {=bool:?}, voltsup3p0v: {=bool:?}, voltsup1p8v: {=bool:?}, sysbus64bsup: {=bool:?}, asyncintsup: {=bool:?}, ifslottype: {:?} }}" , self . tmoutclkfreq () , self . tmoutclkunit () , self . baseclkfreqsd () , self . maxblocklen () , self . extmediabussup () , self . adma2sup () , self . hssup () , self . sdmasup () , self . susressup () , self . voltsup3p3v () , self . voltsup3p0v () , self . voltsup1p8v () , self . sysbus64bsup () , self . asyncintsup () , self . ifslottype ())
    }
}
#[doc = "Capabilities Register to Hold Bits 63~32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capab2(pub u32);
impl Capab2 {
    #[doc = "SDR50 Support."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50sup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDR50 Support."]
    #[inline(always)]
    pub const fn set_sdr50sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDR104 Support."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104sup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDR104 Support."]
    #[inline(always)]
    pub const fn set_sdr104sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDR50 Support."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50sup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDR50 Support."]
    #[inline(always)]
    pub const fn set_ddr50sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Driver Type a Support."]
    #[must_use]
    #[inline(always)]
    pub const fn drvtypasup(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Driver Type a Support."]
    #[inline(always)]
    pub const fn set_drvtypasup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Driver Type C Support."]
    #[must_use]
    #[inline(always)]
    pub const fn drvtypcsup(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Driver Type C Support."]
    #[inline(always)]
    pub const fn set_drvtypcsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Driver Type D Support."]
    #[must_use]
    #[inline(always)]
    pub const fn drvtypdsup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Driver Type D Support."]
    #[inline(always)]
    pub const fn set_drvtypdsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Count for Re-Tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn timcntretun(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Timer Count for Re-Tuning."]
    #[inline(always)]
    pub const fn set_timcntretun(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Use Tuning for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn usetunsdr50(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Use Tuning for SDR50."]
    #[inline(always)]
    pub const fn set_usetunsdr50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Re-tuning Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn retunemodes(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Re-tuning Modes."]
    #[inline(always)]
    pub const fn set_retunemodes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Clock Multiplier."]
    #[must_use]
    #[inline(always)]
    pub const fn clockkmul(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Multiplier."]
    #[inline(always)]
    pub const fn set_clockkmul(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SPI Mode Support."]
    #[must_use]
    #[inline(always)]
    pub const fn spimode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Mode Support."]
    #[inline(always)]
    pub const fn set_spimode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SPI Block Mode Support."]
    #[must_use]
    #[inline(always)]
    pub const fn spiblockmode(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Block Mode Support."]
    #[inline(always)]
    pub const fn set_spiblockmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Capab2 {
    #[inline(always)]
    fn default() -> Capab2 {
        Capab2(0)
    }
}
impl core::fmt::Debug for Capab2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capab2")
            .field("sdr50sup", &self.sdr50sup())
            .field("sdr104sup", &self.sdr104sup())
            .field("ddr50sup", &self.ddr50sup())
            .field("drvtypasup", &self.drvtypasup())
            .field("drvtypcsup", &self.drvtypcsup())
            .field("drvtypdsup", &self.drvtypdsup())
            .field("timcntretun", &self.timcntretun())
            .field("usetunsdr50", &self.usetunsdr50())
            .field("retunemodes", &self.retunemodes())
            .field("clockkmul", &self.clockkmul())
            .field("spimode", &self.spimode())
            .field("spiblockmode", &self.spiblockmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capab2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Capab2 {{ sdr50sup: {=bool:?}, sdr104sup: {=bool:?}, ddr50sup: {=bool:?}, drvtypasup: {=bool:?}, drvtypcsup: {=bool:?}, drvtypdsup: {=bool:?}, timcntretun: {=u8:?}, usetunsdr50: {=bool:?}, retunemodes: {=u8:?}, clockkmul: {=u8:?}, spimode: {=bool:?}, spiblockmode: {=bool:?} }}" , self . sdr50sup () , self . sdr104sup () , self . ddr50sup () , self . drvtypasup () , self . drvtypcsup () , self . drvtypdsup () , self . timcntretun () , self . usetunsdr50 () , self . retunemodes () , self . clockkmul () , self . spimode () , self . spiblockmode ())
    }
}
#[doc = "Core Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg0(pub u32);
impl Cfg0 {
    #[doc = "Tuning Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tuningcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Tuning Counter Value."]
    #[inline(always)]
    pub const fn set_tuningcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Timeout Clock Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn toutclkfreq(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "Timeout Clock Frequency."]
    #[inline(always)]
    pub const fn set_toutclkfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "Timeout Clock Unit in kHz or MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn toutclkunit(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Clock Unit in kHz or MHz."]
    #[inline(always)]
    pub const fn set_toutclkunit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Base Clock Frequency for SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn baseclkfreq(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0xff;
        val as u8
    }
    #[doc = "Base Clock Frequency for SD_CLK."]
    #[inline(always)]
    pub const fn set_baseclkfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 13usize)) | (((val as u32) & 0xff) << 13usize);
    }
    #[doc = "MAX Block Length of Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn maxblklen(&self) -> super::vals::Maxblklen {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Maxblklen::from_bits(val as u8)
    }
    #[doc = "MAX Block Length of Transfer."]
    #[inline(always)]
    pub const fn set_maxblklen(&mut self, val: super::vals::Maxblklen) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "8-bit Interface Support."]
    #[must_use]
    #[inline(always)]
    pub const fn c8bitsup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "8-bit Interface Support."]
    #[inline(always)]
    pub const fn set_c8bitsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ADMA2 Mode Support."]
    #[must_use]
    #[inline(always)]
    pub const fn cadma2sup(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA2 Mode Support."]
    #[inline(always)]
    pub const fn set_cadma2sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "High Speed Mode Support."]
    #[must_use]
    #[inline(always)]
    pub const fn chssup(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Mode Support."]
    #[inline(always)]
    pub const fn set_chssup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SDMA Mode Support."]
    #[must_use]
    #[inline(always)]
    pub const fn csdmasup(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SDMA Mode Support."]
    #[inline(always)]
    pub const fn set_csdmasup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Suspend/Resume Support."]
    #[must_use]
    #[inline(always)]
    pub const fn csuspressup(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend/Resume Support."]
    #[inline(always)]
    pub const fn set_csuspressup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Core 3P3V Support."]
    #[must_use]
    #[inline(always)]
    pub const fn c3p3vsup(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Core 3P3V Support."]
    #[inline(always)]
    pub const fn set_c3p3vsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "3P0V Support."]
    #[must_use]
    #[inline(always)]
    pub const fn c3p0vsup(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "3P0V Support."]
    #[inline(always)]
    pub const fn set_c3p0vsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "1P8V Support."]
    #[must_use]
    #[inline(always)]
    pub const fn c1p8vsup(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "1P8V Support."]
    #[inline(always)]
    pub const fn set_c1p8vsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Cfg0 {
    #[inline(always)]
    fn default() -> Cfg0 {
        Cfg0(0)
    }
}
impl core::fmt::Debug for Cfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg0")
            .field("tuningcnt", &self.tuningcnt())
            .field("toutclkfreq", &self.toutclkfreq())
            .field("toutclkunit", &self.toutclkunit())
            .field("baseclkfreq", &self.baseclkfreq())
            .field("maxblklen", &self.maxblklen())
            .field("c8bitsup", &self.c8bitsup())
            .field("cadma2sup", &self.cadma2sup())
            .field("chssup", &self.chssup())
            .field("csdmasup", &self.csdmasup())
            .field("csuspressup", &self.csuspressup())
            .field("c3p3vsup", &self.c3p3vsup())
            .field("c3p0vsup", &self.c3p0vsup())
            .field("c1p8vsup", &self.c1p8vsup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg0 {{ tuningcnt: {=u8:?}, toutclkfreq: {=u8:?}, toutclkunit: {=bool:?}, baseclkfreq: {=u8:?}, maxblklen: {:?}, c8bitsup: {=bool:?}, cadma2sup: {=bool:?}, chssup: {=bool:?}, csdmasup: {=bool:?}, csuspressup: {=bool:?}, c3p3vsup: {=bool:?}, c3p0vsup: {=bool:?}, c1p8vsup: {=bool:?} }}" , self . tuningcnt () , self . toutclkfreq () , self . toutclkunit () , self . baseclkfreq () , self . maxblklen () , self . c8bitsup () , self . cadma2sup () , self . chssup () , self . csdmasup () , self . csuspressup () , self . c3p3vsup () , self . c3p0vsup () , self . c1p8vsup ())
    }
}
#[doc = "Core Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
    #[doc = "Asynchronous Interrupt Support."]
    #[must_use]
    #[inline(always)]
    pub const fn asyncintrsup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Interrupt Support."]
    #[inline(always)]
    pub const fn set_asyncintrsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Slot Type."]
    #[must_use]
    #[inline(always)]
    pub const fn slottype(&self) -> super::vals::Slottype {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Slottype::from_bits(val as u8)
    }
    #[doc = "Slot Type."]
    #[inline(always)]
    pub const fn set_slottype(&mut self, val: super::vals::Slottype) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Core Support SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn csdr50sup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Core Support SDR50."]
    #[inline(always)]
    pub const fn set_csdr50sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Support SDR104."]
    #[must_use]
    #[inline(always)]
    pub const fn csdr104sup(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Support SDR104."]
    #[inline(always)]
    pub const fn set_csdr104sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Support DDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn cddr50sup(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Support DDR50."]
    #[inline(always)]
    pub const fn set_cddr50sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Support Type a Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn cdrvasup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Support Type a Driver."]
    #[inline(always)]
    pub const fn set_cdrvasup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Support Type C Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn cdrvcsup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Support Type C Driver."]
    #[inline(always)]
    pub const fn set_cdrvcsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Support Type D Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn cdrvdsup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Support Type D Driver."]
    #[inline(always)]
    pub const fn set_cdrvdsup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Retuning Timer Control."]
    #[must_use]
    #[inline(always)]
    pub const fn retuntmrctl(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Retuning Timer Control."]
    #[inline(always)]
    pub const fn set_retuntmrctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "Tuning for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn tunsdr50(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning for SDR50."]
    #[inline(always)]
    pub const fn set_tunsdr50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Retuning Modes."]
    #[must_use]
    #[inline(always)]
    pub const fn retunmodes(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Retuning Modes."]
    #[inline(always)]
    pub const fn set_retunmodes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "SPI Support."]
    #[must_use]
    #[inline(always)]
    pub const fn spisup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Support."]
    #[inline(always)]
    pub const fn set_spisup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Asynchronous Wakeup Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn asyncwkupen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Wakeup Enable."]
    #[inline(always)]
    pub const fn set_asyncwkupen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Cfg1 {
    #[inline(always)]
    fn default() -> Cfg1 {
        Cfg1(0)
    }
}
impl core::fmt::Debug for Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg1")
            .field("asyncintrsup", &self.asyncintrsup())
            .field("slottype", &self.slottype())
            .field("csdr50sup", &self.csdr50sup())
            .field("csdr104sup", &self.csdr104sup())
            .field("cddr50sup", &self.cddr50sup())
            .field("cdrvasup", &self.cdrvasup())
            .field("cdrvcsup", &self.cdrvcsup())
            .field("cdrvdsup", &self.cdrvdsup())
            .field("retuntmrctl", &self.retuntmrctl())
            .field("tunsdr50", &self.tunsdr50())
            .field("retunmodes", &self.retunmodes())
            .field("spisup", &self.spisup())
            .field("asyncwkupen", &self.asyncwkupen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg1 {{ asyncintrsup: {=bool:?}, slottype: {:?}, csdr50sup: {=bool:?}, csdr104sup: {=bool:?}, cddr50sup: {=bool:?}, cdrvasup: {=bool:?}, cdrvcsup: {=bool:?}, cdrvdsup: {=bool:?}, retuntmrctl: {=u8:?}, tunsdr50: {=bool:?}, retunmodes: {=u8:?}, spisup: {=bool:?}, asyncwkupen: {=bool:?} }}" , self . asyncintrsup () , self . slottype () , self . csdr50sup () , self . csdr104sup () , self . cddr50sup () , self . cdrvasup () , self . cdrvcsup () , self . cdrvdsup () , self . retuntmrctl () , self . tunsdr50 () , self . retunmodes () , self . spisup () , self . asyncwkupen ())
    }
}
#[doc = "Core Configuration Preset Value 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgpresetval0(pub u32);
impl Cfgpresetval0 {
    #[doc = "Initial SD_CLK Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn initsdclkfreq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Initial SD_CLK Frequency."]
    #[inline(always)]
    pub const fn set_initsdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Initial Clock Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn initclkgenen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Initial Clock Gen Enable."]
    #[inline(always)]
    pub const fn set_initclkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Initial Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn initdrvst(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "Initial Drive Strength."]
    #[inline(always)]
    pub const fn set_initdrvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Preset Value for Default Speed of SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn dspsdclkfreq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Preset Value for Default Speed of SD_CLK."]
    #[inline(always)]
    pub const fn set_dspsdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Default Speed Clock Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dspclkgenen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Default Speed Clock Gen Enable."]
    #[inline(always)]
    pub const fn set_dspclkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Default Speed Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn dspdrvst(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "Default Speed Drive Strength."]
    #[inline(always)]
    pub const fn set_dspdrvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Cfgpresetval0 {
    #[inline(always)]
    fn default() -> Cfgpresetval0 {
        Cfgpresetval0(0)
    }
}
impl core::fmt::Debug for Cfgpresetval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgpresetval0")
            .field("initsdclkfreq", &self.initsdclkfreq())
            .field("initclkgenen", &self.initclkgenen())
            .field("initdrvst", &self.initdrvst())
            .field("dspsdclkfreq", &self.dspsdclkfreq())
            .field("dspclkgenen", &self.dspclkgenen())
            .field("dspdrvst", &self.dspdrvst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgpresetval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgpresetval0 {{ initsdclkfreq: {=u16:?}, initclkgenen: {=bool:?}, initdrvst: {=u8:?}, dspsdclkfreq: {=u16:?}, dspclkgenen: {=bool:?}, dspdrvst: {=u8:?} }}" , self . initsdclkfreq () , self . initclkgenen () , self . initdrvst () , self . dspsdclkfreq () , self . dspclkgenen () , self . dspdrvst ())
    }
}
#[doc = "Core Configuration Preset Value 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgpresetval1(pub u32);
impl Cfgpresetval1 {
    #[doc = "High Speed SD_CLK Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn hspsdclkfreq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "High Speed SD_CLK Frequency."]
    #[inline(always)]
    pub const fn set_hspsdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "High Speed SD_CLK Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hspclkgenen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed SD_CLK Gen Enable."]
    #[inline(always)]
    pub const fn set_hspclkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "High Speed SD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn hspdrvst(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "High Speed SD Drive Strength."]
    #[inline(always)]
    pub const fn set_hspdrvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Preset Value for SDR12 Speed of SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12sdclkfreq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Preset Value for SDR12 Speed of SD_CLK."]
    #[inline(always)]
    pub const fn set_sdr12sdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "SDR12 Speed Clock Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12clkgenen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SDR12 Speed Clock Gen Enable."]
    #[inline(always)]
    pub const fn set_sdr12clkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SDR12 Speed Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12drvst(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "SDR12 Speed Drive Strength."]
    #[inline(always)]
    pub const fn set_sdr12drvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Cfgpresetval1 {
    #[inline(always)]
    fn default() -> Cfgpresetval1 {
        Cfgpresetval1(0)
    }
}
impl core::fmt::Debug for Cfgpresetval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgpresetval1")
            .field("hspsdclkfreq", &self.hspsdclkfreq())
            .field("hspclkgenen", &self.hspclkgenen())
            .field("hspdrvst", &self.hspdrvst())
            .field("sdr12sdclkfreq", &self.sdr12sdclkfreq())
            .field("sdr12clkgenen", &self.sdr12clkgenen())
            .field("sdr12drvst", &self.sdr12drvst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgpresetval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgpresetval1 {{ hspsdclkfreq: {=u16:?}, hspclkgenen: {=bool:?}, hspdrvst: {=u8:?}, sdr12sdclkfreq: {=u16:?}, sdr12clkgenen: {=bool:?}, sdr12drvst: {=u8:?} }}" , self . hspsdclkfreq () , self . hspclkgenen () , self . hspdrvst () , self . sdr12sdclkfreq () , self . sdr12clkgenen () , self . sdr12drvst ())
    }
}
#[doc = "Core Configuration Preset Value 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgpresetval2(pub u32);
impl Cfgpresetval2 {
    #[doc = "SDR25 SD_CLK Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25sdclkfreq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SDR25 SD_CLK Frequency."]
    #[inline(always)]
    pub const fn set_sdr25sdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SDR25 SD_CLK Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25clkgenen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SDR25 SD_CLK Gen Enable."]
    #[inline(always)]
    pub const fn set_sdr25clkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SDR25 SD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25drvst(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "SDR25 SD Drive Strength."]
    #[inline(always)]
    pub const fn set_sdr25drvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Preset Value for SDR50 Speed of SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50sdclkfreq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Preset Value for SDR50 Speed of SD_CLK."]
    #[inline(always)]
    pub const fn set_sdr50sdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "SDR50 Speed Clock Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50clkgenen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SDR50 Speed Clock Gen Enable."]
    #[inline(always)]
    pub const fn set_sdr50clkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SDR50 Speed Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50drvst(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "SDR50 Speed Drive Strength."]
    #[inline(always)]
    pub const fn set_sdr50drvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Cfgpresetval2 {
    #[inline(always)]
    fn default() -> Cfgpresetval2 {
        Cfgpresetval2(0)
    }
}
impl core::fmt::Debug for Cfgpresetval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgpresetval2")
            .field("sdr25sdclkfreq", &self.sdr25sdclkfreq())
            .field("sdr25clkgenen", &self.sdr25clkgenen())
            .field("sdr25drvst", &self.sdr25drvst())
            .field("sdr50sdclkfreq", &self.sdr50sdclkfreq())
            .field("sdr50clkgenen", &self.sdr50clkgenen())
            .field("sdr50drvst", &self.sdr50drvst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgpresetval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgpresetval2 {{ sdr25sdclkfreq: {=u16:?}, sdr25clkgenen: {=bool:?}, sdr25drvst: {=u8:?}, sdr50sdclkfreq: {=u16:?}, sdr50clkgenen: {=bool:?}, sdr50drvst: {=u8:?} }}" , self . sdr25sdclkfreq () , self . sdr25clkgenen () , self . sdr25drvst () , self . sdr50sdclkfreq () , self . sdr50clkgenen () , self . sdr50drvst ())
    }
}
#[doc = "Core Configuration Preset Value 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgpresetval3(pub u32);
impl Cfgpresetval3 {
    #[doc = "SDR104 SD_CLK Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104sdclkfreq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SDR104 SD_CLK Frequency."]
    #[inline(always)]
    pub const fn set_sdr104sdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SDR104 SD_CLK Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104clkgenen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SDR104 SD_CLK Gen Enable."]
    #[inline(always)]
    pub const fn set_sdr104clkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SDR104 SD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104drvst(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "SDR104 SD Drive Strength."]
    #[inline(always)]
    pub const fn set_sdr104drvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Preset Value for DDR50 Speed of SD_CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50sdclkfreq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Preset Value for DDR50 Speed of SD_CLK."]
    #[inline(always)]
    pub const fn set_ddr50sdclkfreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "DDR50 Speed Clock Gen Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50clkgenen(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DDR50 Speed Clock Gen Enable."]
    #[inline(always)]
    pub const fn set_ddr50clkgenen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DDR50 Speed Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50drvst(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "DDR50 Speed Drive Strength."]
    #[inline(always)]
    pub const fn set_ddr50drvst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Cfgpresetval3 {
    #[inline(always)]
    fn default() -> Cfgpresetval3 {
        Cfgpresetval3(0)
    }
}
impl core::fmt::Debug for Cfgpresetval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgpresetval3")
            .field("sdr104sdclkfreq", &self.sdr104sdclkfreq())
            .field("sdr104clkgenen", &self.sdr104clkgenen())
            .field("sdr104drvst", &self.sdr104drvst())
            .field("ddr50sdclkfreq", &self.ddr50sdclkfreq())
            .field("ddr50clkgenen", &self.ddr50clkgenen())
            .field("ddr50drvst", &self.ddr50drvst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgpresetval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgpresetval3 {{ sdr104sdclkfreq: {=u16:?}, sdr104clkgenen: {=bool:?}, sdr104drvst: {=u8:?}, ddr50sdclkfreq: {=u16:?}, ddr50clkgenen: {=bool:?}, ddr50drvst: {=u8:?} }}" , self . sdr104sdclkfreq () , self . sdr104clkgenen () , self . sdr104drvst () , self . ddr50sdclkfreq () , self . ddr50clkgenen () , self . ddr50drvst ())
    }
}
#[doc = "Clock Control, Timeout Control and Software Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clockctrl(pub u32);
impl Clockctrl {
    #[doc = "Internal Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn intclken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Clock Enable."]
    #[inline(always)]
    pub const fn set_intclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Internal Clock Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn intclkstable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Clock Stable."]
    #[inline(always)]
    pub const fn set_intclkstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SDIO_CLK Pin Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdclken(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO_CLK Pin Clock Enable."]
    #[inline(always)]
    pub const fn set_sdclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Generator Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clkgensel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select."]
    #[inline(always)]
    pub const fn set_clkgensel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Upper Bits of SD_CLK Frequency Select."]
    #[must_use]
    #[inline(always)]
    pub const fn uppsdclkfre(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Upper Bits of SD_CLK Frequency Select."]
    #[inline(always)]
    pub const fn set_uppsdclkfre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "SD_CLK Frequency Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sdclkfreqsel(&self) -> super::vals::Sdclkfreqsel {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Sdclkfreqsel::from_bits(val as u8)
    }
    #[doc = "SD_CLK Frequency Select."]
    #[inline(always)]
    pub const fn set_sdclkfreqsel(&mut self, val: super::vals::Sdclkfreqsel) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Timeout Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dattoutcntval(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Timeout Counter Value."]
    #[inline(always)]
    pub const fn set_dattoutcntval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Software Reset for All."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrsta(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset for All."]
    #[inline(always)]
    pub const fn set_sftrsta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Software Reset for CMD Line."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrstcmd(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset for CMD Line."]
    #[inline(always)]
    pub const fn set_sftrstcmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Software Reset for DAT Line."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrstdat(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset for DAT Line."]
    #[inline(always)]
    pub const fn set_sftrstdat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Clockctrl {
    #[inline(always)]
    fn default() -> Clockctrl {
        Clockctrl(0)
    }
}
impl core::fmt::Debug for Clockctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clockctrl")
            .field("intclken", &self.intclken())
            .field("intclkstable", &self.intclkstable())
            .field("sdclken", &self.sdclken())
            .field("clkgensel", &self.clkgensel())
            .field("uppsdclkfre", &self.uppsdclkfre())
            .field("sdclkfreqsel", &self.sdclkfreqsel())
            .field("dattoutcntval", &self.dattoutcntval())
            .field("sftrsta", &self.sftrsta())
            .field("sftrstcmd", &self.sftrstcmd())
            .field("sftrstdat", &self.sftrstdat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Clockctrl {{ intclken: {=bool:?}, intclkstable: {=bool:?}, sdclken: {=bool:?}, clkgensel: {=bool:?}, uppsdclkfre: {=u8:?}, sdclkfreqsel: {:?}, dattoutcntval: {=u8:?}, sftrsta: {=bool:?}, sftrstcmd: {=bool:?}, sftrstdat: {=bool:?} }}" , self . intclken () , self . intclkstable () , self . sdclken () , self . clkgensel () , self . uppsdclkfre () , self . sdclkfreqsel () , self . dattoutcntval () , self . sftrsta () , self . sftrstcmd () , self . sftrstdat ())
    }
}
#[doc = "Core Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Selective Tap Delay Line Enable on Rxclk_in."]
    #[must_use]
    #[inline(always)]
    pub const fn itapdlyen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selective Tap Delay Line Enable on Rxclk_in."]
    #[inline(always)]
    pub const fn set_itapdlyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects One of 32 Taps on the Rxclk_in Line."]
    #[must_use]
    #[inline(always)]
    pub const fn itapdlysel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects One of 32 Taps on the Rxclk_in Line."]
    #[inline(always)]
    pub const fn set_itapdlysel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Gating Signal for Tap Delay Change."]
    #[must_use]
    #[inline(always)]
    pub const fn itapchgwin(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Gating Signal for Tap Delay Change."]
    #[inline(always)]
    pub const fn set_itapchgwin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Selective Tap Delay Line Enable on SDIO_CLK Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn otapdlyen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Selective Tap Delay Line Enable on SDIO_CLK Pin."]
    #[inline(always)]
    pub const fn set_otapdlyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects One of 32 Taps on the SDIO_CLK Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn otapdlysel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects One of 32 Taps on the SDIO_CLK Pin."]
    #[inline(always)]
    pub const fn set_otapdlysel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "TX Delay Mux Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn txdlymuxsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "TX Delay Mux Selection."]
    #[inline(always)]
    pub const fn set_txdlymuxsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
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
            .field("itapdlyen", &self.itapdlyen())
            .field("itapdlysel", &self.itapdlysel())
            .field("itapchgwin", &self.itapchgwin())
            .field("otapdlyen", &self.otapdlyen())
            .field("otapdlysel", &self.otapdlysel())
            .field("txdlymuxsel", &self.txdlymuxsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ itapdlyen: {=bool:?}, itapdlysel: {=u8:?}, itapchgwin: {=bool:?}, otapdlyen: {=bool:?}, otapdlysel: {=u8:?}, txdlymuxsel: {=u8:?} }}" , self . itapdlyen () , self . itapdlysel () , self . itapchgwin () , self . otapdlyen () , self . otapdlysel () , self . txdlymuxsel ())
    }
}
#[doc = "Force Event Register for Auto CMD Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fevterrstat(pub u32);
impl Fevterrstat {
    #[doc = "Force Event for Command Not Issued By Auto CM12 Not Executed."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12nex(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command Not Issued By Auto CM12 Not Executed."]
    #[inline(always)]
    pub const fn set_ac12nex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Force Event for Auto CMD Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12toe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Auto CMD Timeout Error."]
    #[inline(always)]
    pub const fn set_ac12toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force Event for Auto CMD CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12crce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Auto CMD CRC Error."]
    #[inline(always)]
    pub const fn set_ac12crce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force Event for Auto CMD End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ebe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Auto CMD End Bit Error."]
    #[inline(always)]
    pub const fn set_ac12ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Force Event for Auto CMD Index Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12indxe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Auto CMD Index Error."]
    #[inline(always)]
    pub const fn set_ac12indxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force Event for Command Not Issued By Auto CMD12 Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cnibac12e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub const fn set_cnibac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Force Event for Command Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtoe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command Timeout Error."]
    #[inline(always)]
    pub const fn set_cmdtoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force Event for Command CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrce(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command CRC Error."]
    #[inline(always)]
    pub const fn set_cmdcrce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Event for Command End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdebe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command End Bit Error."]
    #[inline(always)]
    pub const fn set_cmdebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Event for Command Index Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindxe(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Command Index Error."]
    #[inline(always)]
    pub const fn set_cmdindxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Force Event for Data Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dattoe(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Data Timeout Error."]
    #[inline(always)]
    pub const fn set_dattoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Force Event for Data CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn datcrce(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Data CRC Error."]
    #[inline(always)]
    pub const fn set_datcrce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Force Event for Data End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn datebe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Data End Bit Error."]
    #[inline(always)]
    pub const fn set_datebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Force Event for Current Limit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn curlimite(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Current Limit Error."]
    #[inline(always)]
    pub const fn set_curlimite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Force Event for Auto CMD Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Auto CMD Error."]
    #[inline(always)]
    pub const fn set_ac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Force Event for ADMA Error."]
    #[must_use]
    #[inline(always)]
    pub const fn admae(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for ADMA Error."]
    #[inline(always)]
    pub const fn set_admae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Force Event for Tuning Errro."]
    #[must_use]
    #[inline(always)]
    pub const fn tuninge(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event for Tuning Errro."]
    #[inline(always)]
    pub const fn set_tuninge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Force Event for Vendox Specific Error Status."]
    #[must_use]
    #[inline(always)]
    pub const fn venspece(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Force Event for Vendox Specific Error Status."]
    #[inline(always)]
    pub const fn set_venspece(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Fevterrstat {
    #[inline(always)]
    fn default() -> Fevterrstat {
        Fevterrstat(0)
    }
}
impl core::fmt::Debug for Fevterrstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fevterrstat")
            .field("ac12nex", &self.ac12nex())
            .field("ac12toe", &self.ac12toe())
            .field("ac12crce", &self.ac12crce())
            .field("ac12ebe", &self.ac12ebe())
            .field("ac12indxe", &self.ac12indxe())
            .field("cnibac12e", &self.cnibac12e())
            .field("cmdtoe", &self.cmdtoe())
            .field("cmdcrce", &self.cmdcrce())
            .field("cmdebe", &self.cmdebe())
            .field("cmdindxe", &self.cmdindxe())
            .field("dattoe", &self.dattoe())
            .field("datcrce", &self.datcrce())
            .field("datebe", &self.datebe())
            .field("curlimite", &self.curlimite())
            .field("ac12e", &self.ac12e())
            .field("admae", &self.admae())
            .field("tuninge", &self.tuninge())
            .field("venspece", &self.venspece())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fevterrstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Fevterrstat {{ ac12nex: {=bool:?}, ac12toe: {=bool:?}, ac12crce: {=bool:?}, ac12ebe: {=bool:?}, ac12indxe: {=bool:?}, cnibac12e: {=bool:?}, cmdtoe: {=bool:?}, cmdcrce: {=bool:?}, cmdebe: {=bool:?}, cmdindxe: {=bool:?}, dattoe: {=bool:?}, datcrce: {=bool:?}, datebe: {=bool:?}, curlimite: {=bool:?}, ac12e: {=bool:?}, admae: {=bool:?}, tuninge: {=bool:?}, venspece: {=u8:?} }}" , self . ac12nex () , self . ac12toe () , self . ac12crce () , self . ac12ebe () , self . ac12indxe () , self . cnibac12e () , self . cmdtoe () , self . cmdcrce () , self . cmdebe () , self . cmdindxe () , self . dattoe () , self . datcrce () , self . datebe () , self . curlimite () , self . ac12e () , self . admae () , self . tuninge () , self . venspece ())
    }
}
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hostctrl1(pub u32);
impl Hostctrl1 {
    #[doc = "LED Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ledctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LED Control."]
    #[inline(always)]
    pub const fn set_ledctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data Transfer Width 1-bit or 4-bit Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dattranwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data Transfer Width 1-bit or 4-bit Mode."]
    #[inline(always)]
    pub const fn set_dattranwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "High Speed Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Enable."]
    #[inline(always)]
    pub const fn set_hsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dmasel(&self) -> super::vals::Dmasel {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Dmasel::from_bits(val as u8)
    }
    #[doc = "DMA Select."]
    #[inline(always)]
    pub const fn set_dmasel(&mut self, val: super::vals::Dmasel) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Extended Data Transfer Width."]
    #[must_use]
    #[inline(always)]
    pub const fn extdattranwd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Transfer Width."]
    #[inline(always)]
    pub const fn set_extdattranwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Detect Test Level."]
    #[must_use]
    #[inline(always)]
    pub const fn cdtstlvl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Card Detect Test Level."]
    #[inline(always)]
    pub const fn set_cdtstlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Detetct Signal Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn cdsigdet(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Card Detetct Signal Detection."]
    #[inline(always)]
    pub const fn set_cdsigdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SD Bus Power."]
    #[must_use]
    #[inline(always)]
    pub const fn sdbuspower(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SD Bus Power."]
    #[inline(always)]
    pub const fn set_sdbuspower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SD Bus Voltage Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sdbusvoltsel(&self) -> super::vals::Sdbusvoltsel {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Sdbusvoltsel::from_bits(val as u8)
    }
    #[doc = "SD Bus Voltage Select."]
    #[inline(always)]
    pub const fn set_sdbusvoltsel(&mut self, val: super::vals::Sdbusvoltsel) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Hardware Reset Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn hrdrst(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Reset Signal."]
    #[inline(always)]
    pub const fn set_hrdrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Stop at Block Gap Request."]
    #[must_use]
    #[inline(always)]
    pub const fn stopatblkgapreq(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Stop at Block Gap Request."]
    #[inline(always)]
    pub const fn set_stopatblkgapreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Continue Request."]
    #[must_use]
    #[inline(always)]
    pub const fn continuereq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Continue Request."]
    #[inline(always)]
    pub const fn set_continuereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Read Wait Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rdwaitctrl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Read Wait Control."]
    #[inline(always)]
    pub const fn set_rdwaitctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt at Block Gap."]
    #[must_use]
    #[inline(always)]
    pub const fn intatblkgap(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt at Block Gap."]
    #[inline(always)]
    pub const fn set_intatblkgap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SPI Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spimode(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Mode Enable."]
    #[inline(always)]
    pub const fn set_spimode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Boot Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn booten(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Enable."]
    #[inline(always)]
    pub const fn set_booten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Alternate Boot Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altbooten(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Boot Enable."]
    #[inline(always)]
    pub const fn set_altbooten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Boot Ack Check."]
    #[must_use]
    #[inline(always)]
    pub const fn bootackchk(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Ack Check."]
    #[inline(always)]
    pub const fn set_bootackchk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Wakeup Event Enable on Card Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupevntenoncardint(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Event Enable on Card Interrupt."]
    #[inline(always)]
    pub const fn set_wkupevntenoncardint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Wakeup Event Enable on SD Card Insertion."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupevntenoncins(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Event Enable on SD Card Insertion."]
    #[inline(always)]
    pub const fn set_wkupevntenoncins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Wakeup Event Enable on SD Card Removal."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupevntenoncrm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Event Enable on SD Card Removal."]
    #[inline(always)]
    pub const fn set_wkupevntenoncrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Hostctrl1 {
    #[inline(always)]
    fn default() -> Hostctrl1 {
        Hostctrl1(0)
    }
}
impl core::fmt::Debug for Hostctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hostctrl1")
            .field("ledctrl", &self.ledctrl())
            .field("dattranwd", &self.dattranwd())
            .field("hsen", &self.hsen())
            .field("dmasel", &self.dmasel())
            .field("extdattranwd", &self.extdattranwd())
            .field("cdtstlvl", &self.cdtstlvl())
            .field("cdsigdet", &self.cdsigdet())
            .field("sdbuspower", &self.sdbuspower())
            .field("sdbusvoltsel", &self.sdbusvoltsel())
            .field("hrdrst", &self.hrdrst())
            .field("stopatblkgapreq", &self.stopatblkgapreq())
            .field("continuereq", &self.continuereq())
            .field("rdwaitctrl", &self.rdwaitctrl())
            .field("intatblkgap", &self.intatblkgap())
            .field("spimode", &self.spimode())
            .field("booten", &self.booten())
            .field("altbooten", &self.altbooten())
            .field("bootackchk", &self.bootackchk())
            .field("wkupevntenoncardint", &self.wkupevntenoncardint())
            .field("wkupevntenoncins", &self.wkupevntenoncins())
            .field("wkupevntenoncrm", &self.wkupevntenoncrm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hostctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hostctrl1 {{ ledctrl: {=bool:?}, dattranwd: {=bool:?}, hsen: {=bool:?}, dmasel: {:?}, extdattranwd: {=bool:?}, cdtstlvl: {=bool:?}, cdsigdet: {=bool:?}, sdbuspower: {=bool:?}, sdbusvoltsel: {:?}, hrdrst: {=bool:?}, stopatblkgapreq: {=bool:?}, continuereq: {=bool:?}, rdwaitctrl: {=bool:?}, intatblkgap: {=bool:?}, spimode: {=bool:?}, booten: {=bool:?}, altbooten: {=bool:?}, bootackchk: {=bool:?}, wkupevntenoncardint: {=bool:?}, wkupevntenoncins: {=bool:?}, wkupevntenoncrm: {=bool:?} }}" , self . ledctrl () , self . dattranwd () , self . hsen () , self . dmasel () , self . extdattranwd () , self . cdtstlvl () , self . cdsigdet () , self . sdbuspower () , self . sdbusvoltsel () , self . hrdrst () , self . stopatblkgapreq () , self . continuereq () , self . rdwaitctrl () , self . intatblkgap () , self . spimode () , self . booten () , self . altbooten () , self . bootackchk () , self . wkupevntenoncardint () , self . wkupevntenoncins () , self . wkupevntenoncrm ())
    }
}
#[doc = "Normal and Error Interrupt Signal Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc = "Command Complete Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcomsen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdcomsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn trancomsen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Signal Enable."]
    #[inline(always)]
    pub const fn set_trancomsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn blkgapevtsen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Block Gap Event Signal Enable."]
    #[inline(always)]
    pub const fn set_blkgapevtsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaintsen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_dmaintsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufwrrdysen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Write Ready Signal Enable."]
    #[inline(always)]
    pub const fn set_bufwrrdysen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufrdrdysen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Read Ready Signal Enable."]
    #[inline(always)]
    pub const fn set_bufrdrdysen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardinssen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Card Insertion Signal Enable."]
    #[inline(always)]
    pub const fn set_cardinssen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardremsen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Card Removal Signal Enable."]
    #[inline(always)]
    pub const fn set_cardremsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardintsen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Card Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_cardintsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tuning Event Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn retuningevtsen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Re-Tuning Event Signal Enable."]
    #[inline(always)]
    pub const fn set_retuningevtsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Boot Ack Received Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bootackrcvsen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Ack Received Signal Enable."]
    #[inline(always)]
    pub const fn set_bootackrcvsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Boot Terminate Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bootterminatesen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Terminate Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_bootterminatesen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command Timeout Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtouterrsen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Command Timeout Error Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdtouterrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrcerrsen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Command CRC Error Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdcrcerrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdendbiterrsen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Command End Bit Error Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdendbiterrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindexerrsen(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Command Index Error Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdindexerrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Timeout Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dattouterrsen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Data Timeout Error Signal Enable."]
    #[inline(always)]
    pub const fn set_dattouterrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datcrcerrsen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC Error Signal Enable."]
    #[inline(always)]
    pub const fn set_datcrcerrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datendbiterrsen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data End Bit Error Signal Enable."]
    #[inline(always)]
    pub const fn set_datendbiterrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Current Limit Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn currentlimiterrsen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Current Limit Error Signal Enable."]
    #[inline(always)]
    pub const fn set_currentlimiterrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto CMD12 Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autocmderrsen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD12 Error Signal Enable."]
    #[inline(always)]
    pub const fn set_autocmderrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADMA Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn admaerrsen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA Error Signal Enable."]
    #[inline(always)]
    pub const fn set_admaerrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Tuning Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tuningerrsignalenable(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning Error Signal Enable."]
    #[inline(always)]
    pub const fn set_tuningerrsignalenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Target Response Error Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn targetresperrsen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Target Response Error Signal Enable."]
    #[inline(always)]
    pub const fn set_targetresperrsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("cmdcomsen", &self.cmdcomsen())
            .field("trancomsen", &self.trancomsen())
            .field("blkgapevtsen", &self.blkgapevtsen())
            .field("dmaintsen", &self.dmaintsen())
            .field("bufwrrdysen", &self.bufwrrdysen())
            .field("bufrdrdysen", &self.bufrdrdysen())
            .field("cardinssen", &self.cardinssen())
            .field("cardremsen", &self.cardremsen())
            .field("cardintsen", &self.cardintsen())
            .field("retuningevtsen", &self.retuningevtsen())
            .field("bootackrcvsen", &self.bootackrcvsen())
            .field("bootterminatesen", &self.bootterminatesen())
            .field("cmdtouterrsen", &self.cmdtouterrsen())
            .field("cmdcrcerrsen", &self.cmdcrcerrsen())
            .field("cmdendbiterrsen", &self.cmdendbiterrsen())
            .field("cmdindexerrsen", &self.cmdindexerrsen())
            .field("dattouterrsen", &self.dattouterrsen())
            .field("datcrcerrsen", &self.datcrcerrsen())
            .field("datendbiterrsen", &self.datendbiterrsen())
            .field("currentlimiterrsen", &self.currentlimiterrsen())
            .field("autocmderrsen", &self.autocmderrsen())
            .field("admaerrsen", &self.admaerrsen())
            .field("tuningerrsignalenable", &self.tuningerrsignalenable())
            .field("targetresperrsen", &self.targetresperrsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ien {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ien {{ cmdcomsen: {=bool:?}, trancomsen: {=bool:?}, blkgapevtsen: {=bool:?}, dmaintsen: {=bool:?}, bufwrrdysen: {=bool:?}, bufrdrdysen: {=bool:?}, cardinssen: {=bool:?}, cardremsen: {=bool:?}, cardintsen: {=bool:?}, retuningevtsen: {=bool:?}, bootackrcvsen: {=bool:?}, bootterminatesen: {=bool:?}, cmdtouterrsen: {=bool:?}, cmdcrcerrsen: {=bool:?}, cmdendbiterrsen: {=bool:?}, cmdindexerrsen: {=bool:?}, dattouterrsen: {=bool:?}, datcrcerrsen: {=bool:?}, datendbiterrsen: {=bool:?}, currentlimiterrsen: {=bool:?}, autocmderrsen: {=bool:?}, admaerrsen: {=bool:?}, tuningerrsignalenable: {=bool:?}, targetresperrsen: {=bool:?} }}" , self . cmdcomsen () , self . trancomsen () , self . blkgapevtsen () , self . dmaintsen () , self . bufwrrdysen () , self . bufrdrdysen () , self . cardinssen () , self . cardremsen () , self . cardintsen () , self . retuningevtsen () , self . bootackrcvsen () , self . bootterminatesen () , self . cmdtouterrsen () , self . cmdcrcerrsen () , self . cmdendbiterrsen () , self . cmdindexerrsen () , self . dattouterrsen () , self . datcrcerrsen () , self . datendbiterrsen () , self . currentlimiterrsen () , self . autocmderrsen () , self . admaerrsen () , self . tuningerrsignalenable () , self . targetresperrsen ())
    }
}
#[doc = "Normal and Error Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "Command Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcom(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete."]
    #[inline(always)]
    pub const fn set_cmdcom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn trancom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete."]
    #[inline(always)]
    pub const fn set_trancom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event."]
    #[must_use]
    #[inline(always)]
    pub const fn blkgapevt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Block Gap Event."]
    #[inline(always)]
    pub const fn set_blkgapevt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Interrupt."]
    #[inline(always)]
    pub const fn set_dmaint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn bfrwrrdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Write Ready."]
    #[inline(always)]
    pub const fn set_bfrwrrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn bfrrdrdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Read Ready."]
    #[inline(always)]
    pub const fn set_bfrrdrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion."]
    #[must_use]
    #[inline(always)]
    pub const fn cardins(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Card Insertion."]
    #[inline(always)]
    pub const fn set_cardins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal."]
    #[must_use]
    #[inline(always)]
    pub const fn cardrm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Card Removal."]
    #[inline(always)]
    pub const fn set_cardrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cardint(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Card Interrupt."]
    #[inline(always)]
    pub const fn set_cardint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tunning Event."]
    #[must_use]
    #[inline(always)]
    pub const fn retuningevt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Re-Tunning Event."]
    #[inline(always)]
    pub const fn set_retuningevt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Boot Ack Received."]
    #[must_use]
    #[inline(always)]
    pub const fn bootackrcv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Ack Received."]
    #[inline(always)]
    pub const fn set_bootackrcv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Boot Terminate Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bootterminate(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Terminate Interrupt."]
    #[inline(always)]
    pub const fn set_bootterminate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn errint(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt."]
    #[inline(always)]
    pub const fn set_errint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Command Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtouterr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Command Timeout Error."]
    #[inline(always)]
    pub const fn set_cmdtouterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CMD CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrcerr(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CMD CRC Error."]
    #[inline(always)]
    pub const fn set_cmdcrcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdendbiterr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Command End Bit Error."]
    #[inline(always)]
    pub const fn set_cmdendbiterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindexerr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Command Index Error."]
    #[inline(always)]
    pub const fn set_cmdindexerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Time-out Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dattouterr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Data Time-out Error."]
    #[inline(always)]
    pub const fn set_dattouterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn datcrcerr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC Error."]
    #[inline(always)]
    pub const fn set_datcrcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn datendbiterr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data End Bit Error."]
    #[inline(always)]
    pub const fn set_datendbiterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Current Limit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn currentlimiterr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Current Limit Error."]
    #[inline(always)]
    pub const fn set_currentlimiterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto CMD Error."]
    #[must_use]
    #[inline(always)]
    pub const fn autocmderr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD Error."]
    #[inline(always)]
    pub const fn set_autocmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADMA Error."]
    #[must_use]
    #[inline(always)]
    pub const fn admaerr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA Error."]
    #[inline(always)]
    pub const fn set_admaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Specific Error STAT."]
    #[must_use]
    #[inline(always)]
    pub const fn targetresp(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Specific Error STAT."]
    #[inline(always)]
    pub const fn set_targetresp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Ifcr {
    #[inline(always)]
    fn default() -> Ifcr {
        Ifcr(0)
    }
}
impl core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifcr")
            .field("cmdcom", &self.cmdcom())
            .field("trancom", &self.trancom())
            .field("blkgapevt", &self.blkgapevt())
            .field("dmaint", &self.dmaint())
            .field("bfrwrrdy", &self.bfrwrrdy())
            .field("bfrrdrdy", &self.bfrrdrdy())
            .field("cardins", &self.cardins())
            .field("cardrm", &self.cardrm())
            .field("cardint", &self.cardint())
            .field("retuningevt", &self.retuningevt())
            .field("bootackrcv", &self.bootackrcv())
            .field("bootterminate", &self.bootterminate())
            .field("errint", &self.errint())
            .field("cmdtouterr", &self.cmdtouterr())
            .field("cmdcrcerr", &self.cmdcrcerr())
            .field("cmdendbiterr", &self.cmdendbiterr())
            .field("cmdindexerr", &self.cmdindexerr())
            .field("dattouterr", &self.dattouterr())
            .field("datcrcerr", &self.datcrcerr())
            .field("datendbiterr", &self.datendbiterr())
            .field("currentlimiterr", &self.currentlimiterr())
            .field("autocmderr", &self.autocmderr())
            .field("admaerr", &self.admaerr())
            .field("targetresp", &self.targetresp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifcr {{ cmdcom: {=bool:?}, trancom: {=bool:?}, blkgapevt: {=bool:?}, dmaint: {=bool:?}, bfrwrrdy: {=bool:?}, bfrrdrdy: {=bool:?}, cardins: {=bool:?}, cardrm: {=bool:?}, cardint: {=bool:?}, retuningevt: {=bool:?}, bootackrcv: {=bool:?}, bootterminate: {=bool:?}, errint: {=bool:?}, cmdtouterr: {=bool:?}, cmdcrcerr: {=bool:?}, cmdendbiterr: {=bool:?}, cmdindexerr: {=bool:?}, dattouterr: {=bool:?}, datcrcerr: {=bool:?}, datendbiterr: {=bool:?}, currentlimiterr: {=bool:?}, autocmderr: {=bool:?}, admaerr: {=bool:?}, targetresp: {=bool:?} }}" , self . cmdcom () , self . trancom () , self . blkgapevt () , self . dmaint () , self . bfrwrrdy () , self . bfrrdrdy () , self . cardins () , self . cardrm () , self . cardint () , self . retuningevt () , self . bootackrcv () , self . bootterminate () , self . errint () , self . cmdtouterr () , self . cmdcrcerr () , self . cmdendbiterr () , self . cmdindexerr () , self . dattouterr () , self . datcrcerr () , self . datendbiterr () , self . currentlimiterr () , self . autocmderr () , self . admaerr () , self . targetresp ())
    }
}
#[doc = "Normal and Error Interrupt Status Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifenc(pub u32);
impl Ifenc {
    #[doc = "Command Complete Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcomen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete Signal Enable."]
    #[inline(always)]
    pub const fn set_cmdcomen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn trancomen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Signal Enable."]
    #[inline(always)]
    pub const fn set_trancomen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn blkgapevten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Block Gap Event Signal Enable."]
    #[inline(always)]
    pub const fn set_blkgapevten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmainten(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_dmainten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufwrrdyen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Write Ready Signal Enable."]
    #[inline(always)]
    pub const fn set_bufwrrdyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufrdrdyen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Read Ready Signal Enable."]
    #[inline(always)]
    pub const fn set_bufrdrdyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardinsen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Card Insertion Signal Enable."]
    #[inline(always)]
    pub const fn set_cardinsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardrmen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Card Removal Signal Enable."]
    #[inline(always)]
    pub const fn set_cardrmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cardinten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Card Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_cardinten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tunning Event Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn retuningevten(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Re-Tunning Event Signal Enable."]
    #[inline(always)]
    pub const fn set_retuningevten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Boot Ack Received Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bootackrcven(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Ack Received Signal Enable."]
    #[inline(always)]
    pub const fn set_bootackrcven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Boot Terminate Interrupt Signal Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bootterminateen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Terminate Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn set_bootterminateen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command Time-out Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtouterren(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Command Time-out Error Status Enable."]
    #[inline(always)]
    pub const fn set_cmdtouterren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrcerren(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Command CRC Error Status Enable."]
    #[inline(always)]
    pub const fn set_cmdcrcerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdendbiterren(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Command End Bit Error Status Enable."]
    #[inline(always)]
    pub const fn set_cmdendbiterren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindexerren(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Command Index Error Status Enable."]
    #[inline(always)]
    pub const fn set_cmdindexerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Timeout Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dattouterren(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Data Timeout Error Status Enable."]
    #[inline(always)]
    pub const fn set_dattouterren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datcrcerren(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC Error Status Enable."]
    #[inline(always)]
    pub const fn set_datcrcerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datendbiterren(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data End Bit Error Status Enable."]
    #[inline(always)]
    pub const fn set_datendbiterren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Current Limit Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn currentlimiterren(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Current Limit Error Status Enable."]
    #[inline(always)]
    pub const fn set_currentlimiterren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto CMD12 Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autocmderren(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD12 Error Status Enable."]
    #[inline(always)]
    pub const fn set_autocmderren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADMA Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn admaerren(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADMA Error Status Enable."]
    #[inline(always)]
    pub const fn set_admaerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Tuning Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tuningerren(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning Error Status Enable."]
    #[inline(always)]
    pub const fn set_tuningerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Target Response/Host Error Status Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn targetrespen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Target Response/Host Error Status Enable."]
    #[inline(always)]
    pub const fn set_targetrespen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Ifenc {
    #[inline(always)]
    fn default() -> Ifenc {
        Ifenc(0)
    }
}
impl core::fmt::Debug for Ifenc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifenc")
            .field("cmdcomen", &self.cmdcomen())
            .field("trancomen", &self.trancomen())
            .field("blkgapevten", &self.blkgapevten())
            .field("dmainten", &self.dmainten())
            .field("bufwrrdyen", &self.bufwrrdyen())
            .field("bufrdrdyen", &self.bufrdrdyen())
            .field("cardinsen", &self.cardinsen())
            .field("cardrmen", &self.cardrmen())
            .field("cardinten", &self.cardinten())
            .field("retuningevten", &self.retuningevten())
            .field("bootackrcven", &self.bootackrcven())
            .field("bootterminateen", &self.bootterminateen())
            .field("cmdtouterren", &self.cmdtouterren())
            .field("cmdcrcerren", &self.cmdcrcerren())
            .field("cmdendbiterren", &self.cmdendbiterren())
            .field("cmdindexerren", &self.cmdindexerren())
            .field("dattouterren", &self.dattouterren())
            .field("datcrcerren", &self.datcrcerren())
            .field("datendbiterren", &self.datendbiterren())
            .field("currentlimiterren", &self.currentlimiterren())
            .field("autocmderren", &self.autocmderren())
            .field("admaerren", &self.admaerren())
            .field("tuningerren", &self.tuningerren())
            .field("targetrespen", &self.targetrespen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifenc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifenc {{ cmdcomen: {=bool:?}, trancomen: {=bool:?}, blkgapevten: {=bool:?}, dmainten: {=bool:?}, bufwrrdyen: {=bool:?}, bufrdrdyen: {=bool:?}, cardinsen: {=bool:?}, cardrmen: {=bool:?}, cardinten: {=bool:?}, retuningevten: {=bool:?}, bootackrcven: {=bool:?}, bootterminateen: {=bool:?}, cmdtouterren: {=bool:?}, cmdcrcerren: {=bool:?}, cmdendbiterren: {=bool:?}, cmdindexerren: {=bool:?}, dattouterren: {=bool:?}, datcrcerren: {=bool:?}, datendbiterren: {=bool:?}, currentlimiterren: {=bool:?}, autocmderren: {=bool:?}, admaerren: {=bool:?}, tuningerren: {=bool:?}, targetrespen: {=bool:?} }}" , self . cmdcomen () , self . trancomen () , self . blkgapevten () , self . dmainten () , self . bufwrrdyen () , self . bufrdrdyen () , self . cardinsen () , self . cardrmen () , self . cardinten () , self . retuningevten () , self . bootackrcven () , self . bootterminateen () , self . cmdtouterren () , self . cmdcrcerren () , self . cmdendbiterren () , self . cmdindexerren () , self . dattouterren () , self . datcrcerren () , self . datendbiterren () , self . currentlimiterren () , self . autocmderren () , self . admaerren () , self . tuningerren () , self . targetrespen ())
    }
}
#[doc = "Maximum Current Capabilities Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcurcapab(pub u32);
impl Maxcurcapab {
    #[doc = "Maximum Current for 3.3V."]
    #[must_use]
    #[inline(always)]
    pub const fn maxcur3p3val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum Current for 3.3V."]
    #[inline(always)]
    pub const fn set_maxcur3p3val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Maximum Current for 3.0V."]
    #[must_use]
    #[inline(always)]
    pub const fn maxcur3p0val(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum Current for 3.0V."]
    #[inline(always)]
    pub const fn set_maxcur3p0val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Maximum Current for 1.8V."]
    #[must_use]
    #[inline(always)]
    pub const fn maxcur1p8val(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum Current for 1.8V."]
    #[inline(always)]
    pub const fn set_maxcur1p8val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Maxcurcapab {
    #[inline(always)]
    fn default() -> Maxcurcapab {
        Maxcurcapab(0)
    }
}
impl core::fmt::Debug for Maxcurcapab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Maxcurcapab")
            .field("maxcur3p3val", &self.maxcur3p3val())
            .field("maxcur3p0val", &self.maxcur3p0val())
            .field("maxcur1p8val", &self.maxcur1p8val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Maxcurcapab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Maxcurcapab {{ maxcur3p3val: {=u8:?}, maxcur3p0val: {=u8:?}, maxcur1p8val: {=u8:?} }}",
            self.maxcur3p3val(),
            self.maxcur3p0val(),
            self.maxcur1p8val()
        )
    }
}
#[doc = "Present State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsstat(pub u32);
impl Prsstat {
    #[doc = "Command Inhibit (CMD)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdinhibitcmd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Inhibit (CMD)."]
    #[inline(always)]
    pub const fn set_cmdinhibitcmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Inhibit (DAT)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdinhibitdat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command Inhibit (DAT)."]
    #[inline(always)]
    pub const fn set_cmdinhibitdat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAT Line Active."]
    #[must_use]
    #[inline(always)]
    pub const fn datlineactive(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DAT Line Active."]
    #[inline(always)]
    pub const fn set_datlineactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Re-Tuning Request."]
    #[must_use]
    #[inline(always)]
    pub const fn retuningreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Re-Tuning Request."]
    #[inline(always)]
    pub const fn set_retuningreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write Transfer Active."]
    #[must_use]
    #[inline(always)]
    pub const fn wrtranact(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write Transfer Active."]
    #[inline(always)]
    pub const fn set_wrtranact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read Transfer Active."]
    #[must_use]
    #[inline(always)]
    pub const fn rdtranact(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read Transfer Active."]
    #[inline(always)]
    pub const fn set_rdtranact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Buffer Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufferwriteenable(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Write Enable."]
    #[inline(always)]
    pub const fn set_bufferwriteenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Buffer Read Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufrden(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Read Enable."]
    #[inline(always)]
    pub const fn set_bufrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Card Inserted Status."]
    #[must_use]
    #[inline(always)]
    pub const fn cardins(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Card Inserted Status."]
    #[inline(always)]
    pub const fn set_cardins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Card State Stable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn cardstatestable(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Card State Stable Status."]
    #[inline(always)]
    pub const fn set_cardstatestable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Card Detect Pin Level."]
    #[must_use]
    #[inline(always)]
    pub const fn carddetpinlvl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Card Detect Pin Level."]
    #[inline(always)]
    pub const fn set_carddetpinlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write Protect Switch Pin Level."]
    #[must_use]
    #[inline(always)]
    pub const fn wrprotswpinlvl(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protect Switch Pin Level."]
    #[inline(always)]
    pub const fn set_wrprotswpinlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DAT\\[3:0\\] Line Signal Level."]
    #[must_use]
    #[inline(always)]
    pub const fn dat3to0siglvl(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "DAT\\[3:0\\] Line Signal Level."]
    #[inline(always)]
    pub const fn set_dat3to0siglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Line Signal Level."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsiglvl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Command Line Signal Level."]
    #[inline(always)]
    pub const fn set_cmdsiglvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DAT\\[7:4\\] Line Signal Level."]
    #[must_use]
    #[inline(always)]
    pub const fn dat7to4siglvl(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "DAT\\[7:4\\] Line Signal Level."]
    #[inline(always)]
    pub const fn set_dat7to4siglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
}
impl Default for Prsstat {
    #[inline(always)]
    fn default() -> Prsstat {
        Prsstat(0)
    }
}
impl core::fmt::Debug for Prsstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prsstat")
            .field("cmdinhibitcmd", &self.cmdinhibitcmd())
            .field("cmdinhibitdat", &self.cmdinhibitdat())
            .field("datlineactive", &self.datlineactive())
            .field("retuningreq", &self.retuningreq())
            .field("wrtranact", &self.wrtranact())
            .field("rdtranact", &self.rdtranact())
            .field("bufferwriteenable", &self.bufferwriteenable())
            .field("bufrden", &self.bufrden())
            .field("cardins", &self.cardins())
            .field("cardstatestable", &self.cardstatestable())
            .field("carddetpinlvl", &self.carddetpinlvl())
            .field("wrprotswpinlvl", &self.wrprotswpinlvl())
            .field("dat3to0siglvl", &self.dat3to0siglvl())
            .field("cmdsiglvl", &self.cmdsiglvl())
            .field("dat7to4siglvl", &self.dat7to4siglvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prsstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prsstat {{ cmdinhibitcmd: {=bool:?}, cmdinhibitdat: {=bool:?}, datlineactive: {=bool:?}, retuningreq: {=bool:?}, wrtranact: {=bool:?}, rdtranact: {=bool:?}, bufferwriteenable: {=bool:?}, bufrden: {=bool:?}, cardins: {=bool:?}, cardstatestable: {=bool:?}, carddetpinlvl: {=bool:?}, wrprotswpinlvl: {=bool:?}, dat3to0siglvl: {=u8:?}, cmdsiglvl: {=bool:?}, dat7to4siglvl: {=u8:?} }}" , self . cmdinhibitcmd () , self . cmdinhibitdat () , self . datlineactive () , self . retuningreq () , self . wrtranact () , self . rdtranact () , self . bufferwriteenable () , self . bufrden () , self . cardins () , self . cardstatestable () , self . carddetpinlvl () , self . wrprotswpinlvl () , self . dat3to0siglvl () , self . cmdsiglvl () , self . dat7to4siglvl ())
    }
}
#[doc = "Preset Value for Initialization and Default Speed Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstval0(pub u32);
impl Prstval0 {
    #[doc = "SD_CLK Frequency Select Value for Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn initsdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for Initialization."]
    #[inline(always)]
    pub const fn set_initsdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Clock Generator Select Value for Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn initclckgenval(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for Initialization."]
    #[inline(always)]
    pub const fn set_initclckgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Driver Strength Select Value for Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn initdrvstval(&self) -> super::vals::Initdrvstval {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Initdrvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for Initialization."]
    #[inline(always)]
    pub const fn set_initdrvstval(&mut self, val: super::vals::Initdrvstval) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "SD_CLK Frequency Select Value for Default Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn dspsdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for Default Speed."]
    #[inline(always)]
    pub const fn set_dspsdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Clock Generator Select Value for Default Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn dspclkgenval(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for Default Speed."]
    #[inline(always)]
    pub const fn set_dspclkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Driver Strength Select Value for Default Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn dspdrvstval(&self) -> super::vals::Dspdrvstval {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Dspdrvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for Default Speed."]
    #[inline(always)]
    pub const fn set_dspdrvstval(&mut self, val: super::vals::Dspdrvstval) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Prstval0 {
    #[inline(always)]
    fn default() -> Prstval0 {
        Prstval0(0)
    }
}
impl core::fmt::Debug for Prstval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstval0")
            .field("initsdclkfreqval", &self.initsdclkfreqval())
            .field("initclckgenval", &self.initclckgenval())
            .field("initdrvstval", &self.initdrvstval())
            .field("dspsdclkfreqval", &self.dspsdclkfreqval())
            .field("dspclkgenval", &self.dspclkgenval())
            .field("dspdrvstval", &self.dspdrvstval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prstval0 {{ initsdclkfreqval: {=u16:?}, initclckgenval: {=bool:?}, initdrvstval: {:?}, dspsdclkfreqval: {=u16:?}, dspclkgenval: {=bool:?}, dspdrvstval: {:?} }}" , self . initsdclkfreqval () , self . initclckgenval () , self . initdrvstval () , self . dspsdclkfreqval () , self . dspclkgenval () , self . dspdrvstval ())
    }
}
#[doc = "Preset Value for High Speed and SDR12 Modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstval2(pub u32);
impl Prstval2 {
    #[doc = "SD_CLK Frequency Select Value for High Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn hspsdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for High Speed."]
    #[inline(always)]
    pub const fn set_hspsdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Clock Generator Select Value for High Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn hspclkgenval(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for High Speed."]
    #[inline(always)]
    pub const fn set_hspclkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Driver Strength Select Value for High Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn hspdrvstval(&self) -> super::vals::Hspdrvstval {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Hspdrvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for High Speed."]
    #[inline(always)]
    pub const fn set_hspdrvstval(&mut self, val: super::vals::Hspdrvstval) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "SD_CLK Frequency Select Value for SDR12."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12sdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for SDR12."]
    #[inline(always)]
    pub const fn set_sdr12sdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Clock Generator Select Value for SDR12."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12clkgenval(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for SDR12."]
    #[inline(always)]
    pub const fn set_sdr12clkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Driver Strength Select Value for SDR12."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr12drvstval(&self) -> super::vals::Sdr12drvstval {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Sdr12drvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for SDR12."]
    #[inline(always)]
    pub const fn set_sdr12drvstval(&mut self, val: super::vals::Sdr12drvstval) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Prstval2 {
    #[inline(always)]
    fn default() -> Prstval2 {
        Prstval2(0)
    }
}
impl core::fmt::Debug for Prstval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstval2")
            .field("hspsdclkfreqval", &self.hspsdclkfreqval())
            .field("hspclkgenval", &self.hspclkgenval())
            .field("hspdrvstval", &self.hspdrvstval())
            .field("sdr12sdclkfreqval", &self.sdr12sdclkfreqval())
            .field("sdr12clkgenval", &self.sdr12clkgenval())
            .field("sdr12drvstval", &self.sdr12drvstval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prstval2 {{ hspsdclkfreqval: {=u16:?}, hspclkgenval: {=bool:?}, hspdrvstval: {:?}, sdr12sdclkfreqval: {=u16:?}, sdr12clkgenval: {=bool:?}, sdr12drvstval: {:?} }}" , self . hspsdclkfreqval () , self . hspclkgenval () , self . hspdrvstval () , self . sdr12sdclkfreqval () , self . sdr12clkgenval () , self . sdr12drvstval ())
    }
}
#[doc = "Preset Value for SDR25 and SDR50 Modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstval4(pub u32);
impl Prstval4 {
    #[doc = "SD_CLK Frequency Select Value for SDR25."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25sdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for SDR25."]
    #[inline(always)]
    pub const fn set_sdr25sdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Clock Generator Select Value for SDR25."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25clkgenval(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for SDR25."]
    #[inline(always)]
    pub const fn set_sdr25clkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Driver Strength Select Value for SDR25."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr25drvstval(&self) -> super::vals::Sdr25drvstval {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sdr25drvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for SDR25."]
    #[inline(always)]
    pub const fn set_sdr25drvstval(&mut self, val: super::vals::Sdr25drvstval) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "SD_CLK Frequency Select Value for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50sdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for SDR50."]
    #[inline(always)]
    pub const fn set_sdr50sdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Clock Generator Select Value for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50clckgenval(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for SDR50."]
    #[inline(always)]
    pub const fn set_sdr50clckgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Driver Strength Select Value for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50drvstval(&self) -> super::vals::Sdr50drvstval {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Sdr50drvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for SDR50."]
    #[inline(always)]
    pub const fn set_sdr50drvstval(&mut self, val: super::vals::Sdr50drvstval) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Prstval4 {
    #[inline(always)]
    fn default() -> Prstval4 {
        Prstval4(0)
    }
}
impl core::fmt::Debug for Prstval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstval4")
            .field("sdr25sdclkfreqval", &self.sdr25sdclkfreqval())
            .field("sdr25clkgenval", &self.sdr25clkgenval())
            .field("sdr25drvstval", &self.sdr25drvstval())
            .field("sdr50sdclkfreqval", &self.sdr50sdclkfreqval())
            .field("sdr50clckgenval", &self.sdr50clckgenval())
            .field("sdr50drvstval", &self.sdr50drvstval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prstval4 {{ sdr25sdclkfreqval: {=u16:?}, sdr25clkgenval: {=bool:?}, sdr25drvstval: {:?}, sdr50sdclkfreqval: {=u16:?}, sdr50clckgenval: {=bool:?}, sdr50drvstval: {:?} }}" , self . sdr25sdclkfreqval () , self . sdr25clkgenval () , self . sdr25drvstval () , self . sdr50sdclkfreqval () , self . sdr50clckgenval () , self . sdr50drvstval ())
    }
}
#[doc = "Preset Value for SDR104 and DDR50 Modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstval6(pub u32);
impl Prstval6 {
    #[doc = "SD_CLK Frequency Select Value for SDR104."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104sdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for SDR104."]
    #[inline(always)]
    pub const fn set_sdr104sdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Clock Generator Select Value for SDR104."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104clkgenval(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for SDR104."]
    #[inline(always)]
    pub const fn set_sdr104clkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Driver Strength Select Value for SDR104."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104drvstval(&self) -> super::vals::Sdr104drvstval {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sdr104drvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for SDR104."]
    #[inline(always)]
    pub const fn set_sdr104drvstval(&mut self, val: super::vals::Sdr104drvstval) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "SD_CLK Frequency Select Value for DDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50sdclkfreqval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "SD_CLK Frequency Select Value for DDR50."]
    #[inline(always)]
    pub const fn set_ddr50sdclkfreqval(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Clock Generator Select Value for DDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50clkgenval(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Generator Select Value for DDR50."]
    #[inline(always)]
    pub const fn set_ddr50clkgenval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Driver Strength Select Value for DDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50drvstval(&self) -> super::vals::Ddr50drvstval {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Ddr50drvstval::from_bits(val as u8)
    }
    #[doc = "Driver Strength Select Value for DDR50."]
    #[inline(always)]
    pub const fn set_ddr50drvstval(&mut self, val: super::vals::Ddr50drvstval) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Prstval6 {
    #[inline(always)]
    fn default() -> Prstval6 {
        Prstval6(0)
    }
}
impl core::fmt::Debug for Prstval6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstval6")
            .field("sdr104sdclkfreqval", &self.sdr104sdclkfreqval())
            .field("sdr104clkgenval", &self.sdr104clkgenval())
            .field("sdr104drvstval", &self.sdr104drvstval())
            .field("ddr50sdclkfreqval", &self.ddr50sdclkfreqval())
            .field("ddr50clkgenval", &self.ddr50clkgenval())
            .field("ddr50drvstval", &self.ddr50drvstval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstval6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Prstval6 {{ sdr104sdclkfreqval: {=u16:?}, sdr104clkgenval: {=bool:?}, sdr104drvstval: {:?}, ddr50sdclkfreqval: {=u16:?}, ddr50clkgenval: {=bool:?}, ddr50drvstval: {:?} }}" , self . sdr104sdclkfreqval () , self . sdr104clkgenval () , self . sdr104drvstval () , self . ddr50sdclkfreqval () , self . ddr50clkgenval () , self . ddr50drvstval ())
    }
}
#[doc = "I/O LOCATION Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc0(pub u32);
impl Routeloc0 {
    #[doc = "I/O Location for D0-7 Pins."]
    #[must_use]
    #[inline(always)]
    pub const fn datloc(&self) -> super::vals::Datloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Datloc::from_bits(val as u8)
    }
    #[doc = "I/O Location for D0-7 Pins."]
    #[inline(always)]
    pub const fn set_datloc(&mut self, val: super::vals::Datloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "I/O Location for CD."]
    #[must_use]
    #[inline(always)]
    pub const fn cdloc(&self) -> super::vals::Cdloc {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Cdloc::from_bits(val as u8)
    }
    #[doc = "I/O Location for CD."]
    #[inline(always)]
    pub const fn set_cdloc(&mut self, val: super::vals::Cdloc) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "I/O Location for WP."]
    #[must_use]
    #[inline(always)]
    pub const fn wploc(&self) -> super::vals::Wploc {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Wploc::from_bits(val as u8)
    }
    #[doc = "I/O Location for WP."]
    #[inline(always)]
    pub const fn set_wploc(&mut self, val: super::vals::Wploc) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "I/O Location for CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn clkloc(&self) -> super::vals::Clkloc {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::Clkloc::from_bits(val as u8)
    }
    #[doc = "I/O Location for CLK."]
    #[inline(always)]
    pub const fn set_clkloc(&mut self, val: super::vals::Clkloc) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
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
            .field("datloc", &self.datloc())
            .field("cdloc", &self.cdloc())
            .field("wploc", &self.wploc())
            .field("clkloc", &self.clkloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Routeloc0 {{ datloc: {:?}, cdloc: {:?}, wploc: {:?}, clkloc: {:?} }}",
            self.datloc(),
            self.cdloc(),
            self.wploc(),
            self.clkloc()
        )
    }
}
#[doc = "I/O LOCATION Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routeloc1(pub u32);
impl Routeloc1 {
    #[doc = "I/O Location for CMD Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdloc(&self) -> super::vals::Cmdloc {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmdloc::from_bits(val as u8)
    }
    #[doc = "I/O Location for CMD Pin."]
    #[inline(always)]
    pub const fn set_cmdloc(&mut self, val: super::vals::Cmdloc) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Routeloc1 {
    #[inline(always)]
    fn default() -> Routeloc1 {
        Routeloc1(0)
    }
}
impl core::fmt::Debug for Routeloc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Routeloc1")
            .field("cmdloc", &self.cmdloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routeloc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Routeloc1 {{ cmdloc: {:?} }}", self.cmdloc())
    }
}
#[doc = "I/O LOCATION Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Routepen(pub u32);
impl Routepen {
    #[doc = "CLK I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CLK I/O Enable."]
    #[inline(always)]
    pub const fn set_clkpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CMD I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CMD I/O Enable."]
    #[inline(always)]
    pub const fn set_cmdpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Dat0 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d0pen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Dat0 I/O Enable."]
    #[inline(always)]
    pub const fn set_d0pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Dat1 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d1pen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Dat1 I/O Enable."]
    #[inline(always)]
    pub const fn set_d1pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Dat2 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d2pen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Dat2 I/O Enable."]
    #[inline(always)]
    pub const fn set_d2pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Dat3 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d3pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Dat3 I/O Enable."]
    #[inline(always)]
    pub const fn set_d3pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Dat4 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d4pen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Dat4 I/O Enable."]
    #[inline(always)]
    pub const fn set_d4pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Dat5 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d5pen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Dat5 Enable."]
    #[inline(always)]
    pub const fn set_d5pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Dat6 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d6pen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Dat6 Enable."]
    #[inline(always)]
    pub const fn set_d6pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Data7 I/O Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn d7pen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Data7 I/O Enable."]
    #[inline(always)]
    pub const fn set_d7pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("clkpen", &self.clkpen())
            .field("cmdpen", &self.cmdpen())
            .field("d0pen", &self.d0pen())
            .field("d1pen", &self.d1pen())
            .field("d2pen", &self.d2pen())
            .field("d3pen", &self.d3pen())
            .field("d4pen", &self.d4pen())
            .field("d5pen", &self.d5pen())
            .field("d6pen", &self.d6pen())
            .field("d7pen", &self.d7pen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Routepen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Routepen {{ clkpen: {=bool:?}, cmdpen: {=bool:?}, d0pen: {=bool:?}, d1pen: {=bool:?}, d2pen: {=bool:?}, d3pen: {=bool:?}, d4pen: {=bool:?}, d5pen: {=bool:?}, d6pen: {=bool:?}, d7pen: {=bool:?} }}" , self . clkpen () , self . cmdpen () , self . d0pen () , self . d1pen () , self . d2pen () , self . d3pen () , self . d4pen () , self . d5pen () , self . d6pen () , self . d7pen ())
    }
}
#[doc = "Slot Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slotintstat(pub u32);
impl Slotintstat {
    #[doc = "Interrupt Signal for Slot#0."]
    #[must_use]
    #[inline(always)]
    pub const fn intslot0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Signal for Slot#0."]
    #[inline(always)]
    pub const fn set_intslot0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Controller Compliant Spec Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn specvernum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Host Controller Compliant Spec Version Number."]
    #[inline(always)]
    pub const fn set_specvernum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Vendor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn vendvernum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Vendor Version Number."]
    #[inline(always)]
    pub const fn set_vendvernum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Slotintstat {
    #[inline(always)]
    fn default() -> Slotintstat {
        Slotintstat(0)
    }
}
impl core::fmt::Debug for Slotintstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slotintstat")
            .field("intslot0", &self.intslot0())
            .field("specvernum", &self.specvernum())
            .field("vendvernum", &self.vendvernum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slotintstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slotintstat {{ intslot0: {=bool:?}, specvernum: {=u8:?}, vendvernum: {=u8:?} }}",
            self.intslot0(),
            self.specvernum(),
            self.vendvernum()
        )
    }
}
#[doc = "Transfer Mode and Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfrmode(pub u32);
impl Tfrmode {
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Block Count Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn blkcnten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Block Count Enable."]
    #[inline(always)]
    pub const fn set_blkcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto Command Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autocmden(&self) -> super::vals::Autocmden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Autocmden::from_bits(val as u8)
    }
    #[doc = "Auto Command Enable."]
    #[inline(always)]
    pub const fn set_autocmden(&mut self, val: super::vals::Autocmden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Data Transfer Direction Select."]
    #[must_use]
    #[inline(always)]
    pub const fn datdirsel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Transfer Direction Select."]
    #[inline(always)]
    pub const fn set_datdirsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multiple or Single Block Data Transfer Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn multsingblksel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple or Single Block Data Transfer Selection."]
    #[inline(always)]
    pub const fn set_multsingblksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Response Type Select."]
    #[must_use]
    #[inline(always)]
    pub const fn resptypesel(&self) -> super::vals::Resptypesel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Resptypesel::from_bits(val as u8)
    }
    #[doc = "Response Type Select."]
    #[inline(always)]
    pub const fn set_resptypesel(&mut self, val: super::vals::Resptypesel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command CRC Check Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrcchken(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Command CRC Check Enable."]
    #[inline(always)]
    pub const fn set_cmdcrcchken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Command Index Check Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindxchken(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Command Index Check Enable."]
    #[inline(always)]
    pub const fn set_cmdindxchken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Data Present Select."]
    #[must_use]
    #[inline(always)]
    pub const fn datpressel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Data Present Select."]
    #[inline(always)]
    pub const fn set_datpressel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Command Type."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtype(&self) -> super::vals::Cmdtype {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Cmdtype::from_bits(val as u8)
    }
    #[doc = "Command Type."]
    #[inline(always)]
    pub const fn set_cmdtype(&mut self, val: super::vals::Cmdtype) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Command Index."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindex(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Command Index."]
    #[inline(always)]
    pub const fn set_cmdindex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Tfrmode {
    #[inline(always)]
    fn default() -> Tfrmode {
        Tfrmode(0)
    }
}
impl core::fmt::Debug for Tfrmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfrmode")
            .field("dmaen", &self.dmaen())
            .field("blkcnten", &self.blkcnten())
            .field("autocmden", &self.autocmden())
            .field("datdirsel", &self.datdirsel())
            .field("multsingblksel", &self.multsingblksel())
            .field("resptypesel", &self.resptypesel())
            .field("cmdcrcchken", &self.cmdcrcchken())
            .field("cmdindxchken", &self.cmdindxchken())
            .field("datpressel", &self.datpressel())
            .field("cmdtype", &self.cmdtype())
            .field("cmdindex", &self.cmdindex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfrmode {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tfrmode {{ dmaen: {=bool:?}, blkcnten: {=bool:?}, autocmden: {:?}, datdirsel: {=bool:?}, multsingblksel: {=bool:?}, resptypesel: {:?}, cmdcrcchken: {=bool:?}, cmdindxchken: {=bool:?}, datpressel: {=bool:?}, cmdtype: {:?}, cmdindex: {=u8:?} }}" , self . dmaen () , self . blkcnten () , self . autocmden () , self . datdirsel () , self . multsingblksel () , self . resptypesel () , self . cmdcrcchken () , self . cmdindxchken () , self . datpressel () , self . cmdtype () , self . cmdindex ())
    }
}
