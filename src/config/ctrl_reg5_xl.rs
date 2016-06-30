use super::{Register, Param, Dec};

const ERRORS_MASK: u8 = 0b00_000_111;
const ZEN_XL:      u8 = 0b00_100_000;
const YEN_XL:      u8 = 0b00_010_000;
const XEN_XL:      u8 = 0b00_001_000;
const DEC_MASK:    u8 = 0b11_000_000;
const NO_DEC:      u8 = 0b00_000_000;
const DEC_2S:      u8 = 0b01_000_000;
const DEC_4S:      u8 = 0b10_000_000;
const DEC_8S:      u8 = 0b11_000_000;


pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::ZenXl(x) => reg = if x {
                        reg |  ZEN_XL
                    } else {
                        reg & !ZEN_XL
                    },
            Param::YenXl(x) => reg = if x {
                        reg |  YEN_XL
                    } else {
                        reg & !YEN_XL
                    },
            Param::XenXl(x) => reg = if x {
                        reg |  XEN_XL
                    } else {
                        reg & !XEN_XL
                    },
            Param::Dec(x) => {
                reg = reg & !DEC_MASK | match x {
                    Dec::NoDec => NO_DEC,
                    Dec::Dec2S => DEC_2S,
                    Dec::Dec4S => DEC_4S,
                    Dec::Dec8S => DEC_8S,
                }
            }
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg5Xl(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    let mut params = vec![];
    match reg {
        Register::CtrlReg5Xl(r) => {
            if r & ERRORS_MASK != 0 {
                return Err(())
            };
            params.push(Param::Dec(match r & DEC_MASK {
                    NO_DEC => Dec::NoDec,
                    DEC_2S => Dec::Dec2S,
                    DEC_4S => Dec::Dec4S,
                    DEC_8S => Dec::Dec8S,
                _ =>  unreachable!(),
            }));
            params.push(Param::XenXl(r & XEN_XL == XEN_XL));
            params.push(Param::YenXl(r & YEN_XL == YEN_XL));
            params.push(Param::ZenXl(r & ZEN_XL == ZEN_XL));
            Ok(params)
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
        let r1 = Register::CtrlReg5Xl(0b10111000);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
