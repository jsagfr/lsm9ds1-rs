use super::{Register, Param};

const DCRM_G:               u16 = 0b1000_0000_0000_0000;
const INT_GEN_THS_X_G_MASK: u16 = 0b0111_1111_1111_1111;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg = 0x0000;
    for &param in params {
        match param {
            Param::IntGenThsXG(x) if x & !INT_GEN_THS_X_G_MASK != 0 => reg = x,
            Param::DcrmG(x) =>  reg = if x {
                reg |  DCRM_G
            } else {
                reg & !DCRM_G
            },
            _ => return Err(()),
        }
    }
    Ok(Register::IntGenThsXG(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::IntGenThsXG(r) => {
            let int_gen_ths_x_g = Param::IntGenThsXG(r);
            let dcrm_g = Param::DcrmG(r & DCRM_G == DCRM_G);
            Ok(vec![int_gen_ths_x_g, dcrm_g])
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
        let r1 = Register::IntGenThsXG(0xF21A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
