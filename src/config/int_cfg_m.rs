use super::super::Address;
use super::{Register, INT_CFG_M};

const XIEN:   u8 = 0b10000000;
const YIEN:   u8 = 0b01000000;
const ZIEN:   u8 = 0b00100000;
const ERROR1: u8 = 0b00010000;
const ERROR2: u8 = 0b00001000;
const IEA:    u8 = 0b00000100;
const IEL:    u8 = 0b00000010;
const IEN:    u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct IntCfgM {
    xien: bool,
    yien: bool,
    zien: bool,
    iea: bool,
    iel: bool,
    ien: bool,
}

impl Register<u8> for IntCfgM {
    fn addr(&self) -> Address {
        INT_CFG_M
    }
    
    fn default() -> Self {
        IntCfgM {
            xien: false,
            yien: false,
            zien: false,
            iea: false,
            iel: false,
            ien: false,
        }
    }

    fn new(reg: u8) -> Self {
        IntCfgM {
            xien: reg & XIEN != 0,
            yien: reg & YIEN != 0,
            zien: reg & ZIEN != 0,
            iea: reg & IEA != 0,
            iel: reg & IEL != 0,
            ien: reg & IEN != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.xien {XIEN} else {0};
        if self.yien {reg |= YIEN;}
        if self.zien {reg |= ZIEN;}
        if self.iea {reg |= IEA;}
        if self.iel {reg |= IEL;}
        if self.ien {reg |= IEN;}
        reg
    }
}

impl IntCfgM {
    pub fn set_xien(&mut self, value: bool) {self.xien = value}
    pub fn xien(&self) -> bool {self.xien}
    pub fn set_yien(&mut self, value: bool) {self.yien = value}
    pub fn yien(&self) -> bool {self.yien}
    pub fn set_zien(&mut self, value: bool) {self.zien = value}
    pub fn zien(&self) -> bool {self.zien}
    pub fn set_iea(&mut self, value: bool) {self.iea = value}
    pub fn iea(&self) -> bool {self.iea}
    pub fn set_iel(&mut self, value: bool) {self.iel = value}
    pub fn iel(&self) -> bool {self.iel}
    pub fn set_ien(&mut self, value: bool) {self.ien = value}
    pub fn ien(&self) -> bool {self.ien}
}

#[cfg(test)]
mod tests {
    use super::IntCfgM;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b111_00_111;
        let r = IntCfgM::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
