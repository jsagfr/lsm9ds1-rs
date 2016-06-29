const ERROR1: u8 = 0b10000000;
const ERROR2: u8 = 0b01000000;
const ERROR3: u8 = 0b00100000;
const ERROR4: u8 = 0b00010000;
const ERROR5: u8 = 0b00001000;
const ST_G:   u8 = 0b00000100;
const ERROR6: u8 = 0b00000010;
const ST_XL:  u8 = 0b00000001;


reg_is_bools!{
    CtrlReg10 => {
        StG : ST_G,
        StXl : ST_XL,
    }
    bits_errors {
        ERROR1,
        ERROR2,
        ERROR3,
        ERROR4,
        ERROR5,
        ERROR6,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg10(0b0000_0101);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
