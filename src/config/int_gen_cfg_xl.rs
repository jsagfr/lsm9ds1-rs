use super::{Register, Param};

const AOI_XL:    u8 = 0b10000000;
const DETECT_6D: u8 = 0b01000000;
const ZHIE_XL:   u8 = 0b00100000;
const ZLIE_XL:   u8 = 0b00010000;
const YHIE_XL:   u8 = 0b00001000;
const YLIE_XL:   u8 = 0b00000100;
const XHIE_XL:   u8 = 0b00000010;
const XLIE_XL:   u8 = 0b00000001;


pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg = 0x00;
    for &param in params {
        match param {
            Param::AoiXl(x) => reg = if x {
                reg | AOI_XL
            } else {
                reg & !AOI_XL
            },
            Param::Detect6D(x) => reg = if x {
                reg | DETECT_6D
            } else {
                reg & !DETECT_6D
            },
            Param::ZhieXl(x) => reg = if x {
                reg | ZHIE_XL
            } else {
                reg & !ZHIE_XL
            },
            Param::ZlieXl(x) => reg = if x {
                reg | ZLIE_XL
            } else {
                reg & !ZLIE_XL
            },
            Param::YhieXl(x) => reg = if x {
                reg | YHIE_XL
            } else {
                reg & !YHIE_XL
            },
            Param::YlieXl(x) => reg = if x {
                reg | YLIE_XL
            } else {
                reg & !YLIE_XL
            },
            Param::XhieXl(x) => reg = if x {
                reg | XHIE_XL
            } else {
                reg & !XHIE_XL
            },
            Param::XlieXl(x) => reg = if x {
                reg | XLIE_XL
            } else {
                reg & !XLIE_XL
            },
            _ => return Err(()),
        }
    }
    Ok(Register::IntGenCfgXl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    let mut params = vec![];
    match reg {
        Register::IntGenCfgXl(r) => {
            params.push(Param::AoiXl(r & AOI_XL == AOI_XL));
            params.push(Param::Detect6D(r & DETECT_6D == DETECT_6D));
            params.push(Param::ZhieXl(r & ZHIE_XL == ZHIE_XL));
            params.push(Param::ZlieXl(r & ZLIE_XL == ZLIE_XL));
            params.push(Param::YhieXl(r & YHIE_XL == YHIE_XL));
            params.push(Param::YlieXl(r & YLIE_XL == YLIE_XL));
            params.push(Param::XhieXl(r & XHIE_XL == XHIE_XL));
            params.push(Param::XlieXl(r & XLIE_XL == XLIE_XL));
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
        let r1 = Register::IntGenCfgXl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
