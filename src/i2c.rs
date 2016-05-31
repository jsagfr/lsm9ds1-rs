
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

use super::Device;
use register::{ReadAddress, ReadWordAddress, WriteAddress, Write};



const SLAVE_ADDR2: u16 = 0x6b;

pub struct Lsm9ds1I2c {
    acc_dev: LinuxI2CDevice
}

impl Lsm9ds1I2c {
    pub fn new(acc_path: &str) -> Lsm9ds1I2c {
        Lsm9ds1I2c {
            acc_dev: LinuxI2CDevice::new(acc_path, SLAVE_ADDR2).expect("unable to open device")
        }
    }
}



impl Device for Lsm9ds1I2c {
    fn read(&mut self, address: ReadAddress) -> u8 {
        let ReadAddress(v) = address;
        match self.acc_dev.smbus_read_byte_data(v) {
            Ok(r) => r,
            Err(_) => 0
        }
    }

    fn readword(&mut self, address: ReadWordAddress) -> u16 {
        let ReadWordAddress(v) = address;
        match self.acc_dev.smbus_read_word_data(v) {
            Ok(r) => r,
            Err(_) => 0
        }
    }
    
    fn write<T: Write>(&mut self, cmd: T) {
        let WriteAddress(v) = cmd.address();
        self.acc_dev.smbus_write_byte_data(v, cmd.value()).expect("unable to write command on device"); 
   }
}


