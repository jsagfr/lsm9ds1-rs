use super::super::Address;
use super::{Register, INT_GEN_THS_Y_XL};

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenThsYXl {
    int_gen_ths_y_xl: u8,
}

impl Register<u8> for IntGenThsYXl {
    fn addr(&self) -> Address {
        INT_GEN_THS_Y_XL
    }
    
    fn default() -> IntGenThsYXl {
        IntGenThsYXl {
            int_gen_ths_y_xl: 0,
        }
    }

    fn new(reg: u8) -> IntGenThsYXl {
        IntGenThsYXl {
            int_gen_ths_y_xl: reg,
        }
    }

    fn reg(&self) -> u8 {
        self.int_gen_ths_y_xl
    }
}

impl IntGenThsYXl {
    pub fn set_int_gen_ths_y_xl(&mut self, value: u8) {
        self.int_gen_ths_y_xl = value;
    }

    pub fn int_gen_ths_y_xl(&self) -> u8 {
        self.int_gen_ths_y_xl
    }
}


#[cfg(test)]
mod tests {
    use super::IntGenThsYXl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenThsYXl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
