const AOI_XL:    u8 = 0b10000000;
const DETECT_6D: u8 = 0b01000000;
const ZHIE_XL:   u8 = 0b00100000;
const ZLIE_XL:   u8 = 0b00010000;
const YHIE_XL:   u8 = 0b00001000;
const YLIE_XL:   u8 = 0b00000100;
const XHIE_XL:   u8 = 0b00000010;
const XLIE_XL:   u8 = 0b00000001;


reg_is_bools!{
    IntGenCfgXl => {
        AoiXl : AOI_XL,
        Detect6D : DETECT_6D,
        ZhieXl : ZHIE_XL,
        ZlieXl : ZLIE_XL,
        YhieXl : YHIE_XL,
        YlieXl : YLIE_XL,
        XhieXl : XHIE_XL,
        XlieXl : XLIE_XL,
    }
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
