reg_is_param!(IntGenThsXXl);

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::IntGenThsXXl(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
