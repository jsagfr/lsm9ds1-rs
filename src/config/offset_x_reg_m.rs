use super::{Register, OFFSET_X_REG_M};

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetXRegM {
    offset_x_reg_m: u16,
}

impl Register<u16> for OffsetXRegM {
    fn addr(&self) -> u8 {
        OFFSET_X_REG_M
    }
    
    fn default() -> Self {
        OffsetXRegM {
            offset_x_reg_m: 0,
        }
    }

    fn new(reg: u16) -> Self {
        OffsetXRegM {
            offset_x_reg_m: reg,
        }
    }

    fn reg(&self) -> u16 {
        self.offset_x_reg_m
    }
}

impl OffsetXRegM {
    
    pub fn set_offset_x_reg_m(&mut self, value: u16) {
        self.offset_x_reg_m = value;
    }

    pub fn offset_x_reg_m(&self) -> u16 {
        self.offset_x_reg_m
    }
}

#[cfg(test)]
mod tests {
    use super::OffsetXRegM;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0x1A1A;
        let r = OffsetXRegM::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
