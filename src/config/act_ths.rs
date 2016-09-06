use super::{Register, ACT_THS};

const ACT_THS_MASK:  u8 = 0b0111_1111;
const SLEEP_ON_MASK: u8 = 0b1000_0000;

#[derive(Clone, Debug, PartialEq)]
pub struct ActThs {
    act_ths: u8,
    sleep_on: bool,
}

impl Register<u8> for ActThs {
    fn addr(&self) -> u8 {
        ACT_THS
    }
    
    fn default() -> Self {
        ActThs {
            act_ths: 0,
            sleep_on: false,
        }
    }

    fn new(reg: u8) -> Self {
        ActThs {
            act_ths: reg & ACT_THS_MASK,
            sleep_on: reg & SLEEP_ON_MASK != 0,
        }
    }

    fn reg(&self) -> u8 {
        self.act_ths | if self.sleep_on {SLEEP_ON_MASK} else {0}
    }
}

impl ActThs {
    pub fn set_act_ths(&mut self, value: u8) {
        assert!(value <= ACT_THS_MASK);
        self.act_ths = value
    }

    pub fn act_ths(&self) -> u8 {
        self.act_ths
    }

    pub fn set_sleep_on(&mut self, value: bool) {
        self.sleep_on = value
    }

    pub fn sleep_on(&self) -> bool {
        self.sleep_on
    }
}

#[cfg(test)]
mod tests {
    use super::ActThs;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0xF1;
        let r = ActThs::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
