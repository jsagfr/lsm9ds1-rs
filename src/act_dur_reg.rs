/// ACT_THS register is the Activity threshold register.

pub const REG_ADDR:  u8 = 0x05; // TODO: #![feature(associated_consts)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActDurReg {
    bits: u8,
}

impl ActDurReg {
    fn new(reg: u8) -> Self {
        ActDurReg {
            bits: reg,
        }
    }

    fn from_reg(reg: u8) -> Self {
        ActDurReg {
            bits: reg,
        }
    }

    fn to_reg(&self) -> u8 {
        self.bits
    }

    fn reg_addr(&self) -> u8 {
        REG_ADDR
    }
}

#[cfg(test)]
mod tests {
    use super::{ActDurReg};
    #[test]
    fn it_works() {
        let r1 = ActDurReg::new(21);
        let r2 = ActDurReg::from_reg(r1.to_reg());
        assert_eq!(r1, r2);
    }
}
