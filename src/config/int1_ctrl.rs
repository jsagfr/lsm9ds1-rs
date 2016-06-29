const INT1_IG_G:    u8 = 0b10000000;
const INT1_IG_XL:   u8 = 0b01000000;
const INT1_FSS5:    u8 = 0b00100000;
const INT1_OVR:     u8 = 0b00010000;
const INT1_FTH:     u8 = 0b00001000;
const INT1_BOOT:    u8 = 0b00000100;
const INT1_DRDY_G:  u8 = 0b00000010;
const INT1_DRDY_XL: u8 = 0b00000001;

reg_is_bools!{
    Int1Ctrl => {
        Int1IgG : INT1_IG_G,
        Int1IgXl : INT1_IG_XL,
        Int1Fss5 : INT1_FSS5,
        Int1Ovr : INT1_OVR,
        Int1Fth : INT1_FTH,
        Int1Boot : INT1_BOOT,
        Int1DrdyG : INT1_DRDY_G,
        Int1DrdyXl : INT1_DRDY_XL,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::Int1Ctrl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
