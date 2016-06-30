use super::{Register, Param};

const ERRORS_MASK: u8 = 0b00_11_0000;
const LP_MODE:     u8 = 0b10_00_0000;
const HP_EN:       u8 = 0b01_00_0000;
const HPCF_G_MASK: u8 = 0b00_00_1111;


/// HPCF_G to be investigated
pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::LPMode(x) => reg = if x {
                        reg |  LP_MODE
                    } else {
                        reg & !LP_MODE
                    },
            Param::HpEn(x) => reg = if x {
                        reg |  HP_EN
                    } else {
                        reg & !HP_EN
                    },
            Param::HpcfG(x) => {
                if x & !HPCF_G_MASK != 0 {
                    return Err(())
                };
                reg = reg & !HPCF_G_MASK | x;
            }
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg3G(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    let mut params = vec![];
    match reg {
        Register::CtrlReg3G(r) => {
            if r & ERRORS_MASK != 0 {
                return Err(())
            };
            params.push(Param::LPMode(r & LP_MODE == LP_MODE));
            params.push(Param::HpEn(r & HP_EN == HP_EN));
            params.push(Param::HpcfG(r & HPCF_G_MASK));
        }
        _ => return Err(()),
    }
    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg3G(0b1100_1010);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
