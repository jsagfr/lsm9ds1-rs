use super::{Register, Param};

#[allow(unused_variables)]
pub fn from_params(params: &[Param]) -> Result<Register,()> {
    unimplemented!();
}

#[allow(unused_variables)]
pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::CtrlReg1G(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
