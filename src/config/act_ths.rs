use super::{Register, Param};

const ACT_THS_MASK:  u8 = 0b0111_1111;
const SLEEP_ON_MASK: u8 = 0b1000_0000;

/// From params return a register with defaut values or values
/// given in the param or error.
/// 
/// If all parameters are not supplies, defaut values are:
/// * `Param::ActThs(0)`
/// * `Param::SleepOn(State::Disable)`
pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::ActThs(x) => {
                // Check value correctness ("u7")
                match x & !ACT_THS_MASK {
                    0 =>  reg |= x,
                    _ => return Err(()),
                }
            }
            Param::SleepOn(x) => reg = if x {
                reg |  SLEEP_ON_MASK
            } else {
                reg & !SLEEP_ON_MASK
            },
            _ => return Err(()),
        }
    }
    Ok(Register::ActThs(reg))
}

/// Return a list of param for the *act ths* register.
/// 
/// # Errors
///
/// Return an error if the register is not a `Register::ActThs`.
pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::ActThs(r) => {
            let sleep_on = Param::SleepOn(r & SLEEP_ON_MASK == SLEEP_ON_MASK);
            let act_ths = Param::ActThs(r & ACT_THS_MASK);
            Ok(vec![sleep_on, act_ths])
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
        let r1 = Register::ActThs(0xF1);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
