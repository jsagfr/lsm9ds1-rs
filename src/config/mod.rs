#![allow(dead_code)]
use std::collections::HashMap;
use std::iter::Iterator;
use std::slice::Iter;

use super::Address;

#[macro_use]
mod macros;

pub mod act_ths;
pub mod act_dur;
pub mod int_gen_cfg_xl;
pub mod int_gen_ths_x_xl;
pub mod int_gen_ths_y_xl;
pub mod int_gen_ths_z_xl;
pub mod int_gen_dur_xl;
pub mod reference_g;
pub mod int1_ctrl;
pub mod int2_ctrl;
pub mod ctrl_reg1_g;
pub mod ctrl_reg2_g;
pub mod ctrl_reg3_g;
pub mod orient_cfg_g;
pub mod ctrl_reg4;
pub mod ctrl_reg5_xl;
pub mod ctrl_reg6_xl;
pub mod ctrl_reg7_xl;
pub mod ctrl_reg8;
pub mod ctrl_reg9;
pub mod ctrl_reg10;
pub mod fifo_ctrl;
pub mod int_gen_cfg_g;
pub mod int_gen_ths_x_g;
pub mod int_gen_ths_y_g;
pub mod int_gen_ths_z_g;
pub mod int_gen_dur_g;
pub mod offset_x_reg_m;
pub mod offset_y_reg_m;
pub mod offset_z_reg_m;
pub mod ctrl_reg1_m;
pub mod ctrl_reg2_m;
pub mod ctrl_reg3_m;
pub mod ctrl_reg4_m;
pub mod ctrl_reg5_m;
pub mod int_cfg_m;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataRate {
    NA,
    PowerDown,
    DR15Hz,
    DR59Hz,
    DR119Hz,
    DR238Hz,
    DR476Hz,
    DR952Hz,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GyroScale {
    NA,
    FS245Dps,
    FS500Dps,
    FS2000Dps,
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bw {
    A,
    B,
    C,
    D,
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cutoff {
    NA,
    C5Hz,
    C19Hz,
    C38Hz,
    C76Hz,
    C100Hz,
    CHz,
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSel {
    A,
    B,
    C,
    D,
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutSel {
    A,
    B,
    C,
    D,
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigCutoffFreq {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dec {
    NoDec,
    Dec2S,
    Dec4S,
    Dec8S,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMode {
    ByPass,
    StopIfFull,
    ContinusTrig,
    ByPassTrig,
    Overwrite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpMode {
    LowPower,
    MediumPerf,
    HighPerf,
    UltraHighPerf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputDataRate {
    Odr0p625Hz,
    Odr1p25Hz,
    Odr2p5Hz,
    Odr5Hz,
    Odr10Hz,
    Odr20Hz,
    Odr40Hz,
    Odr80Hz,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsM {
    Fs4,
    Fs8,
    Fs12,
    Fs16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsXl {
    Fs2,
    Fs4,
    Fs8,
    Fs16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Md {
    Continuous,
    Single,
    PowerDown,
}


enum_with_type!{
    /// Parameters used to configure or given when reading a
    /// *LSM9DS1*.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Param,
    /// Parameters type used to get a *LSM9DS1* configuration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum_type ParamType : 116; {
        // ActThs
        /// Gyroscope inactivity threshold.
        ///
        /// __TODO: Question: is accelerometer concerned?__
        /// 
        /// * Possible values in `0..2**7`
        /// * Default value is `0`
        variant ActThs => u8,
        /// Gyroscope operating mode during inactivity.
        ///
        /// * Default value is `Disable`
        variant SleepOn => bool,

        // ActDur
        /// Gyroscope inactivity duration.
        ///
        /// __TODO: Question: is accelerometer concerned?__
        /// 
        /// * Possible values in `u8`
        /// * Default value is `0`
        variant ActDur => u8,
        
        // IntGenCfgXl 
        /// AND/OR combination of accelerometer's interrupt events.
        /// 
        /// * Default value: `Or`
        variant AoiXl => bool,
        variant Detect6D => bool,
        variant ZhieXl => bool,
        variant ZlieXl => bool,
        variant YhieXl => bool,
        variant YlieXl => bool,
        variant XhieXl => bool,
        variant XlieXl => bool,

        // IntGenThsXXl
        variant IntGenThsXXl => u8,

        // IntGenThsYXl
        variant IntGenThsYXl => u8,

        // IntGenThsZXl
        variant IntGenThsZXl => u8,

        // IntGenDurXl
        variant WaitXl => bool,
        /// Linear acceleration sensor interrupt duration.
        variant DurXl => u8,

        // ReferenceG
        variant ReferenceG => u8,

        // Int1Ctrl register
        variant Int1IgG => bool,
        variant Int1IgXl => bool,
        variant Int1Fss5 => bool,
        variant Int1Ovr => bool,
        variant Int1Fth => bool,
        variant Int1Boot => bool,
        variant Int1DrdyG => bool,
        variant Int1DrdyXl => bool,

        // Int2Ctrl register:
        variant Int2Inact => bool,
        variant Int2Fss5 => bool,
        variant Int2Ovr => bool,
        variant Int2Fth => bool,
        variant Int2DrdyTemp => bool,
        variant Int2DrdyG => bool,
        variant Int2DrdyXl => bool,

        // CtrlReg1G
        variant OdrG => DataRate,
        variant FsG => GyroScale,
        variant BwG => Bw,

        // CtrlReg2G
        variant IntSel => IntSel,
        variant OutSel => OutSel,

        // CtrlReg3G
        variant LPMode => bool,
        variant HpEn => bool,
        variant HpcfG => u8,

        // OrientCfgG
        variant SignXG => bool,
        variant SignYG => bool,
        variant SignZG => bool,
        /// Carification needed for Orient which is probably not a
        /// simple u3...
        variant Orient => u8,

        // CtrlReg4
        variant ZenG => bool,
        variant YenG => bool,
        variant XenG => bool,
        variant LirXl1 => bool,
        variant I4dXl1 => bool,

        // CtrlReg5XL
        variant ZenXl => bool,
        variant YenXl => bool,
        variant XenXl => bool,
        variant Dec => Dec,

        // CtrlReg6XL
        variant OdrXl => DataRate,
        variant FsXl => FsXl,
        variant BwScalOdr => bool,
        variant BwXl => Bw,

        // CtrlReg7XL
        variant HighRes => bool,
        variant XlDigitalCf => DigCutoffFreq,
        variant FilteredDataSel => bool,
        variant HighPassIntSens => bool,   
        
        // CtrlReg8
        variant Boot => bool,
        variant Bdu => bool,
        variant HLactive => bool,
        variant PpOd => bool,
        variant Sim => bool,
        variant IfAddInc => bool,
        variant Ble => bool,
        variant SwReset => bool,

        // CtrlReg9
        variant SleepG => bool,
        variant FifoTempEn => bool,
        variant DrdyMaskBit => bool,
        variant I2cDisable => bool,
        variant FifoEn => bool,
        variant StopOnFth => bool,

        // CtrlReg10
        variant StG => bool,
        variant StXl => bool,

        // FifoCtrl
        variant Fth => u8,
        variant FMode => FMode,
        
        // IntGenCfgG
        variant AoiG => bool,
        variant LirG => bool,
        variant ZhieG => bool,
        variant ZlieG => bool,
        variant YhieG => bool,
        variant YlieG => bool,
        variant XhieG => bool,
        variant XlieG => bool,
        
        // IntGenThsXG
        variant DcrmG => bool,
        variant IntGenThsXG => u16,

        // IntGenThsYG
        variant IntGenThsYG => u16,

        // IntGenThsZG
        variant IntGenThsZG => u16,

        // IntGenDurG
        variant WaitG => bool,
        variant DurG => u8,

        // OffsetXRegM
        variant OffsetXRegM => u16,

        // OffsetYRegM
        variant OffsetYRegM => u16,
        
        // OffsetZRegM
        variant OffsetZRegM => u16,

        // CtrlReg1M
        variant OpMode => OpMode,
        variant OutputDataRate => OutputDataRate,
        variant TempComp => bool,
        variant SelfTest => bool,

        // CtrlReg2M
        variant FsM => FsM,
        variant RebootM => bool,
        variant SoftResetM => bool,

        // CtrlReg3M
        variant Md => Md,
        variant I2cDisableM => bool,
        variant LowPowerM => bool,
        variant SimM => bool,
        
        // CtrlReg4M
        variant OpModeZ => OpMode,
        variant BigLittleEndian => bool,
        
        // CtrlReg5M
        variant BlockDataUpdate => bool,

        // IntCfgM
        variant Xien => bool,
        variant Yien => bool,
        variant Zien => bool,
        variant Iea => bool,
        variant Iel => bool,
        variant Ien => bool,
    }
}

impl Param {
    pub fn reg_type(&self) -> RegisterType {
        match self.type_of() {
            ParamType::ActThs  => RegisterType::ActThs,
            ParamType::SleepOn => RegisterType::ActThs,

            ParamType::ActDur => RegisterType::ActDur,
            
            ParamType::AoiXl    => RegisterType::IntGenCfgXl,
            ParamType::Detect6D => RegisterType::IntGenCfgXl,
            ParamType::ZhieXl   => RegisterType::IntGenCfgXl,
            ParamType::ZlieXl   => RegisterType::IntGenCfgXl,
            ParamType::YhieXl   => RegisterType::IntGenCfgXl,
            ParamType::YlieXl   => RegisterType::IntGenCfgXl,
            ParamType::XhieXl   => RegisterType::IntGenCfgXl,
            ParamType::XlieXl   => RegisterType::IntGenCfgXl,

            ParamType::IntGenThsXXl => RegisterType::IntGenThsXXl,
            ParamType::IntGenThsYXl => RegisterType::IntGenThsYXl,
            ParamType::IntGenThsZXl => RegisterType::IntGenThsZXl,

            ParamType::WaitXl => RegisterType::IntGenDurXl,
            ParamType::DurXl  => RegisterType::IntGenDurXl,

            ParamType::ReferenceG => RegisterType::ReferenceG,

            // Int1Ctrl register
            ParamType::Int1IgG    => RegisterType::Int1Ctrl,
            ParamType::Int1IgXl   => RegisterType::Int1Ctrl,
            ParamType::Int1Fss5   => RegisterType::Int1Ctrl,
            ParamType::Int1Ovr    => RegisterType::Int1Ctrl,
            ParamType::Int1Fth    => RegisterType::Int1Ctrl,
            ParamType::Int1Boot   => RegisterType::Int1Ctrl,
            ParamType::Int1DrdyG  => RegisterType::Int1Ctrl,
            ParamType::Int1DrdyXl => RegisterType::Int1Ctrl,

            // Int2Ctrl register:
            ParamType::Int2Inact    => RegisterType::Int2Ctrl,
            ParamType::Int2Fss5     => RegisterType::Int2Ctrl,
            ParamType::Int2Ovr      => RegisterType::Int2Ctrl,
            ParamType::Int2Fth      => RegisterType::Int2Ctrl,
            ParamType::Int2DrdyTemp => RegisterType::Int2Ctrl,
            ParamType::Int2DrdyG    => RegisterType::Int2Ctrl,
            ParamType::Int2DrdyXl   => RegisterType::Int2Ctrl,

            // CtrlReg1G
            ParamType::OdrG => RegisterType::CtrlReg1G,
            ParamType::FsG  => RegisterType::CtrlReg1G,
            ParamType::BwG  => RegisterType::CtrlReg1G,

            // CtrlReg2G
            ParamType::IntSel => RegisterType::CtrlReg2G,
            ParamType::OutSel => RegisterType::CtrlReg2G,

            // CtrlReg3G
            ParamType::LPMode => RegisterType::CtrlReg3G,
            ParamType::HpEn   => RegisterType::CtrlReg3G,
            ParamType::HpcfG  => RegisterType::CtrlReg3G,

            // OrientCfgG
            ParamType::SignXG => RegisterType::OrientCfgG,
            ParamType::SignYG => RegisterType::OrientCfgG,
            ParamType::SignZG => RegisterType::OrientCfgG,
            ParamType::Orient => RegisterType::OrientCfgG,

            // CtrlReg4
            ParamType::ZenG   => RegisterType::CtrlReg4,
            ParamType::YenG   => RegisterType::CtrlReg4,
            ParamType::XenG   => RegisterType::CtrlReg4,
            ParamType::LirXl1 => RegisterType::CtrlReg4,
            ParamType::I4dXl1 => RegisterType::CtrlReg4,

            // CtrlReg5XL
            ParamType::ZenXl => RegisterType::CtrlReg5Xl,
            ParamType::YenXl => RegisterType::CtrlReg5Xl,
            ParamType::XenXl => RegisterType::CtrlReg5Xl,
            ParamType::Dec   => RegisterType::CtrlReg5Xl,

            // CtrlReg6XL
            ParamType::OdrXl     => RegisterType::CtrlReg6Xl,
            ParamType::FsXl      => RegisterType::CtrlReg6Xl,
            ParamType::BwScalOdr => RegisterType::CtrlReg6Xl,
            ParamType::BwXl      => RegisterType::CtrlReg6Xl,

            // CtrlReg7XL
            ParamType::HighRes         => RegisterType::CtrlReg7Xl,
            ParamType::XlDigitalCf     => RegisterType::CtrlReg7Xl,
            ParamType::FilteredDataSel => RegisterType::CtrlReg7Xl,
            ParamType::HighPassIntSens => RegisterType::CtrlReg7Xl, 
            
            // CtrlReg8
            ParamType::Boot     => RegisterType::CtrlReg8,
            ParamType::Bdu      => RegisterType::CtrlReg8,
            ParamType::HLactive => RegisterType::CtrlReg8,
            ParamType::PpOd     => RegisterType::CtrlReg8,
            ParamType::Sim      => RegisterType::CtrlReg8,
            ParamType::IfAddInc => RegisterType::CtrlReg8,
            ParamType::Ble      => RegisterType::CtrlReg8,
            ParamType::SwReset  => RegisterType::CtrlReg8,

            // CtrlReg9
            ParamType::SleepG      => RegisterType::CtrlReg9,
            ParamType::FifoTempEn  => RegisterType::CtrlReg9,
            ParamType::DrdyMaskBit => RegisterType::CtrlReg9,
            ParamType::I2cDisable  => RegisterType::CtrlReg9,
            ParamType::FifoEn      => RegisterType::CtrlReg9,
            ParamType::StopOnFth   => RegisterType::CtrlReg9,

            // CtrlReg10
            ParamType::StG  => RegisterType::CtrlReg10,
            ParamType::StXl => RegisterType::CtrlReg10,

            // FifoCtrl
            ParamType::Fth   => RegisterType::FifoCtrl,
            ParamType::FMode => RegisterType::FifoCtrl,
            
            // IntGenCfgG
            ParamType::AoiG  => RegisterType::IntGenCfgG,
            ParamType::LirG  => RegisterType::IntGenCfgG,
            ParamType::ZhieG => RegisterType::IntGenCfgG,
            ParamType::ZlieG => RegisterType::IntGenCfgG,
            ParamType::YhieG => RegisterType::IntGenCfgG,
            ParamType::YlieG => RegisterType::IntGenCfgG,
            ParamType::XhieG => RegisterType::IntGenCfgG,
            ParamType::XlieG => RegisterType::IntGenCfgG,
            
            // IntGenThsXG
            ParamType::DcrmG       => RegisterType::IntGenThsXG,
            ParamType::IntGenThsXG => RegisterType::IntGenThsXG,

            // IntGenThsYG
            ParamType::IntGenThsYG => RegisterType::IntGenThsYG,

            // IntGenThsZG
            ParamType::IntGenThsZG => RegisterType::IntGenThsZG,

            // IntGenDurG
            ParamType::WaitG => RegisterType::IntGenDurG,
            ParamType::DurG  => RegisterType::IntGenDurG,

            // OffsetXRegM
            ParamType::OffsetXRegM => RegisterType::OffsetXRegM,

            // OffsetYRegM
            ParamType::OffsetYRegM => RegisterType::OffsetYRegM,
            
            // OffsetZRegM
            ParamType::OffsetZRegM => RegisterType::OffsetZRegM,

            // CtrlReg1M
            ParamType::OpMode         => RegisterType::CtrlReg1M,
            ParamType::OutputDataRate => RegisterType::CtrlReg1M,
            ParamType::TempComp       => RegisterType::CtrlReg1M,
            ParamType::SelfTest       => RegisterType::CtrlReg1M,

            // CtrlReg2M
            ParamType::FsM        => RegisterType::CtrlReg2M,
            ParamType::RebootM    => RegisterType::CtrlReg2M,
            ParamType::SoftResetM => RegisterType::CtrlReg2M,

            // CtrlReg3M
            ParamType::Md          => RegisterType::CtrlReg3M,
            ParamType::I2cDisableM => RegisterType::CtrlReg3M,
            ParamType::LowPowerM   => RegisterType::CtrlReg3M,
            ParamType::SimM        => RegisterType::CtrlReg3M,
            
            // CtrlReg4M
            ParamType::OpModeZ         => RegisterType::CtrlReg4M,
            ParamType::BigLittleEndian => RegisterType::CtrlReg4M,
            
            // CtrlReg5M
            ParamType::BlockDataUpdate => RegisterType::CtrlReg5M,

            // IntCfgM
            ParamType::Xien => RegisterType::IntCfgM,
            ParamType::Yien => RegisterType::IntCfgM,
            ParamType::Zien => RegisterType::IntCfgM,
            ParamType::Iea  => RegisterType::IntCfgM,
            ParamType::Iel  => RegisterType::IntCfgM,
            ParamType::Ien  => RegisterType::IntCfgM,
        }
    }
}

enum_with_type!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Register,
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum_type RegisterType : 36; {
        variant ActThs => u8,
        variant ActDur => u8,
        variant IntGenCfgXl => u8,
        variant IntGenThsXXl => u8,
        variant IntGenThsYXl => u8,
        variant IntGenThsZXl => u8,
        variant IntGenDurXl => u8,
        variant ReferenceG => u8,
        variant Int1Ctrl => u8,
        variant Int2Ctrl => u8,
        variant CtrlReg1G => u8,
        variant CtrlReg2G => u8,
        variant CtrlReg3G => u8,
        variant OrientCfgG => u8,
        variant CtrlReg4 => u8,
        variant CtrlReg5Xl => u8,
        variant CtrlReg6Xl => u8,
        variant CtrlReg7Xl => u8,
        variant CtrlReg8 => u8,
        variant CtrlReg9 => u8,
        variant CtrlReg10 => u8,
        variant FifoCtrl => u8,
        variant IntGenCfgG => u8,
        variant IntGenThsXG => u16,
        variant IntGenThsYG => u16,
        variant IntGenThsZG => u16,
        variant IntGenDurG => u8,
        variant OffsetXRegM => u16,
        variant OffsetYRegM => u16,
        variant OffsetZRegM => u16,
        variant CtrlReg1M => u8,
        variant CtrlReg2M => u8,
        variant CtrlReg3M => u8,
        variant CtrlReg4M => u8,
        variant CtrlReg5M => u8,
        variant IntCfgM => u8,
        
    }
}

const ACT_THS:          Address = Address::RW(0x04);
const ACT_DUR:          Address = Address::RW(0x05);
const INT_GEN_CFG_XL:   Address = Address::RW(0x06);
const INT_GEN_THS_X_XL: Address = Address::RW(0x07);
const INT_GEN_THS_Y_XL: Address = Address::RW(0x08);
const INT_GEN_THS_Z_XL: Address = Address::RW(0x09);
const INT_GEN_DUR_XL:   Address = Address::RW(0x0A);
const REFERENCE_G:      Address = Address::RW(0x0B);
const INT1_CTRL:        Address = Address::RW(0x0C);
const INT2_CTRL:        Address = Address::RW(0x0D);
const CTRL_REG1_G:      Address = Address::RW(0x10);
const CTRL_REG2_G:      Address = Address::RW(0x11);
const CTRL_REG3_G:      Address = Address::RW(0x12);
const ORIENT_CFG_G:     Address = Address::RW(0x13);
const INT_GEN_SRC_G:    Address = Address::RW(0x14);
const CTRL_REG4:        Address = Address::RW(0x1E);
const CTRL_REG5_XL:     Address = Address::RW(0x1F);
const CTRL_REG6_XL:     Address = Address::RW(0x20);
const CTRL_REG7_XL:     Address = Address::RW(0x21);
const CTRL_REG8:        Address = Address::RW(0x22);
const CTRL_REG9:        Address = Address::RW(0x23);
const CTRL_REG10:       Address = Address::RW(0x24);
const FIFO_CTRL:        Address = Address::RW(0x2E);
const INT_GEN_CFG_G:    Address = Address::RW(0x30);
const INT_GEN_THS_X_G:  Address = Address::RW16(0x31);
const INT_GEN_THS_Y_G:  Address = Address::RW16(0x33);
const INT_GEN_THS_Z_G:  Address = Address::RW16(0x35);
const INT_GEN_DUR_G:    Address = Address::RW(0x37);
const OFFSET_X_REG_M:   Address = Address::RW16(0x05);
const OFFSET_Y_REG_M:   Address = Address::RW16(0x07);
const OFFSET_Z_REG_M:   Address = Address::RW16(0x09);
const CTRL_REG1_M:      Address = Address::RW(0x20);
const CTRL_REG2_M:      Address = Address::RW(0x21);
const CTRL_REG3_M:      Address = Address::RW(0x22);
const CTRL_REG4_M:      Address = Address::RW(0x23);
const CTRL_REG5_M:      Address = Address::RW(0x24);
const OUT_X_M:          Address = Address::R16(0x28);
const OUT_Y_M:          Address = Address::R16(0x2A);
const OUT_Z_M:          Address = Address::R16(0x2C);
const INT_CFG_M:        Address = Address::RW(0x30);
const INT_THS_M:        Address = Address::R16(0x32);

impl Register {
    pub fn address(&self) -> Address {
        match *self {
            Register::ActThs(_) => ACT_THS,
            Register::ActDur(_) => ACT_DUR,
            Register::IntGenCfgXl(_) => INT_GEN_CFG_XL,
            Register::IntGenThsXXl(_) => INT_GEN_THS_X_XL,
            Register::IntGenThsYXl(_) => INT_GEN_THS_Y_XL,
            Register::IntGenThsZXl(_) => INT_GEN_THS_Z_XL,
            Register::IntGenDurXl(_) => INT_GEN_DUR_XL,
            Register::ReferenceG(_) => REFERENCE_G,
            Register::Int1Ctrl(_) => INT1_CTRL,
            Register::Int2Ctrl(_) => INT2_CTRL,
            Register::CtrlReg1G(_) => CTRL_REG1_G,
            Register::CtrlReg2G(_) => CTRL_REG2_G,
            Register::CtrlReg3G(_) => CTRL_REG3_G,
            Register::OrientCfgG(_) => ORIENT_CFG_G,
            Register::CtrlReg4(_) => CTRL_REG4,
            Register::CtrlReg5Xl(_) => CTRL_REG5_XL,
            Register::CtrlReg6Xl(_) => CTRL_REG6_XL,
            Register::CtrlReg7Xl(_) => CTRL_REG7_XL,
            Register::CtrlReg8(_) => CTRL_REG8,
            Register::CtrlReg9(_) => CTRL_REG9,
            Register::CtrlReg10(_) => CTRL_REG10,
            Register::FifoCtrl(_) => FIFO_CTRL,
            Register::IntGenCfgG(_) => INT_GEN_CFG_G,
            Register::IntGenThsXG(_) => INT_GEN_THS_X_G,
            Register::IntGenThsYG(_) => INT_GEN_THS_Y_G,
            Register::IntGenThsZG(_) => INT_GEN_THS_Z_G,
            Register::IntGenDurG(_) => INT_GEN_DUR_G,
            Register::OffsetXRegM(_) => OFFSET_X_REG_M,
            Register::OffsetYRegM(_) => OFFSET_Y_REG_M,
            Register::OffsetZRegM(_) => OFFSET_Z_REG_M,
            Register::CtrlReg1M(_) => CTRL_REG1_M,
            Register::CtrlReg2M(_) => CTRL_REG2_M,
            Register::CtrlReg3M(_) => CTRL_REG3_M,
            Register::CtrlReg4M(_) => CTRL_REG4_M,
            Register::CtrlReg5M(_) => CTRL_REG5_M,
            Register::IntCfgM(_) => INT_CFG_M,
        }
    }
}

impl RegisterType {
    pub fn from_params(&self, params: &[Param]) -> Result<Register,()> {
        match *self {
            RegisterType::ActThs => act_ths::from_params(params),
            RegisterType::ActDur => act_dur::from_params(params),
            RegisterType::IntGenCfgXl => int_gen_cfg_xl::from_params(params),
            RegisterType::IntGenThsXXl => int_gen_ths_x_xl::from_params(params),
            RegisterType::IntGenThsYXl => int_gen_ths_y_xl::from_params(params),
            RegisterType::IntGenThsZXl => int_gen_ths_z_xl::from_params(params),
            RegisterType::IntGenDurXl => int_gen_dur_xl::from_params(params),
            RegisterType::ReferenceG => reference_g::from_params(params),
            RegisterType::Int1Ctrl => int1_ctrl::from_params(params),
            RegisterType::Int2Ctrl => int2_ctrl::from_params(params),
            RegisterType::CtrlReg1G => ctrl_reg1_g::from_params(params),
            RegisterType::CtrlReg2G => ctrl_reg2_g::from_params(params),
            RegisterType::CtrlReg3G => ctrl_reg3_g::from_params(params),
            RegisterType::OrientCfgG => orient_cfg_g::from_params(params),
            RegisterType::CtrlReg4 => ctrl_reg4::from_params(params),
            RegisterType::CtrlReg5Xl => ctrl_reg5_xl::from_params(params),
            RegisterType::CtrlReg6Xl => ctrl_reg6_xl::from_params(params),
            RegisterType::CtrlReg7Xl => ctrl_reg7_xl::from_params(params),
            RegisterType::CtrlReg8 => ctrl_reg8::from_params(params),
            RegisterType::CtrlReg9 => ctrl_reg9::from_params(params),
            RegisterType::CtrlReg10 => ctrl_reg10::from_params(params),
            RegisterType::FifoCtrl => fifo_ctrl::from_params(params),
            RegisterType::IntGenCfgG => int_gen_cfg_g::from_params(params),
            RegisterType::IntGenThsXG => int_gen_ths_x_g::from_params(params),
            RegisterType::IntGenThsYG => int_gen_ths_y_g::from_params(params),
            RegisterType::IntGenThsZG => int_gen_ths_z_g::from_params(params),
            RegisterType::IntGenDurG => int_gen_dur_g::from_params(params),
            RegisterType::OffsetXRegM => offset_x_reg_m::from_params(params),
            RegisterType::OffsetYRegM => offset_y_reg_m::from_params(params),
            RegisterType::OffsetZRegM => offset_z_reg_m::from_params(params),
            RegisterType::CtrlReg1M => ctrl_reg1_m::from_params(params),
            RegisterType::CtrlReg2M => ctrl_reg2_m::from_params(params),
            RegisterType::CtrlReg3M => ctrl_reg3_m::from_params(params),
            RegisterType::CtrlReg4M => ctrl_reg4_m::from_params(params),
            RegisterType::CtrlReg5M => ctrl_reg5_m::from_params(params),
            RegisterType::IntCfgM => int_cfg_m::from_params(params),
        }
    }
    pub fn address(&self) -> Address {
        match *self {
            RegisterType::ActThs => ACT_THS,
            RegisterType::ActDur => ACT_DUR,
            RegisterType::IntGenCfgXl => INT_GEN_CFG_XL,
            RegisterType::IntGenThsXXl => INT_GEN_THS_X_XL,
            RegisterType::IntGenThsYXl => INT_GEN_THS_Y_XL,
            RegisterType::IntGenThsZXl => INT_GEN_THS_Z_XL,
            RegisterType::IntGenDurXl => INT_GEN_DUR_XL,
            RegisterType::ReferenceG => REFERENCE_G,
            RegisterType::Int1Ctrl => INT1_CTRL,
            RegisterType::Int2Ctrl => INT2_CTRL,
            RegisterType::CtrlReg1G => CTRL_REG1_G,
            RegisterType::CtrlReg2G => CTRL_REG2_G,
            RegisterType::CtrlReg3G => CTRL_REG3_G,
            RegisterType::OrientCfgG => ORIENT_CFG_G,
            RegisterType::CtrlReg4 => CTRL_REG4,
            RegisterType::CtrlReg5Xl => CTRL_REG5_XL,
            RegisterType::CtrlReg6Xl => CTRL_REG6_XL,
            RegisterType::CtrlReg7Xl => CTRL_REG7_XL,
            RegisterType::CtrlReg8 => CTRL_REG8,
            RegisterType::CtrlReg9 => CTRL_REG9,
            RegisterType::CtrlReg10 => CTRL_REG10,
            RegisterType::FifoCtrl => FIFO_CTRL,
            RegisterType::IntGenCfgG => INT_GEN_CFG_G,
            RegisterType::IntGenThsXG => INT_GEN_THS_X_G,
            RegisterType::IntGenThsYG => INT_GEN_THS_Y_G,
            RegisterType::IntGenThsZG => INT_GEN_THS_Z_G,
            RegisterType::IntGenDurG => INT_GEN_DUR_G,
            RegisterType::OffsetXRegM => OFFSET_X_REG_M,
            RegisterType::OffsetYRegM => OFFSET_Y_REG_M,
            RegisterType::OffsetZRegM => OFFSET_Z_REG_M,
            RegisterType::CtrlReg1M => CTRL_REG1_M,
            RegisterType::CtrlReg2M => CTRL_REG2_M,
            RegisterType::CtrlReg3M => CTRL_REG3_M,
            RegisterType::CtrlReg4M => CTRL_REG4_M,
            RegisterType::CtrlReg5M => CTRL_REG5_M,
            RegisterType::IntCfgM => INT_CFG_M,
        }
    }

}

/// `ConfParamBuilder` is use to create a partial or total new configuration of
/// a *LSM9DS1*.
/// 
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfParamBuilder, Param};
/// 
/// let conf1 = ConfParamBuilder::new()
///     .set(Param::ActThs(5))
///     .set(Param::SleepOn(true))
///     .build().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct ConfParamBuilder {
    params: Vec<Param>,
}

impl ConfParamBuilder {
    /// `ConfParamBuilder::new()` create a new emtpy `ConfParamBuilder`.
    pub fn new() -> ConfParamBuilder {
        ConfParamBuilder {
            params: Vec::new(),
        }
    }

    /// Set a specific parameter.
    pub fn set<'a>(&'a mut self, param: Param) -> &'a mut ConfParamBuilder {
        self.params.push(param);
        self
    }

    /// Set a list of parameters.
    pub fn set_all<'a>(&'a mut self, params: &[Param]) -> &'a mut ConfParamBuilder {
        self.params.extend(params);
        self
    }

    /// Build a `Config` from a `ConfParamBuilder`.
    ///
    /// If the same parameter is set multiple times, with different
    /// values, the last one is used.
    ///
    /// # Errors
    ///
    /// `build()` may failed if a parameter is not compatible with the *LSM9DS1*
    /// specification.
    ///
    /// # Example
    ///
    /// ```
    /// use lsm9ds1::config::{ConfParamBuilder, Param};
    /// 
    /// let conf1 = ConfParamBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(true))
    ///     .build().unwrap();
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut hash_reg = HashMap::new();
        let mut hash_param = HashMap::new();
        for &p in &self.params {
            hash_param.insert(p.type_of(), p);
            let params = hash_reg.entry(p.reg_type()).or_insert(Vec::new());
            params.push(p);
        }

        let mut registers = Vec::new();
        for (key, params) in hash_reg.iter() {
            registers.push(try!(key.from_params(params)));
        }

        Ok(Config {
            params: hash_param,
            registers: registers,
        })
    }
}

/// `Register` is use to create a partial or total new configuration of
/// a *LSM9DS1* from a set of register values.
/// 
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfRegBuilder, Register};
/// 
/// let conf = ConfRegBuilder::new().
///     set(Register::ActThs(0b1000_0000 | 5))
///     .build().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct ConfRegBuilder {
    registers: Vec<Register>,
}

impl ConfRegBuilder {
    /// `ConfParamBuilder::new()` create a new emtpy `ConfParamBuilder`.
    pub fn new() -> ConfRegBuilder {
        ConfRegBuilder {
            registers: Vec::new(),
        }
    }

    /// Set a specific register.
    pub fn set<'a>(&'a mut self, register: Register) -> &'a mut ConfRegBuilder {
        self.registers.push(register);
        self
    }

    /// Set a list of registers.
    pub fn set_all<'a>(&'a mut self, registers: &[Register]) -> &'a mut ConfRegBuilder {
        self.registers.extend(registers);
        self
    }

    /// Build a `Config` from a `ConfRegBuilder`.
    ///
    /// # Errors
    ///
    /// `build()` may failed if:
    ///
    /// * 2 identic registers have differents values or;
    ///
    /// * if a register value is not compatible with the *LSM9DS1*
    /// specification.
    ///
    /// # Example
    ///
    /// ```
    /// use lsm9ds1::config::{ConfRegBuilder, Register, Param, ParamType};
    /// 
    /// let conf = ConfRegBuilder::new().
    ///     set(Register::ActThs(0b1000_0000 | 5))
    ///     .build().unwrap();
    /// match *conf.param(ParamType::ActThs).unwrap() {
    ///     Param::ActThs(ref act_ths) => assert_eq!(*act_ths, 5),
    ///     _ => panic!(),
    /// };
    /// match *conf.param(ParamType::SleepOn).unwrap() {
    ///     Param::SleepOn(ref sleep_on) => assert_eq!(*sleep_on, true),
    ///     _ => panic!(),
    /// };
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut params = HashMap::new();
        for &reg in &self.registers {
            let reg_params = match reg.type_of() {
                RegisterType::ActThs => try!(act_ths::from_register(reg)),
                RegisterType::ActDur => try!(act_dur::from_register(reg)),
                RegisterType::IntGenCfgXl => try!(int_gen_cfg_xl::from_register(reg)),
                RegisterType::IntGenThsXXl => try!(int_gen_ths_x_xl::from_register(reg)),
                RegisterType::IntGenThsYXl => try!(int_gen_ths_y_xl::from_register(reg)),
                RegisterType::IntGenThsZXl => try!(int_gen_ths_z_xl::from_register(reg)),
                RegisterType::IntGenDurXl => try!(int_gen_dur_xl::from_register(reg)),
                RegisterType::ReferenceG => try!(reference_g::from_register(reg)),
                RegisterType::Int1Ctrl => try!(int1_ctrl::from_register(reg)),
                RegisterType::Int2Ctrl => try!(int2_ctrl::from_register(reg)),
                RegisterType::CtrlReg1G => try!(ctrl_reg1_g::from_register(reg)),
                RegisterType::CtrlReg2G => try!(ctrl_reg2_g::from_register(reg)),
                RegisterType::CtrlReg3G => try!(ctrl_reg3_g::from_register(reg)),
                RegisterType::OrientCfgG => try!(orient_cfg_g::from_register(reg)),
                RegisterType::CtrlReg4 => try!(ctrl_reg4::from_register(reg)),
                RegisterType::CtrlReg5Xl => try!(ctrl_reg5_xl::from_register(reg)),
                RegisterType::CtrlReg6Xl => try!(ctrl_reg6_xl::from_register(reg)),
                RegisterType::CtrlReg7Xl => try!(ctrl_reg7_xl::from_register(reg)),
                RegisterType::CtrlReg8 => try!(ctrl_reg8::from_register(reg)),
                RegisterType::CtrlReg9 => try!(ctrl_reg9::from_register(reg)),
                RegisterType::CtrlReg10 => try!(ctrl_reg10::from_register(reg)),
                RegisterType::FifoCtrl => try!(fifo_ctrl::from_register(reg)),
                RegisterType::IntGenCfgG => try!(int_gen_cfg_g::from_register(reg)),
                RegisterType::IntGenThsXG => try!(int_gen_ths_x_g::from_register(reg)),
                RegisterType::IntGenThsYG => try!(int_gen_ths_y_g::from_register(reg)),
                RegisterType::IntGenThsZG => try!(int_gen_ths_z_g::from_register(reg)),
                RegisterType::IntGenDurG => try!(int_gen_dur_g::from_register(reg)),
                RegisterType::OffsetXRegM => try!(offset_x_reg_m::from_register(reg)),
                RegisterType::OffsetYRegM => try!(offset_y_reg_m::from_register(reg)),
                RegisterType::OffsetZRegM => try!(offset_z_reg_m::from_register(reg)),
                RegisterType::CtrlReg1M => try!(ctrl_reg1_m::from_register(reg)),
                RegisterType::CtrlReg2M => try!(ctrl_reg2_m::from_register(reg)),
                RegisterType::CtrlReg3M => try!(ctrl_reg3_m::from_register(reg)),
                RegisterType::CtrlReg4M => try!(ctrl_reg4_m::from_register(reg)),
                RegisterType::CtrlReg5M => try!(ctrl_reg5_m::from_register(reg)),
                RegisterType::IntCfgM => try!(int_cfg_m::from_register(reg)),
            };
            for p in reg_params {
                params.insert(p.type_of(), p);
            }
        }

        Ok(Config {
            params: params,
            registers: self.registers.clone(),
        })
    }
}


/// `Config` represents a partial or total possible configuration of a
/// *LSM9DS1*.
/// 
/// It can be build from:
///
/// * a set of registers value, with `Register`: It's useful if you
/// want to decode the config read from a *LSM9DS1* device.
///
/// * or from a set of values, with `ConfParamBuilder`: It's useful if
/// you want to encode a configuration to a *LSM9DS1* device.
///
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfParamBuilder, Param, ConfRegBuilder};
/// 
/// let conf1 = ConfParamBuilder::new()
///     .set(Param::ActThs(5))
///     .set(Param::SleepOn(true))
///     .build().unwrap();
/// let conf2 = ConfRegBuilder::new().
///     set_all(conf1.registers())
///     .build().unwrap();
/// 
/// assert_eq!(conf1, conf2);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    params: HashMap<ParamType, Param>,
    registers: Vec<Register>
}

impl Config {
    /// Get the parameter of a parameter type from `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfParamBuilder, Param, ParamType};
    ///
    /// let c = ConfParamBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(true))
    ///     .build().unwrap();
    /// let &p = c.param(ParamType::ActThs).unwrap();
    /// assert_eq!(p, Param::ActThs(5));
    /// ```
    pub fn param(&self, param: ParamType) -> Option<&Param> {
        self.params.get(&param)
    }

    /// List all the parameters set for the current `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfParamBuilder, Param};
    ///
    /// let c = ConfParamBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(true))
    ///     .build().unwrap();
    /// for &p in c.params() {
    ///     assert!(p == Param::ActThs(5) || p == Param::SleepOn(true));
    /// }
    /// ```
    pub fn params(&self) -> Vec<&Param> {
        let params: Vec<&Param> = self.params.values().collect();
        params
    }

    /// List all the registers set for the current `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfParamBuilder, Param, Register, RegisterType};
    ///
    /// let c = ConfParamBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(true))
    ///     .build().unwrap();
    /// for &r in c.registers() {
    ///     assert_eq!(r.type_of(), RegisterType::ActThs);
    /// }
    /// ```
    pub fn registers(&self) -> &[Register] {
        &self.registers
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfParamBuilder, ConfRegBuilder, Param,
                DataRate, GyroScale, Bw, IntSel, OutSel, Dec, DigCutoffFreq,
                FsXl, FMode, OpMode, FsM, Md, OutputDataRate};
    
    #[test]
    fn it_works() {
        let conf1 = ConfParamBuilder::new()
            .set(Param::ActThs(5))
            .set(Param::SleepOn(true))
            .set(Param::ActDur(5))
            .set(Param::AoiXl(true))
            .set(Param::Detect6D(true))
            .set(Param::ZhieXl(true))
            .set(Param::ZlieXl(true))
            .set(Param::YhieXl(true))
            .set(Param::YlieXl(true))
            .set(Param::XhieXl(true))
            .set(Param::XlieXl(true))
            .set(Param::IntGenThsXXl(5))
            .set(Param::IntGenThsYXl(5))
            .set(Param::IntGenThsZXl(5))
            .set(Param::WaitXl(true))
            .set(Param::DurXl(5))
            .set(Param::ReferenceG(5))
            .set(Param::Int1IgG(true))
            .set(Param::Int1IgXl(true))
            .set(Param::Int1Fss5(true))
            .set(Param::Int1Ovr(true))
            .set(Param::Int1Fth(true))
            .set(Param::Int1Boot(true))
            .set(Param::Int1DrdyG(true))
            .set(Param::Int1DrdyXl(true))
            .set(Param::Int2Inact(true))
            .set(Param::Int2Fss5(true))
            .set(Param::Int2Ovr(true))
            .set(Param::Int2Fth(true))
            .set(Param::Int2DrdyTemp(true))
            .set(Param::Int2DrdyG(true))
            .set(Param::Int2DrdyXl(true))
            .set(Param::OdrG(DataRate::DR15Hz))
            .set(Param::FsG(GyroScale::FS245Dps))
            .set(Param::BwG(Bw::B))
            .set(Param::IntSel(IntSel::A))
            .set(Param::OutSel(OutSel::A))
            .set(Param::LPMode(true))
            .set(Param::HpEn(true))
            .set(Param::HpcfG(5))
            .set(Param::SignXG(true))
            .set(Param::SignYG(true))
            .set(Param::SignZG(true))
            .set(Param::Orient(5))
            .set(Param::ZenG(true))
            .set(Param::YenG(true))
            .set(Param::XenG(true))
            .set(Param::LirXl1(true))
            .set(Param::I4dXl1(true))
            .set(Param::ZenXl(true))
            .set(Param::YenXl(true))
            .set(Param::XenXl(true))
            .set(Param::Dec(Dec::Dec2S))
            .set(Param::OdrXl(DataRate::DR15Hz))
            .set(Param::FsXl(FsXl::Fs2))
            .set(Param::BwScalOdr(true))
            .set(Param::BwXl(Bw::B))
            .set(Param::HighRes(true))
            .set(Param::XlDigitalCf(DigCutoffFreq::A))
            .set(Param::FilteredDataSel(true))
            .set(Param::HighPassIntSens(true))   
            .set(Param::Boot(true))
            .set(Param::Bdu(true))
            .set(Param::HLactive(true))
            .set(Param::PpOd(true))
            .set(Param::Sim(true))
            .set(Param::IfAddInc(true))
            .set(Param::Ble(true))
            .set(Param::SwReset(true))
            .set(Param::SleepG(true))
            .set(Param::FifoTempEn(true))
            .set(Param::DrdyMaskBit(true))
            .set(Param::I2cDisable(true))
            .set(Param::FifoEn(true))
            .set(Param::StopOnFth(true))
            .set(Param::StG(true))
            .set(Param::StXl(true))
            .set(Param::Fth(5))
            .set(Param::FMode(FMode::Overwrite))
            .set(Param::AoiG(true))
            .set(Param::LirG(true))
            .set(Param::ZhieG(true))
            .set(Param::ZlieG(true))
            .set(Param::YhieG(true))
            .set(Param::YlieG(true))
            .set(Param::XhieG(true))
            .set(Param::XlieG(true))
            .set(Param::DcrmG(true))
            .set(Param::IntGenThsXG(5))
            .set(Param::IntGenThsYG(5))
            .set(Param::IntGenThsZG(5))
            .set(Param::WaitG(true))
            .set(Param::DurG(5))
            .set(Param::OffsetXRegM(5))
            .set(Param::OffsetYRegM(5))
            .set(Param::OffsetZRegM(5))
            .set(Param::OpMode(OpMode::UltraHighPerf))
            .set(Param::OutputDataRate(OutputDataRate::Odr5Hz))
            .set(Param::TempComp(true))
            .set(Param::SelfTest(true))
            .set(Param::FsM(FsM::Fs4))
            .set(Param::RebootM(true))
            .set(Param::SoftResetM(true))
            .set(Param::Md(Md::Single))
            .set(Param::I2cDisableM(true))
            .set(Param::LowPowerM(true))
            .set(Param::SimM(true))
            .set(Param::OpModeZ(OpMode::UltraHighPerf))
            .set(Param::BigLittleEndian(true))
            .set(Param::BlockDataUpdate(true))
            .set(Param::Xien(true))
            .set(Param::Yien(true))
            .set(Param::Zien(true))
            .set(Param::Iea(true))
            .set(Param::Iel(true))
            .set(Param::Ien(true))
            .build().expect("conf1 build failed");

        let conf2 = ConfRegBuilder::new().
            set_all(conf1.registers())
            .build().expect("conf2 build failed");

        assert_eq!(conf1, conf2);
        
            
    }
}
