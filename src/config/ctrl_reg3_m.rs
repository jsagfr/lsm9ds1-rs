use super::super::Address;
use super::{Register, CTRL_REG3_M, Md};

const ERRORS:         u8 = 0b0_1_0_11_0_00;
const I2C_DISABLE_M:  u8 = 0b1_0_0_00_0_00;
const LP:             u8 = 0b0_0_1_00_0_00;
const SIM_M:          u8 = 0b0_0_0_00_1_00;
const MD_MASK:        u8 = 0b0_0_0_00_0_11;
const MD_CONTINUOUS:  u8 = 0b0_0_0_00_0_00;
const MD_SINGLE:      u8 = 0b0_0_0_00_0_01;
const MD_POWER_DOWN1: u8 = 0b0_0_0_00_0_10;
const MD_POWER_DOWN2: u8 = 0b0_0_0_00_0_11;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg3M {
    md: Md,
    i2c_disable_m: bool,
    low_power_m: bool,
    sim_m: bool,
}

impl Register<u8> for CtrlReg3M {
    fn addr(&self) -> Address {
        CTRL_REG3_M
    }
    
    fn default() -> Self {
        CtrlReg3M {
            md: Md::default(),
            i2c_disable_m: false,
            sim_m: false,
            low_power_m: false,
        }
    }

    fn new(reg: u8) -> Self {
        let md = match reg & MD_MASK {
            MD_CONTINUOUS  => Md::Continuous,
            MD_SINGLE      => Md::Single,
            MD_POWER_DOWN1 => Md::PowerDown,
            MD_POWER_DOWN2 => Md::PowerDown,
            _ => unreachable!(),
        };
        CtrlReg3M {
            md: md,
            i2c_disable_m: reg & I2C_DISABLE_M != 0,
            sim_m: reg & SIM_M != 0,
            low_power_m: reg & LP != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.md {
            Md::Continuous => MD_CONTINUOUS,
            Md::Single     => MD_SINGLE,
            Md::PowerDown  => MD_POWER_DOWN2,
        };
        if self.i2c_disable_m {reg |= I2C_DISABLE_M;}
        if self.sim_m {reg |= SIM_M;}
        if self.low_power_m {reg |= LP;}
        reg
    }
}

impl CtrlReg3M {
    pub fn set_md(&mut self, value: Md) {
        self.md = value
    }

    pub fn md(&self) -> Md {
        self.md
    }

    pub fn set_i2c_disable_m(&mut self, value: bool) {self.i2c_disable_m = value}
    pub fn i2c_disable_m(&self) -> bool {self.i2c_disable_m}
    pub fn set_sim_m(&mut self, value: bool) {self.sim_m = value}
    pub fn sim_m(&self) -> bool {self.sim_m}
    pub fn set_low_power_m(&mut self, value: bool) {self.low_power_m = value}
    pub fn low_power_m(&self) -> bool {self.low_power_m}

}

#[cfg(test)]
mod tests {
    use super::CtrlReg3M;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = !0b0_1_0_11_0_00;
        let r = CtrlReg3M::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
