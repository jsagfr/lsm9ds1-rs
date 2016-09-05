
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

use super::{Interface};
// use register::{ReadAddress, ReadWordAddress, WriteAddress, Write};

const SLAVE_ADDR2: u16 = 0x6b;

pub struct I2cInterface {
    acc_dev: LinuxI2CDevice
}


impl I2cInterface {
    pub fn new(acc_path: &str) -> I2cInterface {
        I2cInterface {
            acc_dev: LinuxI2CDevice::new(acc_path, SLAVE_ADDR2).expect("unable to open device")
        }
    }
}

impl Interface for I2cInterface {
    fn read(&mut self, address: u8) -> Result<u8,()> {
        match self.acc_dev.smbus_read_byte_data(address) {
            Ok(res) => Ok(res),
            _ => Err(())
        }
    }
    
    fn read16(&mut self, address: u8) -> Result<u16,()>{
        match self.acc_dev.smbus_read_word_data(address) {
            Ok(res) => Ok(res),
            _ => Err(())
        }
    }
    
    fn write(&mut self, address: u8, value: u8) -> Result<(),()>{
        match self.acc_dev.smbus_write_byte_data(address, value) {
            Ok(_) => Ok(()),
            _ => Err(())
        }
    }
    
    fn write16(&mut self, address: u8, value: u16) -> Result<(),()>{
        match self.acc_dev.smbus_write_word_data(address, value) {
            Ok(_) => Ok(()),
            _ => Err(())
        }
    }
}

// impl ReadInterface<u16, ReadAddress> for I2cInterface {
//     fn read(&mut self, address: &ReadAddress) -> Result<u16,()> {
//         unimplemented!();
//         // match address {
//         //     Address::R16(add) => self.acc_dev.smbus_read_word_data(add),
//         //     _ => Err(())
//         // }
//     }
// }
// impl WriteInterface<u8, WritableAddress<u8>> for I2cInterface {
//     fn write(&mut self, address: &WritableAddress<u8>, value: u8) -> Result<(),()> {
//         unimplemented!();
//         // match address {
//         //     Address::RW(add) => self.acc_dev.smbus_write_byte_data(add, value).expect("unable to write command on device"),
//         //     _ => Err(())
//         // }
//         // Ok(())
//     }
// }



// pub struct Lsm9ds1I2c {
//     acc_dev: LinuxI2CDevice
// }



// impl Device for Lsm9ds1I2c {
//     fn read(&mut self, address: ReadAddress) -> u8 {
//         let ReadAddress(v) = address;
//         match self.acc_dev.smbus_read_byte_data(v) {
//             Ok(r) => r,
//             Err(_) => 0
//         }
//     }

//     fn readword(&mut self, address: ReadWordAddress) -> u16 {
//         let ReadWordAddress(v) = address;
//         match self.acc_dev.smbus_read_word_data(v) {
//             Ok(r) => r,
//             Err(_) => 0
//         }
//     }
    
//     fn write<T: Write>(&mut self, cmd: T) {
//         let WriteAddress(v) = cmd.address();
//         self.acc_dev.smbus_write_byte_data(v, cmd.value()).expect("unable to write command on device"); 
//    }
// }


