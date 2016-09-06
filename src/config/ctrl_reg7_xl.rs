use super::{Register, CTRL_REG7_XL, DigCutoffFreq};

const ERRORS:   u8 = 0b000_11_0_1_0;
const HIGH_RES: u8 = 0b100_00_0_0_0;
const DCF_MASK: u8 = 0b011_00_0_0_0;
const DCF_A:    u8 = 0b000_00_0_0_0;
const DCF_B:    u8 = 0b001_00_0_0_0;
const DCF_C:    u8 = 0b010_00_0_0_0;
const DCF_D:    u8 = 0b011_00_0_0_0;
const FDS:      u8 = 0b000_00_1_0_0;
const HPIS:     u8 = 0b000_00_0_0_1;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg7XL {
    xl_digital_cf: DigCutoffFreq,
    high_res: bool,
    filtered_data_sel: bool,
    high_pass_int_sens: bool,
}

impl Register<u8> for CtrlReg7XL {
    fn addr(&self) -> u8 {
        CTRL_REG7_XL
    }
    
    fn default() -> Self {
        CtrlReg7XL {
            xl_digital_cf: DigCutoffFreq::default(),
            high_res: false,
            filtered_data_sel: false,
            high_pass_int_sens: false,
        }
    }

    fn new(reg: u8) -> Self {
        let xl_digital_cf = match reg & DCF_MASK {
            DCF_A => DigCutoffFreq::A,
            DCF_B => DigCutoffFreq::B,
            DCF_C => DigCutoffFreq::C,
            DCF_D => DigCutoffFreq::D,
            _ =>  unreachable!(),
        };
        CtrlReg7XL {
            high_res: reg & HIGH_RES != 0,
            filtered_data_sel: reg & FDS != 0,
            high_pass_int_sens: reg & HPIS != 0,
            xl_digital_cf: xl_digital_cf,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.xl_digital_cf {
            DigCutoffFreq::A => DCF_A,
            DigCutoffFreq::B => DCF_B,
            DigCutoffFreq::C => DCF_C,
            DigCutoffFreq::D => DCF_D,
        };
        if self.high_res {reg |= HIGH_RES;}
        if self.filtered_data_sel {reg |= FDS;}
        if self.high_pass_int_sens {reg |= HPIS;}
        reg
    }
}

impl CtrlReg7XL {
    pub fn set_xl_digital_cf(&mut self, value: DigCutoffFreq) {
        self.xl_digital_cf = value
    }

    pub fn xl_digital_cf(&self) -> DigCutoffFreq {
        self.xl_digital_cf
    }

    pub fn set_high_res(&mut self, value: bool) {self.high_res = value}
    pub fn high_res(&self) -> bool {self.high_res}
    pub fn set_filtered_data_sel(&mut self, value: bool) {self.filtered_data_sel = value}
    pub fn filtered_data_sel(&self) -> bool {self.filtered_data_sel}
    pub fn set_high_pass_int_sens(&mut self, value: bool) {self.high_pass_int_sens = value}
    pub fn high_pass_int_sens(&self) -> bool {self.high_pass_int_sens}

}

#[cfg(test)]
mod tests {
    use super::CtrlReg7XL;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = !0b000_11_0_1_0;
        let r = CtrlReg7XL::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
