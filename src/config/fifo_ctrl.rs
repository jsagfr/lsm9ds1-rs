use super::{Register, FIFO_CTRL, FMode};

const FMODE_MASK:          u8 = 0b111_00000;
const FMODE_BY_PASS:       u8 = 0b000_00000;
const FMODE_STOP_IF_FULL:  u8 = 0b001_00000;
const FMODE_CONTINUS_TRIG: u8 = 0b011_00000;
const FMODE_BY_PASS_TRIG:  u8 = 0b100_00000;
const FMODE_OVERWRITE:     u8 = 0b110_00000;
const FIFO_TH_MASK:        u8 = 0b000_11111;

#[derive(Clone, Debug, PartialEq)]
pub struct FifoCtrl {
    fth: u8,
    f_mode: FMode,
}

impl Register<u8> for FifoCtrl {
    fn addr(&self) -> u8 {
        FIFO_CTRL
    }
    
    fn default() -> Self {
        FifoCtrl {
            fth: 0,
            f_mode: FMode::default(),
        }
    }

    fn new(reg: u8) -> Self {
        let f_mode = match reg & FMODE_MASK {
            FMODE_BY_PASS => FMode::ByPass,
            FMODE_STOP_IF_FULL => FMode::StopIfFull,
            FMODE_CONTINUS_TRIG => FMode::ContinusTrig,
            FMODE_BY_PASS_TRIG => FMode::ByPassTrig,
            FMODE_OVERWRITE => FMode::Overwrite,
            _ => unreachable!(),
        };
        FifoCtrl {
            fth: reg & FIFO_TH_MASK,
            f_mode: f_mode,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = self.fth;
        reg |= match self.f_mode {
            FMode::ByPass => FMODE_BY_PASS,
            FMode::StopIfFull => FMODE_STOP_IF_FULL,
            FMode::ContinusTrig => FMODE_CONTINUS_TRIG,
            FMode::ByPassTrig => FMODE_BY_PASS_TRIG,
            FMode::Overwrite => FMODE_OVERWRITE,
        };
        reg
    }
}

impl FifoCtrl {
    pub fn set_fth(&mut self, value: u8) {
        assert!(value <= FIFO_TH_MASK);
        self.fth = value
    }

    pub fn fth(&self) -> u8 {
        self.fth
    }

    pub fn set_f_mode(&mut self, value: FMode) {
        self.f_mode = value
    }

    pub fn f_mode(&self) -> FMode {
        self.f_mode
    }
}

#[cfg(test)]
mod tests {
    use super::FifoCtrl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b110_10010;
        let r = FifoCtrl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
