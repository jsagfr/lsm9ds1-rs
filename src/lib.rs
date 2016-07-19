#![allow(dead_code)]
// extern crate i2cdev;

pub mod config;
use config::{Config, Param, ParamType, Register, RegisterType, ConfRegBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address{
    RW(u8),
    RW16(u8),
    R(u8),
    R16(u8),
}



// use register::{Address};

pub trait Lsm9ds1Device {
    fn read(&mut self, address: Address) -> Result<u8,()>;
    fn readword(&mut self, address: Address) -> Result<u16,()>;
    fn write(&mut self, address: Address, value: u8) -> Result<(),()>;
    fn writeword(&mut self, address: Address, value: u16) -> Result<(),()>;
}


pub struct Lsm9ds1<D: Lsm9ds1Device> {
    config: Config,
    device: D,
}


enum Interrupts{}
    

impl<D: Lsm9ds1Device> Lsm9ds1<D> {
    fn temp(&mut self) -> Result<f32,()> { unimplemented!() }
    fn lx(&mut self) -> Result<f32,()> { unimplemented!() }
    fn ly(&mut self) -> Result<f32,()> { unimplemented!() }
    fn lz(&mut self) -> Result<f32,()> { unimplemented!() }
    fn gx(&mut self) -> Result<f32,()> { unimplemented!() }
    fn gy(&mut self) -> Result<f32,()> { unimplemented!() }
    fn gz(&mut self) -> Result<f32,()> { unimplemented!() }
    fn mx(&mut self) -> Result<f32,()> { unimplemented!() }
    fn my(&mut self) -> Result<f32,()> { unimplemented!() }
    fn mz(&mut self) -> Result<f32,()> { unimplemented!() }
    fn fifo(&mut self) -> Result<f32,()> { unimplemented!() }
    fn linterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }
    fn ginterrupts(&mut self) -> Result<Interrupts,()> { unimplemented!() }

    fn params<'a>(&'a self) -> Vec<&'a Param> { self.config.params() }
    fn param(&mut self, param_type: ParamType) -> Option<&Param> {
        self.config.param(param_type)
    }
    fn apply_config(&mut self, config: Config) {
        unimplemented!()
    }
    fn read_all_config(&mut self) {
        let mut v = vec![];
        for r in RegisterType::iterator() {
            v.push(match *r {
                RegisterType::ActThs => Register::ActThs(self.device.read(r.address()).unwrap()),
                RegisterType::ActDur => Register::ActDur(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenCfgXl => Register::IntGenCfgXl(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenThsXXl => Register::IntGenThsXXl(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenThsYXl => Register::IntGenThsYXl(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenThsZXl => Register::IntGenThsZXl(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenDurXl => Register::IntGenDurXl(self.device.read(r.address()).unwrap()),
                RegisterType::ReferenceG => Register::ReferenceG(self.device.read(r.address()).unwrap()),
                RegisterType::Int1Ctrl => Register::Int1Ctrl(self.device.read(r.address()).unwrap()),
                RegisterType::Int2Ctrl => Register::Int2Ctrl(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg1G => Register::CtrlReg1G(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg2G => Register::CtrlReg2G(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg3G => Register::CtrlReg3G(self.device.read(r.address()).unwrap()),
                RegisterType::OrientCfgG => Register::OrientCfgG(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg4 => Register::CtrlReg4(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg5Xl => Register::CtrlReg5Xl(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg6Xl => Register::CtrlReg6Xl(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg7Xl => Register::CtrlReg7Xl(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg8 => Register::CtrlReg8(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg9 => Register::CtrlReg9(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg10 => Register::CtrlReg10(self.device.read(r.address()).unwrap()),
                RegisterType::FifoCtrl => Register::FifoCtrl(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenCfgG => Register::IntGenCfgG(self.device.read(r.address()).unwrap()),
                RegisterType::IntGenThsXG => Register::IntGenThsXG(self.device.readword(r.address()).unwrap()),
                RegisterType::IntGenThsYG => Register::IntGenThsYG(self.device.readword(r.address()).unwrap()),
                RegisterType::IntGenThsZG => Register::IntGenThsZG(self.device.readword(r.address()).unwrap()),
                RegisterType::IntGenDurG => Register::IntGenDurG(self.device.read(r.address()).unwrap()),
                RegisterType::OffsetXRegM => Register::OffsetXRegM(self.device.readword(r.address()).unwrap()),
                RegisterType::OffsetYRegM => Register::OffsetYRegM(self.device.readword(r.address()).unwrap()),
                RegisterType::OffsetZRegM => Register::OffsetZRegM(self.device.readword(r.address()).unwrap()),
                RegisterType::CtrlReg1M => Register::CtrlReg1M(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg2M => Register::CtrlReg2M(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg3M => Register::CtrlReg3M(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg4M => Register::CtrlReg4M(self.device.read(r.address()).unwrap()),
                RegisterType::CtrlReg5M => Register::CtrlReg5M(self.device.read(r.address()).unwrap()),
                RegisterType::IntCfgM => Register::IntCfgM(self.device.read(r.address()).unwrap()),
            });
        }
        let conf = ConfRegBuilder::new()
            .set_all(&v)
            .build();
    }
    

    // fn status(&mut self) -> Result<Status,()> { unimplemented!() }
    // fn all(&mut self) -> MesureIterator

}

// pub mod accelerometer;
// pub mod i2c;

// use std::i16::MAX;
// use register::{ReadAddress, ReadWordAddress, Write};
// use accelerometer::{Reg6, Reg6FS, OUT_X_ADDRESS_R, OUT_Y_ADDRESS_R, OUT_Z_ADDRESS_R};

// pub trait Device {
//     // TODO: Result<>
//     fn read(&mut self, address: ReadAddress) -> u8;
//     // TODO: Result<>
//     fn readword(&mut self, address: ReadWordAddress) -> u16;
//     // TODO: Result<>
//     fn write<T: Write>(&mut self, cmd: T);   
// }

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
