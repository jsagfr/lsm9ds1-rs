use super::{Register, Param, DataRate, GyroScale, Bw};

const ODR_G_MASK:       u8 = 0b111_00000;
const ODR_G_POWER_DOWN: u8 = 0b000_00000;
const ODR_G_14_9_HZ:    u8 = 0b001_00000;
const ODR_G_59_5_HZ:    u8 = 0b010_00000;
const ODR_G_119_HZ:     u8 = 0b011_00000;
const ODR_G_238_HZ:     u8 = 0b100_00000;
const ODR_G_476_HZ:     u8 = 0b101_00000;
const ODR_G_952_HZ:     u8 = 0b110_00000;
const ODR_G_NA:         u8 = 0b111_00000;
const FS_G_MASK:        u8 = 0b000_11_000;
const FS_G_245_DPS:     u8 = 0b000_00_000;
const FS_G_500_DPS:     u8 = 0b000_01_000;
const FS_G_NA:          u8 = 0b000_10_000;
const FS_G_2000_DPS:    u8 = 0b000_11_000;
const ERROR1:           u8 = 0b0000_0100;
const BW_G_MASK:        u8 = 0b0000_00_11;
const BW_G_A:           u8 = 0b0000_00_00;
const BW_G_B:           u8 = 0b0000_00_01;
const BW_G_C:           u8 = 0b0000_00_10;
const BW_G_D:           u8 = 0b0000_00_11;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::OdrG(x) => {
                reg |= match x {
                    DataRate::PowerDown => ODR_G_POWER_DOWN,
                    DataRate::DR15Hz    => ODR_G_14_9_HZ,
                    DataRate::DR59Hz    => ODR_G_59_5_HZ,
                    DataRate::DR119Hz   => ODR_G_119_HZ,
                    DataRate::DR238Hz   => ODR_G_238_HZ,
                    DataRate::DR476Hz   => ODR_G_476_HZ,
                    DataRate::DR952Hz   => ODR_G_952_HZ,
                    DataRate::NA        => ODR_G_NA,
                }
            }
            Param::FsG(x) => {
                reg |= match x {
                    GyroScale::NA        => FS_G_NA,
                    GyroScale::FS245Dps  => FS_G_245_DPS,
                    GyroScale::FS500Dps  => FS_G_500_DPS,
                    GyroScale::FS2000Dps => FS_G_2000_DPS,
                }
            }
            Param::BwG(x) => {
                reg |= match x {
                    Bw::A => BW_G_A,
                    Bw::B => BW_G_B,
                    Bw::C => BW_G_C,
                    Bw::D => BW_G_D,
                }
            }
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg1G(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg1G(r) => {
            let odr_g = Param::OdrG(match r & ODR_G_MASK {
                ODR_G_POWER_DOWN => DataRate::PowerDown,
                ODR_G_14_9_HZ    => DataRate::DR15Hz,
                ODR_G_59_5_HZ    => DataRate::DR59Hz,
                ODR_G_119_HZ     => DataRate::DR119Hz,
                ODR_G_238_HZ     => DataRate::DR238Hz,
                ODR_G_476_HZ     => DataRate::DR476Hz,
                ODR_G_952_HZ     => DataRate::DR952Hz,
                ODR_G_NA         => DataRate::NA,
                _ =>  unreachable!(),
            });
            let fs_g = Param::FsG(match r & FS_G_MASK {
                FS_G_NA       => GyroScale::NA,
                FS_G_245_DPS  => GyroScale::FS245Dps,
                FS_G_500_DPS  => GyroScale::FS500Dps,
                FS_G_2000_DPS => GyroScale::FS2000Dps,
                _ =>  unreachable!(),
            });
            let bw_g = Param::BwG(match r & BW_G_MASK {
                BW_G_A => Bw::A,
                BW_G_B => Bw::B,
                BW_G_C => Bw::C,
                BW_G_D => Bw::D,
                _ =>  unreachable!(),
            });
            Ok(vec![odr_g, fs_g, bw_g])
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
        let r1 = Register::CtrlReg1G(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
