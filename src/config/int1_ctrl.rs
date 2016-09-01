use super::super::Address;
use super::{Register, INT1_CTRL};

const INT1_IG_G:    u8 = 0b10000000;
const INT1_IG_XL:   u8 = 0b01000000;
const INT1_FSS_5:   u8 = 0b00100000;
const INT1_OVR:     u8 = 0b00010000;
const INT1_FTH:     u8 = 0b00001000;
const INT1_BOOT:    u8 = 0b00000100;
const INT1_DRDY_G:  u8 = 0b00000010;
const INT1_DRDY_XL: u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct Int1Ctrl {
    int1_ig_g: bool,
    int1_ig_xl: bool,
    int1_fss_5: bool,
    int1_ovr: bool,
    int1_fth: bool,
    int1_boot: bool,
    int1_drdy_g: bool,
    int1_drdy_xl: bool,
}

impl Register<u8> for Int1Ctrl {
    fn addr(&self) -> Address {
        INT1_CTRL
    }
    
    fn default() -> Self {
        Int1Ctrl {
            int1_ig_g: false,
            int1_ig_xl: false,
            int1_fss_5: false,
            int1_ovr: false,
            int1_fth: false,
            int1_boot: false,
            int1_drdy_g: false,
            int1_drdy_xl: false,
        }
    }

    fn new(reg: u8) -> Self {
        Int1Ctrl {
            int1_ig_g: reg & INT1_IG_G != 0,
            int1_ig_xl: reg & INT1_IG_XL != 0,
            int1_fss_5: reg & INT1_FSS_5 != 0,
            int1_ovr: reg & INT1_OVR != 0,
            int1_fth: reg & INT1_FTH != 0,
            int1_boot: reg & INT1_BOOT != 0,
            int1_drdy_g: reg & INT1_DRDY_G != 0,
            int1_drdy_xl: reg & INT1_DRDY_XL != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.int1_ig_g {INT1_IG_G} else {0};
        if self.int1_ig_xl {reg |= INT1_IG_XL;}
        if self.int1_fss_5 {reg |= INT1_FSS_5;}
        if self.int1_ovr {reg |= INT1_OVR;}
        if self.int1_fth {reg |= INT1_FTH;}
        if self.int1_boot {reg |= INT1_BOOT;}
        if self.int1_drdy_g {reg |= INT1_DRDY_G;}
        if self.int1_drdy_xl {reg |= INT1_DRDY_XL;}
        reg
    }
}

impl Int1Ctrl {
    pub fn set_int1_ig_g(&mut self, value: bool) {self.int1_ig_g = value}
    pub fn int1_ig_g(&self) -> bool {self.int1_ig_g}
    pub fn set_int1_ig_xl(&mut self, value: bool) {self.int1_ig_xl = value}
    pub fn int1_ig_xl(&self) -> bool {self.int1_ig_xl}
    pub fn set_int1_fss_5(&mut self, value: bool) {self.int1_fss_5 = value}
    pub fn int1_fss_5(&self) -> bool {self.int1_fss_5}
    pub fn set_int1_ovr(&mut self, value: bool) {self.int1_ovr = value}
    pub fn int1_ovr(&self) -> bool {self.int1_ovr}
    pub fn set_int1_fth(&mut self, value: bool) {self.int1_fth = value}
    pub fn int1_fth(&self) -> bool {self.int1_fth}
    pub fn set_int1_boot(&mut self, value: bool) {self.int1_boot = value}
    pub fn int1_boot(&self) -> bool {self.int1_boot}
    pub fn set_int1_drdy_g(&mut self, value: bool) {self.int1_drdy_g = value}
    pub fn int1_drdy_g(&self) -> bool {self.int1_drdy_g}
    pub fn set_int1_drdy_xl(&mut self, value: bool) {self.int1_drdy_xl = value}
    pub fn int1_drdy_xl(&self) -> bool {self.int1_drdy_xl}
}


#[cfg(test)]
mod tests {
    use super::Int1Ctrl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = Int1Ctrl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
