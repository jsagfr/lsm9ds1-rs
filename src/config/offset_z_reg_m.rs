use super::super::Address;
use super::{Register, OFFSET_Z_REG_M};

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetZRegM {
    offset_z_reg_m: u16,
}

impl Register<u16> for OffsetZRegM {
    fn addr(&self) -> Address {
        OFFSET_Z_REG_M
    }
    
    fn default() -> Self {
        OffsetZRegM {
            offset_z_reg_m: 0,
        }
    }

    fn new(reg: u16) -> Self {
        OffsetZRegM {
            offset_z_reg_m: reg,
        }
    }

    fn reg(&self) -> u16 {
        self.offset_z_reg_m
    }
}

impl OffsetZRegM {
    
    pub fn set_offset_z_reg_m(&mut self, value: u16) {
        self.offset_z_reg_m = value;
    }

    pub fn offset_z_reg_m(&self) -> u16 {
        self.offset_z_reg_m
    }
}

#[cfg(test)]
mod tests {
    use super::OffsetZRegM;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0x1A1A;
        let r = OffsetZRegM::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
