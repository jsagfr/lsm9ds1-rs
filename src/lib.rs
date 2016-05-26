#![allow(dead_code)]
pub mod register;
pub mod reg6;
pub mod out_acc;

use std::u16::MAX;
use register::{WriteAddress, ReadAddress, ReadWordAddress, Write};
use reg6::Reg6FS;
use out_acc::{OUT_X_ADDRESS_R, OUT_Y_ADDRESS_R, OUT_Z_ADDRESS_R};

pub trait Device {
    // TODO: Result<>
    fn read(&mut self, address: ReadAddress) -> u8;
    // TODO: Result<>
    fn readword(&mut self, address: ReadWordAddress) -> u16;
    // TODO: Result<>
    fn write<T: Write>(&mut self, cmd: T);   
}


pub struct Lsm9ds1<D: Device> {
    reg6: Option<reg6::Reg6>,
    device: D,
}

impl<D: Device> Lsm9ds1<D> {
    
    pub fn new(device: D) -> Lsm9ds1<D> {
        Lsm9ds1 {
            reg6: None,
            device: device,
        }
    }
    
    // TODO: Result<>
    fn scale_acc(&self, value: u16) -> Option<f32> {
        match self.reg6 {
            Some(r) => {
                let s = match r.fs {
                    Reg6FS::Acc2g  =>  2.0,
                    Reg6FS::Acc4g  =>  4.0,
                    Reg6FS::Acc8g  =>  8.0,
                    Reg6FS::Acc16g => 16.0,
                };
                Some(s * (value as f32) / (MAX as f32))
            }
            None => None,
        }
    }
    
    pub fn x(&mut self) -> Option<f32> {
        let raw_x = self.device.readword(OUT_X_ADDRESS_R);
        self.scale_acc(raw_x)
    }
    
    pub fn y(&mut self) -> Option<f32> {
        let raw_y = self.device.readword(OUT_Y_ADDRESS_R);
        self.scale_acc(raw_y)
    }
    
    pub fn z(&mut self) -> Option<f32> {
        let raw_z = self.device.readword(OUT_Z_ADDRESS_R);
        self.scale_acc(raw_z)
    }

    pub fn write<T: Write>(&mut self, cmd: T) {
        self.device.write(cmd);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
