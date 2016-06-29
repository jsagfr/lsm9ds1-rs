const INT2_INACT:     u8 = 0b10000000;
const ERROR1:         u8 = 0b01000000;
const INT2_FSS5:      u8 = 0b00100000;
const INT2_OVR:       u8 = 0b00010000;
const INT2_FTH:       u8 = 0b00001000;
const INT2_DRDY_TEMP: u8 = 0b00000100;
const INT2_DRDY_G:    u8 = 0b00000010;
const INT2_DRDY_XL:   u8 = 0b00000001;


reg_is_bools!{
    Int2Ctrl => {
        Int2Inact : INT2_INACT,
        Int2Fss5 : INT2_FSS5,
        Int2Ovr : INT2_OVR,
        Int2Fth : INT2_FTH,
        Int2DrdyTemp : INT2_DRDY_TEMP,
        Int2DrdyG : INT2_DRDY_G,
        Int2DrdyXl : INT2_DRDY_XL,
    }
    bits_errors {
        ERROR1,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::Int2Ctrl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
