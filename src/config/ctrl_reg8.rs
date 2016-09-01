use super::super::Address;
use super::{Register, CTRL_REG8};

const BOOT:       u8 = 0b10000000;
const BDU:        u8 = 0b01000000;
const HL_ACTIVE:  u8 = 0b00100000;
const PP_OD:      u8 = 0b00010000;
const SIM:        u8 = 0b00001000;
const IF_ADD_INC: u8 = 0b00000100;
const BLE:        u8 = 0b00000010;
const SW_RESET:   u8 = 0b00000001;


#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg8 {
    boot: bool,
    bdu: bool,
    hl_active: bool,
    pp_od: bool,
    sim: bool,
    if_add_inc: bool,
    ble: bool,
    sw_reset: bool,
}

impl Register<u8> for CtrlReg8 {
    fn addr(&self) -> Address {
        CTRL_REG8
    }
    
    fn default() -> Self {
        CtrlReg8 {
            boot: false,
            bdu: false,
            hl_active: false,
            pp_od: false,
            sim: false,
            if_add_inc: false,
            ble: false,
            sw_reset: false,
        }
    }

    fn new(reg: u8) -> Self {
        CtrlReg8 {
            boot: reg & BOOT != 0,
            bdu: reg & BDU != 0,
            hl_active: reg & HL_ACTIVE != 0,
            pp_od: reg & PP_OD != 0,
            sim: reg & SIM != 0,
            if_add_inc: reg & IF_ADD_INC != 0,
            ble: reg & BLE != 0,
            sw_reset: reg & SW_RESET != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = if self.boot {BOOT} else {0};
        if self.bdu {reg |= BDU;}
        if self.hl_active {reg |= HL_ACTIVE;}
        if self.pp_od {reg |= PP_OD;}
        if self.sim {reg |= SIM;}
        if self.if_add_inc {reg |= IF_ADD_INC;}
        if self.ble {reg |= BLE;}
        if self.sw_reset {reg |= SW_RESET;}
        reg
    }
}

impl CtrlReg8 {
    pub fn set_boot(&mut self, value: bool) {self.boot = value}
    pub fn boot(&self) -> bool {self.boot}
    pub fn set_bdu(&mut self, value: bool) {self.bdu = value}
    pub fn bdu(&self) -> bool {self.bdu}
    pub fn set_hl_active(&mut self, value: bool) {self.hl_active = value}
    pub fn hl_active(&self) -> bool {self.hl_active}
    pub fn set_pp_od(&mut self, value: bool) {self.pp_od = value}
    pub fn pp_od(&self) -> bool {self.pp_od}
    pub fn set_sim(&mut self, value: bool) {self.sim = value}
    pub fn sim(&self) -> bool {self.sim}
    pub fn set_if_add_inc(&mut self, value: bool) {self.if_add_inc = value}
    pub fn if_add_inc(&self) -> bool {self.if_add_inc}
    pub fn set_ble(&mut self, value: bool) {self.ble = value}
    pub fn ble(&self) -> bool {self.ble}
    pub fn set_sw_reset(&mut self, value: bool) {self.sw_reset = value}
    pub fn sw_reset(&self) -> bool {self.sw_reset}
}

#[cfg(test)]
mod tests {
    use super::CtrlReg8;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = CtrlReg8::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
