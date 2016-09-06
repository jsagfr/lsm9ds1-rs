use super::{Register, OFFSET_Y_REG_M};

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetYRegM {
    offset_y_reg_m: u16,
}

impl Register<u16> for OffsetYRegM {
    fn addr(&self) -> u8 {
        OFFSET_Y_REG_M
    }
    
    fn default() -> Self {
        OffsetYRegM {
            offset_y_reg_m: 0,
        }
    }

    fn new(reg: u16) -> Self {
        OffsetYRegM {
            offset_y_reg_m: reg,
        }
    }

    fn reg(&self) -> u16 {
        self.offset_y_reg_m
    }
}

impl OffsetYRegM {
    
    pub fn set_offset_y_reg_m(&mut self, value: u16) {
        self.offset_y_reg_m = value;
    }

    pub fn offset_y_reg_m(&self) -> u16 {
        self.offset_y_reg_m
    }
}

#[cfg(test)]
mod tests {
    use super::OffsetYRegM;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0x1A1A;
        let r = OffsetYRegM::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
