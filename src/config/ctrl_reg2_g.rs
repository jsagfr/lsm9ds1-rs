use super::super::Address;
use super::{Register, CTRL_REG2_G, IntSel, OutSel};

const ERRORS_MASK:  u8 = 0b1111_00_00;
const INT_SEL_MASK: u8 = 0b0000_11_00;
const INT_SEL_A:    u8 = 0b0000_00_00;
const INT_SEL_B:    u8 = 0b0000_01_00;
const INT_SEL_C:    u8 = 0b0000_10_00;
const INT_SEL_D:    u8 = 0b0000_11_00;
const OUT_SEL_MASK: u8 = 0b0000_00_11;
const OUT_SEL_A:    u8 = 0b0000_00_00;
const OUT_SEL_B:    u8 = 0b0000_00_01;
const OUT_SEL_C:    u8 = 0b0000_00_10;
const OUT_SEL_D:    u8 = 0b0000_00_11;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg2G {
    int_sel: IntSel,
    out_sel: OutSel,
}

impl Register<u8> for CtrlReg2G {
    fn addr(&self) -> Address {
        CTRL_REG2_G
    }
    
    fn default() -> Self {
        CtrlReg2G {
            int_sel: IntSel::default(),
            out_sel: OutSel::default(),
        }
    }

    fn new(reg: u8) -> Self {
        let int_sel = match reg & INT_SEL_MASK {
            INT_SEL_A => IntSel::A,
            INT_SEL_B => IntSel::B,
            INT_SEL_C => IntSel::C,
            INT_SEL_D => IntSel::D,
            _ =>  unreachable!(),
        };
        let out_sel = match reg & OUT_SEL_MASK {
            OUT_SEL_A => OutSel::A,
            OUT_SEL_B => OutSel::B,
            OUT_SEL_C => OutSel::C,
            OUT_SEL_D => OutSel::D,
            _ =>  unreachable!(),
        };
        CtrlReg2G {
            int_sel: int_sel,
            out_sel: out_sel,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.int_sel {
            IntSel::A => INT_SEL_A,
            IntSel::B => INT_SEL_B,
            IntSel::C => INT_SEL_C,
            IntSel::D => INT_SEL_D,
        };
        reg |= match self.out_sel {
            OutSel::A => OUT_SEL_A,
            OutSel::B => OUT_SEL_B,
            OutSel::C => OUT_SEL_C,
            OutSel::D => OUT_SEL_D,
        };
        reg
    }
}

impl CtrlReg2G {
    pub fn set_int_sel(&mut self, value: IntSel) {
        self.int_sel = value
    }

    pub fn int_sel(&self) -> IntSel {
        self.int_sel
    }

    pub fn set_out_sel(&mut self, value: OutSel) {
        self.out_sel = value
    }

    pub fn out_sel(&self) -> OutSel {
        self.out_sel
    }
}

#[cfg(test)]
mod tests {
    use super::CtrlReg2G;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x0A;
        let r = CtrlReg2G::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
