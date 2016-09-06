use super::super::Address;
use super::{Register, CTRL_REG3_G};

const ERRORS_MASK:  u8 = 0b00_11_0000;
const LP_MODE_MASK: u8 = 0b10_00_0000;
const HP_EN_MASK:   u8 = 0b01_00_0000;
const HPCF_G_MASK:  u8 = 0b00_00_1111;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg3G {
    lp_mode: bool,
    hp_en: bool,
    hpcf_g: u8,
}

impl Register<u8> for CtrlReg3G {
    fn addr(&self) -> Address {
        CTRL_REG3_G
    }
    
    fn default() -> Self {
        CtrlReg3G {
            hpcf_g: 0,
            lp_mode: false,
            hp_en: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg3G {
            hpcf_g: reg & HPCF_G_MASK,
            lp_mode: reg & LP_MODE_MASK != 0,
            hp_en: reg & HP_EN_MASK != 0,
        }
    }

    fn reg(&self) -> u8 {
        self.hpcf_g | if self.lp_mode {LP_MODE_MASK} else {0} | if self.hp_en {HP_EN_MASK} else {0}
    }
}

impl CtrlReg3G {
    pub fn set_hpcf_g(&mut self, value: u8) {
        assert!(value <= HPCF_G_MASK);
        self.hpcf_g = value
    }

    pub fn hpcf_g(&self) -> u8 {
        self.hpcf_g
    }

    pub fn set_lp_mode(&mut self, value: bool) {
        self.lp_mode = value
    }

    pub fn lp_mode(&self) -> bool {
        self.lp_mode
    }

    pub fn set_hp_en(&mut self, value: bool) {
        self.hp_en = value
    }

    pub fn hp_en(&self) -> bool {
        self.hp_en
    }
}


#[cfg(test)]
mod tests {
    use super::CtrlReg3G;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b1100_1010;
        let r = CtrlReg3G::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
