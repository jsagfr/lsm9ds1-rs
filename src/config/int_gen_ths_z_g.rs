use super::super::Address;
use super::{Register, INT_GEN_THS_Z_G};

const INT_GEN_THS_Z_G_MASK: u16 = 0b0111_1111_1111_1111;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsZG {
    int_gen_ths_z_g: u16,
}

impl Register<u16> for IntGenThsZG {
    fn addr(&self) -> Address {
        INT_GEN_THS_Z_G
    }
    
    fn default() -> Self {
        IntGenThsZG {
            int_gen_ths_z_g: 0,
        }
    }

    fn new(reg: u16) -> Self {
        assert!(reg <= INT_GEN_THS_Z_G_MASK);
        IntGenThsZG {
            int_gen_ths_z_g: reg,
        }
    }

    fn reg(&self) -> u16 {
        self.int_gen_ths_z_g
    }
}

impl IntGenThsZG {
    
    pub fn set_int_gen_ths_z_g(&mut self, value: u16) {
        assert!(value <= INT_GEN_THS_Z_G_MASK);
        self.int_gen_ths_z_g = value;
    }

    pub fn int_gen_ths_z_g(&self) -> u16 {
        self.int_gen_ths_z_g
    }
}

#[cfg(test)]
mod tests {
    use super::IntGenThsZG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0x721A;
        let r = IntGenThsZG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
