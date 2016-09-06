use super::{Register, INT_GEN_THS_X_XL};

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsXXl {
    int_gen_ths_x_xl: u8,
}

impl Register<u8> for IntGenThsXXl {
    // const ADDR: Address = Address::RW(0x04);
    fn addr(&self) -> u8 {
        INT_GEN_THS_X_XL
    }
    
    fn default() -> IntGenThsXXl {
        IntGenThsXXl {
            int_gen_ths_x_xl: 0,
        }
    }

    fn new(reg: u8) -> IntGenThsXXl {
        IntGenThsXXl {
            int_gen_ths_x_xl: reg,
        }
    }

    fn reg(&self) -> u8 {
        self.int_gen_ths_x_xl
    }
}

impl IntGenThsXXl {
    pub fn set_int_gen_ths_x_xl(&mut self, value: u8) {
        self.int_gen_ths_x_xl = value;
    }

    pub fn int_gen_ths_x_xl(&self) -> u8 {
        self.int_gen_ths_x_xl
    }
}

#[cfg(test)]
mod tests {
    use super::IntGenThsXXl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenThsXXl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
