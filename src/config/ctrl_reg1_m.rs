use super::super::Address;
use super::{Register, CTRL_REG1_M, OpMode, OutputDataRate};

const TEMP_COMP:          u8 = 0b1_00_000_0_0;

const OM_MASK:            u8 = 0b0_11_000_0_0;
const OM_LOW_POWER:       u8 = 0b0_00_000_0_0;
const OM_MEDIUM_PERF:     u8 = 0b0_01_000_0_0;
const OM_HIGH_PERF:       u8 = 0b0_10_000_0_0;
const OM_ULTRA_HIGH_PERF: u8 = 0b0_11_000_0_0;

const ODR_MASK:           u8 = 0b0_00_111_0_0;
const ODR_0_625_HZ:       u8 = 0b0_00_000_0_0;
const ODR_1_25_HZ:        u8 = 0b0_00_001_0_0;
const ODR_2_5_HZ:         u8 = 0b0_00_010_0_0;
const ODR_5_HZ:           u8 = 0b0_00_011_0_0;
const ODR_10_HZ:          u8 = 0b0_00_100_0_0;
const ODR_20_HZ:          u8 = 0b0_00_101_0_0;
const ODR_40_HZ:          u8 = 0b0_00_110_0_0;
const ODR_80_HZ:          u8 = 0b0_00_111_0_0;

const ERRORS:             u8 = 0b0_00_000_1_0;

const SELF_TEST:          u8 = 0b0_00_000_0_1;

#[derive(Clone, Debug, PartialEq)]
pub struct CtrlReg1M {
    op_mode: OpMode,
    output_data_rate: OutputDataRate,
    temp_comp: bool,
    self_test: bool,
}

impl Register<u8> for CtrlReg1M {
    fn addr(&self) -> Address {
        CTRL_REG1_M
    }
    
    fn default() -> Self {
        CtrlReg1M {
            op_mode: OpMode::default(),
            output_data_rate: OutputDataRate::default(),
            temp_comp: false,
            self_test: false,
        }
    }

    fn new(reg: u8) -> Self {
        let op_mode = match reg & OM_MASK {
            OM_LOW_POWER       => OpMode::LowPower,
            OM_MEDIUM_PERF     => OpMode::MediumPerf,
            OM_HIGH_PERF       => OpMode::HighPerf,
            OM_ULTRA_HIGH_PERF => OpMode::UltraHighPerf,
            _ => unreachable!(),
        };
        let output_data_rate = match reg & ODR_MASK {
            ODR_0_625_HZ => OutputDataRate::Odr0p625Hz,
            ODR_1_25_HZ  => OutputDataRate::Odr1p25Hz,
            ODR_2_5_HZ   => OutputDataRate::Odr2p5Hz,
            ODR_5_HZ     => OutputDataRate::Odr5Hz,
            ODR_10_HZ    => OutputDataRate::Odr10Hz,
            ODR_20_HZ    => OutputDataRate::Odr20Hz,
            ODR_40_HZ    => OutputDataRate::Odr40Hz,
            ODR_80_HZ    => OutputDataRate::Odr80Hz,
            _ => unreachable!(),
        };
        CtrlReg1M {
            op_mode: op_mode,
            output_data_rate: output_data_rate,
            temp_comp: reg & TEMP_COMP != 0,
            self_test: reg & SELF_TEST != 0,
        }
    }

    fn reg(&self) -> u8 {
        let mut reg = match self.op_mode {
            OpMode::LowPower      => OM_LOW_POWER,
            OpMode::MediumPerf    => OM_MEDIUM_PERF,
            OpMode::HighPerf      => OM_HIGH_PERF,
            OpMode::UltraHighPerf => OM_ULTRA_HIGH_PERF,
        };
        reg |= match self.output_data_rate {
            OutputDataRate::Odr0p625Hz => ODR_0_625_HZ,
            OutputDataRate::Odr1p25Hz  => ODR_1_25_HZ,
            OutputDataRate::Odr2p5Hz   => ODR_2_5_HZ,
            OutputDataRate::Odr5Hz     => ODR_5_HZ,
            OutputDataRate::Odr10Hz    => ODR_10_HZ,
            OutputDataRate::Odr20Hz    => ODR_20_HZ,
            OutputDataRate::Odr40Hz    => ODR_40_HZ,
            OutputDataRate::Odr80Hz    => ODR_80_HZ,
        };
        if self.temp_comp {reg |= TEMP_COMP;}
        if self.self_test {reg |= SELF_TEST;}
        reg
    }
}

impl CtrlReg1M {
    pub fn set_op_mode(&mut self, value: OpMode) {
        self.op_mode = value
    }

    pub fn op_mode(&self) -> OpMode {
        self.op_mode
    }

    pub fn set_output_data_rate(&mut self, value: OutputDataRate) {
        self.output_data_rate = value
    }

    pub fn output_data_rate(&self) -> OutputDataRate {
        self.output_data_rate
    }

    pub fn set_temp_comp(&mut self, value: bool) {self.temp_comp = value}
    pub fn temp_comp(&self) -> bool {self.temp_comp}

    pub fn set_self_test(&mut self, value: bool) {self.self_test = value}
    pub fn self_test(&self) -> bool {self.self_test}

}

#[cfg(test)]
mod tests {
    use super::CtrlReg1M;
    use super::super::Register;

    #[test]
    fn it_works() {
        const REG: u8 = 0b1_11_111_0_1;
        let r = CtrlReg1M::new(REG);
        assert_eq!(r.reg(), REG);
    }
}
