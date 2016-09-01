use super::Address;

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

impl DataRate {
    fn default() -> DataRate {
        DataRate::PowerDown
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GyroScale {
    NA,
    FS245Dps,
    FS500Dps,
    FS2000Dps,
}

impl GyroScale {
    fn default() -> GyroScale {
        GyroScale::FS245Dps
    }
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bw {
    A,
    B,
    C,
    D,
}

impl Bw {
    fn default() -> Bw {
        Bw::A
    }
}

// /// Clarifications needed
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cutoff {
//     NA,
//     C5Hz,
//     C19Hz,
//     C38Hz,
//     C76Hz,
//     C100Hz,
//     CHz,
// }

// impl Cutoff {
//     fn default() -> Cutoff {
//         Cutoff::C5Hz
//     }
// }

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSel {
    A,
    B,
    C,
    D,
}

impl IntSel {
    fn default() -> IntSel {
        IntSel::A
    }
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutSel {
    A,
    B,
    C,
    D,
}

impl OutSel {
    fn default() -> OutSel {
        OutSel::A
    }
}

/// Clarifications needed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigCutoffFreq {
    A,
    B,
    C,
    D,
}

impl DigCutoffFreq {
    fn default() -> DigCutoffFreq {
        DigCutoffFreq::A
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dec {
    NoDec,
    Dec2S,
    Dec4S,
    Dec8S,
}

impl Dec {
    fn default() -> Dec {
        Dec::NoDec
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMode {
    ByPass,
    StopIfFull,
    ContinusTrig,
    ByPassTrig,
    Overwrite,
}

impl FMode {
    fn default() -> FMode {
        FMode::ByPass
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpMode {
    LowPower,
    MediumPerf,
    HighPerf,
    UltraHighPerf,
}

impl OpMode {
    fn default() -> OpMode {
        OpMode::LowPower
    }
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

impl OutputDataRate {
    fn default() -> OutputDataRate {
        OutputDataRate::Odr0p625Hz
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsM {
    Fs4,
    Fs8,
    Fs12,
    Fs16,
}

impl FsM {
    fn default() -> FsM {
        FsM::Fs4
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsXl {
    Fs2,
    Fs4,
    Fs8,
    Fs16,
}

impl FsXl {
    fn default() -> FsXl {
        FsXl::Fs2
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Md {
    Continuous,
    Single,
    PowerDown,
}

impl Md {
    fn default() -> Md {
        Md::PowerDown
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

#[derive(Clone, Debug, PartialEq)]
pub struct PatchConfig {
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub act_ths: act_ths::ActThs,
    pub act_dur: act_dur::ActDur,
    pub int_gen_cfg_xl: int_gen_cfg_xl::IntGenCfgXl,
    pub int_gen_ths_x_xl: int_gen_ths_x_xl::IntGenThsXXl,
    pub int_gen_ths_y_xl: int_gen_ths_y_xl::IntGenThsYXl,
    pub int_gen_ths_z_xl: int_gen_ths_z_xl::IntGenThsZXl,
    pub int_gen_dur_xl: int_gen_dur_xl::IntGenDurXl,
    pub reference_g: reference_g::ReferenceG,
    pub int1_ctrl: int1_ctrl::Int1Ctrl,
    pub int2_ctrl: int2_ctrl::Int2Ctrl,
    pub ctrl_reg1_g: ctrl_reg1_g::CtrlReg1G,
    pub ctrl_reg2_g: ctrl_reg2_g::CtrlReg2G,
    pub ctrl_reg3_g: ctrl_reg3_g::CtrlReg3G,
    pub orient_cfg_g: orient_cfg_g::OrientCfgG,
    pub ctrl_reg4: ctrl_reg4::CtrlReg4,
    pub ctrl_reg5_xl: ctrl_reg5_xl::CtrlReg5XL,
    pub ctrl_reg6_xl: ctrl_reg6_xl::CtrlReg6XL,
    pub ctrl_reg7_xl: ctrl_reg7_xl::CtrlReg7XL,
    pub ctrl_reg8: ctrl_reg8::CtrlReg8,
    pub ctrl_reg9: ctrl_reg9::CtrlReg9,
    pub ctrl_reg10: ctrl_reg10::CtrlReg10,
    pub fifo_ctrl: fifo_ctrl::FifoCtrl,
    pub int_gen_cfg_g: int_gen_cfg_g::IntGenCfgG,
    pub int_gen_ths_x_g: int_gen_ths_x_g::IntGenThsXG,
    pub int_gen_ths_y_g: int_gen_ths_y_g::IntGenThsYG,
    pub int_gen_ths_z_g: int_gen_ths_z_g::IntGenThsZG,
    pub int_gen_dur_g: int_gen_dur_g::IntGenDurG,
    pub offset_x_reg_m: offset_x_reg_m::OffsetXRegM,
    pub offset_y_reg_m: offset_y_reg_m::OffsetYRegM,
    pub offset_z_reg_m: offset_z_reg_m::OffsetZRegM,
    pub ctrl_reg1_m: ctrl_reg1_m::CtrlReg1M,
    pub ctrl_reg2_m: ctrl_reg2_m::CtrlReg2M,
    pub ctrl_reg3_m: ctrl_reg3_m::CtrlReg3M,
    pub ctrl_reg4_m: ctrl_reg4_m::CtrlReg4M,
    pub ctrl_reg5_m: ctrl_reg5_m::CtrlReg5M,
    pub int_cfg_m: int_cfg_m::IntCfgM,
}

impl Config {
    fn default() -> Config {
        Config {
            act_ths: act_ths::ActThs::default(),
            act_dur: act_dur::ActDur::default(),
            int_gen_cfg_xl: int_gen_cfg_xl::IntGenCfgXl::default(),
            int_gen_ths_x_xl: int_gen_ths_x_xl::IntGenThsXXl::default(),
            int_gen_ths_y_xl: int_gen_ths_y_xl::IntGenThsYXl::default(),
            int_gen_ths_z_xl: int_gen_ths_z_xl::IntGenThsZXl::default(),
            int_gen_dur_xl: int_gen_dur_xl::IntGenDurXl::default(),
            reference_g: reference_g::ReferenceG::default(),
            int1_ctrl: int1_ctrl::Int1Ctrl::default(),
            int2_ctrl: int2_ctrl::Int2Ctrl::default(),
            ctrl_reg1_g: ctrl_reg1_g::CtrlReg1G::default(),
            ctrl_reg2_g: ctrl_reg2_g::CtrlReg2G::default(),
            ctrl_reg3_g: ctrl_reg3_g::CtrlReg3G::default(),
            orient_cfg_g: orient_cfg_g::OrientCfgG::default(),
            ctrl_reg4: ctrl_reg4::CtrlReg4::default(),
            ctrl_reg5_xl: ctrl_reg5_xl::CtrlReg5XL::default(),
            ctrl_reg6_xl: ctrl_reg6_xl::CtrlReg6XL::default(),
            ctrl_reg7_xl: ctrl_reg7_xl::CtrlReg7XL::default(),
            ctrl_reg8: ctrl_reg8::CtrlReg8::default(),
            ctrl_reg9: ctrl_reg9::CtrlReg9::default(),
            ctrl_reg10: ctrl_reg10::CtrlReg10::default(),
            fifo_ctrl: fifo_ctrl::FifoCtrl::default(),
            int_gen_cfg_g: int_gen_cfg_g::IntGenCfgG::default(),
            int_gen_ths_x_g: int_gen_ths_x_g::IntGenThsXG::default(),
            int_gen_ths_y_g: int_gen_ths_y_g::IntGenThsYG::default(),
            int_gen_ths_z_g: int_gen_ths_z_g::IntGenThsZG::default(),
            int_gen_dur_g: int_gen_dur_g::IntGenDurG::default(),
            offset_x_reg_m: offset_x_reg_m::OffsetXRegM::default(),
            offset_y_reg_m: offset_y_reg_m::OffsetYRegM::default(),
            offset_z_reg_m: offset_z_reg_m::OffsetZRegM::default(),
            ctrl_reg1_m: ctrl_reg1_m::CtrlReg1M::default(),
            ctrl_reg2_m: ctrl_reg2_m::CtrlReg2M::default(),
            ctrl_reg3_m: ctrl_reg3_m::CtrlReg3M::default(),
            ctrl_reg4_m: ctrl_reg4_m::CtrlReg4M::default(),
            ctrl_reg5_m: ctrl_reg5_m::CtrlReg5M::default(),
            int_cfg_m: int_cfg_m::IntCfgM::default(),
        }
    }

    pub fn act_ths(&self) -> u8 {self.act_ths.act_ths()}
    pub fn set_act_ths(&mut self, value: u8) {self.act_ths.set_act_ths(value);}
    pub fn sleep_on(&self) -> bool {self.act_ths.sleep_on()}
    pub fn set_sleep_on(&mut self, value: bool) {self.act_ths.set_sleep_on(value);}
    pub fn act_dur(&self) -> u8 {self.act_dur.act_dur()}
    pub fn set_act_dur(&mut self, value: u8) {self.act_dur.set_act_dur(value);}
    pub fn aoi_xl(&self) -> bool {self.int_gen_cfg_xl.aoi_xl()}
    pub fn set_aoi_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_aoi_xl(value)}
    pub fn detect_6d(&self) -> bool {self.int_gen_cfg_xl.detect_6d()}
    pub fn set_detect_6d(&mut self, value: bool) {self.int_gen_cfg_xl.set_detect_6d(value)}
    pub fn zhie_xl(&self) -> bool {self.int_gen_cfg_xl.zhie_xl()}
    pub fn set_zhie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_zhie_xl(value)}
    pub fn zlie_xl(&self) -> bool {self.int_gen_cfg_xl.zlie_xl()}
    pub fn set_zlie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_zlie_xl(value)}
    pub fn yhie_xl(&self) -> bool {self.int_gen_cfg_xl.yhie_xl()}
    pub fn set_yhie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_yhie_xl(value)}
    pub fn ylie_xl(&self) -> bool {self.int_gen_cfg_xl.ylie_xl()}
    pub fn set_ylie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_ylie_xl(value)}
    pub fn xhie_xl(&self) -> bool {self.int_gen_cfg_xl.xhie_xl()}
    pub fn set_xhie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_xhie_xl(value)}
    pub fn xlie_xl(&self) -> bool {self.int_gen_cfg_xl.xlie_xl()}
    pub fn set_xlie_xl(&mut self, value: bool) {self.int_gen_cfg_xl.set_xlie_xl(value)}
    pub fn int_gen_ths_x_xl(&self) -> u8 {self.int_gen_ths_x_xl.int_gen_ths_x_xl()}
    pub fn set_int_gen_ths_x_xl(&mut self, value: u8) {self.int_gen_ths_x_xl.set_int_gen_ths_x_xl(value)}
    pub fn int_gen_ths_y_xl(&self) -> u8 {self.int_gen_ths_y_xl.int_gen_ths_y_xl()}
    pub fn set_int_gen_ths_y_xl(&mut self, value: u8) {self.int_gen_ths_y_xl.set_int_gen_ths_y_xl(value)}
    pub fn int_gen_ths_z_xl(&self) -> u8 {self.int_gen_ths_z_xl.int_gen_ths_z_xl()}
    pub fn set_int_gen_ths_z_xl(&mut self, value: u8) {self.int_gen_ths_z_xl.set_int_gen_ths_z_xl(value)}
    pub fn wait_xl(&self) -> bool {self.int_gen_dur_xl.wait_xl()}
    pub fn set_wait_xl(&mut self, value: bool) {self.int_gen_dur_xl.set_wait_xl(value)}
    pub fn dur_xl(&self) -> u8 {self.int_gen_dur_xl.dur_xl()}
    pub fn set_dur_xl(&mut self, value: u8) {self.int_gen_dur_xl.set_dur_xl(value)}
    pub fn reference_g(&self) -> u8 {self.reference_g.reference_g()}
    pub fn set_reference_g(&mut self, value: u8) {self.reference_g.set_reference_g(value)}
    pub fn int1_ig_g(&self) -> bool {self.int1_ctrl.int1_ig_g()}
    pub fn set_int1_ig_g(&mut self, value: bool) {self.int1_ctrl.set_int1_ig_g(value)}
    pub fn int1_ig_xl(&self) -> bool {self.int1_ctrl.int1_ig_xl()}
    pub fn set_int1_ig_xl(&mut self, value: bool) {self.int1_ctrl.set_int1_ig_xl(value)}
    pub fn int1_fss_5(&self) -> bool {self.int1_ctrl.int1_fss_5()}
    pub fn set_int1_fss_5(&mut self, value: bool) {self.int1_ctrl.set_int1_fss_5(value)}
    pub fn int1_ovr(&self) -> bool {self.int1_ctrl.int1_ovr()}
    pub fn set_int1_ovr(&mut self, value: bool) {self.int1_ctrl.set_int1_ovr(value)}
    pub fn int1_fth(&self) -> bool {self.int1_ctrl.int1_fth()}
    pub fn set_int1_fth(&mut self, value: bool) {self.int1_ctrl.set_int1_fth(value)}
    pub fn int1_boot(&self) -> bool {self.int1_ctrl.int1_boot()}
    pub fn set_int1_boot(&mut self, value: bool) {self.int1_ctrl.set_int1_boot(value)}
    pub fn int1_drdy_g(&self) -> bool {self.int1_ctrl.int1_drdy_g()}
    pub fn set_int1_drdy_g(&mut self, value: bool) {self.int1_ctrl.set_int1_drdy_g(value)}
    pub fn int1_drdy_xl(&self) -> bool {self.int1_ctrl.int1_drdy_xl()}
    pub fn set_int1_drdy_xl(&mut self, value: bool) {self.int1_ctrl.set_int1_drdy_xl(value)}
    pub fn int2_inact(&self) -> bool {self.int2_ctrl.int2_inact()}
    pub fn set_int2_inact(&mut self, value: bool) {self.int2_ctrl.set_int2_inact(value)}
    pub fn int2_fss_5(&self) -> bool {self.int2_ctrl.int2_fss_5()}
    pub fn set_int2_fss_5(&mut self, value: bool) {self.int2_ctrl.set_int2_fss_5(value)}
    pub fn int2_ovr(&self) -> bool {self.int2_ctrl.int2_ovr()}
    pub fn set_int2_ovr(&mut self, value: bool) {self.int2_ctrl.set_int2_ovr(value)}
    pub fn int2_fth(&self) -> bool {self.int2_ctrl.int2_fth()}
    pub fn set_int2_fth(&mut self, value: bool) {self.int2_ctrl.set_int2_fth(value)}
    pub fn int2_drdy_temp(&self) -> bool {self.int2_ctrl.int2_drdy_temp()}
    pub fn set_int2_drdy_temp(&mut self, value: bool) {self.int2_ctrl.set_int2_drdy_temp(value)}
    pub fn int2_drdy_g(&self) -> bool {self.int2_ctrl.int2_drdy_g()}
    pub fn set_int2_drdy_g(&mut self, value: bool) {self.int2_ctrl.set_int2_drdy_g(value)}
    pub fn int2_drdy_xl(&self) -> bool {self.int2_ctrl.int2_drdy_xl()}
    pub fn set_int2_drdy_xl(&mut self, value: bool) {self.int2_ctrl.set_int2_drdy_xl(value)}
    pub fn odr_g(&self) -> DataRate {self.ctrl_reg1_g.odr_g()}
    pub fn set_odr_g(&mut self, value: DataRate) {self.ctrl_reg1_g.set_odr_g(value)}
    pub fn fs_g(&self) -> GyroScale {self.ctrl_reg1_g.fs_g()}
    pub fn set_fs_g(&mut self, value: GyroScale) {self.ctrl_reg1_g.set_fs_g(value)}
    pub fn bw_g(&self) -> Bw {self.ctrl_reg1_g.bw_g()}
    pub fn set_bw_g(&mut self, value: Bw) {self.ctrl_reg1_g.set_bw_g(value)}
    pub fn int_sel(&self) -> IntSel {self.ctrl_reg2_g.int_sel()}
    pub fn set_int_sel(&mut self, value: IntSel) {self.ctrl_reg2_g.set_int_sel(value)}
    pub fn out_sel(&self) -> OutSel {self.ctrl_reg2_g.out_sel()}
    pub fn set_out_sel(&mut self, value: OutSel) {self.ctrl_reg2_g.set_out_sel(value)}
    pub fn lp_mode(&self) -> bool {self.ctrl_reg3_g.lp_mode()}
    pub fn set_lp_mode(&mut self, value: bool) {self.ctrl_reg3_g.set_lp_mode(value)}
    pub fn hp_en(&self) -> bool {self.ctrl_reg3_g.hp_en()}
    pub fn set_hp_en(&mut self, value: bool) {self.ctrl_reg3_g.set_hp_en(value)}
    pub fn hpcf_g(&self) -> u8 {self.ctrl_reg3_g.hpcf_g()}
    pub fn set_hpcf_g(&mut self, value: u8) {self.ctrl_reg3_g.set_hpcf_g(value)}
    pub fn sign_x_g(&self) -> bool {self.orient_cfg_g.sign_x_g()}
    pub fn set_sign_x_g(&mut self, value: bool) {self.orient_cfg_g.set_sign_x_g(value)}
    pub fn sign_y_g(&self) -> bool {self.orient_cfg_g.sign_y_g()}
    pub fn set_sign_y_g(&mut self, value: bool) {self.orient_cfg_g.set_sign_y_g(value)}
    pub fn sign_z_g(&self) -> bool {self.orient_cfg_g.sign_z_g()}
    pub fn set_sign_z_g(&mut self, value: bool) {self.orient_cfg_g.set_sign_z_g(value)}
    pub fn orient(&self) -> u8 {self.orient_cfg_g.orient()}
    pub fn set_orient(&mut self, value: u8) {self.orient_cfg_g.set_orient(value)}
    pub fn zen_g(&self) -> bool {self.ctrl_reg4.zen_g()}
    pub fn set_zen_g(&mut self, value: bool) {self.ctrl_reg4.set_zen_g(value)}
    pub fn yen_g(&self) -> bool {self.ctrl_reg4.yen_g()}
    pub fn set_yen_g(&mut self, value: bool) {self.ctrl_reg4.set_yen_g(value)}
    pub fn xen_g(&self) -> bool {self.ctrl_reg4.xen_g()}
    pub fn set_xen_g(&mut self, value: bool) {self.ctrl_reg4.set_xen_g(value)}
    pub fn lir_xl_1(&self) -> bool {self.ctrl_reg4.lir_xl_1()}
    pub fn set_lir_xl_1(&mut self, value: bool) {self.ctrl_reg4.set_lir_xl_1(value)}
    pub fn i_4d_xl_1(&self) -> bool {self.ctrl_reg4.i_4d_xl_1()}
    pub fn set_i_4d_xl_1(&mut self, value: bool) {self.ctrl_reg4.set_i_4d_xl_1(value)}
    pub fn zen_xl(&self) -> bool {self.ctrl_reg5_xl.zen_xl()}
    pub fn set_zen_xl(&mut self, value: bool) {self.ctrl_reg5_xl.set_zen_xl(value)}
    pub fn yen_xl(&self) -> bool {self.ctrl_reg5_xl.yen_xl()}
    pub fn set_yen_xl(&mut self, value: bool) {self.ctrl_reg5_xl.set_yen_xl(value)}
    pub fn xen_xl(&self) -> bool {self.ctrl_reg5_xl.xen_xl()}
    pub fn set_xen_xl(&mut self, value: bool) {self.ctrl_reg5_xl.set_xen_xl(value)}
    pub fn dec(&self) -> Dec {self.ctrl_reg5_xl.dec()}
    pub fn set_dec(&mut self, value: Dec) {self.ctrl_reg5_xl.set_dec(value)}
    pub fn odr_xl(&self) -> DataRate {self.ctrl_reg6_xl.odr_xl()}
    pub fn set_odr_xl(&mut self, value: DataRate) {self.ctrl_reg6_xl.set_odr_xl(value)}
    pub fn fs_xl(&self) -> FsXl {self.ctrl_reg6_xl.fs_xl()}
    pub fn set_fs_xl(&mut self, value: FsXl) {self.ctrl_reg6_xl.set_fs_xl(value)}
    pub fn bw_scal_odr(&self) -> bool {self.ctrl_reg6_xl.bw_scal_odr()}
    pub fn set_bw_scal_odr(&mut self, value: bool) {self.ctrl_reg6_xl.set_bw_scal_odr(value)}
    pub fn bw_xl(&self) -> Bw {self.ctrl_reg6_xl.bw_xl()}
    pub fn set_bw_xl(&mut self, value: Bw) {self.ctrl_reg6_xl.set_bw_xl(value)}
    pub fn high_res(&self) -> bool {self.ctrl_reg7_xl.high_res()}
    pub fn set_high_res(&mut self, value: bool) {self.ctrl_reg7_xl.set_high_res(value)}
    pub fn xl_digital_cf(&self) -> DigCutoffFreq {self.ctrl_reg7_xl.xl_digital_cf()}
    pub fn set_xl_digital_cf(&mut self, value: DigCutoffFreq) {self.ctrl_reg7_xl.set_xl_digital_cf(value)}
    pub fn filtered_data_sel(&self) -> bool {self.ctrl_reg7_xl.filtered_data_sel()}
    pub fn set_filtered_data_sel(&mut self, value: bool) {self.ctrl_reg7_xl.set_filtered_data_sel(value)}
    pub fn high_pass_int_sens(&self) -> bool {self.ctrl_reg7_xl.high_pass_int_sens()}
    pub fn set_high_pass_int_sens(&mut self, value: bool) {self.ctrl_reg7_xl.set_high_pass_int_sens(value)}
    pub fn boot(&self) -> bool {self.ctrl_reg8.boot()}
    pub fn set_boot(&mut self, value: bool) {self.ctrl_reg8.set_boot(value)}
    pub fn bdu(&self) -> bool {self.ctrl_reg8.bdu()}
    pub fn set_bdu(&mut self, value: bool) {self.ctrl_reg8.set_bdu(value)}
    pub fn hl_active(&self) -> bool {self.ctrl_reg8.hl_active()}
    pub fn set_hl_active(&mut self, value: bool) {self.ctrl_reg8.set_hl_active(value)}
    pub fn pp_od(&self) -> bool {self.ctrl_reg8.pp_od()}
    pub fn set_pp_od(&mut self, value: bool) {self.ctrl_reg8.set_pp_od(value)}
    pub fn sim(&self) -> bool {self.ctrl_reg8.sim()}
    pub fn set_sim(&mut self, value: bool) {self.ctrl_reg8.set_sim(value)}
    pub fn if_add_inc(&self) -> bool {self.ctrl_reg8.if_add_inc()}
    pub fn set_if_add_inc(&mut self, value: bool) {self.ctrl_reg8.set_if_add_inc(value)}
    pub fn ble(&self) -> bool {self.ctrl_reg8.ble()}
    pub fn set_ble(&mut self, value: bool) {self.ctrl_reg8.set_ble(value)}
    pub fn sw_reset(&self) -> bool {self.ctrl_reg8.sw_reset()}
    pub fn set_sw_reset(&mut self, value: bool) {self.ctrl_reg8.set_sw_reset(value)}
    pub fn sleep_g(&self) -> bool {self.ctrl_reg9.sleep_g()}
    pub fn set_sleep_g(&mut self, value: bool) {self.ctrl_reg9.set_sleep_g(value)}
    pub fn fifo_temp_en(&self) -> bool {self.ctrl_reg9.fifo_temp_en()}
    pub fn set_fifo_temp_en(&mut self, value: bool) {self.ctrl_reg9.set_fifo_temp_en(value)}
    pub fn drdy_mask_bit(&self) -> bool {self.ctrl_reg9.drdy_mask_bit()}
    pub fn set_drdy_mask_bit(&mut self, value: bool) {self.ctrl_reg9.set_drdy_mask_bit(value)}
    pub fn i2c_disable(&self) -> bool {self.ctrl_reg9.i2c_disable()}
    pub fn set_i2c_disable(&mut self, value: bool) {self.ctrl_reg9.set_i2c_disable(value)}
    pub fn fifo_en(&self) -> bool {self.ctrl_reg9.fifo_en()}
    pub fn set_fifo_en(&mut self, value: bool) {self.ctrl_reg9.set_fifo_en(value)}
    pub fn stop_on_fth(&self) -> bool {self.ctrl_reg9.stop_on_fth()}
    pub fn set_stop_on_fth(&mut self, value: bool) {self.ctrl_reg9.set_stop_on_fth(value)}
    pub fn st_g(&self) -> bool {self.ctrl_reg10.st_g()}
    pub fn set_st_g(&mut self, value: bool) {self.ctrl_reg10.set_st_g(value)}
    pub fn st_xl(&self) -> bool {self.ctrl_reg10.st_xl()}
    pub fn set_st_xl(&mut self, value: bool) {self.ctrl_reg10.set_st_xl(value)}
    pub fn fth(&self) -> u8 {self.fifo_ctrl.fth()}
    pub fn set_fth(&mut self, value: u8) {self.fifo_ctrl.set_fth(value)}
    pub fn f_mode(&self) -> FMode {self.fifo_ctrl.f_mode()}
    pub fn set_f_mode(&mut self, value: FMode) {self.fifo_ctrl.set_f_mode(value)}
    pub fn aoi_g(&self) -> bool {self.int_gen_cfg_g.aoi_g()}
    pub fn set_aoi_g(&mut self, value: bool) {self.int_gen_cfg_g.set_aoi_g(value)}
    pub fn lir_g(&self) -> bool {self.int_gen_cfg_g.lir_g()}
    pub fn set_lir_g(&mut self, value: bool) {self.int_gen_cfg_g.set_lir_g(value)}
    pub fn zhie_g(&self) -> bool {self.int_gen_cfg_g.zhie_g()}
    pub fn set_zhie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_zhie_g(value)}
    pub fn zlie_g(&self) -> bool {self.int_gen_cfg_g.zlie_g()}
    pub fn set_zlie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_zlie_g(value)}
    pub fn yhie_g(&self) -> bool {self.int_gen_cfg_g.yhie_g()}
    pub fn set_yhie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_yhie_g(value)}
    pub fn ylie_g(&self) -> bool {self.int_gen_cfg_g.ylie_g()}
    pub fn set_ylie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_ylie_g(value)}
    pub fn xhie_g(&self) -> bool {self.int_gen_cfg_g.xhie_g()}
    pub fn set_xhie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_xhie_g(value)}
    pub fn xlie_g(&self) -> bool {self.int_gen_cfg_g.xlie_g()}
    pub fn set_xlie_g(&mut self, value: bool) {self.int_gen_cfg_g.set_xlie_g(value)}
    pub fn dcrm_g(&self) -> bool {self.int_gen_ths_x_g.dcrm_g()}
    pub fn set_dcrm_g(&mut self, value: bool) {self.int_gen_ths_x_g.set_dcrm_g(value)}
    pub fn int_gen_ths_x_g(&self) -> u16 {self.int_gen_ths_x_g.int_gen_ths_x_g()}
    pub fn set_int_gen_ths_x_g(&mut self, value: u16) {self.int_gen_ths_x_g.set_int_gen_ths_x_g(value)}
    pub fn int_gen_ths_y_g(&self) -> u16 {self.int_gen_ths_y_g.int_gen_ths_y_g()}
    pub fn set_int_gen_ths_y_g(&mut self, value: u16) {self.int_gen_ths_y_g.set_int_gen_ths_y_g(value)}
    pub fn int_gen_ths_z_g(&self) -> u16 {self.int_gen_ths_z_g.int_gen_ths_z_g()}
    pub fn set_int_gen_ths_z_g(&mut self, value: u16) {self.int_gen_ths_z_g.set_int_gen_ths_z_g(value)}
    pub fn wait_g(&self) -> bool {self.int_gen_dur_g.wait_g()}
    pub fn set_wait_g(&mut self, value: bool) {self.int_gen_dur_g.set_wait_g(value)}
    pub fn dur_g(&self) -> u8 {self.int_gen_dur_g.dur_g()}
    pub fn set_dur_g(&mut self, value: u8) {self.int_gen_dur_g.set_dur_g(value)}
    pub fn offset_x_reg_m(&self) -> u16 {self.offset_x_reg_m.offset_x_reg_m()}
    pub fn set_offset_x_reg_m(&mut self, value: u16) {self.offset_x_reg_m.set_offset_x_reg_m(value)}
    pub fn offset_y_reg_m(&self) -> u16 {self.offset_y_reg_m.offset_y_reg_m()}
    pub fn set_offset_y_reg_m(&mut self, value: u16) {self.offset_y_reg_m.set_offset_y_reg_m(value)}
    pub fn offset_z_reg_m(&self) -> u16 {self.offset_z_reg_m.offset_z_reg_m()}
    pub fn set_offset_z_reg_m(&mut self, value: u16) {self.offset_z_reg_m.set_offset_z_reg_m(value)}
    pub fn op_mode(&self) -> OpMode {self.ctrl_reg1_m.op_mode()}
    pub fn set_op_mode(&mut self, value: OpMode) {self.ctrl_reg1_m.set_op_mode(value)}
    pub fn output_data_rate(&self) -> OutputDataRate {self.ctrl_reg1_m.output_data_rate()}
    pub fn set_output_data_rate(&mut self, value: OutputDataRate) {self.ctrl_reg1_m.set_output_data_rate(value)}
    pub fn temp_comp(&self) -> bool {self.ctrl_reg1_m.temp_comp()}
    pub fn set_temp_comp(&mut self, value: bool) {self.ctrl_reg1_m.set_temp_comp(value)}
    pub fn self_test(&self) -> bool {self.ctrl_reg1_m.self_test()}
    pub fn set_self_test(&mut self, value: bool) {self.ctrl_reg1_m.set_self_test(value)}
    pub fn fs_m(&self) -> FsM {self.ctrl_reg2_m.fs_m()}
    pub fn set_fs_m(&mut self, value: FsM) {self.ctrl_reg2_m.set_fs_m(value)}
    pub fn reboot_m(&self) -> bool {self.ctrl_reg2_m.reboot_m()}
    pub fn set_reboot_m(&mut self, value: bool) {self.ctrl_reg2_m.set_reboot_m(value)}
    pub fn soft_reset_m(&self) -> bool {self.ctrl_reg2_m.soft_reset_m()}
    pub fn set_soft_reset_m(&mut self, value: bool) {self.ctrl_reg2_m.set_soft_reset_m(value)}
    pub fn md(&self) -> Md {self.ctrl_reg3_m.md()}
    pub fn set_md(&mut self, value: Md) {self.ctrl_reg3_m.set_md(value)}
    pub fn i2c_disable_m(&self) -> bool {self.ctrl_reg3_m.i2c_disable_m()}
    pub fn set_i2c_disable_m(&mut self, value: bool) {self.ctrl_reg3_m.set_i2c_disable_m(value)}
    pub fn low_power_m(&self) -> bool {self.ctrl_reg3_m.low_power_m()}
    pub fn set_low_power_m(&mut self, value: bool) {self.ctrl_reg3_m.set_low_power_m(value)}
    pub fn sim_m(&self) -> bool {self.ctrl_reg3_m.sim_m()}
    pub fn set_sim_m(&mut self, value: bool) {self.ctrl_reg3_m.set_sim_m(value)}
    pub fn op_mode_z(&self) -> OpMode {self.ctrl_reg4_m.op_mode_z()}
    pub fn set_op_mode_z(&mut self, value: OpMode) {self.ctrl_reg4_m.set_op_mode_z(value)}
    pub fn big_little_endian(&self) -> bool {self.ctrl_reg4_m.big_little_endian()}
    pub fn set_big_little_endian(&mut self, value: bool) {self.ctrl_reg4_m.set_big_little_endian(value)}
    pub fn block_data_update(&self) -> bool {self.ctrl_reg5_m.block_data_update()}
    pub fn set_block_data_update(&mut self, value: bool) {self.ctrl_reg5_m.set_block_data_update(value)}
    pub fn xien(&self) -> bool {self.int_cfg_m.xien()}
    pub fn set_xien(&mut self, value: bool) {self.int_cfg_m.set_xien(value)}
    pub fn yien(&self) -> bool {self.int_cfg_m.yien()}
    pub fn set_yien(&mut self, value: bool) {self.int_cfg_m.set_yien(value)}
    pub fn zien(&self) -> bool {self.int_cfg_m.zien()}
    pub fn set_zien(&mut self, value: bool) {self.int_cfg_m.set_zien(value)}
    pub fn iea(&self) -> bool {self.int_cfg_m.iea()}
    pub fn set_iea(&mut self, value: bool) {self.int_cfg_m.set_iea(value)}
    pub fn iel(&self) -> bool {self.int_cfg_m.iel()}
    pub fn set_iel(&mut self, value: bool) {self.int_cfg_m.set_iel(value)}
    pub fn ien(&self) -> bool {self.int_cfg_m.ien()}
    pub fn set_ien(&mut self, value: bool) {self.int_cfg_m.set_ien(value)}
}

pub trait Register<RegSize> {
    fn addr(&self) -> Address;
    
    fn default() -> Self;

    fn new(reg: RegSize) -> Self;

    fn reg(&self) -> RegSize;
}



#[cfg(test)]
mod tests {
    // use super::{ConfParamBuilder, ConfRegBuilder, Param,
    //             DataRate, GyroScale, Bw, IntSel, OutSel, Dec, DigCutoffFreq,
    //             FsXl, FMode, OpMode, FsM, Md, OutputDataRate};
    
    #[test]
    fn it_works() {
        unimplemented!();
            
    }
}
