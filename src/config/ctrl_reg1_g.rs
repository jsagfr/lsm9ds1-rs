use super::super::Address;
use super::{Register, CTRL_REG1_G, DataRate, GyroScale, Bw};

const ODR_G_MASK:       u8 = 0b111_00000;
const ODR_G_POWER_DOWN: u8 = 0b000_00000;
const ODR_G_14_9_HZ:    u8 = 0b001_00000;
const ODR_G_59_5_HZ:    u8 = 0b010_00000;
const ODR_G_119_HZ:     u8 = 0b011_00000;
const ODR_G_238_HZ:     u8 = 0b100_00000;
const ODR_G_476_HZ:     u8 = 0b101_00000;
const ODR_G_952_HZ:     u8 = 0b110_00000;
const ODR_G_NA:         u8 = 0b111_00000;
const FS_G_MASK:        u8 = 0b000_11_000;
const FS_G_245_DPS:     u8 = 0b000_00_000;
const FS_G_500_DPS:     u8 = 0b000_01_000;
const FS_G_NA:          u8 = 0b000_10_000;
const FS_G_2000_DPS:    u8 = 0b000_11_000;
const ERROR1:           u8 = 0b0000_0100;
const BW_G_MASK:        u8 = 0b0000_00_11;
const BW_G_A:           u8 = 0b0000_00_00;
const BW_G_B:           u8 = 0b0000_00_01;
const BW_G_C:           u8 = 0b0000_00_10;
const BW_G_D:           u8 = 0b0000_00_11;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg1G {
    odr_g: DataRate,
    fs_g: GyroScale,
    bw_g: Bw,
}

impl Register<u8> for CtrlReg1G {
    fn addr(&self) -> Address {
        CTRL_REG1_G
    }
    
    fn default() -> Self {
        CtrlReg1G {
            odr_g: DataRate::default(),
            fs_g: GyroScale::default(),
            bw_g: Bw::default(),
        }
    }

    fn new(reg: u8) -> Self {
        let odr_g = match reg & ODR_G_MASK {
            ODR_G_POWER_DOWN => DataRate::PowerDown,
            ODR_G_14_9_HZ    => DataRate::DR15Hz,
            ODR_G_59_5_HZ    => DataRate::DR59Hz,
            ODR_G_119_HZ     => DataRate::DR119Hz,
            ODR_G_238_HZ     => DataRate::DR238Hz,
            ODR_G_476_HZ     => DataRate::DR476Hz,
            ODR_G_952_HZ     => DataRate::DR952Hz,
            ODR_G_NA         => DataRate::NA,
            _ =>  unreachable!(),
        };
        let fs_g = match reg & FS_G_MASK {
            FS_G_NA       => GyroScale::NA,
            FS_G_245_DPS  => GyroScale::FS245Dps,
            FS_G_500_DPS  => GyroScale::FS500Dps,
            FS_G_2000_DPS => GyroScale::FS2000Dps,
            _ =>  unreachable!(),
        };
        let bw_g = match reg & BW_G_MASK {
            BW_G_A => Bw::A,
            BW_G_B => Bw::B,
            BW_G_C => Bw::C,
            BW_G_D => Bw::D,
            _ =>  unreachable!(),
        };
        CtrlReg1G {
            odr_g: odr_g,
            fs_g: fs_g,
            bw_g: bw_g,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.odr_g {
            DataRate::PowerDown => ODR_G_POWER_DOWN,
            DataRate::DR15Hz    => ODR_G_14_9_HZ,
            DataRate::DR59Hz    => ODR_G_59_5_HZ,
            DataRate::DR119Hz   => ODR_G_119_HZ,
            DataRate::DR238Hz   => ODR_G_238_HZ,
            DataRate::DR476Hz   => ODR_G_476_HZ,
            DataRate::DR952Hz   => ODR_G_952_HZ,
            DataRate::NA        => ODR_G_NA,
        };
        reg |= match self.fs_g {
            GyroScale::NA        => FS_G_NA,
            GyroScale::FS245Dps  => FS_G_245_DPS,
            GyroScale::FS500Dps  => FS_G_500_DPS,
            GyroScale::FS2000Dps => FS_G_2000_DPS,
        };
        reg |= match self.bw_g {
            Bw::A => BW_G_A,
            Bw::B => BW_G_B,
            Bw::C => BW_G_C,
            Bw::D => BW_G_D,
        };
        reg
    }
}

impl CtrlReg1G {
    pub fn set_odr_g(&mut self, value: DataRate) {
        self.odr_g = value
    }

    pub fn odr_g(&self) -> DataRate {
        self.odr_g
    }

    pub fn set_fs_g(&mut self, value: GyroScale) {
        self.fs_g = value
    }

    pub fn fs_g(&self) -> GyroScale {
        self.fs_g
    }

    pub fn set_bw_g(&mut self, value: Bw) {
        self.bw_g = value
    }

    pub fn bw_g(&self) -> Bw {
        self.bw_g
    }
}

#[cfg(test)]
mod tests {
    use super::CtrlReg1G;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0x1A;
        let r = CtrlReg1G::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
