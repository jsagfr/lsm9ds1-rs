extern crate i2cdev;
extern crate lsm9ds1;

use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

use lsm9ds1::{Lsm9ds1, Device, Cmd};
use lsm9ds1::register::{ReadAddress, ReadWordAddress, WriteAddress, Write};
use lsm9ds1::reg6::{Reg6Builder, Reg6ODR, Reg6FS};


use std::thread;
use std::time::Duration;

const SLAVE_ADDR2: u16 = 0x6b;

struct Lsm9ds1I2c {
    acc_dev: LinuxI2CDevice
}

impl Lsm9ds1I2c {
    fn new(acc_path: &str) -> Lsm9ds1I2c {
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



fn main() {
    let dev_path = "/dev/i2c-2";
    let d = Duration::new(0, 10_000_000);

    let i2c = Lsm9ds1I2c::new(dev_path);

    let mut l = Lsm9ds1::new(i2c);
    let r = Reg6Builder::new()
        .ord(Reg6ODR::Freq119Hz)
        .fs(Reg6FS::Acc2g)
        .finalize();
    l.write(Cmd::Reg6(r));

    for _ in 0..100 {
        println!("x: {:?}, y:{:?}, z:{:?}",
                 l.x().unwrap(),
                 l.y().unwrap(),
                 l.z().unwrap());
        thread::sleep(d);
    }
}
