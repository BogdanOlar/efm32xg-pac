#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp0mode {
    #[doc = "LESENSE does not control ACMP0."]
    Disable = 0x0,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0."]
    Mux = 0x01,
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0."]
    Muxthres = 0x02,
    _RESERVED_3 = 0x03,
}
impl Acmp0mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp0mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp0mode {
    #[inline(always)]
    fn from(val: u8) -> Acmp0mode {
        Acmp0mode::from_bits(val)
    }
}
impl From<Acmp0mode> for u8 {
    #[inline(always)]
    fn from(val: Acmp0mode) -> u8 {
        Acmp0mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp1mode {
    #[doc = "LESENSE does not control ACMP1."]
    Disable = 0x0,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1."]
    Mux = 0x01,
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1."]
    Muxthres = 0x02,
    _RESERVED_3 = 0x03,
}
impl Acmp1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp1mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp1mode {
    #[inline(always)]
    fn from(val: u8) -> Acmp1mode {
        Acmp1mode::from_bits(val)
    }
}
impl From<Acmp1mode> for u8 {
    #[inline(always)]
    fn from(val: Acmp1mode) -> u8 {
        Acmp1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auxpresc {
    #[doc = "High frequency timer is clocked with AUXHFRCO/1."]
    Div1 = 0x0,
    #[doc = "High frequency timer is clocked with AUXHFRCO/2."]
    Div2 = 0x01,
    #[doc = "High frequency timer is clocked with AUXHFRCO/4."]
    Div4 = 0x02,
    #[doc = "High frequency timer is clocked with AUXHFRCO/8."]
    Div8 = 0x03,
}
impl Auxpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auxpresc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auxpresc {
    #[inline(always)]
    fn from(val: u8) -> Auxpresc {
        Auxpresc::from_bits(val)
    }
}
impl From<Auxpresc> for u8 {
    #[inline(always)]
    fn from(val: Auxpresc) -> u8 {
        Auxpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Biasmode {
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE."]
    Donttouch = 0x0,
    #[doc = "Bias module duty cycled between low power and high accuracy mode."]
    Dutycycle = 0x01,
    #[doc = "Bias module always in high accuracy mode."]
    Highacc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Biasmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Biasmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Biasmode {
    #[inline(always)]
    fn from(val: u8) -> Biasmode {
        Biasmode::from_bits(val)
    }
}
impl From<Biasmode> for u8 {
    #[inline(always)]
    fn from(val: Biasmode) -> u8 {
        Biasmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0 {
    #[doc = "CH0 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH0 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH0 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0 {
    #[inline(always)]
    fn from(val: u8) -> Ch0 {
        Ch0::from_bits(val)
    }
}
impl From<Ch0> for u8 {
    #[inline(always)]
    fn from(val: Ch0) -> u8 {
        Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch0EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch0EvalMode {
        Ch0EvalMode::from_bits(val)
    }
}
impl From<Ch0EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch0EvalMode) -> u8 {
        Ch0EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch0EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch0EvalStrsample {
        Ch0EvalStrsample::from_bits(val)
    }
}
impl From<Ch0EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch0EvalStrsample) -> u8 {
        Ch0EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch0InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch0InteractExmode {
        Ch0InteractExmode::from_bits(val)
    }
}
impl From<Ch0InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch0InteractExmode) -> u8 {
        Ch0InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch0InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch0InteractSample {
        Ch0InteractSample::from_bits(val)
    }
}
impl From<Ch0InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch0InteractSample) -> u8 {
        Ch0InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch0InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch0InteractSetif {
        Ch0InteractSetif::from_bits(val)
    }
}
impl From<Ch0InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch0InteractSetif) -> u8 {
        Ch0InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1 {
    #[doc = "CH1 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH1 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH1 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1 {
    #[inline(always)]
    fn from(val: u8) -> Ch1 {
        Ch1::from_bits(val)
    }
}
impl From<Ch1> for u8 {
    #[inline(always)]
    fn from(val: Ch1) -> u8 {
        Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10 {
    #[doc = "CH10 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH10 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH10 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10 {
    #[inline(always)]
    fn from(val: u8) -> Ch10 {
        Ch10::from_bits(val)
    }
}
impl From<Ch10> for u8 {
    #[inline(always)]
    fn from(val: Ch10) -> u8 {
        Ch10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch10EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch10EvalMode {
        Ch10EvalMode::from_bits(val)
    }
}
impl From<Ch10EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch10EvalMode) -> u8 {
        Ch10EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch10EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch10EvalStrsample {
        Ch10EvalStrsample::from_bits(val)
    }
}
impl From<Ch10EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch10EvalStrsample) -> u8 {
        Ch10EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch10InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch10InteractExmode {
        Ch10InteractExmode::from_bits(val)
    }
}
impl From<Ch10InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch10InteractExmode) -> u8 {
        Ch10InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch10InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch10InteractSample {
        Ch10InteractSample::from_bits(val)
    }
}
impl From<Ch10InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch10InteractSample) -> u8 {
        Ch10InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch10InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch10InteractSetif {
        Ch10InteractSetif::from_bits(val)
    }
}
impl From<Ch10InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch10InteractSetif) -> u8 {
        Ch10InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11 {
    #[doc = "CH11 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH11 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH11 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11 {
    #[inline(always)]
    fn from(val: u8) -> Ch11 {
        Ch11::from_bits(val)
    }
}
impl From<Ch11> for u8 {
    #[inline(always)]
    fn from(val: Ch11) -> u8 {
        Ch11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch11EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch11EvalMode {
        Ch11EvalMode::from_bits(val)
    }
}
impl From<Ch11EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch11EvalMode) -> u8 {
        Ch11EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch11EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch11EvalStrsample {
        Ch11EvalStrsample::from_bits(val)
    }
}
impl From<Ch11EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch11EvalStrsample) -> u8 {
        Ch11EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch11InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch11InteractExmode {
        Ch11InteractExmode::from_bits(val)
    }
}
impl From<Ch11InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch11InteractExmode) -> u8 {
        Ch11InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch11InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch11InteractSample {
        Ch11InteractSample::from_bits(val)
    }
}
impl From<Ch11InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch11InteractSample) -> u8 {
        Ch11InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch11InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch11InteractSetif {
        Ch11InteractSetif::from_bits(val)
    }
}
impl From<Ch11InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch11InteractSetif) -> u8 {
        Ch11InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12 {
    #[doc = "CH12 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH12 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH12 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12 {
    #[inline(always)]
    fn from(val: u8) -> Ch12 {
        Ch12::from_bits(val)
    }
}
impl From<Ch12> for u8 {
    #[inline(always)]
    fn from(val: Ch12) -> u8 {
        Ch12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch12EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch12EvalMode {
        Ch12EvalMode::from_bits(val)
    }
}
impl From<Ch12EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch12EvalMode) -> u8 {
        Ch12EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch12EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch12EvalStrsample {
        Ch12EvalStrsample::from_bits(val)
    }
}
impl From<Ch12EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch12EvalStrsample) -> u8 {
        Ch12EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch12InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch12InteractExmode {
        Ch12InteractExmode::from_bits(val)
    }
}
impl From<Ch12InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch12InteractExmode) -> u8 {
        Ch12InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch12InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch12InteractSample {
        Ch12InteractSample::from_bits(val)
    }
}
impl From<Ch12InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch12InteractSample) -> u8 {
        Ch12InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch12InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch12InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch12InteractSetif {
        Ch12InteractSetif::from_bits(val)
    }
}
impl From<Ch12InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch12InteractSetif) -> u8 {
        Ch12InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13 {
    #[doc = "CH13 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH13 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH13 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13 {
    #[inline(always)]
    fn from(val: u8) -> Ch13 {
        Ch13::from_bits(val)
    }
}
impl From<Ch13> for u8 {
    #[inline(always)]
    fn from(val: Ch13) -> u8 {
        Ch13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch13EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch13EvalMode {
        Ch13EvalMode::from_bits(val)
    }
}
impl From<Ch13EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch13EvalMode) -> u8 {
        Ch13EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch13EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch13EvalStrsample {
        Ch13EvalStrsample::from_bits(val)
    }
}
impl From<Ch13EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch13EvalStrsample) -> u8 {
        Ch13EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch13InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch13InteractExmode {
        Ch13InteractExmode::from_bits(val)
    }
}
impl From<Ch13InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch13InteractExmode) -> u8 {
        Ch13InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch13InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch13InteractSample {
        Ch13InteractSample::from_bits(val)
    }
}
impl From<Ch13InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch13InteractSample) -> u8 {
        Ch13InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch13InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch13InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch13InteractSetif {
        Ch13InteractSetif::from_bits(val)
    }
}
impl From<Ch13InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch13InteractSetif) -> u8 {
        Ch13InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14 {
    #[doc = "CH14 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH14 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH14 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14 {
    #[inline(always)]
    fn from(val: u8) -> Ch14 {
        Ch14::from_bits(val)
    }
}
impl From<Ch14> for u8 {
    #[inline(always)]
    fn from(val: Ch14) -> u8 {
        Ch14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch14EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch14EvalMode {
        Ch14EvalMode::from_bits(val)
    }
}
impl From<Ch14EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch14EvalMode) -> u8 {
        Ch14EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch14EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch14EvalStrsample {
        Ch14EvalStrsample::from_bits(val)
    }
}
impl From<Ch14EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch14EvalStrsample) -> u8 {
        Ch14EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch14InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch14InteractExmode {
        Ch14InteractExmode::from_bits(val)
    }
}
impl From<Ch14InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch14InteractExmode) -> u8 {
        Ch14InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch14InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch14InteractSample {
        Ch14InteractSample::from_bits(val)
    }
}
impl From<Ch14InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch14InteractSample) -> u8 {
        Ch14InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch14InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch14InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch14InteractSetif {
        Ch14InteractSetif::from_bits(val)
    }
}
impl From<Ch14InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch14InteractSetif) -> u8 {
        Ch14InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15 {
    #[doc = "CH15 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH15 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH15 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15 {
    #[inline(always)]
    fn from(val: u8) -> Ch15 {
        Ch15::from_bits(val)
    }
}
impl From<Ch15> for u8 {
    #[inline(always)]
    fn from(val: Ch15) -> u8 {
        Ch15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch15EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch15EvalMode {
        Ch15EvalMode::from_bits(val)
    }
}
impl From<Ch15EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch15EvalMode) -> u8 {
        Ch15EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch15EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch15EvalStrsample {
        Ch15EvalStrsample::from_bits(val)
    }
}
impl From<Ch15EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch15EvalStrsample) -> u8 {
        Ch15EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch15InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch15InteractExmode {
        Ch15InteractExmode::from_bits(val)
    }
}
impl From<Ch15InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch15InteractExmode) -> u8 {
        Ch15InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch15InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch15InteractSample {
        Ch15InteractSample::from_bits(val)
    }
}
impl From<Ch15InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch15InteractSample) -> u8 {
        Ch15InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch15InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch15InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch15InteractSetif {
        Ch15InteractSetif::from_bits(val)
    }
}
impl From<Ch15InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch15InteractSetif) -> u8 {
        Ch15InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch1EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch1EvalMode {
        Ch1EvalMode::from_bits(val)
    }
}
impl From<Ch1EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch1EvalMode) -> u8 {
        Ch1EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch1EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch1EvalStrsample {
        Ch1EvalStrsample::from_bits(val)
    }
}
impl From<Ch1EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch1EvalStrsample) -> u8 {
        Ch1EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch1InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch1InteractExmode {
        Ch1InteractExmode::from_bits(val)
    }
}
impl From<Ch1InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch1InteractExmode) -> u8 {
        Ch1InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch1InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch1InteractSample {
        Ch1InteractSample::from_bits(val)
    }
}
impl From<Ch1InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch1InteractSample) -> u8 {
        Ch1InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch1InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch1InteractSetif {
        Ch1InteractSetif::from_bits(val)
    }
}
impl From<Ch1InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch1InteractSetif) -> u8 {
        Ch1InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2 {
    #[doc = "CH2 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH2 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH2 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2 {
    #[inline(always)]
    fn from(val: u8) -> Ch2 {
        Ch2::from_bits(val)
    }
}
impl From<Ch2> for u8 {
    #[inline(always)]
    fn from(val: Ch2) -> u8 {
        Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch2EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch2EvalMode {
        Ch2EvalMode::from_bits(val)
    }
}
impl From<Ch2EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch2EvalMode) -> u8 {
        Ch2EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch2EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch2EvalStrsample {
        Ch2EvalStrsample::from_bits(val)
    }
}
impl From<Ch2EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch2EvalStrsample) -> u8 {
        Ch2EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch2InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch2InteractExmode {
        Ch2InteractExmode::from_bits(val)
    }
}
impl From<Ch2InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch2InteractExmode) -> u8 {
        Ch2InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch2InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch2InteractSample {
        Ch2InteractSample::from_bits(val)
    }
}
impl From<Ch2InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch2InteractSample) -> u8 {
        Ch2InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch2InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch2InteractSetif {
        Ch2InteractSetif::from_bits(val)
    }
}
impl From<Ch2InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch2InteractSetif) -> u8 {
        Ch2InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3 {
    #[doc = "CH3 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH3 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH3 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3 {
    #[inline(always)]
    fn from(val: u8) -> Ch3 {
        Ch3::from_bits(val)
    }
}
impl From<Ch3> for u8 {
    #[inline(always)]
    fn from(val: Ch3) -> u8 {
        Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch3EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch3EvalMode {
        Ch3EvalMode::from_bits(val)
    }
}
impl From<Ch3EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch3EvalMode) -> u8 {
        Ch3EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch3EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch3EvalStrsample {
        Ch3EvalStrsample::from_bits(val)
    }
}
impl From<Ch3EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch3EvalStrsample) -> u8 {
        Ch3EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch3InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch3InteractExmode {
        Ch3InteractExmode::from_bits(val)
    }
}
impl From<Ch3InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch3InteractExmode) -> u8 {
        Ch3InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch3InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch3InteractSample {
        Ch3InteractSample::from_bits(val)
    }
}
impl From<Ch3InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch3InteractSample) -> u8 {
        Ch3InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch3InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch3InteractSetif {
        Ch3InteractSetif::from_bits(val)
    }
}
impl From<Ch3InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch3InteractSetif) -> u8 {
        Ch3InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4 {
    #[doc = "CH4 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH4 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH4 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4 {
    #[inline(always)]
    fn from(val: u8) -> Ch4 {
        Ch4::from_bits(val)
    }
}
impl From<Ch4> for u8 {
    #[inline(always)]
    fn from(val: Ch4) -> u8 {
        Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch4EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch4EvalMode {
        Ch4EvalMode::from_bits(val)
    }
}
impl From<Ch4EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch4EvalMode) -> u8 {
        Ch4EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch4EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch4EvalStrsample {
        Ch4EvalStrsample::from_bits(val)
    }
}
impl From<Ch4EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch4EvalStrsample) -> u8 {
        Ch4EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch4InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch4InteractExmode {
        Ch4InteractExmode::from_bits(val)
    }
}
impl From<Ch4InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch4InteractExmode) -> u8 {
        Ch4InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch4InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch4InteractSample {
        Ch4InteractSample::from_bits(val)
    }
}
impl From<Ch4InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch4InteractSample) -> u8 {
        Ch4InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch4InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch4InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch4InteractSetif {
        Ch4InteractSetif::from_bits(val)
    }
}
impl From<Ch4InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch4InteractSetif) -> u8 {
        Ch4InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5 {
    #[doc = "CH5 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH5 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH5 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5 {
    #[inline(always)]
    fn from(val: u8) -> Ch5 {
        Ch5::from_bits(val)
    }
}
impl From<Ch5> for u8 {
    #[inline(always)]
    fn from(val: Ch5) -> u8 {
        Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch5EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch5EvalMode {
        Ch5EvalMode::from_bits(val)
    }
}
impl From<Ch5EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch5EvalMode) -> u8 {
        Ch5EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch5EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch5EvalStrsample {
        Ch5EvalStrsample::from_bits(val)
    }
}
impl From<Ch5EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch5EvalStrsample) -> u8 {
        Ch5EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch5InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch5InteractExmode {
        Ch5InteractExmode::from_bits(val)
    }
}
impl From<Ch5InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch5InteractExmode) -> u8 {
        Ch5InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch5InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch5InteractSample {
        Ch5InteractSample::from_bits(val)
    }
}
impl From<Ch5InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch5InteractSample) -> u8 {
        Ch5InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch5InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch5InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch5InteractSetif {
        Ch5InteractSetif::from_bits(val)
    }
}
impl From<Ch5InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch5InteractSetif) -> u8 {
        Ch5InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6 {
    #[doc = "CH6 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH6 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH6 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6 {
    #[inline(always)]
    fn from(val: u8) -> Ch6 {
        Ch6::from_bits(val)
    }
}
impl From<Ch6> for u8 {
    #[inline(always)]
    fn from(val: Ch6) -> u8 {
        Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch6EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch6EvalMode {
        Ch6EvalMode::from_bits(val)
    }
}
impl From<Ch6EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch6EvalMode) -> u8 {
        Ch6EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch6EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch6EvalStrsample {
        Ch6EvalStrsample::from_bits(val)
    }
}
impl From<Ch6EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch6EvalStrsample) -> u8 {
        Ch6EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch6InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch6InteractExmode {
        Ch6InteractExmode::from_bits(val)
    }
}
impl From<Ch6InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch6InteractExmode) -> u8 {
        Ch6InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch6InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch6InteractSample {
        Ch6InteractSample::from_bits(val)
    }
}
impl From<Ch6InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch6InteractSample) -> u8 {
        Ch6InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch6InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch6InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch6InteractSetif {
        Ch6InteractSetif::from_bits(val)
    }
}
impl From<Ch6InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch6InteractSetif) -> u8 {
        Ch6InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7 {
    #[doc = "CH7 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH7 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH7 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7 {
    #[inline(always)]
    fn from(val: u8) -> Ch7 {
        Ch7::from_bits(val)
    }
}
impl From<Ch7> for u8 {
    #[inline(always)]
    fn from(val: Ch7) -> u8 {
        Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch7EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch7EvalMode {
        Ch7EvalMode::from_bits(val)
    }
}
impl From<Ch7EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch7EvalMode) -> u8 {
        Ch7EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch7EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch7EvalStrsample {
        Ch7EvalStrsample::from_bits(val)
    }
}
impl From<Ch7EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch7EvalStrsample) -> u8 {
        Ch7EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch7InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch7InteractExmode {
        Ch7InteractExmode::from_bits(val)
    }
}
impl From<Ch7InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch7InteractExmode) -> u8 {
        Ch7InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch7InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch7InteractSample {
        Ch7InteractSample::from_bits(val)
    }
}
impl From<Ch7InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch7InteractSample) -> u8 {
        Ch7InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch7InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch7InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch7InteractSetif {
        Ch7InteractSetif::from_bits(val)
    }
}
impl From<Ch7InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch7InteractSetif) -> u8 {
        Ch7InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8 {
    #[doc = "CH8 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH8 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH8 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8 {
    #[inline(always)]
    fn from(val: u8) -> Ch8 {
        Ch8::from_bits(val)
    }
}
impl From<Ch8> for u8 {
    #[inline(always)]
    fn from(val: Ch8) -> u8 {
        Ch8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch8EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch8EvalMode {
        Ch8EvalMode::from_bits(val)
    }
}
impl From<Ch8EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch8EvalMode) -> u8 {
        Ch8EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch8EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch8EvalStrsample {
        Ch8EvalStrsample::from_bits(val)
    }
}
impl From<Ch8EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch8EvalStrsample) -> u8 {
        Ch8EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch8InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch8InteractExmode {
        Ch8InteractExmode::from_bits(val)
    }
}
impl From<Ch8InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch8InteractExmode) -> u8 {
        Ch8InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch8InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch8InteractSample {
        Ch8InteractSample::from_bits(val)
    }
}
impl From<Ch8InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch8InteractSample) -> u8 {
        Ch8InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch8InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch8InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch8InteractSetif {
        Ch8InteractSetif::from_bits(val)
    }
}
impl From<Ch8InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch8InteractSetif) -> u8 {
        Ch8InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9 {
    #[doc = "CH9 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "CH9 output is high in idle phase."]
    High = 0x01,
    #[doc = "CH9 output is low in idle phase."]
    Low = 0x02,
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15."]
    Dac = 0x03,
}
impl Ch9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9 {
    #[inline(always)]
    fn from(val: u8) -> Ch9 {
        Ch9::from_bits(val)
    }
}
impl From<Ch9> for u8 {
    #[inline(always)]
    fn from(val: Ch9) -> u8 {
        Ch9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9EvalMode {
    #[doc = "Threshold comparison is used to evaluate sensor result."]
    Thres = 0x0,
    #[doc = "Sliding window is used to evaluate sensor result."]
    Slidingwin = 0x01,
    #[doc = "Step detection is used to evaluate sensor result."]
    Stepdet = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch9EvalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9EvalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9EvalMode {
    #[inline(always)]
    fn from(val: u8) -> Ch9EvalMode {
        Ch9EvalMode::from_bits(val)
    }
}
impl From<Ch9EvalMode> for u8 {
    #[inline(always)]
    fn from(val: Ch9EvalMode) -> u8 {
        Ch9EvalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9EvalStrsample {
    #[doc = "Nothing will be stored in the result buffer."]
    Disable = 0x0,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    Data = 0x01,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ch9EvalStrsample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9EvalStrsample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9EvalStrsample {
    #[inline(always)]
    fn from(val: u8) -> Ch9EvalStrsample {
        Ch9EvalStrsample::from_bits(val)
    }
}
impl From<Ch9EvalStrsample> for u8 {
    #[inline(always)]
    fn from(val: Ch9EvalStrsample) -> u8 {
        Ch9EvalStrsample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9InteractExmode {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Push Pull, GPIO is driven high."]
    High = 0x01,
    #[doc = "Push Pull, GPIO is driven low."]
    Low = 0x02,
    #[doc = "VDAC output."]
    Dacout = 0x03,
}
impl Ch9InteractExmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9InteractExmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9InteractExmode {
    #[inline(always)]
    fn from(val: u8) -> Ch9InteractExmode {
        Ch9InteractExmode::from_bits(val)
    }
}
impl From<Ch9InteractExmode> for u8 {
    #[inline(always)]
    fn from(val: Ch9InteractExmode) -> u8 {
        Ch9InteractExmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9InteractSample {
    #[doc = "Counter output will be used in evaluation."]
    Acmpcount = 0x0,
    #[doc = "ACMP output will be used in evaluation."]
    Acmp = 0x01,
    #[doc = "ADC output will be used in evaluation."]
    Adc = 0x02,
    #[doc = "Differential ADC output will be used in evaluation."]
    Adcdiff = 0x03,
}
impl Ch9InteractSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9InteractSample {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9InteractSample {
    #[inline(always)]
    fn from(val: u8) -> Ch9InteractSample {
        Ch9InteractSample::from_bits(val)
    }
}
impl From<Ch9InteractSample> for u8 {
    #[inline(always)]
    fn from(val: Ch9InteractSample) -> u8 {
        Ch9InteractSample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch9InteractSetif {
    #[doc = "No interrupt is generated."]
    None = 0x0,
    #[doc = "Set interrupt flag if the sensor triggers."]
    Level = 0x01,
    #[doc = "Set interrupt flag on positive edge of the sensor state."]
    Posedge = 0x02,
    #[doc = "Set interrupt flag on negative edge of the sensor state."]
    Negedge = 0x03,
    #[doc = "Set interrupt flag on both edges of the sensor state."]
    Bothedges = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ch9InteractSetif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9InteractSetif {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9InteractSetif {
    #[inline(always)]
    fn from(val: u8) -> Ch9InteractSetif {
        Ch9InteractSetif::from_bits(val)
    }
}
impl From<Ch9InteractSetif> for u8 {
    #[inline(always)]
    fn from(val: Ch9InteractSetif) -> u8 {
        Ch9InteractSetif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmawu {
    #[doc = "No DMA wake-up from EM2."]
    Disable = 0x0,
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer."]
    Bufdatav = 0x01,
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration."]
    Buflevel = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dmawu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmawu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmawu {
    #[inline(always)]
    fn from(val: u8) -> Dmawu {
        Dmawu::from_bits(val)
    }
}
impl From<Dmawu> for u8 {
    #[inline(always)]
    fn from(val: Dmawu) -> u8 {
        Dmawu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf0 {
    #[doc = "ALTEX0 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX0 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX0 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf0 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf0 {
        Idleconf0::from_bits(val)
    }
}
impl From<Idleconf0> for u8 {
    #[inline(always)]
    fn from(val: Idleconf0) -> u8 {
        Idleconf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf1 {
    #[doc = "ALTEX1 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX1 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX1 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf1 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf1 {
        Idleconf1::from_bits(val)
    }
}
impl From<Idleconf1> for u8 {
    #[inline(always)]
    fn from(val: Idleconf1) -> u8 {
        Idleconf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf2 {
    #[doc = "ALTEX2 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX2 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX2 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf2 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf2 {
        Idleconf2::from_bits(val)
    }
}
impl From<Idleconf2> for u8 {
    #[inline(always)]
    fn from(val: Idleconf2) -> u8 {
        Idleconf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf3 {
    #[doc = "ALTEX3 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX3 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX3 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf3 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf3 {
        Idleconf3::from_bits(val)
    }
}
impl From<Idleconf3> for u8 {
    #[inline(always)]
    fn from(val: Idleconf3) -> u8 {
        Idleconf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf4 {
    #[doc = "ALTEX4 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX4 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX4 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf4 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf4 {
        Idleconf4::from_bits(val)
    }
}
impl From<Idleconf4> for u8 {
    #[inline(always)]
    fn from(val: Idleconf4) -> u8 {
        Idleconf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf5 {
    #[doc = "ALTEX5 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX5 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX5 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf5 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf5 {
        Idleconf5::from_bits(val)
    }
}
impl From<Idleconf5> for u8 {
    #[inline(always)]
    fn from(val: Idleconf5) -> u8 {
        Idleconf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf6 {
    #[doc = "ALTEX6 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX6 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX6 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf6 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf6 {
        Idleconf6::from_bits(val)
    }
}
impl From<Idleconf6> for u8 {
    #[inline(always)]
    fn from(val: Idleconf6) -> u8 {
        Idleconf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idleconf7 {
    #[doc = "ALTEX7 output is disabled in idle phase."]
    Disable = 0x0,
    #[doc = "ALTEX7 output is high in idle phase."]
    High = 0x01,
    #[doc = "ALTEX7 output is low in idle phase."]
    Low = 0x02,
    _RESERVED_3 = 0x03,
}
impl Idleconf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idleconf7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idleconf7 {
    #[inline(always)]
    fn from(val: u8) -> Idleconf7 {
        Idleconf7::from_bits(val)
    }
}
impl From<Idleconf7> for u8 {
    #[inline(always)]
    fn from(val: Idleconf7) -> u8 {
        Idleconf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lfpresc {
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1."]
    Div1 = 0x0,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2."]
    Div2 = 0x01,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4."]
    Div4 = 0x02,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8."]
    Div8 = 0x03,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16."]
    Div16 = 0x04,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32."]
    Div32 = 0x05,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64."]
    Div64 = 0x06,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128."]
    Div128 = 0x07,
}
impl Lfpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfpresc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfpresc {
    #[inline(always)]
    fn from(val: u8) -> Lfpresc {
        Lfpresc::from_bits(val)
    }
}
impl From<Lfpresc> for u8 {
    #[inline(always)]
    fn from(val: Lfpresc) -> u8 {
        Lfpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcpresc {
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1."]
    Div1 = 0x0,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2."]
    Div2 = 0x01,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4."]
    Div4 = 0x02,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8."]
    Div8 = 0x03,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16."]
    Div16 = 0x04,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32."]
    Div32 = 0x05,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64."]
    Div64 = 0x06,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128."]
    Div128 = 0x07,
}
impl Pcpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcpresc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcpresc {
    #[inline(always)]
    fn from(val: u8) -> Pcpresc {
        Pcpresc::from_bits(val)
    }
}
impl From<Pcpresc> for u8 {
    #[inline(always)]
    fn from(val: Pcpresc) -> u8 {
        Pcpresc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prssel0 {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    #[doc = "PRS Channel 12 selected as input."]
    Prsch12 = 0x0c,
    #[doc = "PRS Channel 13 selected as input."]
    Prsch13 = 0x0d,
    #[doc = "PRS Channel 14 selected as input."]
    Prsch14 = 0x0e,
    #[doc = "PRS Channel 15 selected as input."]
    Prsch15 = 0x0f,
    #[doc = "PRS Channel 16 selected as input."]
    Prsch16 = 0x10,
    #[doc = "PRS Channel 17 selected as input."]
    Prsch17 = 0x11,
    #[doc = "PRS Channel 18 selected as input."]
    Prsch18 = 0x12,
    #[doc = "PRS Channel 19 selected as input."]
    Prsch19 = 0x13,
    #[doc = "PRS Channel 20 selected as input."]
    Prsch20 = 0x14,
    #[doc = "PRS Channel 21 selected as input."]
    Prsch21 = 0x15,
    #[doc = "PRS Channel 22 selected as input."]
    Prsch22 = 0x16,
    #[doc = "PRS Channel 23 selected as input."]
    Prsch23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Prssel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prssel0 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prssel0 {
    #[inline(always)]
    fn from(val: u8) -> Prssel0 {
        Prssel0::from_bits(val)
    }
}
impl From<Prssel0> for u8 {
    #[inline(always)]
    fn from(val: Prssel0) -> u8 {
        Prssel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prssel1 {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    #[doc = "PRS Channel 12 selected as input."]
    Prsch12 = 0x0c,
    #[doc = "PRS Channel 13 selected as input."]
    Prsch13 = 0x0d,
    #[doc = "PRS Channel 14 selected as input."]
    Prsch14 = 0x0e,
    #[doc = "PRS Channel 15 selected as input."]
    Prsch15 = 0x0f,
    #[doc = "PRS Channel 16 selected as input."]
    Prsch16 = 0x10,
    #[doc = "PRS Channel 17 selected as input."]
    Prsch17 = 0x11,
    #[doc = "PRS Channel 18 selected as input."]
    Prsch18 = 0x12,
    #[doc = "PRS Channel 19 selected as input."]
    Prsch19 = 0x13,
    #[doc = "PRS Channel 20 selected as input."]
    Prsch20 = 0x14,
    #[doc = "PRS Channel 21 selected as input."]
    Prsch21 = 0x15,
    #[doc = "PRS Channel 22 selected as input."]
    Prsch22 = 0x16,
    #[doc = "PRS Channel 23 selected as input."]
    Prsch23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Prssel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prssel1 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prssel1 {
    #[inline(always)]
    fn from(val: u8) -> Prssel1 {
        Prssel1::from_bits(val)
    }
}
impl From<Prssel1> for u8 {
    #[inline(always)]
    fn from(val: Prssel1) -> u8 {
        Prssel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prssel2 {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    #[doc = "PRS Channel 12 selected as input."]
    Prsch12 = 0x0c,
    #[doc = "PRS Channel 13 selected as input."]
    Prsch13 = 0x0d,
    #[doc = "PRS Channel 14 selected as input."]
    Prsch14 = 0x0e,
    #[doc = "PRS Channel 15 selected as input."]
    Prsch15 = 0x0f,
    #[doc = "PRS Channel 16 selected as input."]
    Prsch16 = 0x10,
    #[doc = "PRS Channel 17 selected as input."]
    Prsch17 = 0x11,
    #[doc = "PRS Channel 18 selected as input."]
    Prsch18 = 0x12,
    #[doc = "PRS Channel 19 selected as input."]
    Prsch19 = 0x13,
    #[doc = "PRS Channel 20 selected as input."]
    Prsch20 = 0x14,
    #[doc = "PRS Channel 21 selected as input."]
    Prsch21 = 0x15,
    #[doc = "PRS Channel 22 selected as input."]
    Prsch22 = 0x16,
    #[doc = "PRS Channel 23 selected as input."]
    Prsch23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Prssel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prssel2 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prssel2 {
    #[inline(always)]
    fn from(val: u8) -> Prssel2 {
        Prssel2::from_bits(val)
    }
}
impl From<Prssel2> for u8 {
    #[inline(always)]
    fn from(val: Prssel2) -> u8 {
        Prssel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prssel3 {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    #[doc = "PRS Channel 12 selected as input."]
    Prsch12 = 0x0c,
    #[doc = "PRS Channel 13 selected as input."]
    Prsch13 = 0x0d,
    #[doc = "PRS Channel 14 selected as input."]
    Prsch14 = 0x0e,
    #[doc = "PRS Channel 15 selected as input."]
    Prsch15 = 0x0f,
    #[doc = "PRS Channel 16 selected as input."]
    Prsch16 = 0x10,
    #[doc = "PRS Channel 17 selected as input."]
    Prsch17 = 0x11,
    #[doc = "PRS Channel 18 selected as input."]
    Prsch18 = 0x12,
    #[doc = "PRS Channel 19 selected as input."]
    Prsch19 = 0x13,
    #[doc = "PRS Channel 20 selected as input."]
    Prsch20 = 0x14,
    #[doc = "PRS Channel 21 selected as input."]
    Prsch21 = 0x15,
    #[doc = "PRS Channel 22 selected as input."]
    Prsch22 = 0x16,
    #[doc = "PRS Channel 23 selected as input."]
    Prsch23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Prssel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prssel3 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prssel3 {
    #[inline(always)]
    fn from(val: u8) -> Prssel3 {
        Prssel3::from_bits(val)
    }
}
impl From<Prssel3> for u8 {
    #[inline(always)]
    fn from(val: Prssel3) -> u8 {
        Prssel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scanconf {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    Dirmap = 0x0,
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    Invmap = 0x01,
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers."]
    Toggle = 0x02,
    #[doc = "The decoder state defines the CONF registers to be used."]
    Decdef = 0x03,
}
impl Scanconf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scanconf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scanconf {
    #[inline(always)]
    fn from(val: u8) -> Scanconf {
        Scanconf::from_bits(val)
    }
}
impl From<Scanconf> for u8 {
    #[inline(always)]
    fn from(val: Scanconf) -> u8 {
        Scanconf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scanmode {
    #[doc = "A new scan is started each time the period counter overflows."]
    Periodic = 0x0,
    #[doc = "A single scan is performed when START in CMD is set."]
    Oneshot = 0x01,
    #[doc = "Pulse on PRS channel."]
    Prs = 0x02,
    _RESERVED_3 = 0x03,
}
impl Scanmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scanmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scanmode {
    #[inline(always)]
    fn from(val: u8) -> Scanmode {
        Scanmode::from_bits(val)
    }
}
impl From<Scanmode> for u8 {
    #[inline(always)]
    fn from(val: Scanmode) -> u8 {
        Scanmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Warmupmode {
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle."]
    Normal = 0x0,
    #[doc = "The analog comparators are kept powered up when LESENSE is idle."]
    Keepacmpwarm = 0x01,
    #[doc = "The VDAC is kept powered up when LESENSE is idle."]
    Keepdacwarm = 0x02,
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle."]
    Keepacmpdacwarm = 0x03,
}
impl Warmupmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Warmupmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Warmupmode {
    #[inline(always)]
    fn from(val: u8) -> Warmupmode {
        Warmupmode::from_bits(val)
    }
}
impl From<Warmupmode> for u8 {
    #[inline(always)]
    fn from(val: Warmupmode) -> u8 {
        Warmupmode::to_bits(val)
    }
}
