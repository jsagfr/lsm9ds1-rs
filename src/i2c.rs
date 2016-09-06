
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

use super::{Interface, Address};
// use register::{ReadAddress, ReadWordAddress, WriteAddress, Write};

const I2C_ACC_GYRO_ADDR: u16 = 0x6b;
const I2C_MAGN_ADDR:     u16 = 0x1e;

pub struct I2cInterface {
    acc_gyro_dev: LinuxI2CDevice,
    magn_dev: LinuxI2CDevice,
}


impl I2cInterface {
    pub fn new(i2c_path: &str) -> I2cInterface {
        I2cInterface {
            acc_gyro_dev: LinuxI2CDevice::new(i2c_path, I2C_ACC_GYRO_ADDR).expect("unable to open device"),
            magn_dev: LinuxI2CDevice::new(i2c_path, I2C_MAGN_ADDR).expect("unable to open device"),
        }
    }
}

impl Interface for I2cInterface {
    fn read(&mut self, address: Address) -> Result<u8,()> {
        match address {
            Address::AccGyro(addr) => {
                match self.acc_gyro_dev.smbus_read_byte_data(addr) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
            Address::Magn(addr) => {
                match self.magn_dev.smbus_read_byte_data(addr) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
        }
    }
    
    fn read16(&mut self, address: Address) -> Result<u16,()>{
        match address {
            Address::AccGyro(addr) => {
                match self.acc_gyro_dev.smbus_read_word_data(addr) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
            Address::Magn(addr) => {
                match self.magn_dev.smbus_read_word_data(addr) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
        }
    }
    
    fn write(&mut self, address: Address, value: u8) -> Result<(),()>{
        match address {
            Address::AccGyro(addr) => {
                match self.acc_gyro_dev.smbus_write_byte_data(addr, value) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
            Address::Magn(addr) => {
                match self.magn_dev.smbus_write_byte_data(addr, value) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
        }
    }
    
    fn write16(&mut self, address: Address, value: u16) -> Result<(),()>{
        match address {
            Address::AccGyro(addr) => {
                match self.acc_gyro_dev.smbus_write_word_data(addr, value) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
            Address::Magn(addr) => {
                match self.magn_dev.smbus_write_word_data(addr, value) {
                    Ok(res) => Ok(res),
                    _ => Err(())
                }
            }
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


