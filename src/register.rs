pub trait RegisterAddress {
    fn address(self) -> u8;
}


pub trait RegisterReadable<Output> {
    fn read(value: u8) -> Output;
}


pub trait RegisterWritable {
    fn write(self) -> u8;
}

