extern crate lsm9ds1;

use lsm9ds1::Lsm9ds1;
use lsm9ds1::i2c::I2cInterface;
use lsm9ds1::config::{Config, FsXl, DataRate, OutputDataRate, Md};

use std::mem;
use std::io::{self, Write};

const G: f32 = 9.81;

fn main() {
    let default = Config::default();
    println!("let default = Config::default();");
    println!("  mem::size_of::<Config>(): {}", mem::size_of::<Config>());
    println!("  mem::size_of_val::<Config>(&default): {}", mem::size_of_val::<Config>(&default));
    let interface = I2cInterface::new("/dev/i2c-2");
    let mut lsm9ds1 = Lsm9ds1::from_interface(interface).unwrap();
    lsm9ds1.set_fs_xl(FsXl::Fs8).expect("Unable to set fs_xl");
    lsm9ds1.set_odr_g(DataRate::DR119Hz).expect("Unable to set odr_g");
    lsm9ds1.set_odr_xl(DataRate::DR119Hz).expect("Unable to set odr_xl");
    lsm9ds1.set_output_data_rate(OutputDataRate::Odr10Hz).expect("Unable to set output_data_rate");
    lsm9ds1.set_md(Md::Continuous);
    lsm9ds1.re_read_config().expect("Unable to reread config");
    let t = lsm9ds1.temp().unwrap();
    println!("temp: {}\r", t);
    for _ in 0..100_000 {
        print!("a⃗=[{:5.2}, {:5.2}, {:5.2}] m/s⁻², g=[{:5.2}, {:5.2}, {:5.2}] °/s⁻², m=[{:5.2}, {:5.2}, {:5.2}] gauss\r",
                 G * lsm9ds1.lx().unwrap(), G * lsm9ds1.ly().unwrap(), G * lsm9ds1.lz().unwrap(),
                 lsm9ds1.gx().unwrap(), lsm9ds1.gy().unwrap(), lsm9ds1.gz().unwrap(),
                 lsm9ds1.mx().unwrap(), lsm9ds1.my().unwrap(), lsm9ds1.mz().unwrap(),
        );
        io::stdout().flush().unwrap();
    }
}
