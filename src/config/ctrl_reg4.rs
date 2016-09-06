use super::super::Address;
use super::{Register, CTRL_REG4};

const ERROR1:   u8 = 0b10000000;
const ERROR2:   u8 = 0b01000000;
const ZEN_G:    u8 = 0b00100000;
const YEN_G:    u8 = 0b00010000;
const XEN_G:    u8 = 0b00001000;
const ERROR3:   u8 = 0b00000100;
const LIR_XL_1: u8 = 0b00000010;
const I_4D_XL_1:u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg4 {
    zen_g: bool,
    yen_g: bool,
    xen_g: bool,
    lir_xl_1: bool,
    i_4d_xl_1: bool,
}

impl Register<u8> for CtrlReg4 {
    fn addr(&self) -> Address {
        CTRL_REG4
    }
    
    fn default() -> Self {
        CtrlReg4 {
            zen_g: false,
            yen_g: false,
            xen_g: false,
            lir_xl_1: false,
            i_4d_xl_1: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg4 {
            zen_g: reg & ZEN_G != 0,
            yen_g: reg & YEN_G != 0,
            xen_g: reg & XEN_G != 0,
            lir_xl_1: reg & LIR_XL_1 != 0,
            i_4d_xl_1: reg & I_4D_XL_1 != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.zen_g {ZEN_G} else {0};
        if self.yen_g {reg |= YEN_G;}
        if self.xen_g {reg |= XEN_G;}
        if self.lir_xl_1 {reg |= LIR_XL_1;}
        if self.i_4d_xl_1 {reg |= I_4D_XL_1;}
        reg
    }
}

impl CtrlReg4 {
    pub fn set_zen_g(&mut self, value: bool) {self.zen_g = value}
    pub fn zen_g(&self) -> bool {self.zen_g}
    pub fn set_yen_g(&mut self, value: bool) {self.yen_g = value}
    pub fn yen_g(&self) -> bool {self.yen_g}
    pub fn set_xen_g(&mut self, value: bool) {self.xen_g = value}
    pub fn xen_g(&self) -> bool {self.xen_g}
    pub fn set_lir_xl_1(&mut self, value: bool) {self.lir_xl_1 = value}
    pub fn lir_xl_1(&self) -> bool {self.lir_xl_1}
    pub fn set_i_4d_xl_1(&mut self, value: bool) {self.i_4d_xl_1 = value}
    pub fn i_4d_xl_1(&self) -> bool {self.i_4d_xl_1}
}


#[cfg(test)]
mod tests {
    use super::CtrlReg4;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = CtrlReg4::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
