use super::{Register, ACT_DUR};

#[derive(Clone, Debug, PartialEq)]
pub struct ActDur {
    act_dur: u8,
}

impl Register<u8> for ActDur {
    // const ADDR: Address = Address::RW(0x04);
    fn addr(&self) -> u8 {
        ACT_DUR
    }
    
    fn default() -> ActDur {
        ActDur {
            act_dur: 0,
        }
    }

    fn new(reg: u8) -> ActDur {
        ActDur {
            act_dur: reg,
        }
    }

    fn reg(&self) -> u8 {
        self.act_dur
    }
}

impl ActDur {
    pub fn set_act_dur(&mut self, value: u8) {
        self.act_dur = value;
    }

    pub fn act_dur(&self) -> u8 {
        self.act_dur
    }
}

#[cfg(test)]
mod tests {
    use super::ActDur;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = ActDur::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
