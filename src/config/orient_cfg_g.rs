use super::{Register, Param};

const ERRORS_MASK: u8 = 0b11_000_000;
const SIGN_X_G:    u8 = 0b11_100_000;
const SIGN_Y_G:    u8 = 0b00_010_000;
const SIGN_Z_G:    u8 = 0b00_001_000;
const ORIENT_MASK: u8 = 0b00_000_111;

/// Orient is not clear
pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::SignXG(x) => reg = if x {
                        reg |  SIGN_X_G
                    } else {
                        reg & !SIGN_X_G
                    },
            Param::SignYG(x) => reg = if x {
                        reg |  SIGN_Y_G
                    } else {
                        reg & !SIGN_Y_G
                    },
            Param::SignZG(x) => reg = if x {
                        reg |  SIGN_Z_G
                    } else {
                        reg & !SIGN_Z_G
                    },
            Param::Orient(x) => {
                if x & !ORIENT_MASK != 0 {
                    return Err(())
                };
                reg = reg & !ORIENT_MASK | x;
            }
            _ => return Err(()),
        }
    }
    Ok(Register::OrientCfgG(reg))
}

/// Orient is not clear
pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    let mut params = vec![];
    match reg {
        Register::OrientCfgG(r) => {
            if r & ERRORS_MASK != 0 {
                return Err(())
            };
            params.push(Param::SignXG(r & SIGN_Z_G == SIGN_Z_G));
            params.push(Param::SignYG(r & SIGN_Z_G == SIGN_Y_G));
            params.push(Param::SignZG(r & SIGN_Z_G == SIGN_Z_G));
            params.push(Param::Orient(r & ORIENT_MASK));
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
        let r1 = Register::OrientCfgG(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
