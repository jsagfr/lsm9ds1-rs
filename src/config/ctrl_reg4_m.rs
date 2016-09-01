use super::super::Address;
use super::{Register, CTRL_REG4_M, OpMode};

const ERRORS:             u8 = 0b1111_00_0_1;
const BLE:                u8 = 0b0000_00_1_0;
const OM_MASK:            u8 = 0b0000_11_0_0;
const OM_LOW_POWER:       u8 = 0b0000_00_0_0;
const OM_MEDIUM_PERF:     u8 = 0b0000_01_0_0;
const OM_HIGH_PERF:       u8 = 0b0000_10_0_0;
const OM_ULTRA_HIGH_PERF: u8 = 0b0000_11_0_0;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg4M {
    op_mode_z: OpMode,
    big_little_endian: bool,
}

impl Register<u8> for CtrlReg4M {
    fn addr(&self) -> Address {
        CTRL_REG4_M
    }
    
    fn default() -> Self {
        CtrlReg4M {
            op_mode_z: OpMode::default(),
            big_little_endian: false,
        }
    }

    fn new(reg: u8) -> Self {
        let op_mode_z = match reg & OM_MASK {
            OM_LOW_POWER       => OpMode::LowPower,
            OM_MEDIUM_PERF     => OpMode::MediumPerf,
            OM_HIGH_PERF       => OpMode::HighPerf,
            OM_ULTRA_HIGH_PERF => OpMode::UltraHighPerf,
            _ => unreachable!(),
        };
        CtrlReg4M {
            op_mode_z: op_mode_z,
            big_little_endian: reg & BLE != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.op_mode_z {
            OpMode::LowPower      => OM_LOW_POWER,
            OpMode::MediumPerf    => OM_MEDIUM_PERF,
            OpMode::HighPerf      => OM_HIGH_PERF,
            OpMode::UltraHighPerf => OM_ULTRA_HIGH_PERF,
        };
        if self.big_little_endian {reg |= BLE;}
        reg
    }
}

impl CtrlReg4M {
    pub fn set_op_mode_z(&mut self, value: OpMode) {
        self.op_mode_z = value
    }

    pub fn op_mode_z(&self) -> OpMode {
        self.op_mode_z
    }

    pub fn set_big_little_endian(&mut self, value: bool) {self.big_little_endian = value}
    pub fn big_little_endian(&self) -> bool {self.big_little_endian}
}

#[cfg(test)]
mod tests {
    use super::CtrlReg4M;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = !0b1111_00_0_1;
        let r = CtrlReg4M::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
