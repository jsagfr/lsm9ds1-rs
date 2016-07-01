use super::{Register, Param, FMode};

const FMODE_MASK:          u8 = 0b111_00000;
const FMODE_BY_PASS:       u8 = 0b000_00000;
const FMODE_STOP_IF_FULL:  u8 = 0b001_00000;
const FMODE_CONTINUS_TRIG: u8 = 0b011_00000;
const FMODE_BY_PASS_TRIG:  u8 = 0b100_00000;
const FMODE_OVERWRITE:     u8 = 0b110_00000;
const FIFO_TH_MASK:        u8 = 0b000_11111;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::Fth(x) => {
                // Check value correctness ("u5")
                match x & !FIFO_TH_MASK {
                    0 =>  reg |= x,
                    _ => return Err(()),
                }
            }
            Param::FMode(x) => {
                reg = reg & !FMODE_MASK | match x {
                    FMode::ByPass => FMODE_BY_PASS,
                    FMode::StopIfFull => FMODE_STOP_IF_FULL,
                    FMode::ContinusTrig => FMODE_CONTINUS_TRIG,
                    FMode::ByPassTrig => FMODE_BY_PASS_TRIG,
                    FMode::Overwrite => FMODE_OVERWRITE,
                }
            }
            _ => return Err(()),
        }
    }
    Ok(Register::FifoCtrl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::FifoCtrl(r) => {
            let fmode = Param::FMode(match r & FMODE_MASK {
                FMODE_BY_PASS => FMode::ByPass,
                FMODE_STOP_IF_FULL => FMode::StopIfFull,
                FMODE_CONTINUS_TRIG => FMode::ContinusTrig,
                FMODE_BY_PASS_TRIG => FMode::ByPassTrig,
                FMODE_OVERWRITE => FMode::Overwrite,
                _ => unreachable!(),
            });
            let fth = Param::Fth(r & FIFO_TH_MASK);
            Ok(vec![fmode, fth])
        }
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::FifoCtrl(0b110_10010);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
