/// ACT_THS register is the Activity threshold register.

pub const REG_ADDR:  u8 = 0x06; // TODO: #![feature(associated_consts)]

bitflags! {
    flags IntGenCfgXL: u8 {
        const AOI_XL    = 0b10000000,
        const DETECT_6D = 0b01000000,
        const ZHIE_XL   = 0b00100000,
        const ZLIE_XL   = 0b00010000,
        const YHIE_XL   = 0b00001000,
        const YLIE_XL   = 0b00000100,
        const XHIE_XL   = 0b00000010,
        const XLIE_XL   = 0b00000001,
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interrupt {
    Enable,
    Disable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntGenCfgXLReg {
    aoi_xl:    Interrupt,
    detect_6d: Interrupt,
    zhie_xl:   Interrupt,
    zlie_xl:   Interrupt,
    yhie_xl:   Interrupt,
    ylie_xl:   Interrupt,
    xhie_xl:   Interrupt,
    xlie_xl:   Interrupt,
    bits: IntGenCfgXL,
}

impl IntGenCfgXLReg {

    fn new(reg: IntGenCfgXL) -> Self {
        ActThsReg {
            bits: reg,
        }
    }

    fn from_reg(reg: u8) -> Self {
        ActThsReg {
            bits: IntGenCfgXL::from_bits(reg),
        }
    }

    fn to_reg(&self) -> u8 {
        self.bits.bits()
    }

    fn reg_addr(&self) -> u8 {
        REG_ADDR
    }
}

#[cfg(test)]
mod tests {
    use super::{IntGenCfgXL, REG_ADDR};
    #[test]
    fn it_works() {
        let r1 = IntGenCfgXL::new(IntGenCfgXL);
        let r2 = ActDurReg::from_reg(r1.to_reg());
    }
}
