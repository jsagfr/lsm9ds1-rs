use super::{Register, Param, DataRate, FsXl, Bw};

const ODR_XL_MASK:       u8 = 0b111_00000;
const ODR_XL_POWER_DOWN: u8 = 0b000_00000;
const ODR_XL_14_9_HZ:    u8 = 0b001_00000;
const ODR_XL_59_5_HZ:    u8 = 0b010_00000;
const ODR_XL_119_HZ:     u8 = 0b011_00000;
const ODR_XL_238_HZ:     u8 = 0b100_00000;
const ODR_XL_476_HZ:     u8 = 0b101_00000;
const ODR_XL_952_HZ:     u8 = 0b110_00000;
const ODR_XL_NA:         u8 = 0b111_00000;
const FS_XL_MASK:        u8 = 0b000_11_000;
const FS_XL_2:           u8 = 0b000_00_000;
const FS_XL_4:           u8 = 0b000_01_000;
const FS_XL_8:           u8 = 0b000_10_000;
const FS_XL_16:          u8 = 0b000_11_000;
const BW_SCAL_ODR:       u8 = 0b0000_0100;
const BW_XL_MASK:        u8 = 0b0000_00_11;
const BW_XL_A:           u8 = 0b0000_00_00;
const BW_XL_B:           u8 = 0b0000_00_01;
const BW_XL_C:           u8 = 0b0000_00_10;
const BW_XL_D:           u8 = 0b0000_00_11;



pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::OdrXl(x) => {
                reg |= match x {
                    DataRate::PowerDown => ODR_XL_POWER_DOWN,
                    DataRate::DR15Hz    => ODR_XL_14_9_HZ,
                    DataRate::DR59Hz    => ODR_XL_59_5_HZ,
                    DataRate::DR119Hz   => ODR_XL_119_HZ,
                    DataRate::DR238Hz   => ODR_XL_238_HZ,
                    DataRate::DR476Hz   => ODR_XL_476_HZ,
                    DataRate::DR952Hz   => ODR_XL_952_HZ,
                    DataRate::NA        => ODR_XL_NA,
                }
            }
            Param::FsXl(x) => {
                reg = reg & !FS_XL_MASK | match x {
                    FsXl::Fs2  => FS_XL_2,
                    FsXl::Fs4  => FS_XL_4,
                    FsXl::Fs8  => FS_XL_8,
                    FsXl::Fs16 => FS_XL_16,
                }
            }
            Param::BwScalOdr(x) =>  reg = if x {
                reg |  BW_SCAL_ODR
            } else {
                reg & !BW_SCAL_ODR
            },
            Param::BwXl(x) => {
                reg |= match x {
                    Bw::A => BW_XL_A,
                    Bw::B => BW_XL_B,
                    Bw::C => BW_XL_C,
                    Bw::D => BW_XL_D,
                }
            }
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg6Xl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg6Xl(r) => {
            let odr_xl = Param::OdrXl(match r & ODR_XL_MASK {
                ODR_XL_POWER_DOWN => DataRate::PowerDown,
                ODR_XL_14_9_HZ    => DataRate::DR15Hz,
                ODR_XL_59_5_HZ    => DataRate::DR59Hz,
                ODR_XL_119_HZ     => DataRate::DR119Hz,
                ODR_XL_238_HZ     => DataRate::DR238Hz,
                ODR_XL_476_HZ     => DataRate::DR476Hz,
                ODR_XL_952_HZ     => DataRate::DR952Hz,
                ODR_XL_NA         => DataRate::NA,
                _ =>  unreachable!(),
            });
            let fs_xl = Param::FsXl(match r & FS_XL_MASK {
                FS_XL_2  => FsXl::Fs2,
                FS_XL_4  => FsXl::Fs4,
                FS_XL_8  => FsXl::Fs8,
                FS_XL_16 => FsXl::Fs16,
                _ =>  unreachable!(),
            });
            let bso = Param::BwScalOdr(r & BW_SCAL_ODR == BW_SCAL_ODR);
            let bw_xl = Param::BwXl(match r & BW_XL_MASK {
                BW_XL_A => Bw::A,
                BW_XL_B => Bw::B,
                BW_XL_C => Bw::C,
                BW_XL_D => Bw::D,
                _ =>  unreachable!(),
            });
            Ok(vec![odr_xl, fs_xl, bso, bw_xl])
        }
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg6Xl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
