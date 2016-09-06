use super::super::Address;
use super::{Register, CTRL_REG9};

const ERROR1:        u8 = 0b10000000;
const SLEEP_G:       u8 = 0b01000000;
const ERROR2:        u8 = 0b00100000;
const FIFO_TEMP_EN:  u8 = 0b00010000;
const DRDY_MASK_BIT: u8 = 0b00001000;
const I2C_DISABLE:   u8 = 0b00000100;
const FIFO_EN:       u8 = 0b00000010;
const STOP_ON_FTH:   u8 = 0b00000001;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg9 {
    sleep_g: bool,
    fifo_temp_en: bool,
    drdy_mask_bit: bool,
    i2c_disable: bool,
    fifo_en: bool,
    stop_on_fth: bool,
}

impl Register<u8> for CtrlReg9 {
    fn addr(&self) -> Address {
        CTRL_REG9
    }
    
    fn default() -> Self {
        CtrlReg9 {
            sleep_g: false,
            fifo_temp_en: false,
            drdy_mask_bit: false,
            i2c_disable: false,
            fifo_en: false,
            stop_on_fth: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg9 {
            sleep_g: reg & SLEEP_G != 0,
            fifo_temp_en: reg & FIFO_TEMP_EN != 0,
            drdy_mask_bit: reg & DRDY_MASK_BIT != 0,
            i2c_disable: reg & I2C_DISABLE != 0,
            fifo_en: reg & FIFO_EN != 0,
            stop_on_fth: reg & STOP_ON_FTH != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.sleep_g {SLEEP_G} else {0};
        if self.fifo_temp_en {reg |= FIFO_TEMP_EN;}
        if self.drdy_mask_bit {reg |= DRDY_MASK_BIT;}
        if self.i2c_disable {reg |= I2C_DISABLE;}
        if self.fifo_en {reg |= FIFO_EN;}
        if self.stop_on_fth {reg |= STOP_ON_FTH;}
        reg
    }
}

impl CtrlReg9 {
    pub fn set_sleep_g(&mut self, value: bool) {self.sleep_g = value}
    pub fn sleep_g(&self) -> bool {self.sleep_g}
    pub fn set_fifo_temp_en(&mut self, value: bool) {self.fifo_temp_en = value}
    pub fn fifo_temp_en(&self) -> bool {self.fifo_temp_en}
    pub fn set_drdy_mask_bit(&mut self, value: bool) {self.drdy_mask_bit = value}
    pub fn drdy_mask_bit(&self) -> bool {self.drdy_mask_bit}
    pub fn set_i2c_disable(&mut self, value: bool) {self.i2c_disable = value}
    pub fn i2c_disable(&self) -> bool {self.i2c_disable}
    pub fn set_fifo_en(&mut self, value: bool) {self.fifo_en = value}
    pub fn fifo_en(&self) -> bool {self.fifo_en}
    pub fn set_stop_on_fth(&mut self, value: bool) {self.stop_on_fth = value}
    pub fn stop_on_fth(&self) -> bool {self.stop_on_fth}
}


#[cfg(test)]
mod tests {
    use super::CtrlReg9;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = CtrlReg9::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
