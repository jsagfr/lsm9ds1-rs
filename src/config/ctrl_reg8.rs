const BOOT:       u8 = 0b10000000;
const BDU:        u8 = 0b01000000;
const H_LACTIVE:  u8 = 0b00100000;
const PP_OD:      u8 = 0b00010000;
const SIM:        u8 = 0b00001000;
const IF_ADD_INC: u8 = 0b00000100;
const BLE:        u8 = 0b00000010;
const SW_RESET:   u8 = 0b00000001;


reg_is_bools!{
    CtrlReg8 => {
        Boot : BOOT,
        Bdu : BDU,
        HLactive : H_LACTIVE,
        PpOd : PP_OD,
        Sim : SIM,
        IfAddInc : IF_ADD_INC,
        Ble : BLE,
        SwReset : SW_RESET,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg8(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
