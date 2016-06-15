/// ACT_THS register is the Activity threshold register.

pub const REG_ADDR:  u8 = 0x04; // TODO: #![feature(associated_consts)]
const ACT_THS_MASK:  u8 = 0b0111_1111;
const SLEEP_ON_MASK: u8 = 0b1000_0000;

/// Gyroscope operating mode during inactivity:
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SleepOn {
    /// gyroscope in power-down
    ON  = 0b1000_0000,
    /// gyroscope in sleep mode
    OFF = 0b0000_0000,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ActThsRegBuilder {
    sleep_on: SleepOn,
    act_ths: u8,
}

impl ActThsRegBuilder {
    fn new() -> Self {
        ActThsRegBuilder {
            sleep_on: SleepOn::OFF,
            act_ths: 0,
        }
    }

    fn sleep_on<'a>(&'a mut self, value: SleepOn) -> &'a mut Self {
        self.sleep_on = value;
        self
    }

    fn act_ths<'a>(&'a mut self, value: u8) -> &'a mut Self {
        self.act_ths = value;
        self
    }

    fn finalize(&self) -> Result<ActThsReg, ()> {
        match self.act_ths & !ACT_THS_MASK {
            0 => {
                let act_ths = self.act_ths & ACT_THS_MASK;
                Ok(ActThsReg {
                    sleep_on: self.sleep_on,
                    act_ths: self.act_ths,
                    bits: act_ths | (self.sleep_on as u8),
                })
            }
            _ => Err(()),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActThsReg {
    sleep_on: SleepOn,
    act_ths: u8,
    bits: u8,
}

impl ActThsReg {

    fn from_reg(reg: u8) -> Self {
        ActThsReg {
            sleep_on: match SLEEP_ON_MASK & reg {
                x if x == SleepOn::ON as u8  => SleepOn::ON,
                x if x == SleepOn::OFF as u8 => SleepOn::OFF,
                _ => unreachable!("Wrong code in ActThsReg::from_reg"),
            },
            act_ths: ACT_THS_MASK & reg,
            bits: reg,
        }
    }

    fn to_reg(&self) -> u8 {
        self.bits
    }

    fn reg_addr(&self) -> u8 {
        REG_ADDR
    }
}

#[cfg(test)]
mod tests {
    use super::{ActThsRegBuilder, ActThsReg, SleepOn, REG_ADDR};
    #[test]
    fn it_works() {
        let on_7 = ActThsRegBuilder::new()
            .sleep_on(SleepOn::ON)
            .act_ths(7)
            .finalize()
            .unwrap();
        let r_on_7 = ActThsReg::from_reg(on_7.to_reg());
        assert_eq!(on_7, r_on_7);

        assert_eq!(on_7.reg_addr(), REG_ADDR);
        assert_eq!(r_on_7.reg_addr(), REG_ADDR);
    }
}
