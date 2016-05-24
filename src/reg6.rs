use register::{RegisterAddress, RegisterReadable, RegisterWritable};

const CTRL_REG6_XL: u8 = 0x20;

#[derive(Debug, Clone, Copy)]
enum Reg6ODR {
    PowerDown = 0b000_00000,
    Freq10Hz  = 0b001_00000,
    Freq50Hz  = 0b010_00000,
    Freq119Hz = 0b011_00000,
    Freq238Hz = 0b100_00000,
    Freq476Hz = 0b101_00000,
    Freq952Hz = 0b110_00000,
}

#[derive(Debug, Clone, Copy)]
enum Reg6FS {
    Acc2g  = 0b000_00_000,
    Acc16g = 0b000_01_000,
    Acc4g  = 0b000_10_000,
    Acc8g  = 0b000_11_000,
}

#[derive(Debug, Clone, Copy)]
enum Reg6BS {
    Auto = 0b00000_0_00,
    Manu = 0b00000_1_00,
}

#[derive(Debug, Clone, Copy)]
enum Reg6BW {
    Freq408Hz = 0b000000_00,
    Freq211Hz = 0b000000_01,
    Freq105Hz = 0b000000_10,
    Freq50Hz  = 0b000000_11,
}


#[derive(Debug, Clone, Copy)]
struct Reg6Builder {
    odr: Reg6ODR,
    fs:  Reg6FS,
    bs:  Reg6BS,
    bw:  Reg6BW,
}


#[derive(Debug, Clone, Copy)]
struct Reg6 {
    odr: Reg6ODR,
    fs:  Reg6FS,
    bs:  Reg6BS,
    bw:  Reg6BW,
}

impl RegisterAddress for Reg6 {
    fn address(self) -> u8 {
        CTRL_REG6_XL
    }
}

// TODO: Result or N/A?
// TODO: Bit matching?
impl RegisterReadable<Reg6> for Reg6 {
    fn read(value: u8) -> Reg6 {
        let odr = match value & 0b111_00000 {
            0b000_00000 => Reg6ODR::PowerDown,
            0b001_00000 => Reg6ODR::Freq10Hz,
            0b010_00000 => Reg6ODR::Freq50Hz,
            0b011_00000 => Reg6ODR::Freq119Hz,
            0b100_00000 => Reg6ODR::Freq238Hz,
            0b101_00000 => Reg6ODR::Freq476Hz,
            0b110_00000 => Reg6ODR::Freq952Hz,
            _ => panic!("unimplemented"),
        };
        let fs = match value & 0b000_11_000 {
            0b000_00_000 => Reg6FS::Acc2g,
            0b000_01_000 => Reg6FS::Acc16g,
            0b000_10_000 => Reg6FS::Acc4g,
            0b000_11_000 => Reg6FS::Acc8g,
            _ => panic!("unpossible"),
        };
        let bs = match value & 0b00000_1_00 {
            0b00000_0_00 => Reg6BS::Auto,
            0b00000_1_00 => Reg6BS::Manu,
            _ => panic!("unpossible"),
        };
        let bw = match value & 0b000000_11 {
            0b000000_00 => Reg6BW::Freq408Hz,
            0b000000_01 => Reg6BW::Freq211Hz,
            0b000000_10 => Reg6BW::Freq105Hz,
            0b000000_11 => Reg6BW::Freq50Hz,
            _ => panic!("unpossible"),
        };
        
        Reg6 {
            odr: odr,
            fs:  fs,
            bs:  bs,
            bw:  bw,
        }
    }
}

impl RegisterWritable for Reg6 {
    fn write(self) -> u8 {
        (self.odr as u8) | (self.fs  as u8) | (self.bs  as u8) | (self.bw  as u8)
    }
}


impl Reg6Builder {
    fn new() -> Reg6Builder {
        Reg6Builder {
            odr: Reg6ODR::PowerDown,
            fs:  Reg6FS::Acc2g,
            bs:  Reg6BS::Auto,
            bw:  Reg6BW::Freq408Hz,
        }
    }

    fn ord(&mut self, cmd: Reg6ODR) -> &mut Reg6Builder {
        self.odr = cmd;
        self
    }

    fn fs(&mut self, cmd: Reg6FS) -> &mut Reg6Builder {
        self.fs = cmd;
        self
    }

    fn bs(&mut self, cmd: Reg6BS) -> &mut Reg6Builder {
        self.bs = cmd;
        self
    }

    fn bw(&mut self, cmd: Reg6BW) -> &mut Reg6Builder {
        self.bw = cmd;
        self
    }
    
    fn finalize(&self) -> Reg6 {
        Reg6 {
            odr: self.odr,
            fs:  self.fs,
            bs:  self.bs,
            bw:  self.bw,            
        }
    }
}



#[cfg(test)]
mod tests {
    use super::{Reg6, Reg6Builder, Reg6ODR, Reg6FS, Reg6BS, Reg6BW};
    use register::{RegisterAddress, RegisterReadable, RegisterWritable};
    
    #[test]
    fn address() {
        let r = Reg6Builder::new().finalize();
        assert_eq!(r.address(), 0x20);
    }

    #[test]
    fn write_reg() {
        let r = Reg6Builder::new().finalize();
        assert_eq!(r.write(), 0b00000000);
        let r = Reg6Builder::new()
            .ord(Reg6ODR::Freq119Hz)
            .fs(Reg6FS::Acc8g)
            .bs(Reg6BS::Manu)
            .bw(Reg6BW::Freq105Hz)
            .finalize();
        assert_eq!(r.write(), 0b011_11_1_10);
    }

    #[test]
    fn read_reg() {
        // TODO: Quickcheck
        let r = Reg6::read(0b011_11_1_10);
        assert_eq!(r.write(), 0b011_11_1_10);            
    }
}
