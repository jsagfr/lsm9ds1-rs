use super::super::Address;
use super::{Register, INT_GEN_THS_X_G};

const DCRM_G_MASK:          u16 = 0b1000_0000_0000_0000;
const INT_GEN_THS_X_G_MASK: u16 = 0b0111_1111_1111_1111;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsXG {
    dcrm_g: bool,
    int_gen_ths_x_g: u16,
}

impl Register<u16> for IntGenThsXG {
    fn addr(&self) -> Address {
        INT_GEN_THS_X_G
    }
    
    fn default() -> Self {
        IntGenThsXG {
            int_gen_ths_x_g: 0,
            dcrm_g: false,
        }
    }

    fn new(reg: u16) -> Self {
        IntGenThsXG {
            int_gen_ths_x_g: reg & INT_GEN_THS_X_G_MASK,
            dcrm_g: reg & DCRM_G_MASK != 0,
        }
    }

    fn reg(&self) -> u16 {
        self.int_gen_ths_x_g | if self.dcrm_g {DCRM_G_MASK} else {0}
    }
}

impl IntGenThsXG {
    pub fn set_int_gen_ths_x_g(&mut self, value: u16) {
        assert!(value <= INT_GEN_THS_X_G_MASK);
        self.int_gen_ths_x_g = value
    }

    pub fn int_gen_ths_x_g(&self) -> u16 {
        self.int_gen_ths_x_g
    }

    pub fn set_dcrm_g(&mut self, value: bool) {
        self.dcrm_g = value
    }

    pub fn dcrm_g(&self) -> bool {
        self.dcrm_g
    }
}

#[cfg(test)]
mod tests {
    use super::IntGenThsXG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0xF21A;
        let r = IntGenThsXG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
