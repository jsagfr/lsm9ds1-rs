use super::{Register, Param, FsM};

const ERRORS:     u8 = 0b1_00_1_00_11;
const FS_MASK:    u8 = 0b0_11_0_00_00;
const FS_4_G:     u8 = 0b0_00_0_00_00;
const FS_8_G:     u8 = 0b0_01_0_00_00;
const FS_12_G:    u8 = 0b0_10_0_00_00;
const FS_16_G:    u8 = 0b0_11_0_00_00;
const REBOOT:     u8 = 0b0_00_0_10_00;
const SOFT_RESET: u8 = 0b0_00_0_01_00;

pub fn from_params(params: &[Param]) -> Result<Register,()> {
    let mut reg: u8 = 0x00;     // Default is 0.
    for &param in params {
        match param {
            Param::FsM(x) => {
                reg = reg & !FS_MASK | match x {
                    FsM::Fs4  => FS_4_G,
                    FsM::Fs8  => FS_8_G,
                    FsM::Fs12 => FS_12_G,
                    FsM::Fs16 => FS_16_G,
                }
            }
            Param::RebootM(x) =>  reg = if x {
                reg |  REBOOT
            } else {
                reg & !REBOOT
            },
            Param::SoftResetM(x) =>  reg = if x {
                reg |  SOFT_RESET
            } else {
                reg & !SOFT_RESET
            },
            _ => return Err(()),
        }
    }
    Ok(Register::CtrlReg2M(reg))
}

pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::CtrlReg2M(r) => {
            if r & ERRORS != 0 {
                return Err(())
            };
            let reboot_m = Param::RebootM(r & REBOOT == REBOOT);
            let soft_reset = Param::SoftResetM(r & SOFT_RESET == SOFT_RESET);
            let fs = Param::FsM(match r & FS_MASK {
                FS_4_G  => FsM::Fs4,
                FS_8_G  => FsM::Fs8,
                FS_12_G => FsM::Fs12,
                FS_16_G => FsM::Fs16,
                _ => unreachable!(),
            });
            Ok(vec![reboot_m, soft_reset, fs])
        }
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg2M(0b0_11_0_11_00);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
