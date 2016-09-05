use super::{Register, INT_GEN_THS_Z_XL};

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsZXl {
    int_gen_ths_z_xl: u8,
}

impl Register<u8> for IntGenThsZXl {
    fn addr(&self) -> u8 {
        INT_GEN_THS_Z_XL
    }
    
    fn default() -> IntGenThsZXl {
        IntGenThsZXl {
            int_gen_ths_z_xl: 0,
        }
    }

    fn new(reg: u8) -> IntGenThsZXl {
        IntGenThsZXl {
            int_gen_ths_z_xl: reg,
        }
    }

    fn reg(&self) -> u8 {
        self.int_gen_ths_z_xl
    }
}

impl IntGenThsZXl {
    pub fn set_int_gen_ths_z_xl(&mut self, value: u8) {
        self.int_gen_ths_z_xl = value;
    }

    pub fn int_gen_ths_z_xl(&self) -> u8 {
        self.int_gen_ths_z_xl
    }
}


#[cfg(test)]
mod tests {
    use super::IntGenThsZXl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenThsZXl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
