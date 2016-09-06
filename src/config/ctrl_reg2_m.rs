use super::{Register, CTRL_REG2_M, FsM};

const ERRORS:       u8 = 0b1_00_1_00_11;
const FS_MASK:      u8 = 0b0_11_0_00_00;
const FS_4_G:       u8 = 0b0_00_0_00_00;
const FS_8_G:       u8 = 0b0_01_0_00_00;
const FS_12_G:      u8 = 0b0_10_0_00_00;
const FS_16_G:      u8 = 0b0_11_0_00_00;
const REBOOT_M:     u8 = 0b0_00_0_10_00;
const SOFT_RESET_M: u8 = 0b0_00_0_01_00;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg2M {
    fs_m: FsM,
    reboot_m: bool,
    soft_reset_m: bool,
}

impl Register<u8> for CtrlReg2M {
    fn addr(&self) -> u8 {
        CTRL_REG2_M
    }
    
    fn default() -> Self {
        CtrlReg2M {
            fs_m: FsM::default(),
            reboot_m: false,
            soft_reset_m: false,
        }
    }

    fn new(reg: u8) -> Self {
        let fs_m = match reg & FS_MASK {
            FS_4_G  => FsM::Fs4,
            FS_8_G  => FsM::Fs8,
            FS_12_G => FsM::Fs12,
            FS_16_G => FsM::Fs16,
            _ => unreachable!(),
        };
        CtrlReg2M {
            fs_m: fs_m,
            reboot_m: reg & REBOOT_M != 0,
            soft_reset_m: reg & SOFT_RESET_M != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.fs_m {
            FsM::Fs4  => FS_4_G,
            FsM::Fs8  => FS_8_G,
            FsM::Fs12 => FS_12_G,
            FsM::Fs16 => FS_16_G,
        };
        if self.reboot_m {reg |= REBOOT_M;}
        if self.soft_reset_m {reg |= SOFT_RESET_M;}
        reg
    }
}

impl CtrlReg2M {
    pub fn set_fs_m(&mut self, value: FsM) {
        self.fs_m = value
    }

    pub fn fs_m(&self) -> FsM {
        self.fs_m
    }

    pub fn set_reboot_m(&mut self, value: bool) {self.reboot_m = value}
    pub fn reboot_m(&self) -> bool {self.reboot_m}
    pub fn set_soft_reset_m(&mut self, value: bool) {self.soft_reset_m = value}
    pub fn soft_reset_m(&self) -> bool {self.soft_reset_m}

}

#[cfg(test)]
mod tests {
    use super::CtrlReg2M;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b0_11_0_11_00;
        let r = CtrlReg2M::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
