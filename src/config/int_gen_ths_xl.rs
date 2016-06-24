macro_rules! int_gen_ths_xl {
    ($x:ident) => {
        use super::super::{Register, Param};

        pub fn from_params(params: &[Param]) -> Result<Register,()> {
            let mut reg = 0x00;
            for &param in params {
                match param {
                    Param::$x(x) => reg = x,
                    _ => return Err(()),
                }
            }
            Ok(Register::$x(reg))
        }

        pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
            match reg {
                Register::$x(r) => {
                    Ok(vec![Param::$x(r)])
                }
                _ => Err(()),
            }
        }

        #[cfg(test)]
        mod tests {
            use super::super::super::{Register};
            use super::{from_register, from_params};

            #[test]
            fn it_works() {
                let r1 = Register::$x(0x1A);
                let r2 = from_params(&from_register(r1).unwrap()).unwrap();
                assert_eq!(r1, r2);
            }
        }
    };
}

pub mod x {
    int_gen_ths_xl!(IntGenThsXXl);
}

pub mod y {
    int_gen_ths_xl!(IntGenThsYXl);
}

pub mod z {
    int_gen_ths_xl!(IntGenThsZXl);
}
