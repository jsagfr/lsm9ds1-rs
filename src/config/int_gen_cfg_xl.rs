use super::{Register, Param, State, AndOrState};

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
            Param::AoiXl(x) => match x {
                AndOrState::And => reg |=  AOI_XL,
                AndOrState::Or  => reg &= !AOI_XL,
            },
            Param::Detect6D(x) => match x {
                State::Enable  => reg |=  DETECT_6D,
                State::Disable => reg &= !DETECT_6D,
            },
            Param::ZhieXl(x) => match x {
                State::Enable  => reg |=  ZHIE_XL,
                State::Disable => reg &= !ZHIE_XL,
            },
            Param::ZlieXl(x) => match x {
                State::Enable  => reg |=  ZLIE_XL,
                State::Disable => reg &= !ZLIE_XL,
            },
            Param::YhieXl(x) => match x {
                State::Enable  => reg |=  YHIE_XL,
                State::Disable => reg &= !YHIE_XL,
            },
            Param::YlieXl(x) => match x {
                State::Enable  => reg |=  YLIE_XL,
                State::Disable => reg &= !YLIE_XL,
            },
            Param::XhieXl(x) => match x {
                State::Enable  => reg |=  XHIE_XL,
                State::Disable => reg &= !XHIE_XL,
            },
            Param::XlieXl(x) => match x {
                State::Enable  => reg |=  XLIE_XL,
                State::Disable => reg &= !XLIE_XL,
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
            params.push(Param::AoiXl(
                match r & AOI_XL {
                    AOI_XL => AndOrState::And,
                    0      => AndOrState::Or,
                    _      => unreachable!(),
                }));
            params.push(Param::Detect6D(
                match r & DETECT_6D {
                    DETECT_6D => State::Enable,
                    0         => State::Disable,
                    _         => unreachable!(),
                }));
            params.push(Param::ZhieXl(
                match r & ZHIE_XL {
                    ZHIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
            params.push(Param::ZlieXl(
                match r & ZLIE_XL {
                    ZLIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
            params.push(Param::YhieXl(
                match r & YHIE_XL {
                    YHIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
            params.push(Param::YlieXl(
                match r & YLIE_XL {
                    YLIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
            params.push(Param::XhieXl(
                match r & XHIE_XL {
                    XHIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
            params.push(Param::XlieXl(
                match r & XLIE_XL {
                    XLIE_XL => State::Enable,
                    0       => State::Disable,
                    _       => unreachable!(),
                }));
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
