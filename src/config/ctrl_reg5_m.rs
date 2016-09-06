use super::super::Address;
use super::{Register, CTRL_REG5_M};

const ERRORS:                 u8 = 0b1_0_111111;
const BLOCK_DATA_UPDATE_MASK: u8 = 0b0_1_000000;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg5M {
    block_data_update: bool,
}

impl Register<u8> for CtrlReg5M {
    fn addr(&self) -> Address {
        CTRL_REG5_M
    }
    
    fn default() -> Self {
        CtrlReg5M {
            block_data_update: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg5M {
            block_data_update: reg & BLOCK_DATA_UPDATE_MASK != 0,
        }
    }

    fn reg(&self) -> u8 {
        if self.block_data_update {BLOCK_DATA_UPDATE_MASK} else {0}
    }
}

impl CtrlReg5M {
    pub fn set_block_data_update(&mut self, value: bool) {
        self.block_data_update = value
    }

    pub fn block_data_update(&self) -> bool {
        self.block_data_update
    }
}

#[cfg(test)]
mod tests {
    use super::CtrlReg5M;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b0_1_000000;
        let r = CtrlReg5M::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
