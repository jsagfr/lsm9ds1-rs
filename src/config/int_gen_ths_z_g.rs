use super::{Register, Param};

const INT_GEN_THS_Z_G_MASK: u16 = 0b0111_1111_1111_1111;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg = 0x0000;
    for &param in params {
        match param {
            Param::IntGenThsZG(x) if x & !INT_GEN_THS_Z_G_MASK != 0 => reg = x,
            _ => return Err(()),
        }
    }
    Ok(Register::IntGenThsZG(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::IntGenThsZG(r) => {
            Ok(vec![Param::IntGenThsZG(r)])
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
        let r1 = Register::IntGenThsZG(0xF21A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
