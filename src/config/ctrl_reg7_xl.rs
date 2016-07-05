use super::{Register, Param, DigCutoffFreq};

const ERRORS:   u8 = 0b000_11_0_1_0;
const HR:       u8 = 0b100_00_0_0_0;
const DCF_MASK: u8 = 0b011_00_0_0_0;
const DCF_A:    u8 = 0b000_00_0_0_0;
const DCF_B:    u8 = 0b001_00_0_0_0;
const DCF_C:    u8 = 0b010_00_0_0_0;
const DCF_D:    u8 = 0b011_00_0_0_0;
const FDS:      u8 = 0b000_00_1_0_0;
const HPIS:     u8 = 0b000_00_0_0_1;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::HighRes(x) =>  reg = if x {
                reg |  HR
            } else {
                reg & !HR
            },
            Param::XlDigitalCf(x) => {
                reg = reg & !DCF_MASK | match x {
                    DigCutoffFreq::A => DCF_A,
                    DigCutoffFreq::B => DCF_B,
                    DigCutoffFreq::C => DCF_C,
                    DigCutoffFreq::D => DCF_D,
                }
            }
            Param::FilteredDataSel(x) =>  reg = if x {
                reg |  FDS
            } else {
                reg & !FDS
            },
            Param::HighPassIntSens(x) =>  reg = if x {
                reg |  HPIS
            } else {
                reg & !HPIS
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg7Xl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg7Xl(r) => {
            let hr = Param::HighRes(r & HR == HR);
            let dcf = Param::XlDigitalCf(match r & DCF_MASK {
                DCF_A => DigCutoffFreq::A,
                DCF_B => DigCutoffFreq::B,
                DCF_C => DigCutoffFreq::C,
                DCF_D => DigCutoffFreq::D,
                _ =>  unreachable!(),
            });
            let fds = Param::FilteredDataSel(r & FDS == FDS);
            let hpis = Param::HighPassIntSens(r & HPIS == HPIS);
            Ok(vec![hr, dcf, fds, hpis])
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
        let r1 = Register::CtrlReg7Xl(!0b000_11_0_1_0);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
