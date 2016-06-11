extern crate lsm9ds1;

// use lsm9ds1::{Lsm9ds1, Cmd};
// use lsm9ds1::i2c::Lsm9ds1I2c;
// use lsm9ds1::accelerometer::{Reg6Builder, Reg6ODR, Reg6FS};



use std::thread;
use std::time::Duration;


fn main() {
    // let dev_path = "/dev/i2c-2";
    let d = Duration::new(0, 10_000_000);

    // let i2c = Lsm9ds1I2c::new(dev_path);

    // let mut l = Lsm9ds1::new(i2c);
    // let r = Reg6Builder::new()
    //     .ord(Reg6ODR::Freq119Hz)
    //     .fs(Reg6FS::Acc2g)
    //     .finalize();
    // l.write(Cmd::Reg6(r));

    for _ in 0..100 {
        // println!("x:{:>#13.10}, y:{:>#13.10}, z:{:>#13.10}",
        //          l.x().unwrap(),
        //          l.y().unwrap(),
        //          l.z().unwrap());
        thread::sleep(d);
    }
}
