// TODO: Waitning for: #![feature(associated_consts)] in main stream

#![allow(dead_code)]
// extern crate i2cdev;
// #[macro_use]
// extern crate bitflags;


// mod register;
pub mod config;
use config::{
    Config, PatchConfig, Register,
    DataRate,
    GyroScale,
    Bw,
    IntSel,
    OutSel,
    DigCutoffFreq,
    Dec,
    FMode,
    OpMode,
    OutputDataRate,
    FsM,
    FsXl,
    Md,
};
// use config::{Config, Param, Register};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address{
    RW(u8),
    RW16(u8),
    R(u8),
    R16(u8),
}



// use register::{Address};

// pub trait Lsm9ds1Device {
//     fn read(&mut self, address: Address) -> Result<u8,()>;
//     fn readword(&mut self, address: Address) -> Result<u8,()>;
//     fn write(&mut self, address: Address, value: u8) -> Result<(),()>;
//     fn writeword(&mut self, address: Address, value: u16) -> Result<(),()>;
// }


pub struct Lsm9ds1<I: Interface> {
    config: Config,
    interface: I
}

enum Interrupts{}
// enum ParamType{
    
// }
// enum Param{}

impl<I: Interface> Lsm9ds1<I> {
    pub fn temp(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn lx(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn ly(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn lz(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn gx(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn gy(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn gz(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn mx(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn my(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn mz(&mut self) -> Result<f32,()> { unimplemented!() }
    pub fn fifo(&mut self) -> Result<f32,()> { unimplemented!() }
    // pub fn linterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }
    // pub fn ginterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }

    pub fn apply_config(&mut self, config: Config) -> Result<(),()> { unimplemented!() }
    pub fn apply_patch_config(&mut self, patch: PatchConfig) -> Result<(),()> { unimplemented!() }
    pub fn read_config(&mut self) -> Result<(),()> { unimplemented!() }
    

    // fn status(&mut self) -> Result<Status,()> { unimplemented!() }
    // fn all(&mut self) -> MesureIterator

    pub fn act_ths(&self) -> u8 {self.config.act_ths()}
    pub fn set_act_ths(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.act_ths.clone();
        reg.set_act_ths(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.act_ths = reg;
        Ok(())
    }
    
    pub fn sleep_on(&self) -> bool {self.config.sleep_on()}
    pub fn set_sleep_on(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.act_ths.clone();
        reg.set_sleep_on(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.act_ths = reg;
        Ok(())
    }
    
    pub fn act_dur(&self) -> u8 {self.config.act_dur()}
    pub fn set_act_dur(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.act_dur.clone();
        reg.set_act_dur(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.act_dur = reg;
        Ok(())
    }
    
    pub fn aoi_xl(&self) -> bool {self.config.aoi_xl()}
    pub fn set_aoi_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_aoi_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn detect_6d(&self) -> bool {self.config.detect_6d()}
    pub fn set_detect_6d(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_detect_6d(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn zhie_xl(&self) -> bool {self.config.zhie_xl()}
    pub fn set_zhie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_zhie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn zlie_xl(&self) -> bool {self.config.zlie_xl()}
    pub fn set_zlie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_zlie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn yhie_xl(&self) -> bool {self.config.yhie_xl()}
    pub fn set_yhie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_yhie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn ylie_xl(&self) -> bool {self.config.ylie_xl()}
    pub fn set_ylie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_yhie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn xhie_xl(&self) -> bool {self.config.xhie_xl()}
    pub fn set_xhie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_xhie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn xlie_xl(&self) -> bool {self.config.xlie_xl()}
    pub fn set_xlie_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_cfg_xl.clone();
        reg.set_xlie_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_cfg_xl = reg;
        Ok(())
    }
    
    pub fn int_gen_ths_x_xl(&self) -> u8 {self.config.int_gen_ths_x_xl()}
    pub fn set_int_gen_ths_x_xl(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.int_gen_ths_x_xl.clone();
        reg.set_int_gen_ths_x_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_ths_x_xl = reg;
        Ok(())
    }
        
    pub fn int_gen_ths_y_xl(&self) -> u8 {self.config.int_gen_ths_y_xl()}
    pub fn set_int_gen_ths_y_xl(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.int_gen_ths_y_xl.clone();
        reg.set_int_gen_ths_y_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_ths_y_xl = reg;
        Ok(())
    }
    
    pub fn int_gen_ths_z_xl(&self) -> u8 {self.config.int_gen_ths_z_xl()}
    pub fn set_int_gen_ths_z_xl(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.int_gen_ths_z_xl.clone();
        reg.set_int_gen_ths_z_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_ths_z_xl = reg;
        Ok(())
    }
    
    pub fn wait_xl(&self) -> bool {self.config.wait_xl()}
    pub fn set_wait_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int_gen_dur_xl.clone();
        reg.set_wait_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_dur_xl = reg;
        Ok(())
    }
    
    pub fn dur_xl(&self) -> u8 {self.config.dur_xl()}
    pub fn set_dur_xl(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.int_gen_dur_xl.clone();
        reg.set_dur_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int_gen_dur_xl = reg;
        Ok(())
    }
    
    pub fn reference_g(&self) -> u8 {self.config.reference_g()}
    pub fn set_reference_g(&mut self, value: u8) -> Result<(),()> {
        let mut reg = self.config.reference_g.clone();
        reg.set_reference_g(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.reference_g = reg;
        Ok(())
    }
    
    pub fn int1_ig_g(&self) -> bool {self.config.int1_ig_g()}
    pub fn set_int1_ig_g(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_ig_g(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_ig_xl(&self) -> bool {self.config.int1_ig_xl()}
    pub fn set_int1_ig_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_ig_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_fss_5(&self) -> bool {self.config.int1_fss_5()}
    pub fn set_int1_fss_5(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_fss_5(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_ovr(&self) -> bool {self.config.int1_ovr()}
    pub fn set_int1_ovr(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_ovr(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_fth(&self) -> bool {self.config.int1_fth()}
    pub fn set_int1_fth(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_fth(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_boot(&self) -> bool {self.config.int1_boot()}
    pub fn set_int1_boot(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_boot(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_drdy_g(&self) -> bool {self.config.int1_drdy_g()}
    pub fn set_int1_drdy_g(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_drdy_g(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int1_drdy_xl(&self) -> bool {self.config.int1_drdy_xl()}
    pub fn set_int1_drdy_xl(&mut self, value: bool) -> Result<(),()> {
        let mut reg = self.config.int1_ctrl.clone();
        reg.set_int1_drdy_xl(value);
        try!(self.interface.write(reg.addr(), reg.reg()));
        self.config.int1_ctrl = reg;
        Ok(())
    }
    
    pub fn int2_inact(&self) -> bool {self.config.int2_inact()}
    pub fn set_int2_inact(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_fss_5(&self) -> bool {self.config.int2_fss_5()}
    pub fn set_int2_fss_5(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_ovr(&self) -> bool {self.config.int2_ovr()}
    pub fn set_int2_ovr(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_fth(&self) -> bool {self.config.int2_fth()}
    pub fn set_int2_fth(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_drdy_temp(&self) -> bool {self.config.int2_drdy_temp()}
    pub fn set_int2_drdy_temp(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_drdy_g(&self) -> bool {self.config.int2_drdy_g()}
    pub fn set_int2_drdy_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn int2_drdy_xl(&self) -> bool {self.config.int2_drdy_xl()}
    pub fn set_int2_drdy_xl(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn odr_g(&self) -> DataRate {self.config.odr_g()}
    pub fn set_odr_g(&mut self, value: DataRate) -> Result<(),()> {unimplemented!()}
    
    pub fn fs_g(&self) -> GyroScale {self.config.fs_g()}
    pub fn set_fs_g(&mut self, value: GyroScale) -> Result<(),()> {unimplemented!()}
    
    pub fn bw_g(&self) -> Bw {self.config.bw_g()}
    pub fn set_bw_g(&mut self, value: Bw) -> Result<(),()> {unimplemented!()}
    
    pub fn int_sel(&self) -> IntSel {self.config.int_sel()}
    pub fn set_int_sel(&mut self, value: IntSel) -> Result<(),()> {unimplemented!()}
    
    pub fn out_sel(&self) -> OutSel {self.config.out_sel()}
    pub fn set_out_sel(&mut self, value: OutSel) -> Result<(),()> {unimplemented!()}
    
    pub fn lp_mode(&self) -> bool {self.config.lp_mode()}
    pub fn set_lp_mode(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn hp_en(&self) -> bool {self.config.hp_en()}
    pub fn set_hp_en(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn hpcf_g(&self) -> u8 {self.config.hpcf_g()}
    pub fn set_hpcf_g(&mut self, value: u8) -> Result<(),()> {unimplemented!()}
    
    pub fn sign_x_g(&self) -> bool {self.config.sign_x_g()}
    pub fn set_sign_x_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn sign_y_g(&self) -> bool {self.config.sign_y_g()}
    pub fn set_sign_y_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn sign_z_g(&self) -> bool {self.config.sign_z_g()}
    pub fn set_sign_z_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn orient(&self) -> u8 {self.config.orient()}
    pub fn set_orient(&mut self, value: u8) -> Result<(),()> {unimplemented!()}
    
    pub fn zen_g(&self) -> bool {self.config.zen_g()}
    pub fn set_zen_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn yen_g(&self) -> bool {self.config.yen_g()}
    pub fn set_yen_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn xen_g(&self) -> bool {self.config.xen_g()}
    pub fn set_xen_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn lir_xl_1(&self) -> bool {self.config.lir_xl_1()}
    pub fn set_lir_xl_1(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn i_4d_xl_1(&self) -> bool {self.config.i_4d_xl_1()}
    pub fn set_i_4d_xl_1(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn zen_xl(&self) -> bool {self.config.zen_xl()}
    pub fn set_zen_xl(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn yen_xl(&self) -> bool {self.config.yen_xl()}
    pub fn set_yen_xl(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn xen_xl(&self) -> bool {self.config.xen_xl()}
    pub fn set_xen_xl(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn dec(&self) -> Dec {self.config.dec()}
    pub fn set_dec(&mut self, value: Dec) -> Result<(),()> {unimplemented!()}
    
    pub fn odr_xl(&self) -> DataRate {self.config.odr_xl()}
    pub fn set_odr_xl(&mut self, value: DataRate) -> Result<(),()> {unimplemented!()}
    
    pub fn fs_xl(&self) -> FsXl {self.config.fs_xl()}
    pub fn set_fs_xl(&mut self, value: FsXl) -> Result<(),()> {unimplemented!()}
    
    pub fn bw_scal_odr(&self) -> bool {self.config.bw_scal_odr()}
    pub fn set_bw_scal_odr(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn bw_xl(&self) -> Bw {self.config.bw_xl()}
    pub fn set_bw_xl(&mut self, value: Bw) -> Result<(),()> {unimplemented!()}
    
    pub fn high_res(&self) -> bool {self.config.high_res()}
    pub fn set_high_res(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn xl_digital_cf(&self) -> DigCutoffFreq {self.config.xl_digital_cf()}
    pub fn set_xl_digital_cf(&mut self, value: DigCutoffFreq) -> Result<(),()> {unimplemented!()}
    
    pub fn filtered_data_sel(&self) -> bool {self.config.filtered_data_sel()}
    pub fn set_filtered_data_sel(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn high_pass_int_sens(&self) -> bool {self.config.high_pass_int_sens()}
    pub fn set_high_pass_int_sens(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn boot(&self) -> bool {self.config.boot()}
    pub fn set_boot(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn bdu(&self) -> bool {self.config.bdu()}
    pub fn set_bdu(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn hl_active(&self) -> bool {self.config.hl_active()}
    pub fn set_hl_active(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn pp_od(&self) -> bool {self.config.pp_od()}
    pub fn set_pp_od(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn sim(&self) -> bool {self.config.sim()}
    pub fn set_sim(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn if_add_inc(&self) -> bool {self.config.if_add_inc()}
    pub fn set_if_add_inc(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn ble(&self) -> bool {self.config.ble()}
    pub fn set_ble(&mut self, value: bool) -> Result<(),()> {unimplemented!()}
    
    pub fn sw_reset(&self) -> bool {self.config.sw_reset()}
    pub fn set_sw_reset(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn sleep_g(&self) -> bool {self.config.sleep_g()}
    pub fn set_sleep_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn fifo_temp_en(&self) -> bool {self.config.fifo_temp_en()}
    pub fn set_fifo_temp_en(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn drdy_mask_bit(&self) -> bool {self.config.drdy_mask_bit()}
    pub fn set_drdy_mask_bit(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn i2c_disable(&self) -> bool {self.config.i2c_disable()}
    pub fn set_i2c_disable(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn fifo_en(&self) -> bool {self.config.fifo_en()}
    pub fn set_fifo_en(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn stop_on_fth(&self) -> bool {self.config.stop_on_fth()}
    pub fn set_stop_on_fth(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn st_g(&self) -> bool {self.config.st_g()}
    pub fn set_st_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn st_xl(&self) -> bool {self.config.st_xl()}
    pub fn set_st_xl(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn fth(&self) -> u8 {self.config.fth()}
    pub fn set_fth(&mut self, value: u8) -> Result<(),()> {unimplemented!()}

    pub fn f_mode(&self) -> FMode {self.config.f_mode()}
    pub fn set_f_mode(&mut self, value: FMode) -> Result<(),()> {unimplemented!()}

    pub fn aoi_g(&self) -> bool {self.config.aoi_g()}
    pub fn set_aoi_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn lir_g(&self) -> bool {self.config.lir_g()}
    pub fn set_lir_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn zhie_g(&self) -> bool {self.config.zhie_g()}
    pub fn set_zhie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn zlie_g(&self) -> bool {self.config.zlie_g()}
    pub fn set_zlie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn yhie_g(&self) -> bool {self.config.yhie_g()}
    pub fn set_yhie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn ylie_g(&self) -> bool {self.config.ylie_g()}
    pub fn set_ylie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn xhie_g(&self) -> bool {self.config.xhie_g()}
    pub fn set_xhie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn xlie_g(&self) -> bool {self.config.xlie_g()}
    pub fn set_xlie_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn dcrm_g(&self) -> bool {self.config.dcrm_g()}
    pub fn set_dcrm_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn int_gen_ths_x_g(&self) -> u16 {self.config.int_gen_ths_x_g()}
    pub fn set_int_gen_ths_x_g(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn int_gen_ths_y_g(&self) -> u16 {self.config.int_gen_ths_y_g()}
    pub fn set_int_gen_ths_y_g(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn int_gen_ths_z_g(&self) -> u16 {self.config.int_gen_ths_z_g()}
    pub fn set_int_gen_ths_z_g(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn wait_g(&self) -> bool {self.config.wait_g()}
    pub fn set_wait_g(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn dur_g(&self) -> u8 {self.config.dur_g()}
    pub fn set_dur_g(&mut self, value: u8) -> Result<(),()> {unimplemented!()}

    pub fn offset_x_reg_m(&self) -> u16 {self.config.offset_x_reg_m()}
    pub fn set_offset_x_reg_m(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn offset_y_reg_m(&self) -> u16 {self.config.offset_y_reg_m()}
    pub fn set_offset_y_reg_m(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn offset_z_reg_m(&self) -> u16 {self.config.offset_z_reg_m()}
    pub fn set_offset_z_reg_m(&mut self, value: u16) -> Result<(),()> {unimplemented!()}

    pub fn op_mode(&self) -> OpMode {self.config.op_mode()}
    pub fn set_op_mode(&mut self, value: OpMode) -> Result<(),()> {unimplemented!()}

    pub fn output_data_rate(&self) -> OutputDataRate {self.config.output_data_rate()}
    pub fn set_output_data_rate(&mut self, value: OutputDataRate) -> Result<(),()> {unimplemented!()}

    pub fn temp_comp(&self) -> bool {self.config.temp_comp()}
    pub fn set_temp_comp(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn self_test(&self) -> bool {self.config.self_test()}
    pub fn set_self_test(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn fs_m(&self) -> FsM {self.config.fs_m()}
    pub fn set_fs_m(&mut self, value: FsM) -> Result<(),()> {unimplemented!()}

    pub fn reboot_m(&self) -> bool {self.config.reboot_m()}
    pub fn set_reboot_m(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn soft_reset_m(&self) -> bool {self.config.soft_reset_m()}
    pub fn set_soft_reset_m(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn md(&self) -> Md {self.config.md()}
    pub fn set_md(&mut self, value: Md) -> Result<(),()> {unimplemented!()}

    pub fn i2c_disable_m(&self) -> bool {self.config.i2c_disable_m()}
    pub fn set_i2c_disable_m(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn low_power_m(&self) -> bool {self.config.low_power_m()}
    pub fn set_low_power_m(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn sim_m(&self) -> bool {self.config.sim_m()}
    pub fn set_sim_m(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn op_mode_z(&self) -> OpMode {self.config.op_mode_z()}
    pub fn set_op_mode_z(&mut self, value: OpMode) -> Result<(),()> {unimplemented!()}

    pub fn big_little_endian(&self) -> bool {self.config.big_little_endian()}
    pub fn set_big_little_endian(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn block_data_update(&self) -> bool {self.config.block_data_update()}
    pub fn set_block_data_update(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn xien(&self) -> bool {self.config.xien()}
    pub fn set_xien(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn yien(&self) -> bool {self.config.yien()}
    pub fn set_yien(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn zien(&self) -> bool {self.config.zien()}
    pub fn set_zien(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn iea(&self) -> bool {self.config.iea()}
    pub fn set_iea(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn iel(&self) -> bool {self.config.iel()}
    pub fn set_iel(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

    pub fn ien(&self) -> bool {self.config.ien()}
    pub fn set_ien(&mut self, value: bool) -> Result<(),()> {unimplemented!()}

}

// pub mod accelerometer;
// pub mod i2c;

// use std::i16::MAX;
// use register::{ReadAddress, ReadWordAddress, Write};
// use accelerometer::{Reg6, Reg6FS, OUT_X_ADDRESS_R, OUT_Y_ADDRESS_R, OUT_Z_ADDRESS_R};

pub trait Interface {
    // // TODO: Result<>
    // fn read(&mut self, address: ReadAddress) -> u8;
    // // TODO: Result<>
    // fn readword(&mut self, address: ReadWordAddress) -> u16;
    // // TODO: Result<>
    fn write<T>(&mut self, address: Address, value: T) -> Result<(),()>;
}

// pub enum Cmd {
//     Reg6(Reg6),
// }


// pub struct Lsm9ds1<D: Device> {
//     reg6: Option<Reg6>,
//     device: D,
// }

// impl<D: Device> Lsm9ds1<D> {
    
//     pub fn new(device: D) -> Lsm9ds1<D> {
//         Lsm9ds1 {
//             reg6: None,
//             device: device,
//         }
//     }
    
//     // TODO: Result<>
//     fn scale_acc(&self, value: i16) -> Option<f32> {
//         match self.reg6 {
//             Some(r) => {
//                 let s = match r.fs {
//                     Reg6FS::Acc2g  =>  2.0,
//                     Reg6FS::Acc4g  =>  4.0,
//                     Reg6FS::Acc8g  =>  8.0,
//                     Reg6FS::Acc16g => 16.0,
//                 };
//                 Some(s * (value as f32) / (MAX as f32))
//             }
//             None => None,
//         }
//     }
    
//     pub fn x(&mut self) -> Option<f32> {
//         let raw_x = self.device.readword(OUT_X_ADDRESS_R);
//         self.scale_acc(raw_x as i16)
//     }
    
//     pub fn y(&mut self) -> Option<f32> {
//         let raw_y = self.device.readword(OUT_Y_ADDRESS_R);
//         self.scale_acc(raw_y as i16)
//     }
    
//     pub fn z(&mut self) -> Option<f32> {
//         let raw_z = self.device.readword(OUT_Z_ADDRESS_R);
//         self.scale_acc(raw_z as i16)
//     }

//     pub fn write(&mut self, cmd: Cmd) {
//         match cmd {
//             Cmd::Reg6(c) => {
//                 self.device.write(c);
//                 // TODO: set reg6 only if write occurs without error
//                 self.reg6 = Some(c);
//             }
//         };
//     }

// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
