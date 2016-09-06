use super::super::Address;
use super::{Register, REFERENCE_G};

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceG {
    reference_g: u8,
}

impl Register<u8> for ReferenceG {
    fn addr(&self) -> Address {
        REFERENCE_G
    }
    
    fn default() -> Self {
        ReferenceG {
            reference_g: 0,
        }
    }

    fn new(reg: u8) -> Self {
        ReferenceG {
            reference_g: reg,
        }
    }

    fn reg(&self) -> u8 {
        self.reference_g
    }

}

impl ReferenceG {
    
    pub fn set_reference_g(&mut self, value: u8) {
        self.reference_g = value;
    }

    pub fn reference_g(&self) -> u8 {
        self.reference_g
    }
}

#[cfg(test)]
mod tests {
    use super::ReferenceG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = ReferenceG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
