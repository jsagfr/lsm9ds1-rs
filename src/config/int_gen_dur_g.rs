use super::{Register, INT_GEN_DUR_G};

const WAIT_G_MASK: u8 = 0b10000000;
const DUR_G_MASK:  u8 = 0b01111111;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenDurG {
    wait_g: bool,
    dur_g: u8,
}

impl Register<u8> for IntGenDurG {
    fn addr(&self) -> u8 {
        INT_GEN_DUR_G
    }
    
    fn default() -> Self {
        IntGenDurG {
            wait_g: false,
            dur_g: 0,
        }
    }

    fn new(reg: u8) -> Self {
        IntGenDurG {
            wait_g: reg & WAIT_G_MASK != 0,
            dur_g: reg & DUR_G_MASK,
        }
    }

    fn reg(&self) -> u8 {
        self.dur_g | if self.wait_g {WAIT_G_MASK} else {0}
    }
}

impl IntGenDurG {
    pub fn set_dur_g(&mut self, value: u8) {
        assert!(value <= DUR_G_MASK);
        self.dur_g = value
    }

    pub fn dur_g(&self) -> u8 {
        self.dur_g
    }

    pub fn set_wait_g(&mut self, value: bool) {
        self.wait_g = value
    }

    pub fn wait_g(&self) -> bool {
        self.wait_g
    }
}

#[cfg(test)]
mod tests {
    use super::IntGenDurG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenDurG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
