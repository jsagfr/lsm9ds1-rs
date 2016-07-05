use super::{Register, Param, OpMode};

const ERRORS:             u8 = 0b1111_00_0_1;
const BLE:                u8 = 0b0000_00_1_0;
const OM_MASK:            u8 = 0b0000_11_0_0;
const OM_LOW_POWER:       u8 = 0b0000_00_0_0;
const OM_MEDIUM_PERF:     u8 = 0b0000_01_0_0;
const OM_HIGH_PERF:       u8 = 0b0000_10_0_0;
const OM_ULTRA_HIGH_PERF: u8 = 0b0000_11_0_0;


pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::OpModeZ(x) => {
                reg = reg & !OM_MASK | match x {
                    OpMode::LowPower      => OM_LOW_POWER,
                    OpMode::MediumPerf    => OM_MEDIUM_PERF,
                    OpMode::HighPerf      => OM_HIGH_PERF,
                    OpMode::UltraHighPerf => OM_ULTRA_HIGH_PERF,
                }
            }
            Param::BigLittleEndian(x) =>  reg = if x {
                reg |  BLE
            } else {
                reg & !BLE
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg4M(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg4M(r) => {
            if r & ERRORS != 0 {
                return Err(())
            };
            let ble = Param::BigLittleEndian(r & BLE == BLE);
            let omz = Param::OpModeZ(match r & OM_MASK {
                OM_LOW_POWER       => OpMode::LowPower,
                OM_MEDIUM_PERF     => OpMode::MediumPerf,
                OM_HIGH_PERF       => OpMode::HighPerf,
                OM_ULTRA_HIGH_PERF => OpMode::UltraHighPerf,
                _ => unreachable!(),
            });
            Ok(vec![ble, omz])
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
        let r1 = Register::CtrlReg4M(!0b1111_00_0_1);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
