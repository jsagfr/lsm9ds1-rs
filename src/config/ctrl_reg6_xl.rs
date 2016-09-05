use super::{Register, CTRL_REG6_XL, DataRate, FsXl, Bw};

const ODR_XL_MASK:       u8 = 0b111_00000;
const ODR_XL_POWER_DOWN: u8 = 0b000_00000;
const ODR_XL_14_9_HZ:    u8 = 0b001_00000;
const ODR_XL_59_5_HZ:    u8 = 0b010_00000;
const ODR_XL_119_HZ:     u8 = 0b011_00000;
const ODR_XL_238_HZ:     u8 = 0b100_00000;
const ODR_XL_476_HZ:     u8 = 0b101_00000;
const ODR_XL_952_HZ:     u8 = 0b110_00000;
const ODR_XL_NA:         u8 = 0b111_00000;
const FS_XL_MASK:        u8 = 0b000_11_000;
const FS_XL_2:           u8 = 0b000_00_000;
const FS_XL_4:           u8 = 0b000_01_000;
const FS_XL_8:           u8 = 0b000_10_000;
const FS_XL_16:          u8 = 0b000_11_000;
const BW_SCAL_ODR:       u8 = 0b0000_0100;
const BW_XL_MASK:        u8 = 0b0000_00_11;
const BW_XL_A:           u8 = 0b0000_00_00;
const BW_XL_B:           u8 = 0b0000_00_01;
const BW_XL_C:           u8 = 0b0000_00_10;
const BW_XL_D:           u8 = 0b0000_00_11;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg6XL {
    odr_xl: DataRate,
    fs_xl: FsXl,
    bw_scal_odr: bool,
    bw_xl: Bw,
}

impl Register<u8> for CtrlReg6XL {
    fn addr(&self) -> u8 {
        CTRL_REG6_XL
    }
    
    fn default() -> Self {
        CtrlReg6XL {
            odr_xl: DataRate::default(),
            fs_xl: FsXl::default(),
            bw_scal_odr: false,
            bw_xl: Bw::default(),
        }
    }

    fn new(reg: u8) -> Self {
        let odr_xl = match reg & ODR_XL_MASK {
            ODR_XL_POWER_DOWN => DataRate::PowerDown,
            ODR_XL_14_9_HZ    => DataRate::DR15Hz,
            ODR_XL_59_5_HZ    => DataRate::DR59Hz,
            ODR_XL_119_HZ     => DataRate::DR119Hz,
            ODR_XL_238_HZ     => DataRate::DR238Hz,
            ODR_XL_476_HZ     => DataRate::DR476Hz,
            ODR_XL_952_HZ     => DataRate::DR952Hz,
            ODR_XL_NA         => DataRate::NA,
            _ =>  unreachable!(),
        };
        let fs_xl = match reg & FS_XL_MASK {
            FS_XL_2  => FsXl::Fs2,
            FS_XL_4  => FsXl::Fs4,
            FS_XL_8  => FsXl::Fs8,
            FS_XL_16 => FsXl::Fs16,
            _ =>  unreachable!(),
        };
        let bw_xl = match reg & BW_XL_MASK {
            BW_XL_A => Bw::A,
            BW_XL_B => Bw::B,
            BW_XL_C => Bw::C,
            BW_XL_D => Bw::D,
            _ =>  unreachable!(),
        };
        CtrlReg6XL {
            odr_xl: odr_xl,
            fs_xl: fs_xl,
            bw_scal_odr: reg & BW_SCAL_ODR != 0,
            bw_xl: bw_xl,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.odr_xl {
            DataRate::PowerDown => ODR_XL_POWER_DOWN,
            DataRate::DR15Hz    => ODR_XL_14_9_HZ,
            DataRate::DR59Hz    => ODR_XL_59_5_HZ,
            DataRate::DR119Hz   => ODR_XL_119_HZ,
            DataRate::DR238Hz   => ODR_XL_238_HZ,
            DataRate::DR476Hz   => ODR_XL_476_HZ,
            DataRate::DR952Hz   => ODR_XL_952_HZ,
            DataRate::NA        => ODR_XL_NA,
        };
        reg |= match self.fs_xl {
            FsXl::Fs2  => FS_XL_2,
            FsXl::Fs4  => FS_XL_4,
            FsXl::Fs8  => FS_XL_8,
            FsXl::Fs16 => FS_XL_16,
        };
        if self.bw_scal_odr {reg |= BW_SCAL_ODR;}
        reg |= match self.bw_xl {
            Bw::A => BW_XL_A,
            Bw::B => BW_XL_B,
            Bw::C => BW_XL_C,
            Bw::D => BW_XL_D,
        };
        reg
    }
}

impl CtrlReg6XL {
    pub fn set_odr_xl(&mut self, value: DataRate) {
        self.odr_xl = value
    }

    pub fn odr_xl(&self) -> DataRate {
        self.odr_xl
    }

    pub fn set_fs_xl(&mut self, value: FsXl) {
        self.fs_xl = value
    }

    pub fn fs_xl(&self) -> FsXl {
        self.fs_xl
    }

    pub fn set_bw_scal_odr(&mut self, value: bool) {self.bw_scal_odr = value}
    pub fn bw_scal_odr(&self) -> bool {self.bw_scal_odr}

    pub fn set_bw_xl(&mut self, value: Bw) {
        self.bw_xl = value
    }

    pub fn bw_xl(&self) -> Bw {
        self.bw_xl
    }

}

#[cfg(test)]
mod tests {
    use super::CtrlReg6XL;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = CtrlReg6XL::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
