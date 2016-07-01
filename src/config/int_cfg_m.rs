const XIEN:   u8 = 0b10000000;
const YIEN:   u8 = 0b01000000;
const ZIEN:   u8 = 0b00100000;
const ERROR1: u8 = 0b00010000;
const ERROR2: u8 = 0b00001000;
const IEA:    u8 = 0b00000100;
const IEL:    u8 = 0b00000010;
const IEN:    u8 = 0b00000001;


reg_is_bools!{
    IntCfgM => {
        Xien : XIEN,
        Yien : YIEN,
        Zien : ZIEN,
        Iea : IEA,
        Iel : IEL,
        Ien : IEN,
    }
    bits_errors {
        ERROR1,
        ERROR2,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::IntCfgM(0b111_00_111);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
