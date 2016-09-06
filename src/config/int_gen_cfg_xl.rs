use super::{Register, INT_GEN_CFG_XL};

const AOI_XL:    u8 = 0b10000000;
const DETECT_6D: u8 = 0b01000000;
const ZHIE_XL:   u8 = 0b00100000;
const ZLIE_XL:   u8 = 0b00010000;
const YHIE_XL:   u8 = 0b00001000;
const YLIE_XL:   u8 = 0b00000100;
const XHIE_XL:   u8 = 0b00000010;
const XLIE_XL:   u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenCfgXl {
    aoi_xl: bool,
    detect_6d: bool,
    zhie_xl: bool,
    zlie_xl: bool,
    yhie_xl: bool,
    ylie_xl: bool,
    xhie_xl: bool,
    xlie_xl: bool,
}

impl Register<u8> for IntGenCfgXl {
    fn addr(&self) -> u8 {
        INT_GEN_CFG_XL
    }
    
    fn default() -> Self {
        IntGenCfgXl {
            aoi_xl: false,
            detect_6d: false,
            zhie_xl: false,
            zlie_xl: false,
            yhie_xl: false,
            ylie_xl: false,
            xhie_xl: false,
            xlie_xl: false,
        }
    }

    fn new(reg: u8) -> Self {
        IntGenCfgXl {
            aoi_xl: reg & AOI_XL != 0,
            detect_6d: reg & DETECT_6D != 0,
            zhie_xl: reg & ZHIE_XL != 0,
            zlie_xl: reg & ZLIE_XL != 0,
            yhie_xl: reg & YHIE_XL != 0,
            ylie_xl: reg & YLIE_XL != 0,
            xhie_xl: reg & XHIE_XL != 0,
            xlie_xl: reg & XLIE_XL != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.aoi_xl {AOI_XL} else {0};
        if self.detect_6d {reg |= DETECT_6D;}
        if self.zhie_xl {reg |= ZHIE_XL;}
        if self.zlie_xl {reg |= ZLIE_XL;}
        if self.yhie_xl {reg |= YHIE_XL;}
        if self.ylie_xl {reg |= YLIE_XL;}
        if self.xhie_xl {reg |= XHIE_XL;}
        if self.xlie_xl {reg |= XLIE_XL;}
        reg
    }
}

impl IntGenCfgXl {
    pub fn set_aoi_xl(&mut self, value: bool) {self.aoi_xl = value}
    pub fn aoi_xl(&self) -> bool {self.aoi_xl}
    pub fn set_detect_6d(&mut self, value: bool) {self.detect_6d = value}
    pub fn detect_6d(&self) -> bool {self.detect_6d}
    pub fn set_zhie_xl(&mut self, value: bool) {self.zhie_xl = value}
    pub fn zhie_xl(&self) -> bool {self.zhie_xl}
    pub fn set_zlie_xl(&mut self, value: bool) {self.zlie_xl = value}
    pub fn zlie_xl(&self) -> bool {self.zlie_xl}
    pub fn set_yhie_xl(&mut self, value: bool) {self.yhie_xl = value}
    pub fn yhie_xl(&self) -> bool {self.yhie_xl}
    pub fn set_ylie_xl(&mut self, value: bool) {self.ylie_xl = value}
    pub fn ylie_xl(&self) -> bool {self.ylie_xl}
    pub fn set_xhie_xl(&mut self, value: bool) {self.xhie_xl = value}
    pub fn xhie_xl(&self) -> bool {self.xhie_xl}
    pub fn set_xlie_xl(&mut self, value: bool) {self.xlie_xl = value}
    pub fn xlie_xl(&self) -> bool {self.xlie_xl}
}


#[cfg(test)]
mod tests {
    use super::IntGenCfgXl;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenCfgXl::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
