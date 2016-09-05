use super::{Register, ORIENT_CFG_G};

const ERRORS_MASK:   u8 = 0b11_000_000;
const SIGN_X_G_MASK: u8 = 0b00_100_000;
const SIGN_Y_G_MASK: u8 = 0b00_010_000;
const SIGN_Z_G_MASK: u8 = 0b00_001_000;
const ORIENT_MASK:   u8 = 0b00_000_111;

#[derive(Clone, Debug, PartialEq)]
pub struct OrientCfgG {
    sign_x_g: bool,
    sign_y_g: bool,
    sign_z_g: bool,
    orient: u8,
}

impl Register<u8> for OrientCfgG {
    fn addr(&self) -> u8 {
        ORIENT_CFG_G
    }
    
    fn default() -> Self {
        OrientCfgG {
            orient: 0,
            sign_x_g: false,
            sign_y_g: false,
            sign_z_g: false,
        }
    }

    fn new(reg: u8) -> Self {
        OrientCfgG {
            orient: reg & ORIENT_MASK,
            sign_x_g: reg & SIGN_X_G_MASK != 0,
            sign_y_g: reg & SIGN_Y_G_MASK != 0,
            sign_z_g: reg & SIGN_Z_G_MASK != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = self.orient;
        if self.sign_x_g {reg |= SIGN_X_G_MASK;}
        if self.sign_y_g {reg |= SIGN_Y_G_MASK;}
        if self.sign_z_g {reg |= SIGN_Z_G_MASK;}
        reg
    }
}

impl OrientCfgG {
    pub fn set_orient(&mut self, value: u8) {
        assert!(value <= ORIENT_MASK);
        self.orient = value
    }

    pub fn orient(&self) -> u8 {
        self.orient
    }

    pub fn set_sign_x_g(&mut self, value: bool) {
        self.sign_x_g = value
    }

    pub fn sign_x_g(&self) -> bool {
        self.sign_x_g
    }

    pub fn set_sign_y_g(&mut self, value: bool) {
        self.sign_y_g = value
    }

    pub fn sign_y_g(&self) -> bool {
        self.sign_y_g
    }

    pub fn set_sign_z_g(&mut self, value: bool) {
        self.sign_z_g = value
    }

    pub fn sign_z_g(&self) -> bool {
        self.sign_z_g
    }
}


#[cfg(test)]
mod tests {
    use super::OrientCfgG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b00_101_011;
        let r = OrientCfgG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
