use super::{Register, Param, ParamType};

pub fn from_params(params: &[Param]) -> Option<Register> {
    let possible_params = vec![ParamType::ActDur];
    let mut found = false;
    for &param in params {
        if possible_params.contains(&param.type_of()) {
            found = true;
            break;
        }
    }
    if found {
        let mut reg = 0x00;
        for &param in params {
            match param {
                Param::ActDur(x) => reg = x,
                _ => (),
            }
        }
        Some(Register::ActDur(reg))
    } else {
        None
    }
}

/// Return a list of param for the *ACT_DUR* register.
/// 
/// # Errors
///
/// Return an error if the register is not a `Register::ActDur`.
pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
    match reg {
        Register::ActDur(r) => {
            Ok(vec![Param::ActDur(r)])
        }
        _ => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Register};
    use super::{from_register, from_params};

    #[test]
    fn it_works() {
        let r1 = Register::ActDur(0x1A);
        let r2 = from_params(&from_register(r1).unwrap()).unwrap();
        assert_eq!(r1, r2);
    }
}
