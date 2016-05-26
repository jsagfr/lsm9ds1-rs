use register::ReadWordAddress;

const OUT_X_XL: u8 = 0x28;
const OUT_Y_XL: u8 = 0x2A;
const OUT_Z_XL: u8 = 0x2C;

pub const OUT_X_ADDRESS_R: ReadWordAddress = ReadWordAddress(OUT_X_XL);
pub const OUT_Y_ADDRESS_R: ReadWordAddress = ReadWordAddress(OUT_Y_XL);
pub const OUT_Z_ADDRESS_R: ReadWordAddress = ReadWordAddress(OUT_Z_XL);

