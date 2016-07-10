#![allow(dead_code)]
// extern crate i2cdev;
// #[macro_use]
// extern crate bitflags;


// mod register;
pub mod config;

// mod act_ths_reg;
// mod act_dur_reg;
// mod int_gen_cfg_xl;

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


pub struct Lsm9ds1 {
    // TBC
}

enum Interrupts{}
// enum ParamType{
    
// }
// enum Param{}

impl Lsm9ds1 {
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
    // fn params(&mut self) -> Vec<Param> { unimplemented!() }
    // fn param(&mut self, param_type: ParamType) -> Result<Param, ()> { unimplemented!() }
    // fn set_params(&mut self, params: Vec<Param>) -> Result<(), ()> { unimplemented!() }
    // fn set_param(&mut self, param_type: ParamType) -> Result<(), ()> { unimplemented!() }
    

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
