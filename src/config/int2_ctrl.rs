use super::{Register, INT2_CTRL};

const INT2_INACT:     u8 = 0b10000000;
const ERROR1:         u8 = 0b01000000;
const INT2_FSS_5:     u8 = 0b00100000;
const INT2_OVR:       u8 = 0b00010000;
const INT2_FTH:       u8 = 0b00001000;
const INT2_DRDY_TEMP: u8 = 0b00000100;
const INT2_DRDY_G:    u8 = 0b00000010;
const INT2_DRDY_XL:   u8 = 0b00000001;


#[derive(Clone, Debug, PartialEq)]
pub struct Int2Ctrl {
    int2_inact: bool,
    int2_fss_5: bool,
    int2_ovr: bool,
    int2_fth: bool,
    int2_drdy_temp: bool,
    int2_drdy_g: bool,
    int2_drdy_xl: bool,
}

impl Register<u8> for Int2Ctrl {
    fn addr(&self) -> u8 {
        INT2_CTRL
    }
    
    fn default() -> Self {
        Int2Ctrl {
            int2_inact: false,
            int2_fss_5: false,
            int2_ovr: false,
            int2_fth: false,
            int2_drdy_temp: false,
            int2_drdy_g: false,
            int2_drdy_xl: false,
        }
    }

    fn new(reg: u8) -> Self {
        Int2Ctrl {
            int2_inact: reg & INT2_INACT != 0,
            int2_fss_5: reg & INT2_FSS_5 != 0,
            int2_ovr: reg & INT2_OVR != 0,
            int2_fth: reg & INT2_FTH != 0,
            int2_drdy_temp: reg & INT2_DRDY_TEMP != 0,
            int2_drdy_g: reg & INT2_DRDY_G != 0,
            int2_drdy_xl: reg & INT2_DRDY_XL != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.int2_inact {INT2_INACT} else {0};
        if self.int2_fss_5 {reg |= INT2_FSS_5;}
        if self.int2_ovr {reg |= INT2_OVR;}
        if self.int2_fth {reg |= INT2_FTH;}
        if self.int2_drdy_temp {reg |= INT2_DRDY_TEMP;}
        if self.int2_drdy_g {reg |= INT2_DRDY_G;}
        if self.int2_drdy_xl {reg |= INT2_DRDY_XL;}
        reg
    }
}

impl Int2Ctrl {
    pub fn set_int2_inact(&mut self, value: bool) {self.int2_inact = value}
    pub fn int2_inact(&self) -> bool {self.int2_inact}
    pub fn set_int2_fss_5(&mut self, value: bool) {self.int2_fss_5 = value}
    pub fn int2_fss_5(&self) -> bool {self.int2_fss_5}
    pub fn set_int2_ovr(&mut self, value: bool) {self.int2_ovr = value}
    pub fn int2_ovr(&self) -> bool {self.int2_ovr}
    pub fn set_int2_fth(&mut self, value: bool) {self.int2_fth = value}
    pub fn int2_fth(&self) -> bool {self.int2_fth}
    pub fn set_int2_drdy_temp(&mut self, value: bool) {self.int2_drdy_temp = value}
    pub fn int2_drdy_temp(&self) -> bool {self.int2_drdy_temp}
    pub fn set_int2_drdy_g(&mut self, value: bool) {self.int2_drdy_g = value}
    pub fn int2_drdy_g(&self) -> bool {self.int2_drdy_g}
    pub fn set_int2_drdy_xl(&mut self, value: bool) {self.int2_drdy_xl = value}
    pub fn int2_drdy_xl(&self) -> bool {self.int2_drdy_xl}
}

#[cfg(test)]
mod tests {
    use super::Int2Ctrl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = Int2Ctrl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
