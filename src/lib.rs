// TODO: Waitning for: #![feature(associated_consts)] in main stream
#![allow(dead_code)]

extern crate i2cdev;
pub mod i2c;

#[macro_use]
mod macros;
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

use std::u16::MAX;
use std::f32;
// use config::{Config, Param, Register};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Address{
//     RW(u8),
//     RW16(u8),
//     R(u8),
//     R16(u8),
// }

// pub trait ReadableAddress<T> {
//     fn read_addr(&self) -> u8;
// }

// pub trait WritableAddress<T> {
//     fn write_addr(&self) -> u8;
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct ReadAddress(u8);

// impl ReadableAddress<u8> for ReadAddress {
//     fn read_addr(&self) -> u8 {
//         let &ReadAddress(value) = self;
//         return value
//     }
// }

// impl ReadableAddress<u16> for ReadAddress {
//     fn read_addr(&self) -> u8 {
//         let &ReadAddress(value) = self;
//         return value
//     }
// }

// pub trait ReadInterface<T, A:ReadableAddress<T>> {
//     fn read(&mut self, address: &A) -> Result<T,()>;
// }
// pub trait WriteInterface<T, A:ReadableAddress<T>> {
//     fn write(&mut self, address: &A, value: T) -> Result<(),()>;
// }
pub trait Interface {
    fn read(&mut self, address: u8) -> Result<u8,()>;
    fn read16(&mut self, address: u8) -> Result<u16,()>;
    fn write(&mut self, address: u8, value: u8) -> Result<(),()>;
    fn write16(&mut self, address: u8, value: u16) -> Result<(),()>;
}

pub struct Lsm9ds1<I: Interface> {
    config: Config,
    interface: I,
}

enum Interrupts{}

const OUT_TEMP:   u8 = 0x15;
const OUT_X_G:    u8 = 0x18;
const OUT_Y_G:    u8 = 0x1A;
const OUT_Z_G:    u8 = 0x1C;
const OUT_X_XL:   u8 = 0x28;
const OUT_Y_XL:   u8 = 0x2A;
const OUT_Z_XL:   u8 = 0x2C;
const WHO_AM_I:   u8 = 0x0F;
const WHO_AM_I_M: u8 = 0x0F;
const I_AM:       u8 = 0b0110_1000;
const I_AM_M:     u8 = 0b0011_1101;

pub fn check_device<I: Interface>(interface: &mut I) -> Result<(),()> {
    let g_xl = try!(interface.read(WHO_AM_I));
    match g_xl {
        I_AM => Ok(()),
        _ => Err(())
    }
    // let m = try!(interface.read(CHECK_M));
    // match (g_xl, m) {
    //     (ID_G_XL, ID_M) => Ok(()),
    //     (_, ID_M) => Err(()),
    //     (ID_G_XL, _) => Err(()),
    //     (_, _) => Err(()),
    // }
}

impl<I: Interface> Lsm9ds1<I> {
    pub fn new(mut interface: I, config: Config) -> Result<Lsm9ds1<I>,()> {
        try!(check_device(&mut interface));
        let mut lsm9ds1 = Lsm9ds1 {
            config: config,
            interface: interface,
        };
        try!(lsm9ds1.re_apply_config());
        Ok(lsm9ds1)
    }

    pub fn from_interface(mut interface: I) -> Result<Lsm9ds1<I>,()> {
        try!(check_device(&mut interface));
        let mut lsm9ds1 = Lsm9ds1 {
            config: config::Config::default(),
            interface: interface,
        };
        try!(lsm9ds1.re_read_config());
        Ok(lsm9ds1)
    }

    pub fn temp(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_TEMP) {
            Ok(value) => Ok(f32::from(value as i16)),
            Err(_) => Err(())
        }
    }
    pub fn lx(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_X_XL) {
            Ok(value) => Ok(self.config.fs_xl().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    pub fn ly(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_Y_XL) {
            Ok(value) => Ok(self.config.fs_xl().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    pub fn lz(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_Z_XL) {
            Ok(value) => Ok(self.config.fs_xl().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    pub fn gx(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_X_G) {
            Ok(value) => Ok(self.config.fs_g().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    pub fn gy(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_Y_G) {
            Ok(value) => Ok(self.config.fs_g().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    pub fn gz(&mut self) -> Result<f32,()> {
        match self.interface.read16(OUT_Z_G) {
            Ok(value) => Ok(self.config.fs_g().value() * f32::from(value as i16) / (MAX as f32)),
            Err(_) => Err(())
        }
    }
    // pub fn mx(&mut self) -> Result<f32,()> {
    //     match self.interface.read16(OUT_TEMP) {
    //         Ok(value) => Ok(value as f16),
    //         Err(_) => Err(())
    //     }
    // }
    // pub fn my(&mut self) -> Result<f32,()> {
    //     match self.interface.read16(OUT_TEMP) {
    //         Ok(value) => Ok(value as f16),
    //         Err(_) => Err(())
    //     }
    // }
    // pub fn mz(&mut self) -> Result<f32,()> {
    //     match self.interface.read16(OUT_TEMP) {
    //         Ok(value) => Ok(value as f16),
    //         Err(_) => Err(())
    //     }
    // }
    pub fn fifo(&mut self) -> Result<f32,()> { unimplemented!() }
    // pub fn linterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }
    // pub fn ginterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }

    pub fn re_apply_config(&mut self) -> Result<(),()> {
        try!(self.interface.write(self.config.act_ths.addr(), self.config.act_ths.reg()));
        try!(self.interface.write(self.config.act_dur.addr(), self.config.act_dur.reg()));
        try!(self.interface.write(self.config.int_gen_cfg_xl.addr(), self.config.int_gen_cfg_xl.reg()));
        try!(self.interface.write(self.config.int_gen_ths_x_xl.addr(), self.config.int_gen_ths_x_xl.reg()));
        try!(self.interface.write(self.config.int_gen_ths_y_xl.addr(), self.config.int_gen_ths_y_xl.reg()));
        try!(self.interface.write(self.config.int_gen_ths_z_xl.addr(), self.config.int_gen_ths_z_xl.reg()));
        try!(self.interface.write(self.config.int_gen_dur_xl.addr(), self.config.int_gen_dur_xl.reg()));
        try!(self.interface.write(self.config.reference_g.addr(), self.config.reference_g.reg()));
        try!(self.interface.write(self.config.int1_ctrl.addr(), self.config.int1_ctrl.reg()));
        try!(self.interface.write(self.config.int2_ctrl.addr(), self.config.int2_ctrl.reg()));
        try!(self.interface.write(self.config.ctrl_reg1_g.addr(), self.config.ctrl_reg1_g.reg()));
        try!(self.interface.write(self.config.ctrl_reg2_g.addr(), self.config.ctrl_reg2_g.reg()));
        try!(self.interface.write(self.config.ctrl_reg3_g.addr(), self.config.ctrl_reg3_g.reg()));
        try!(self.interface.write(self.config.orient_cfg_g.addr(), self.config.orient_cfg_g.reg()));
        try!(self.interface.write(self.config.ctrl_reg4.addr(), self.config.ctrl_reg4.reg()));
        try!(self.interface.write(self.config.ctrl_reg5_xl.addr(), self.config.ctrl_reg5_xl.reg()));
        try!(self.interface.write(self.config.ctrl_reg6_xl.addr(), self.config.ctrl_reg6_xl.reg()));
        try!(self.interface.write(self.config.ctrl_reg7_xl.addr(), self.config.ctrl_reg7_xl.reg()));
        try!(self.interface.write(self.config.ctrl_reg8.addr(), self.config.ctrl_reg8.reg()));
        try!(self.interface.write(self.config.ctrl_reg9.addr(), self.config.ctrl_reg9.reg()));
        try!(self.interface.write(self.config.ctrl_reg10.addr(), self.config.ctrl_reg10.reg()));
        try!(self.interface.write(self.config.fifo_ctrl.addr(), self.config.fifo_ctrl.reg()));
        try!(self.interface.write(self.config.int_gen_cfg_g.addr(), self.config.int_gen_cfg_g.reg()));
        try!(self.interface.write16(self.config.int_gen_ths_x_g.addr(), self.config.int_gen_ths_x_g.reg()));
        try!(self.interface.write16(self.config.int_gen_ths_y_g.addr(), self.config.int_gen_ths_y_g.reg()));
        try!(self.interface.write16(self.config.int_gen_ths_z_g.addr(), self.config.int_gen_ths_z_g.reg()));
        try!(self.interface.write(self.config.int_gen_dur_g.addr(), self.config.int_gen_dur_g.reg()));
        // try!(self.interface.write16(self.config.offset_x_reg_m.addr(), self.config.offset_x_reg_m.reg()));
        // try!(self.interface.write16(self.config.offset_y_reg_m.addr(), self.config.offset_y_reg_m.reg()));
        // try!(self.interface.write16(self.config.offset_z_reg_m.addr(), self.config.offset_z_reg_m.reg()));
        // try!(self.interface.write(self.config.ctrl_reg1_m.addr(), self.config.ctrl_reg1_m.reg()));
        // try!(self.interface.write(self.config.ctrl_reg2_m.addr(), self.config.ctrl_reg2_m.reg()));
        // try!(self.interface.write(self.config.ctrl_reg3_m.addr(), self.config.ctrl_reg3_m.reg()));
        // try!(self.interface.write(self.config.ctrl_reg4_m.addr(), self.config.ctrl_reg4_m.reg()));
        // try!(self.interface.write(self.config.ctrl_reg5_m.addr(), self.config.ctrl_reg5_m.reg()));
        // try!(self.interface.write(self.config.int_cfg_m.addr(), self.config.int_cfg_m.reg()));
        Ok(())
    }

    pub fn apply_config(&mut self, config: Config) -> Result<(),()> {
        try!(self.interface.write(config.act_ths.addr(), config.act_ths.reg()));
        try!(self.interface.write(config.act_dur.addr(), config.act_dur.reg()));
        try!(self.interface.write(config.int_gen_cfg_xl.addr(), config.int_gen_cfg_xl.reg()));
        try!(self.interface.write(config.int_gen_ths_x_xl.addr(), config.int_gen_ths_x_xl.reg()));
        try!(self.interface.write(config.int_gen_ths_y_xl.addr(), config.int_gen_ths_y_xl.reg()));
        try!(self.interface.write(config.int_gen_ths_z_xl.addr(), config.int_gen_ths_z_xl.reg()));
        try!(self.interface.write(config.int_gen_dur_xl.addr(), config.int_gen_dur_xl.reg()));
        try!(self.interface.write(config.reference_g.addr(), config.reference_g.reg()));
        try!(self.interface.write(config.int1_ctrl.addr(), config.int1_ctrl.reg()));
        try!(self.interface.write(config.int2_ctrl.addr(), config.int2_ctrl.reg()));
        try!(self.interface.write(config.ctrl_reg1_g.addr(), config.ctrl_reg1_g.reg()));
        try!(self.interface.write(config.ctrl_reg2_g.addr(), config.ctrl_reg2_g.reg()));
        try!(self.interface.write(config.ctrl_reg3_g.addr(), config.ctrl_reg3_g.reg()));
        try!(self.interface.write(config.orient_cfg_g.addr(), config.orient_cfg_g.reg()));
        try!(self.interface.write(config.ctrl_reg4.addr(), config.ctrl_reg4.reg()));
        try!(self.interface.write(config.ctrl_reg5_xl.addr(), config.ctrl_reg5_xl.reg()));
        try!(self.interface.write(config.ctrl_reg6_xl.addr(), config.ctrl_reg6_xl.reg()));
        try!(self.interface.write(config.ctrl_reg7_xl.addr(), config.ctrl_reg7_xl.reg()));
        try!(self.interface.write(config.ctrl_reg8.addr(), config.ctrl_reg8.reg()));
        try!(self.interface.write(config.ctrl_reg9.addr(), config.ctrl_reg9.reg()));
        try!(self.interface.write(config.ctrl_reg10.addr(), config.ctrl_reg10.reg()));
        try!(self.interface.write(config.fifo_ctrl.addr(), config.fifo_ctrl.reg()));
        try!(self.interface.write(config.int_gen_cfg_g.addr(), config.int_gen_cfg_g.reg()));
        try!(self.interface.write16(config.int_gen_ths_x_g.addr(), config.int_gen_ths_x_g.reg()));
        try!(self.interface.write16(config.int_gen_ths_y_g.addr(), config.int_gen_ths_y_g.reg()));
        try!(self.interface.write16(config.int_gen_ths_z_g.addr(), config.int_gen_ths_z_g.reg()));
        try!(self.interface.write(config.int_gen_dur_g.addr(), config.int_gen_dur_g.reg()));
        // try!(self.interface.write16(config.offset_x_reg_m.addr(), config.offset_x_reg_m.reg()));
        // try!(self.interface.write16(config.offset_y_reg_m.addr(), config.offset_y_reg_m.reg()));
        // try!(self.interface.write16(config.offset_z_reg_m.addr(), config.offset_z_reg_m.reg()));
        // try!(self.interface.write(config.ctrl_reg1_m.addr(), config.ctrl_reg1_m.reg()));
        // try!(self.interface.write(config.ctrl_reg2_m.addr(), config.ctrl_reg2_m.reg()));
        // try!(self.interface.write(config.ctrl_reg3_m.addr(), config.ctrl_reg3_m.reg()));
        // try!(self.interface.write(config.ctrl_reg4_m.addr(), config.ctrl_reg4_m.reg()));
        // try!(self.interface.write(config.ctrl_reg5_m.addr(), config.ctrl_reg5_m.reg()));
        // try!(self.interface.write(config.int_cfg_m.addr(), config.int_cfg_m.reg()));
        self.config = config;
        Ok(())
    }
    pub fn apply_patch_config(&mut self, patch: PatchConfig) -> Result<(),()> { unimplemented!() }
    pub fn re_read_config(&mut self) -> Result<(),()> {
        self.config.act_ths = config::act_ths::ActThs::new(
            try!(self.interface.read(self.config.act_ths.addr())));
        self.config.act_dur = config::act_dur::ActDur::new(
            try!(self.interface.read(self.config.act_dur.addr())));
        self.config.int_gen_cfg_xl = config::int_gen_cfg_xl::IntGenCfgXl::new(
            try!(self.interface.read(self.config.int_gen_cfg_xl.addr())));
        self.config.int_gen_ths_x_xl = config::int_gen_ths_x_xl::IntGenThsXXl::new(
            try!(self.interface.read(self.config.int_gen_ths_x_xl.addr())));
        self.config.int_gen_ths_y_xl = config::int_gen_ths_y_xl::IntGenThsYXl::new(
            try!(self.interface.read(self.config.int_gen_ths_y_xl.addr())));
        self.config.int_gen_ths_z_xl = config::int_gen_ths_z_xl::IntGenThsZXl::new(
            try!(self.interface.read(self.config.int_gen_ths_z_xl.addr())));
        self.config.int_gen_dur_xl = config::int_gen_dur_xl::IntGenDurXl::new(
            try!(self.interface.read(self.config.int_gen_dur_xl.addr())));
        self.config.reference_g = config::reference_g::ReferenceG::new(
            try!(self.interface.read(self.config.reference_g.addr())));
        self.config.int1_ctrl = config::int1_ctrl::Int1Ctrl::new(
            try!(self.interface.read(self.config.int1_ctrl.addr())));
        self.config.int2_ctrl = config::int2_ctrl::Int2Ctrl::new(
            try!(self.interface.read(self.config.int2_ctrl.addr())));
        self.config.ctrl_reg1_g = config::ctrl_reg1_g::CtrlReg1G::new(
            try!(self.interface.read(self.config.ctrl_reg1_g.addr())));
        self.config.ctrl_reg2_g = config::ctrl_reg2_g::CtrlReg2G::new(
            try!(self.interface.read(self.config.ctrl_reg2_g.addr())));
        self.config.ctrl_reg3_g = config::ctrl_reg3_g::CtrlReg3G::new(
            try!(self.interface.read(self.config.ctrl_reg3_g.addr())));
        self.config.orient_cfg_g = config::orient_cfg_g::OrientCfgG::new(
            try!(self.interface.read(self.config.orient_cfg_g.addr())));
        self.config.ctrl_reg4 = config::ctrl_reg4::CtrlReg4::new(
            try!(self.interface.read(self.config.ctrl_reg4.addr())));
        self.config.ctrl_reg5_xl = config::ctrl_reg5_xl::CtrlReg5XL::new(
            try!(self.interface.read(self.config.ctrl_reg5_xl.addr())));
        self.config.ctrl_reg6_xl = config::ctrl_reg6_xl::CtrlReg6XL::new(
            try!(self.interface.read(self.config.ctrl_reg6_xl.addr())));
        self.config.ctrl_reg7_xl = config::ctrl_reg7_xl::CtrlReg7XL::new(
            try!(self.interface.read(self.config.ctrl_reg7_xl.addr())));
        self.config.ctrl_reg8 = config::ctrl_reg8::CtrlReg8::new(
            try!(self.interface.read(self.config.ctrl_reg8.addr())));
        self.config.ctrl_reg9 = config::ctrl_reg9::CtrlReg9::new(
            try!(self.interface.read(self.config.ctrl_reg9.addr())));
        self.config.ctrl_reg10 = config::ctrl_reg10::CtrlReg10::new(
            try!(self.interface.read(self.config.ctrl_reg10.addr())));
        self.config.fifo_ctrl = config::fifo_ctrl::FifoCtrl::new(
            try!(self.interface.read(self.config.fifo_ctrl.addr())));
        self.config.int_gen_cfg_g = config::int_gen_cfg_g::IntGenCfgG::new(
            try!(self.interface.read(self.config.int_gen_cfg_g.addr())));
        self.config.int_gen_ths_x_g = config::int_gen_ths_x_g::IntGenThsXG::new(
            try!(self.interface.read16(self.config.int_gen_ths_x_g.addr())));
        self.config.int_gen_ths_y_g = config::int_gen_ths_y_g::IntGenThsYG::new(
            try!(self.interface.read16(self.config.int_gen_ths_y_g.addr())));
        self.config.int_gen_ths_z_g = config::int_gen_ths_z_g::IntGenThsZG::new(
            try!(self.interface.read16(self.config.int_gen_ths_z_g.addr())));
        self.config.int_gen_dur_g = config::int_gen_dur_g::IntGenDurG::new(
            try!(self.interface.read(self.config.int_gen_dur_g.addr())));
        // self.config.offset_x_reg_m = config::offset_x_reg_m::OffsetXRegM::new(
        //     try!(self.interface.read16(self.config.offset_x_reg_m.addr())));
        // self.config.offset_y_reg_m = config::offset_y_reg_m::OffsetYRegM::new(
        //     try!(self.interface.read16(self.config.offset_y_reg_m.addr())));
        // self.config.offset_z_reg_m = config::offset_z_reg_m::OffsetZRegM::new(
        //     try!(self.interface.read16(self.config.offset_z_reg_m.addr())));
        // self.config.ctrl_reg1_m = config::ctrl_reg1_m::CtrlReg1M::new(
        //     try!(self.interface.read(self.config.ctrl_reg1_m.addr())));
        // self.config.ctrl_reg2_m = config::ctrl_reg2_m::CtrlReg2M::new(
        //     try!(self.interface.read(self.config.ctrl_reg2_m.addr())));
        // self.config.ctrl_reg3_m = config::ctrl_reg3_m::CtrlReg3M::new(
        //     try!(self.interface.read(self.config.ctrl_reg3_m.addr())));
        // self.config.ctrl_reg4_m = config::ctrl_reg4_m::CtrlReg4M::new(
        //     try!(self.interface.read(self.config.ctrl_reg4_m.addr())));
        // self.config.ctrl_reg5_m = config::ctrl_reg5_m::CtrlReg5M::new(
        //     try!(self.interface.read(self.config.ctrl_reg5_m.addr())));
        // self.config.int_cfg_m = config::int_cfg_m::IntCfgM::new(
        //     try!(self.interface.read(self.config.int_cfg_m.addr())));
        Ok(())
    }
    

    // fn status(&mut self) -> Result<Status,()> { unimplemented!() }
    // fn all(&mut self) -> MesureIterator

    pub fn act_ths(&self) -> u8 {self.config.act_ths()}
    pub fn set_act_ths(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, act_ths, set_act_ths, value)
    }
    
    pub fn sleep_on(&self) -> bool {self.config.sleep_on()}
    pub fn set_sleep_on(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, act_ths, set_sleep_on, value)
    }
    
    pub fn act_dur(&self) -> u8 {self.config.act_dur()}
    pub fn set_act_dur(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, act_dur, set_act_dur, value)
    }
    
    pub fn aoi_xl(&self) -> bool {self.config.aoi_xl()}
    pub fn set_aoi_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_aoi_xl, value)
    }
    
    pub fn detect_6d(&self) -> bool {self.config.detect_6d()}
    pub fn set_detect_6d(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_detect_6d, value)
    }
    
    pub fn zhie_xl(&self) -> bool {self.config.zhie_xl()}
    pub fn set_zhie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_zhie_xl, value)
    }
    
    pub fn zlie_xl(&self) -> bool {self.config.zlie_xl()}
    pub fn set_zlie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_zlie_xl, value)
    }
    
    pub fn yhie_xl(&self) -> bool {self.config.yhie_xl()}
    pub fn set_yhie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_yhie_xl, value)
    }
    
    pub fn ylie_xl(&self) -> bool {self.config.ylie_xl()}
    pub fn set_ylie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_ylie_xl, value)
    }
    
    pub fn xhie_xl(&self) -> bool {self.config.xhie_xl()}
    pub fn set_xhie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_xhie_xl, value)
    }
    
    pub fn xlie_xl(&self) -> bool {self.config.xlie_xl()}
    pub fn set_xlie_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_xl, set_xlie_xl, value)
    }
    
    pub fn int_gen_ths_x_xl(&self) -> u8 {self.config.int_gen_ths_x_xl()}
    pub fn set_int_gen_ths_x_xl(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, int_gen_ths_x_xl, set_int_gen_ths_x_xl, value)
    }
        
    pub fn int_gen_ths_y_xl(&self) -> u8 {self.config.int_gen_ths_y_xl()}
    pub fn set_int_gen_ths_y_xl(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, int_gen_ths_y_xl, set_int_gen_ths_y_xl, value)
    }
    
    pub fn int_gen_ths_z_xl(&self) -> u8 {self.config.int_gen_ths_z_xl()}
    pub fn set_int_gen_ths_z_xl(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, int_gen_ths_z_xl, set_int_gen_ths_z_xl, value)
    }
    
    pub fn wait_xl(&self) -> bool {self.config.wait_xl()}
    pub fn set_wait_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_dur_xl, set_wait_xl, value)
    }
    
    pub fn dur_xl(&self) -> u8 {self.config.dur_xl()}
    pub fn set_dur_xl(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, int_gen_dur_xl, set_dur_xl, value)
    }
    
    pub fn reference_g(&self) -> u8 {self.config.reference_g()}
    pub fn set_reference_g(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, reference_g, set_reference_g, value)
    }
    
    pub fn int1_ig_g(&self) -> bool {self.config.int1_ig_g()}
    pub fn set_int1_ig_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_ig_g, value)
    }
    
    pub fn int1_ig_xl(&self) -> bool {self.config.int1_ig_xl()}
    pub fn set_int1_ig_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_ig_xl, value)
    }
    
    pub fn int1_fss_5(&self) -> bool {self.config.int1_fss_5()}
    pub fn set_int1_fss_5(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_fss_5, value)
    }
    
    pub fn int1_ovr(&self) -> bool {self.config.int1_ovr()}
    pub fn set_int1_ovr(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_ovr, value)
    }
    
    pub fn int1_fth(&self) -> bool {self.config.int1_fth()}
    pub fn set_int1_fth(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_fth, value)
    }
    
    pub fn int1_boot(&self) -> bool {self.config.int1_boot()}
    pub fn set_int1_boot(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_boot, value)
    }
    
    pub fn int1_drdy_g(&self) -> bool {self.config.int1_drdy_g()}
    pub fn set_int1_drdy_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_drdy_g, value)
    }
    
    pub fn int1_drdy_xl(&self) -> bool {self.config.int1_drdy_xl()}
    pub fn set_int1_drdy_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int1_ctrl, set_int1_drdy_xl, value)
    }
    
    pub fn int2_inact(&self) -> bool {self.config.int2_inact()}
    pub fn set_int2_inact(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_inact, value)
    }
    
    pub fn int2_fss_5(&self) -> bool {self.config.int2_fss_5()}
    pub fn set_int2_fss_5(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_fss_5, value)
    }
    
    pub fn int2_ovr(&self) -> bool {self.config.int2_ovr()}
    pub fn set_int2_ovr(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_ovr, value)
    }
    
    pub fn int2_fth(&self) -> bool {self.config.int2_fth()}
    pub fn set_int2_fth(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_fth, value)
    }
    
    pub fn int2_drdy_temp(&self) -> bool {self.config.int2_drdy_temp()}
    pub fn set_int2_drdy_temp(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_drdy_temp, value)
    }
    
    pub fn int2_drdy_g(&self) -> bool {self.config.int2_drdy_g()}
    pub fn set_int2_drdy_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_drdy_g, value)
    }
    
    pub fn int2_drdy_xl(&self) -> bool {self.config.int2_drdy_xl()}
    pub fn set_int2_drdy_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int2_ctrl, set_int2_drdy_xl, value)
    }

    pub fn odr_g(&self) -> DataRate {self.config.odr_g()}
    pub fn set_odr_g(&mut self, value: DataRate) -> Result<(),()> {
        set_value!(self, ctrl_reg1_g, set_odr_g, value)
    }
    
    pub fn fs_g(&self) -> GyroScale {self.config.fs_g()}
    pub fn set_fs_g(&mut self, value: GyroScale) -> Result<(),()> {
        set_value!(self, ctrl_reg1_g, set_fs_g, value)
    }
    
    pub fn bw_g(&self) -> Bw {self.config.bw_g()}
    pub fn set_bw_g(&mut self, value: Bw) -> Result<(),()> {
        set_value!(self, ctrl_reg1_g, set_bw_g, value)
    }
    
    pub fn int_sel(&self) -> IntSel {self.config.int_sel()}
    pub fn set_int_sel(&mut self, value: IntSel) -> Result<(),()> {
        set_value!(self, ctrl_reg2_g, set_int_sel, value)
    }
    
    pub fn out_sel(&self) -> OutSel {self.config.out_sel()}
    pub fn set_out_sel(&mut self, value: OutSel) -> Result<(),()> {
        set_value!(self, ctrl_reg2_g, set_out_sel, value)
    }
    
    pub fn lp_mode(&self) -> bool {self.config.lp_mode()}
    pub fn set_lp_mode(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg3_g, set_lp_mode, value)
    }
    
    pub fn hp_en(&self) -> bool {self.config.hp_en()}
    pub fn set_hp_en(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg3_g, set_hp_en, value)
    }
    
    pub fn hpcf_g(&self) -> u8 {self.config.hpcf_g()}
    pub fn set_hpcf_g(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, ctrl_reg3_g, set_hpcf_g, value)
    }
    
    pub fn sign_x_g(&self) -> bool {self.config.sign_x_g()}
    pub fn set_sign_x_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, orient_cfg_g, set_sign_x_g, value)
    }
    
    pub fn sign_y_g(&self) -> bool {self.config.sign_y_g()}
    pub fn set_sign_y_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, orient_cfg_g, set_sign_y_g, value)
    }
    
    pub fn sign_z_g(&self) -> bool {self.config.sign_z_g()}
    pub fn set_sign_z_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, orient_cfg_g, set_sign_z_g, value)
    }
    
    pub fn orient(&self) -> u8 {self.config.orient()}
    pub fn set_orient(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, orient_cfg_g, set_orient, value)
    }
    
    pub fn zen_g(&self) -> bool {self.config.zen_g()}
    pub fn set_zen_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4, set_zen_g, value)
    }
    
    pub fn yen_g(&self) -> bool {self.config.yen_g()}
    pub fn set_yen_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4, set_yen_g, value)
    }
    
    pub fn xen_g(&self) -> bool {self.config.xen_g()}
    pub fn set_xen_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4, set_xen_g, value)
    }
    
    pub fn lir_xl_1(&self) -> bool {self.config.lir_xl_1()}
    pub fn set_lir_xl_1(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4, set_lir_xl_1, value)
    }
    
    pub fn i_4d_xl_1(&self) -> bool {self.config.i_4d_xl_1()}
    pub fn set_i_4d_xl_1(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4, set_i_4d_xl_1, value)
    }
    
    pub fn zen_xl(&self) -> bool {self.config.zen_xl()}
    pub fn set_zen_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg5_xl, set_zen_xl, value)
    }
    
    pub fn yen_xl(&self) -> bool {self.config.yen_xl()}
    pub fn set_yen_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg5_xl, set_yen_xl, value)
    }
    
    pub fn xen_xl(&self) -> bool {self.config.xen_xl()}
    pub fn set_xen_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg5_xl, set_xen_xl, value)
    }
    
    pub fn dec(&self) -> Dec {self.config.dec()}
    pub fn set_dec(&mut self, value: Dec) -> Result<(),()> {
        set_value!(self, ctrl_reg5_xl, set_dec, value)
    }
    
    pub fn odr_xl(&self) -> DataRate {self.config.odr_xl()}
    pub fn set_odr_xl(&mut self, value: DataRate) -> Result<(),()> {
        set_value!(self, ctrl_reg6_xl, set_odr_xl, value)
    }
    
    pub fn fs_xl(&self) -> FsXl {self.config.fs_xl()}
    pub fn set_fs_xl(&mut self, value: FsXl) -> Result<(),()> {
        set_value!(self, ctrl_reg6_xl, set_fs_xl, value)
    }
    
    pub fn bw_scal_odr(&self) -> bool {self.config.bw_scal_odr()}
    pub fn set_bw_scal_odr(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg6_xl, set_bw_scal_odr, value)
    }
    
    pub fn bw_xl(&self) -> Bw {self.config.bw_xl()}
    pub fn set_bw_xl(&mut self, value: Bw) -> Result<(),()> {
        set_value!(self, ctrl_reg6_xl, set_bw_xl, value)
    }
    
    pub fn high_res(&self) -> bool {self.config.high_res()}
    pub fn set_high_res(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg7_xl, set_high_res, value)
    }
    
    pub fn xl_digital_cf(&self) -> DigCutoffFreq {self.config.xl_digital_cf()}
    pub fn set_xl_digital_cf(&mut self, value: DigCutoffFreq) -> Result<(),()> {
        set_value!(self, ctrl_reg7_xl, set_xl_digital_cf, value)
    }
    
    pub fn filtered_data_sel(&self) -> bool {self.config.filtered_data_sel()}
    pub fn set_filtered_data_sel(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg7_xl, set_filtered_data_sel, value)
    }
    
    pub fn high_pass_int_sens(&self) -> bool {self.config.high_pass_int_sens()}
    pub fn set_high_pass_int_sens(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg7_xl, set_high_pass_int_sens, value)
    }
    
    pub fn boot(&self) -> bool {self.config.boot()}
    pub fn set_boot(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_boot, value)
    }

    pub fn bdu(&self) -> bool {self.config.bdu()}
    pub fn set_bdu(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_bdu, value)
    }
    
    pub fn hl_active(&self) -> bool {self.config.hl_active()}
    pub fn set_hl_active(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_hl_active, value)
    }
    
    pub fn pp_od(&self) -> bool {self.config.pp_od()}
    pub fn set_pp_od(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_pp_od, value)
    }
    
    pub fn sim(&self) -> bool {self.config.sim()}
    pub fn set_sim(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_sim, value)
    }
    
    pub fn if_add_inc(&self) -> bool {self.config.if_add_inc()}
    pub fn set_if_add_inc(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_if_add_inc, value)
    }
    
    pub fn ble(&self) -> bool {self.config.ble()}
    pub fn set_ble(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_ble, value)
    }
    
    pub fn sw_reset(&self) -> bool {self.config.sw_reset()}
    pub fn set_sw_reset(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg8, set_sw_reset, value)
    }

    pub fn sleep_g(&self) -> bool {self.config.sleep_g()}
    pub fn set_sleep_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_sleep_g, value)
    }

    pub fn fifo_temp_en(&self) -> bool {self.config.fifo_temp_en()}
    pub fn set_fifo_temp_en(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_fifo_temp_en, value)
    }

    pub fn drdy_mask_bit(&self) -> bool {self.config.drdy_mask_bit()}
    pub fn set_drdy_mask_bit(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_drdy_mask_bit, value)
    }

    pub fn i2c_disable(&self) -> bool {self.config.i2c_disable()}
    pub fn set_i2c_disable(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_i2c_disable, value)
    }

    pub fn fifo_en(&self) -> bool {self.config.fifo_en()}
    pub fn set_fifo_en(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_fifo_en, value)
    }

    pub fn stop_on_fth(&self) -> bool {self.config.stop_on_fth()}
    pub fn set_stop_on_fth(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg9, set_stop_on_fth, value)
    }

    pub fn st_g(&self) -> bool {self.config.st_g()}
    pub fn set_st_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg10, set_st_g, value)
    }

    pub fn st_xl(&self) -> bool {self.config.st_xl()}
    pub fn set_st_xl(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg10, set_st_xl, value)
    }

    pub fn fth(&self) -> u8 {self.config.fth()}
    pub fn set_fth(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, fifo_ctrl, set_fth, value)
    }

    pub fn f_mode(&self) -> FMode {self.config.f_mode()}
    pub fn set_f_mode(&mut self, value: FMode) -> Result<(),()> {
        set_value!(self, fifo_ctrl, set_f_mode, value)
    }

    pub fn aoi_g(&self) -> bool {self.config.aoi_g()}
    pub fn set_aoi_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_aoi_g, value)
    }

    pub fn lir_g(&self) -> bool {self.config.lir_g()}
    pub fn set_lir_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_lir_g, value)
    }

    pub fn zhie_g(&self) -> bool {self.config.zhie_g()}
    pub fn set_zhie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_zhie_g, value)
    }

    pub fn zlie_g(&self) -> bool {self.config.zlie_g()}
    pub fn set_zlie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_zlie_g, value)
    }

    pub fn yhie_g(&self) -> bool {self.config.yhie_g()}
    pub fn set_yhie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_yhie_g, value)
    }

    pub fn ylie_g(&self) -> bool {self.config.ylie_g()}
    pub fn set_ylie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_ylie_g, value)
    }

    pub fn xhie_g(&self) -> bool {self.config.xhie_g()}
    pub fn set_xhie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_xhie_g, value)
    }

    pub fn xlie_g(&self) -> bool {self.config.xlie_g()}
    pub fn set_xlie_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_cfg_g, set_xlie_g, value)
    }

    pub fn dcrm_g(&self) -> bool {self.config.dcrm_g()}
    pub fn set_dcrm_g(&mut self, value: bool) -> Result<(),()> {
        set_value16!(self, int_gen_ths_x_g, set_dcrm_g, value)
    }

    pub fn int_gen_ths_x_g(&self) -> u16 {self.config.int_gen_ths_x_g()}
    pub fn set_int_gen_ths_x_g(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, int_gen_ths_x_g, set_int_gen_ths_x_g, value)
    }

    pub fn int_gen_ths_y_g(&self) -> u16 {self.config.int_gen_ths_y_g()}
    pub fn set_int_gen_ths_y_g(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, int_gen_ths_y_g, set_int_gen_ths_y_g, value)
    }

    pub fn int_gen_ths_z_g(&self) -> u16 {self.config.int_gen_ths_z_g()}
    pub fn set_int_gen_ths_z_g(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, int_gen_ths_z_g, set_int_gen_ths_z_g, value)
    }

    pub fn wait_g(&self) -> bool {self.config.wait_g()}
    pub fn set_wait_g(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_gen_dur_g, set_wait_g, value)
    }

    pub fn dur_g(&self) -> u8 {self.config.dur_g()}
    pub fn set_dur_g(&mut self, value: u8) -> Result<(),()> {
        set_value!(self, int_gen_dur_g, set_dur_g, value)
    }

    pub fn offset_x_reg_m(&self) -> u16 {self.config.offset_x_reg_m()}
    pub fn set_offset_x_reg_m(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, offset_x_reg_m, set_offset_x_reg_m, value)
    }

    pub fn offset_y_reg_m(&self) -> u16 {self.config.offset_y_reg_m()}
    pub fn set_offset_y_reg_m(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, offset_y_reg_m, set_offset_y_reg_m, value)
    }

    pub fn offset_z_reg_m(&self) -> u16 {self.config.offset_z_reg_m()}
    pub fn set_offset_z_reg_m(&mut self, value: u16) -> Result<(),()> {
        set_value16!(self, offset_z_reg_m, set_offset_z_reg_m, value)
    }

    pub fn op_mode(&self) -> OpMode {self.config.op_mode()}
    pub fn set_op_mode(&mut self, value: OpMode) -> Result<(),()> {
        set_value!(self, ctrl_reg1_m, set_op_mode, value)
    }

    pub fn output_data_rate(&self) -> OutputDataRate {self.config.output_data_rate()}
    pub fn set_output_data_rate(&mut self, value: OutputDataRate) -> Result<(),()> {
        set_value!(self, ctrl_reg1_m, set_output_data_rate, value)
    }

    pub fn temp_comp(&self) -> bool {self.config.temp_comp()}
    pub fn set_temp_comp(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg1_m, set_temp_comp, value)
    }

    pub fn self_test(&self) -> bool {self.config.self_test()}
    pub fn set_self_test(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg1_m, set_self_test, value)
    }

    pub fn fs_m(&self) -> FsM {self.config.fs_m()}
    pub fn set_fs_m(&mut self, value: FsM) -> Result<(),()> {
        set_value!(self, ctrl_reg2_m, set_fs_m, value)
    }

    pub fn reboot_m(&self) -> bool {self.config.reboot_m()}
    pub fn set_reboot_m(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg2_m, set_reboot_m, value)
    }

    pub fn soft_reset_m(&self) -> bool {self.config.soft_reset_m()}
    pub fn set_soft_reset_m(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg2_m, set_soft_reset_m, value)
    }

    pub fn md(&self) -> Md {self.config.md()}
    pub fn set_md(&mut self, value: Md) -> Result<(),()> {
        set_value!(self, ctrl_reg3_m, set_md, value)
    }

    pub fn i2c_disable_m(&self) -> bool {self.config.i2c_disable_m()}
    pub fn set_i2c_disable_m(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg3_m, set_i2c_disable_m, value)
    }

    pub fn low_power_m(&self) -> bool {self.config.low_power_m()}
    pub fn set_low_power_m(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg3_m, set_low_power_m, value)
    }

    pub fn sim_m(&self) -> bool {self.config.sim_m()}
    pub fn set_sim_m(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg3_m, set_sim_m, value)
    }

    pub fn op_mode_z(&self) -> OpMode {self.config.op_mode_z()}
    pub fn set_op_mode_z(&mut self, value: OpMode) -> Result<(),()> {
        set_value!(self, ctrl_reg4_m, set_op_mode_z, value)
    }

    pub fn big_little_endian(&self) -> bool {self.config.big_little_endian()}
    pub fn set_big_little_endian(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg4_m, set_big_little_endian, value)
    }

    pub fn block_data_update(&self) -> bool {self.config.block_data_update()}
    pub fn set_block_data_update(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, ctrl_reg5_m, set_block_data_update, value)
    }

    pub fn xien(&self) -> bool {self.config.xien()}
    pub fn set_xien(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_xien, value)
    }

    pub fn yien(&self) -> bool {self.config.yien()}
    pub fn set_yien(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_yien, value)
    }

    pub fn zien(&self) -> bool {self.config.zien()}
    pub fn set_zien(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_zien, value)
    }

    pub fn iea(&self) -> bool {self.config.iea()}
    pub fn set_iea(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_iea, value)
    }

    pub fn iel(&self) -> bool {self.config.iel()}
    pub fn set_iel(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_iel, value)
    }

    pub fn ien(&self) -> bool {self.config.ien()}
    pub fn set_ien(&mut self, value: bool) -> Result<(),()> {
        set_value!(self, int_cfg_m, set_ien, value)
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
