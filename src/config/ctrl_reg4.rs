const ERROR1:   u8 = 0b10000000;
const ERROR2:   u8 = 0b01000000;
const ZEN_G:    u8 = 0b00100000;
const YEN_G:    u8 = 0b00010000;
const XEN_G:    u8 = 0b00001000;
const ERROR3:   u8 = 0b00000100;
const LIR_XL1:  u8 = 0b00000010;
const I_4D_XL1: u8 = 0b00000001;


reg_is_bools!{
    CtrlReg4 => {
        ZenG : ZEN_G,
        YenG : YEN_G,
        XenG : XEN_G,
        LirXl1 : LIR_XL1,
        I4dXl1 : I_4D_XL1,
    }
    bits_errors {
        ERROR1,
        ERROR2,
        ERROR3,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg4(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
