use super::{Register, Param, OpMode, OutputDataRate};

const TEMP_COMP:          u8 = 0b1_00_000_0_0;

const OM_MASK:            u8 = 0b0_11_000_0_0;
const OM_LOW_POWER:       u8 = 0b0_00_000_0_0;
const OM_MEDIUM_PERF:     u8 = 0b0_01_000_0_0;
const OM_HIGH_PERF:       u8 = 0b0_10_000_0_0;
const OM_ULTRA_HIGH_PERF: u8 = 0b0_11_000_0_0;

const ODR_MASK:           u8 = 0b0_00_111_0_0;
const ODR_0_625_HZ:       u8 = 0b0_00_000_0_0;
const ODR_1_25_HZ:        u8 = 0b0_00_001_0_0;
const ODR_2_5_HZ:         u8 = 0b0_00_010_0_0;
const ODR_5_HZ:           u8 = 0b0_00_011_0_0;
const ODR_10_HZ:          u8 = 0b0_00_100_0_0;
const ODR_20_HZ:          u8 = 0b0_00_101_0_0;
const ODR_40_HZ:          u8 = 0b0_00_110_0_0;
const ODR_80_HZ:          u8 = 0b0_00_111_0_0;

const ERRORS:             u8 = 0b0_00_000_1_0;

const SELF_TEST:          u8 = 0b0_00_000_0_1;



pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::OpMode(x) => {
                reg = reg & !OM_MASK | match x {
                    OpMode::LowPower      => OM_LOW_POWER,
                    OpMode::MediumPerf    => OM_MEDIUM_PERF,
                    OpMode::HighPerf      => OM_HIGH_PERF,
                    OpMode::UltraHighPerf => OM_ULTRA_HIGH_PERF,
                }
            }
            Param::OutputDataRate(x) => {
                reg = reg & !ODR_MASK | match x {
                    OutputDataRate::Odr0p625Hz => ODR_0_625_HZ,
                    OutputDataRate::Odr1p25Hz  => ODR_1_25_HZ,
                    OutputDataRate::Odr2p5Hz   => ODR_2_5_HZ,
                    OutputDataRate::Odr5Hz     => ODR_5_HZ,
                    OutputDataRate::Odr10Hz    => ODR_10_HZ,
                    OutputDataRate::Odr20Hz    => ODR_20_HZ,
                    OutputDataRate::Odr40Hz    => ODR_40_HZ,
                    OutputDataRate::Odr80Hz    => ODR_80_HZ,
                }
            }
            Param::TempComp(x) =>  reg = if x {
                reg |  TEMP_COMP
            } else {
                reg & !TEMP_COMP
            },
            Param::SelfTest(x) =>  reg = if x {
                reg |  SELF_TEST
            } else {
                reg & !SELF_TEST
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg1M(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg1M(r) => {
            if r & ERRORS == ERRORS {
                return Err(())
            };
            let temp_comp = Param::TempComp(r & TEMP_COMP == TEMP_COMP);
            let self_test = Param::SelfTest(r & SELF_TEST == SELF_TEST);
            let om = Param::OpMode(match r & OM_MASK {
                OM_LOW_POWER       => OpMode::LowPower,
                OM_MEDIUM_PERF     => OpMode::MediumPerf,
                OM_HIGH_PERF       => OpMode::HighPerf,
                OM_ULTRA_HIGH_PERF => OpMode::UltraHighPerf,
                _ => unreachable!(),
            });
            let odr = Param::OutputDataRate(match r & ODR_MASK {
                ODR_0_625_HZ => OutputDataRate::Odr0p625Hz,
                ODR_1_25_HZ  => OutputDataRate::Odr1p25Hz,
                ODR_2_5_HZ   => OutputDataRate::Odr2p5Hz,
                ODR_5_HZ     => OutputDataRate::Odr5Hz,
                ODR_10_HZ    => OutputDataRate::Odr10Hz,
                ODR_20_HZ    => OutputDataRate::Odr20Hz,
                ODR_40_HZ    => OutputDataRate::Odr40Hz,
                ODR_80_HZ    => OutputDataRate::Odr80Hz,
                _ => unreachable!(),
            });
            Ok(vec![temp_comp, self_test, om, odr])
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
        let r1 = Register::CtrlReg1M(0b1_11_111_0_1);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
