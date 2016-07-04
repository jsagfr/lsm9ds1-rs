use super::{Register, Param, Md};

const ERRORS:         u8 = 0b0_1_0_11_0_00;
const I2C_DISABLE:    u8 = 0b1_0_0_00_0_00;
const LP:             u8 = 0b0_0_1_00_0_00;
const SIM:            u8 = 0b0_0_0_00_1_00;
const MD_MASK:        u8 = 0b0_0_0_00_0_11;
const MD_CONTINUOUS:  u8 = 0b0_0_0_00_0_00;
const MD_SINGLE:      u8 = 0b0_0_0_00_0_01;
const MD_POWER_DOWN1: u8 = 0b0_0_0_00_0_10;
const MD_POWER_DOWN2: u8 = 0b0_0_0_00_0_11;



pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::Md(x) => {
                reg = reg & !MD_MASK | match x {
                    Md::Continuous => MD_CONTINUOUS,
                    Md::Single     => MD_SINGLE,
                    Md::PowerDown  => MD_POWER_DOWN2,
                }
            }
            Param::I2cDisableM(x) =>  reg = if x {
                reg |  I2C_DISABLE
            } else {
                reg & !I2C_DISABLE
            },
            Param::LowPowerM(x) =>  reg = if x {
                reg |  LP
            } else {
                reg & !LP
            },
            Param::SimM(x) =>  reg = if x {
                reg |  SIM
            } else {
                reg & !SIM
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg3M(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg3M(r) => {
            if r & ERRORS != 0 {
                return Err(())
            };
            let i2c_disable_m = Param::I2cDisableM(r & I2C_DISABLE == I2C_DISABLE);
            let low_power_m = Param::LowPowerM(r & LP == LP);
            let simm = Param::SimM(r & SIM == SIM);
            let fs = Param::Md(match r & MD_MASK {
                MD_CONTINUOUS  => Md::Continuous,
                MD_SINGLE      => Md::Single,
                MD_POWER_DOWN1 => Md::PowerDown,
                MD_POWER_DOWN2 => Md::PowerDown,
                _ => unreachable!(),
            });
            Ok(vec![i2c_disable_m, low_power_m, simm, fs])
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
        let r1 = Register::CtrlReg3M(!0b0_1_0_11_0_00);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
