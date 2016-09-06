use super::{Register, INT_GEN_DUR_XL};

const WAIT_XL_MASK: u8 = 0b10000000;
const DUR_XL_MASK:  u8 = 0b01111111;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenDurXl {
    dur_xl: u8,
    wait_xl: bool,
}

impl Register<u8> for IntGenDurXl {
    fn addr(&self) -> u8 {
        INT_GEN_DUR_XL
    }
    
    fn default() -> Self {
        IntGenDurXl {
            dur_xl: 0,
            wait_xl: false,
        }
    }

    fn new(reg: u8) -> Self {
        IntGenDurXl {
            dur_xl: reg & DUR_XL_MASK,
            wait_xl: if reg & WAIT_XL_MASK == 0 {false} else {true},
        }
    }

    fn reg(&self) -> u8 {
        self.dur_xl | if self.wait_xl {WAIT_XL_MASK} else {0}
    }
}

impl IntGenDurXl {
    pub fn set_dur_xl(&mut self, value: u8) {
        assert!(value <= DUR_XL_MASK);
        self.dur_xl = value
    }

    pub fn dur_xl(&self) -> u8 {
        self.dur_xl
    }

    pub fn set_wait_xl(&mut self, value: bool) {
        self.wait_xl = value
    }

    pub fn wait_xl(&self) -> bool {
        self.wait_xl
    }
}



#[cfg(test)]
mod tests {
    use super::IntGenDurXl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenDurXl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
