use super::{Register, Param};

const ERRORS: u8 = 0b1_0_111111;
const BDU:    u8 = 0b0_1_000000;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::BlockDataUpdate(x) =>  reg = if x {
                reg |  BDU
            } else {
                reg & !BDU
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg5M(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg5M(r) => {
            if r & ERRORS != 0 {
                return Err(())
            };
            let bdu = Param::BlockDataUpdate(r & BDU == BDU);
            Ok(vec![bdu])
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
        let r1 = Register::CtrlReg5M(0b0_1_000000);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
