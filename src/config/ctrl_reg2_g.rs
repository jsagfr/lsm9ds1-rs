use super::{Register, Param, IntSel, OutSel};

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


pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::IntSel(x) => {
                reg |= match x {
                    IntSel::A => INT_SEL_A,
                    IntSel::B => INT_SEL_B,
                    IntSel::C => INT_SEL_C,
                    IntSel::D => INT_SEL_D,
                }
            }
            Param::OutSel(x) => {
                reg |= match x {
                    OutSel::A => OUT_SEL_A,
                    OutSel::B => OUT_SEL_B,
                    OutSel::C => OUT_SEL_C,
                    OutSel::D => OUT_SEL_D,
                }
            }
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg2G(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg2G(r) => {
            if r & ERRORS_MASK != 0 {
                return Err(())
            };
            let int_sel = Param::IntSel(match r & INT_SEL_MASK {
                INT_SEL_A => IntSel::A,
                INT_SEL_B => IntSel::B,
                INT_SEL_C => IntSel::C,
                INT_SEL_D => IntSel::D,
                _ =>  unreachable!(),
            });
            let out_sel = Param::OutSel(match r & OUT_SEL_MASK {
                OUT_SEL_A => OutSel::A,
                OUT_SEL_B => OutSel::B,
                OUT_SEL_C => OutSel::C,
                OUT_SEL_D => OutSel::D,
                _ =>  unreachable!(),
            });
            Ok(vec![int_sel, out_sel])
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
        let r1 = Register::CtrlReg2G(0x0A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
