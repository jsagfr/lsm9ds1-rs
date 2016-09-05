use super::{Register, INT_GEN_CFG_G};

const AOI_G:  u8 = 0b10000000;
const LIR_G:  u8 = 0b01000000;
const ZHIE_G: u8 = 0b00100000;
const ZLIE_G: u8 = 0b00010000;
const YHIE_G: u8 = 0b00001000;
const YLIE_G: u8 = 0b00000100;
const XHIE_G: u8 = 0b00000010;
const XLIE_G: u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct IntGenCfgG {
    aoi_g: bool,
    lir_g: bool,
    zhie_g: bool,
    zlie_g: bool,
    yhie_g: bool,
    ylie_g: bool,
    xhie_g: bool,
    xlie_g: bool,
}

impl Register<u8> for IntGenCfgG {
    fn addr(&self) -> u8 {
        INT_GEN_CFG_G
    }
    
    fn default() -> Self {
        IntGenCfgG {
            aoi_g: false,
            lir_g: false,
            zhie_g: false,
            zlie_g: false,
            yhie_g: false,
            ylie_g: false,
            xhie_g: false,
            xlie_g: false,
        }
    }

    fn new(reg: u8) -> Self {
        IntGenCfgG {
            aoi_g: reg & AOI_G != 0,
            lir_g: reg & LIR_G != 0,
            zhie_g: reg & ZHIE_G != 0,
            zlie_g: reg & ZLIE_G != 0,
            yhie_g: reg & YHIE_G != 0,
            ylie_g: reg & YLIE_G != 0,
            xhie_g: reg & XHIE_G != 0,
            xlie_g: reg & XLIE_G != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.aoi_g {AOI_G} else {0};
        if self.lir_g {reg |= LIR_G;}
        if self.zhie_g {reg |= ZHIE_G;}
        if self.zlie_g {reg |= ZLIE_G;}
        if self.yhie_g {reg |= YHIE_G;}
        if self.ylie_g {reg |= YLIE_G;}
        if self.xhie_g {reg |= XHIE_G;}
        if self.xlie_g {reg |= XLIE_G;}
        reg
    }
}

impl IntGenCfgG {
    pub fn set_aoi_g(&mut self, value: bool) {self.aoi_g = value}
    pub fn aoi_g(&self) -> bool {self.aoi_g}
    pub fn set_lir_g(&mut self, value: bool) {self.lir_g = value}
    pub fn lir_g(&self) -> bool {self.lir_g}
    pub fn set_zhie_g(&mut self, value: bool) {self.zhie_g = value}
    pub fn zhie_g(&self) -> bool {self.zhie_g}
    pub fn set_zlie_g(&mut self, value: bool) {self.zlie_g = value}
    pub fn zlie_g(&self) -> bool {self.zlie_g}
    pub fn set_yhie_g(&mut self, value: bool) {self.yhie_g = value}
    pub fn yhie_g(&self) -> bool {self.yhie_g}
    pub fn set_ylie_g(&mut self, value: bool) {self.ylie_g = value}
    pub fn ylie_g(&self) -> bool {self.ylie_g}
    pub fn set_xhie_g(&mut self, value: bool) {self.xhie_g = value}
    pub fn xhie_g(&self) -> bool {self.xhie_g}
    pub fn set_xlie_g(&mut self, value: bool) {self.xlie_g = value}
    pub fn xlie_g(&self) -> bool {self.xlie_g}
}


#[cfg(test)]
mod tests {
    use super::IntGenCfgG;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = IntGenCfgG::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
