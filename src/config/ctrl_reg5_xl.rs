use super::{Register, CTRL_REG5_XL, Dec};

const ERRORS_MASK: u8 = 0b00_000_111;
const ZEN_XL:      u8 = 0b00_100_000;
const YEN_XL:      u8 = 0b00_010_000;
const XEN_XL:      u8 = 0b00_001_000;
const DEC_MASK:    u8 = 0b11_000_000;
const NO_DEC:      u8 = 0b00_000_000;
const DEC_2S:      u8 = 0b01_000_000;
const DEC_4S:      u8 = 0b10_000_000;
const DEC_8S:      u8 = 0b11_000_000;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg5XL {
    zen_xl: bool,
    yen_xl: bool,
    xen_xl: bool,
    dec: Dec,
}

impl Register<u8> for CtrlReg5XL {
    fn addr(&self) -> u8 {
        CTRL_REG5_XL
    }
    
    fn default() -> Self {
        CtrlReg5XL {
            zen_xl: false,
            yen_xl: false,
            xen_xl: false,
            dec: Dec::default(),
        }
    }

    fn new(reg: u8) -> Self {
        let dec = match reg & DEC_MASK {
            NO_DEC => Dec::NoDec,
            DEC_2S => Dec::Dec2S,
            DEC_4S => Dec::Dec4S,
            DEC_8S => Dec::Dec8S,
            _ =>  unreachable!(),
        };
        CtrlReg5XL {
            zen_xl: reg & ZEN_XL != 0,
            yen_xl: reg & YEN_XL != 0,
            xen_xl: reg & XEN_XL != 0,
            dec: dec,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.dec {
            Dec::NoDec => NO_DEC,
            Dec::Dec2S => DEC_2S,
            Dec::Dec4S => DEC_4S,
            Dec::Dec8S => DEC_8S,
        };
        if self.zen_xl {reg |= ZEN_XL;}
        if self.yen_xl {reg |= YEN_XL;}
        if self.xen_xl {reg |= XEN_XL;}
        reg
    }
}

impl CtrlReg5XL {
    pub fn set_dec(&mut self, value: Dec) {
        self.dec = value
    }

    pub fn dec(&self) -> Dec {
        self.dec
    }

    pub fn set_zen_xl(&mut self, value: bool) {self.zen_xl = value}
    pub fn zen_xl(&self) -> bool {self.zen_xl}
    pub fn set_yen_xl(&mut self, value: bool) {self.yen_xl = value}
    pub fn yen_xl(&self) -> bool {self.yen_xl}
    pub fn set_xen_xl(&mut self, value: bool) {self.xen_xl = value}
    pub fn xen_xl(&self) -> bool {self.xen_xl}

}

#[cfg(test)]
mod tests {
    use super::CtrlReg5XL;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b10111000;
        let r = CtrlReg5XL::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
