#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteAddress(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadAddress(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadWordAddress(pub u8);

pub trait Write {
    fn address(&self) -> WriteAddress;
    fn value(&self) -> u8;
}
