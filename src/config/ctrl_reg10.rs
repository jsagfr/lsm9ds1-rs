use super::super::Address;
use super::{Register, CTRL_REG10};

const ERROR1: u8 = 0b10000000;
const ERROR2: u8 = 0b01000000;
const ERROR3: u8 = 0b00100000;
const ERROR4: u8 = 0b00010000;
const ERROR5: u8 = 0b00001000;
const ST_G:   u8 = 0b00000100;
const ERROR6: u8 = 0b00000010;
const ST_XL:  u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg10 {
    st_g: bool,
    st_xl: bool,
}

impl Register<u8> for CtrlReg10 {
    fn addr(&self) -> Address {
        CTRL_REG10
    }
    
    fn default() -> Self {
        CtrlReg10 {
            st_g: false,
            st_xl: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg10 {
            st_g: reg & ST_G != 0,
            st_xl: reg & ST_XL != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.st_g {ST_G} else {0};
        if self.st_xl {reg |= ST_XL;}
        reg
    }
}

impl CtrlReg10 {
    pub fn set_st_g(&mut self, value: bool) {self.st_g = value}
    pub fn st_g(&self) -> bool {self.st_g}
    pub fn set_st_xl(&mut self, value: bool) {self.st_xl = value}
    pub fn st_xl(&self) -> bool {self.st_xl}
}

#[cfg(test)]
mod tests {
    use super::CtrlReg10;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b0000_0101;
        let r = CtrlReg10::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
