const AOI_G:  u8 = 0b10000000;
const LIR_G:  u8 = 0b01000000;
const ZHIE_G: u8 = 0b00100000;
const ZLIE_G: u8 = 0b00010000;
const YHIE_G: u8 = 0b00001000;
const YLIE_G: u8 = 0b00000100;
const XHIE_G: u8 = 0b00000010;
const XLIE_G: u8 = 0b00000001;


reg_is_bools!{
    IntGenCfgG => {
        AoiG : AOI_G,
        LirG : LIR_G,
        ZhieG : ZHIE_G,
        ZlieG : ZLIE_G,
        YhieG : YHIE_G,
        YlieG : YLIE_G,
        XhieG : XHIE_G,
        XlieG : XLIE_G,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::IntGenCfgG(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
