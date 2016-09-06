use super::{Register, INT_GEN_THS_Y_G};

const INT_GEN_THS_Y_G_MASK: u16 = 0b0111_1111_1111_1111;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsYG {
    int_gen_ths_y_g: u16,
}

impl Register<u16> for IntGenThsYG {
    fn addr(&self) -> u8 {
        INT_GEN_THS_Y_G
    }
    
    fn default() -> Self {
        IntGenThsYG {
            int_gen_ths_y_g: 0,
        }
    }

    fn new(reg: u16) -> Self {
        assert!(reg <= INT_GEN_THS_Y_G_MASK);
        IntGenThsYG {
            int_gen_ths_y_g: reg,
        }
    }

    fn reg(&self) -> u16 {
        self.int_gen_ths_y_g
    }
}

impl IntGenThsYG {
    
    pub fn set_int_gen_ths_y_g(&mut self, value: u16) {
        assert!(value <= INT_GEN_THS_Y_G_MASK);
        self.int_gen_ths_y_g = value;
    }

    pub fn int_gen_ths_y_g(&self) -> u16 {
        self.int_gen_ths_y_g
    }
}

#[cfg(test)]
mod tests {
    use super::IntGenThsYG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u16 = 0x721A;
        let r = IntGenThsYG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
