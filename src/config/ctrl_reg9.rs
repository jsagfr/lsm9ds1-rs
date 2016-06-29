const ERROR1:        u8 = 0b10000000;
const SLEEP_G:       u8 = 0b01000000;
const ERROR2:        u8 = 0b00100000;
const FIFO_TEMP_EN:  u8 = 0b00010000;
const DRDY_MASK_BIT: u8 = 0b00001000;
const I2C_DISABLE:   u8 = 0b00000100;
const FIFO_EN:       u8 = 0b00000010;
const STOP_ON_FTH:   u8 = 0b00000001;


reg_is_bools!{
    CtrlReg9 => {
        SleepG : SLEEP_G,
        FifoTempEn : FIFO_TEMP_EN,
        DrdyMaskBit : DRDY_MASK_BIT,
        I2cDisable : I2C_DISABLE,
        FifoEn : FIFO_EN,
        StopOnFth : STOP_ON_FTH,
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
        let r1 = Register::CtrlReg9(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
