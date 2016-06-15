#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address{
    RW(u8),
    RW16(u8),
    R(u8),
    R16(u8),
}

const ACT_THS:          Address = Address::RW(0x04);
const ACT_DUR:          Address = Address::RW(0x05);
const INT_GEN_CFG_XL:   Address = Address::RW(0x06);
const INT_GEN_THS_X_XL: Address = Address::RW(0x07);
const INT_GEN_THS_Y_XL: Address = Address::RW(0x08);
const INT_GEN_THS_Z_XL: Address = Address::RW(0x09);
const INT_GEN_DUR_XL:   Address = Address::RW(0x0A);
const REFERENCE_G:      Address = Address::RW(0x0B);
const INT1_CTRL:        Address = Address::RW(0x0C);
const INT2_CTRL:        Address = Address::RW(0x0D);
const WHO_AM_I:         Address = Address::R(0x0F);
const CTRL_REG1_G:      Address = Address::RW(0x10);
const CTRL_REG2_G:      Address = Address::RW(0x11);
const CTRL_REG3_G:      Address = Address::RW(0x12);
const ORIENT_CFG_G:     Address = Address::RW(0x13);
const INT_GEN_SRC_G:    Address = Address::RW(0x14);
const OUT_TEMP:         Address = Address::R16(0x15);
const STATUS_REG:       Address = Address::R(0x17);
const OUT_X_G:          Address = Address::R16(0x18);
const OUT_Y_G:          Address = Address::R16(0x1A);
const OUT_Z_G:          Address = Address::R16(0x1C);
const CTRL_REG4:        Address = Address::RW(0x1E);
const CTRL_REG5_XL:     Address = Address::RW(0x1F);
const CTRL_REG6_XL:     Address = Address::RW(0x20);
const CTRL_REG7_XL:     Address = Address::RW(0x21);
const CTRL_REG8:        Address = Address::RW(0x22);
const CTRL_REG9:        Address = Address::RW(0x23);
const CTRL_REG10:       Address = Address::RW(0x24);
const INT_GEN_SRC_XL:   Address = Address::R(0x26);
const STATUS_REG_BIS:   Address = Address::R(0x27);
const OUT_X_XL:         Address = Address::R16(0x28);
const OUT_Y_XL:         Address = Address::R16(0x2A);
const OUT_Z_XL:         Address = Address::R16(0x2C);
const FIFO_CTRL:        Address = Address::RW(0x2E);
const FIFO_SRC:         Address = Address::R(0x2F);
const INT_GEN_CFG_G:    Address = Address::RW(0x30);
const INT_GEN_THS_X_G:  Address = Address::RW16(0x31);
const INT_GEN_THS_Y_G:  Address = Address::RW16(0x33);
const INT_GEN_THS_Z_G:  Address = Address::RW16(0x35);
const INT_GEN_DUR_G:    Address = Address::RW(0x37);
const OFFSET_X_REG_M:   Address = Address::RW16(0x05);
const OFFSET_Y_REG_M:   Address = Address::RW16(0x07);
const OFFSET_Z_REG_M:   Address = Address::RW16(0x09);
const WHO_AM_I_M:       Address = Address::R(0x0F);
const CTRL_REG1_M:      Address = Address::RW(0x20);
const CTRL_REG2_M:      Address = Address::RW(0x21);
const CTRL_REG3_M:      Address = Address::RW(0x22);
const CTRL_REG4_M:      Address = Address::RW(0x23);
const CTRL_REG5_M:      Address = Address::RW(0x24);
const STATUS_REG_M:     Address = Address::R(0x27);
const OUT_X_M:          Address = Address::R16(0x28);
const OUT_Y_M:          Address = Address::R16(0x2A);
const OUT_Z_M:          Address = Address::R16(0x2C);
const INT_CFG_M:        Address = Address::RW(0x30);
const INT_SRC_M:        Address = Address::R(0x31);
const INT_THS_M:        Address = Address::R16(0x32);




mod accelerometer_gyroscope {

    enum ParamType{
        SleepOnInactEn,
        ActThs,
        ActDur,    
    }
    enum Param {
        SleepOnInactEn(bool),
        ActThs(u8),
        ActDur(u8),
    }

    enum SleepAT {
        On,
        Off
    }

    // fn decode(value: u8) -> (SleepAT, u8) {
    //     (match value & SLEEP_ON_INACT_EN as u8 {
    //         SLEEP_ON_INACT_EN as u8 => SleepAT::On,
    //         _ => SleepAT::Off,
    //     }, value & ACT_THS_VALUE)
    // }

    // bitflags! {
    //     flags TActThs: u8 {
    //         const T_SLEEP_ON_INACT_EN = 0b10000000,
    //     }
    // }

    // impl TActThs {
    //     value(v: u8) -> Result<Self,()> {
    //         match v & T_SLEEP_ON_INACT_EN.bits() {
    //             0 => Ok(),
    //             _ => Err(),
    //         }
    //     }

    //     // value(&mut self, v: u8) {
            
    //     // }
    // }

    
    const SLEEP_ON_INACT_EN: u8 = 0b10000000;
    const ACT_THS_MASK:      u8 = 0b01111111;
    
    struct ActThsReg {}

    impl ActThsReg {
        fn decode(value: u8) -> Vec<Param> {
            let a = Param::SleepOnInactEn(value & SLEEP_ON_INACT_EN != 0);
            // let a = match value & SLEEP_ON_INACT_EN {
            //     SLEEP_ON_INACT_EN => Param::SleepOnInactEn(true),
            //     _                 => Param::SleepOnInactEn(false),
            // };
            let b = Param::ActThs(value & ACT_THS_MASK);
            vec![a, b]
        }
        fn encode(params: Vec<Param>) -> u8 {
            let mut res: u8 = 0;
            for param in params {
                match param {                   
                    Param::SleepOnInactEn(true)  => res |=  SLEEP_ON_INACT_EN,
                    Param::SleepOnInactEn(false) => res |= !SLEEP_ON_INACT_EN,
                    Param::ActThs(v)         => res |= v,
                    _                 => {},
                }
            }
            res
        }
    }

    
    bitflags! {
        flags IntGenCfgXL: u8 {
            const AOI_XL    = 0b10000000,
            const DETECT_6D = 0b01000000,
            const ZHIE_XL   = 0b00100000,
            const ZLIE_XL   = 0b00010000,
            const YHIE_XL   = 0b00001000,
            const YLIE_XL   = 0b00000100,
            const XHIE_XL   = 0b00000010,
            const XLIE_XL   = 0b00000001,
        }
    }

    bitflags! {
        flags IntGenDurXL: u8 {
            const WAIT_XL = 0b10000000,
            const DUR_XL  = 0b01111111,
        }
    }

    bitflags! {
        flags Int1Ctrl: u8 {
            const INT1_IG_G    = 0b10000000,
            const INT1_IG_XL   = 0b01000000,
            const INT1_FSS5    = 0b00100000,
            const INT1_OVR     = 0b00010000,
            const INT1_FTH     = 0b00001000,
            const INT1_BOOT    = 0b00000100,
            const INT1_DRDY_G  = 0b00000010,
            const INT1_DRDY_XL = 0b00000001,
        }
    }

    bitflags! {
        flags Int2Ctrl: u8 {
            const INT2_IN_ACT    = 0b10000000,
            const INT2_FSS5      = 0b00100000,
            const INT2_OVR       = 0b00010000,
            const INT2_FTH       = 0b00001000,
            const INT2_DRDY_TEMP = 0b00000100,
            const INT2_DRDY_G    = 0b00000010,
            const INT2_DRDY_XL   = 0b00000001,
        }
    }

    const I_AM: u8 = 0b01101000;

    bitflags! {
        flags CtrlReg1G: u8 {
            const ODR_G_POWER_DOWN = 0b000_00_0_00,
            const ODR_G_14_9HZ     = 0b001_00_0_00,
            const ODR_G_59_5HZ     = 0b010_00_0_00,
            const ODR_G_119HZ      = 0b011_00_0_00,
            const ODR_G_238HZ      = 0b100_00_0_00,
            const ODR_G_476HZ      = 0b101_00_0_00,
            const ODR_G_952HZ      = 0b110_00_0_00,
            const FS_G_245DPS      = 0b000_00_0_00,
            const FS_G_500DPS      = 0b000_10_0_00,
            const FS_G_2000DPS     = 0b000_11_0_00,
            const BW_G             = 0b000_00_0_11,
        }
    }

    bitflags! {
        flags CtrlReg2G: u8 {
            const INT_SEL = 0b0000_11_00,
            const OUT_SEL = 0b0000_00_11,
        }
    }

    bitflags! {
        flags CtrlReg3G: u8 {
            const LP_MODE = 0b1_0_00_0000,
            const HP_EN   = 0b0_1_00_0000,
            const HPCF_G  = 0b0_0_00_1111,
        }
    }

    bitflags! {
        flags OrientCfgG: u8 {
            const SIGNX_G = 0b00_1_0_0_000,
            const SIGNY_G = 0b00_0_1_0_000,
            const SIGNZ_G = 0b00_0_0_1_000,
            const ORIENT  = 0b00_0_0_0_111,
        }
    }

    bitflags! {
        flags IntGenSrcG: u8 {
            const IA_G = 0b01000000,
            const ZH_G = 0b00100000,
            const ZL_G = 0b00010000,
            const YH_G = 0b00001000,
            const YL_G = 0b00000100,
            const XH_G = 0b00000010,
            const XL_G = 0b00000001,
        }
    }

    bitflags! {
        flags StatusReg: u8 {
            const IG_XL       = 0b01000000,
            const IG_G        = 0b00100000,
            const INACT       = 0b00010000,
            const BOOT_STATUS = 0b00001000,
            const TDA         = 0b00000100,
            const GDA         = 0b00000010,
            const XLDA        = 0b00000001,
        }
    }

    bitflags! {
        flags CtrlReg4: u8 {
            const ZEN_G   = 0b00_100_0_00,
            const YEN_G   = 0b00_010_0_00,
            const XEN_G   = 0b00_001_0_00,
            const LIR_XL1 = 0b00_000_0_10,
            const I4D_XL1 = 0b00_000_0_01,
        }
    }

    bitflags! {
        flags CtrlReg5XL: u8 {
            const DEC    = 0b11_0_0_0_000,
            const ZEN_XL = 0b00_1_0_0_000,
            const YEN_XL = 0b00_0_1_0_000,
            const XEN_XL = 0b00_0_0_1_000,
        }
    }

    bitflags! {
        flags CtrlReg6XL: u8 {
            const ODR_POWER_DOWN = 0b000_00_0_00,
            const ODR_10HZ       = 0b001_00_0_00,
            const ODR_50HZ       = 0b010_00_0_00,
            const ODR_119HZ      = 0b011_00_0_00,
            const ODR_238HZ      = 0b100_00_0_00,
            const ODR_476HZ      = 0b101_00_0_00,
            const ODR_952HZ      = 0b110_00_0_00,
            const FS_2G          = 0b000_00_0_00,
            const FS_16G         = 0b000_01_0_00,
            const FS_4G          = 0b000_10_0_00,
            const FS_8G          = 0b000_11_0_00,
            const BS_AUTO        = 0b000_00_0_00,
            const BS_MANU        = 0b000_00_1_00,
            const BW_408HZ       = 0b000_00_0_00,
            const BW_211HZ       = 0b000_00_0_01,
            const BW_105HZ       = 0b000_00_0_10,
            const BW_50HZ        = 0b000_00_0_11,
        }
    }

    bitflags! {
        flags CtrlReg7XL: u8 {
            const HR            = 0b1_00_00_0_0_0,
            const DCF_ODR_S_50  = 0b0_00_00_0_0_0,
            const DCF_ODR_S_100 = 0b0_01_00_0_0_0,
            const DCF_ODR_S_9   = 0b0_10_00_0_0_0,
            const DCF_ODR_S_400 = 0b0_11_00_0_0_0,
            const DCF_ODR       = 0b0_11_00_0_0_0,
            const FDS           = 0b0_00_00_1_0_0,
            const HPIS1         = 0b0_00_00_0_0_1,
        }
    }

    bitflags! {
        flags CtrlReg8: u8 {
            const BOOT       = 0b10000000,
            const BDU        = 0b01000000,
            const H_LACTIVE  = 0b00100000,
            const PP_OD      = 0b00010000,
            const SIM        = 0b00001000,
            const IF_ADD_INC = 0b00000100,
            const BLE        = 0b00000010,
            const SW_RESET   = 0b00000001,
        }
    }

    bitflags! {
        flags CtrlReg9: u8 {
            const SLEEP_G       = 0b0_1_0_00000,
            const FIFO_TEMP_EN  = 0b0_0_0_10000,
            const DRDY_MASK_BIT = 0b0_0_0_01000,
            const I2C_DISABLE   = 0b0_0_0_00100,
            const FIFO_EN       = 0b0_0_0_00010,
            const STOP_ON_FTH   = 0b0_0_0_00001,
        }
    }

    bitflags! {
        flags CtrlReg10: u8 {
            const ST_G  = 0b00000_1_0_0,
            const ST_XL = 0b00000_0_0_1,
        }
    }

    bitflags! {
        flags IntGenSrcXL: u8 {
            const IS_XL = 0b01000000,
            const ZH_XL = 0b00100000,
            const ZL_XL = 0b00010000,
            const YH_XL = 0b00001000,
            const YL_XL = 0b00000100,
            const XH_XL = 0b00000010,
            const XL_XL = 0b00000001,
        }
    }

    bitflags! {
        flags FifoCtrl: u8 {
            const FMODE                  = 0b111_00000,
            const FMODE_BYPASS           = 0b000_00000,
            const FMODE_UNTIL_FULL       = 0b001_00000,
            const FMODE_TRIG_TO_FIFO     = 0b011_00000,
            const FMODE_TRIG_TO_OVERRIDE = 0b100_00000,
            const FMODE_OVERRIDE         = 0b110_00000,
            const FTH_VALUE              = 0b000_11111,
        }
    }

    bitflags! {
        flags FifoSrc: u8 {
            const FTH  = 0b10_000000,
            const OVRN = 0b01_000000,
            const FSS  = 0b00_111111,
        }
    }

    bitflags! {
        flags IntGenCfgG: u8 {
            const AOI_G  = 0b10000000,
            const LIR_G  = 0b01000000,
            const ZHIE_G = 0b00100000,
            const ZLIE_G = 0b00010000,
            const YHIE_G = 0b00001000,
            const YLIE_G = 0b00000100,
            const XHIE_G = 0b00000010,
            const XLIE_G = 0b00000001,
        }
    }

    bitflags! {
        flags IntGenThsXG: u16 {
            const DCRM_G  = 0b1_0000000_00000000,
            const THS_G_X = 0b0_1111111_11111111,
        }
    }

    bitflags! {
        flags IntGenThsYG: u16 {
            const THS_G_Y = 0b0_1111111_11111111,
        }
    }

    bitflags! {
        flags IntGenThsZG: u16 {
            const THS_G_Z = 0b0_1111111_11111111,
        }
    }

    bitflags! {
        flags IntGenDurG: u8 {
            const WAIT_G = 0b1_0000000,
            const DUR_G  = 0b0_1111111,
        }
    }
}


mod magnetometer {
    const I_AM_M: u8 = 0b00111101;

    bitflags! {
        flags CtrlReg1M: u8 {
            const TEMP_COMP    = 0b1_00_000_0_0,
            const OM           = 0b0_11_000_0_0,
            const OM_LOW_POWER = 0b0_00_000_0_0,
            const OM_MED_PERF  = 0b0_01_000_0_0,
            const OM_HI_PERF   = 0b0_10_000_0_0,
            const OM_UHI_PERF  = 0b0_11_000_0_0,
            const DO           = 0b0_00_111_0_0,
            const DO_0_625HZ   = 0b0_00_111_0_0,
            const DO_1_25HZ    = 0b0_00_111_0_0,
            const DO_2_5HZ     = 0b0_00_111_0_0,
            const DO_5HZ       = 0b0_00_111_0_0,
            const DO_10HZ      = 0b0_00_111_0_0,
            const DO_20HZ      = 0b0_00_111_0_0,
            const DO_40HZ      = 0b0_00_111_0_0,
            const DO_80HZ      = 0b0_00_111_0_0,
            const ST           = 0b0_00_000_0_1,
        }
    }

    bitflags! {
        flags CtrlReg2M: u8 {
            const FS         = 0b0_11_0_0_0_00,
            const FS_4GAUSS  = 0b0_00_0_0_0_00,
            const FS_8GAUSS  = 0b0_01_0_0_0_00,
            const FS_12GAUSS = 0b0_10_0_0_0_00,
            const FS_16GAUSS = 0b0_11_0_0_0_00,
            const REBOOT     = 0b0_00_0_1_0_00,
            const SOFT_RST   = 0b0_00_0_0_1_00,
        }
    }

    bitflags! {
        flags CtrlReg3M: u8 {
            const I2C_DISABLE = 0b1_0_0_00_0_00,
            const LP          = 0b0_0_1_00_0_00,
            const SIM         = 0b0_0_0_00_1_00,
            const MD_CONT     = 0b0_0_0_00_0_00,
            const MD_SINGLE   = 0b0_0_0_00_0_01,
            const MD_PWR_DWN  = 0b0_0_0_00_0_10,
        }
    }

    bitflags! {
        flags CtrlReg4M: u8 {
            const OMZ           = 0b0000_11_0_0,
            const OMZ_LOW_POWER = 0b0000_00_0_0,
            const OMZ_MED_PERF  = 0b0000_01_0_0,
            const OMZ_HI_PERF   = 0b0000_10_0_0,
            const OMZ_UHI_PERF  = 0b0000_11_0_0,
            const BLE           = 0b0000_00_1_0,
        }
    }

    bitflags! {
        flags StatusRegM: u8 {
            const ZYXOR = 0b10000000,
            const ZOR   = 0b01000000,
            const YOR   = 0b00100000,
            const XOR   = 0b00010000,
            const ZYXDA = 0b00001000,
            const ZDA   = 0b00000100,
            const YDA   = 0b00000010,
            const XDA   = 0b00000001,
        }
    }

    bitflags! {
        flags IntCfgM: u8 {
            const XIEN = 0b100_00_000,
            const YIEN = 0b010_00_000,
            const ZIEN = 0b001_00_000,
            const IEA  = 0b000_00_100,
            const IEL  = 0b000_00_010,
            const IEN  = 0b000_00_001,
        }
    }

    bitflags! {
        flags IntSrcM: u8 {
            const PTH_X = 0b10000000,
            const PTH_Y = 0b01000000,
            const PTH_Z = 0b00100000,
            const NTH_X = 0b00010000,
            const NTH_Y = 0b00001000,
            const NTH_Z = 0b00000100,
            const MROI  = 0b00000010,
            const INT   = 0b00000001,
        }
    }

    bitflags! {
        flags IntThs: u16 {
            const INT_THS = 0b01111111,
        }
    }

}
