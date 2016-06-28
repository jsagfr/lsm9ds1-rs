use super::{Register, Param};

const WAIT_XL_MASK: u8 = 0b10000000;
const DUR_XL_MASK:  u8 = 0b01111111;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::DurXl(x) => {
                // Check value correctness ("u7")
                match x & !DUR_XL_MASK {
                    0 =>  reg |= x,
                    _ => return Err(()),
                }
            }
            Param::WaitXl(x) => reg = if x {
                reg |  WAIT_XL_MASK
            } else {
                reg & !WAIT_XL_MASK
            },
            _ => return Err(()),
        }
    }
    Ok(Register::IntGenDurXl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::IntGenDurXl(r) => {
            let dur_xl = Param::WaitXl(r & WAIT_XL_MASK == WAIT_XL_MASK);
            let wait_xl = Param::DurXl(r & DUR_XL_MASK);
            Ok(vec![dur_xl, wait_xl])
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
        let r1 = Register::IntGenDurXl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
