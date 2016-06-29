macro_rules! enum_with_type {
    ( $(#[$Eattr:meta])* enum $E:ident,
      $(#[$ETattr:meta])* enum_type $ET:ident {
        $($(#[$eattr:meta])* variant $e:ident => $et:ty ),+
            $(,)*
    }) => {
        $(#[$Eattr])*
        pub enum $E {
            $($(#[$eattr])* $e($et)),+
        }
        
        $(#[$ETattr])*
        pub enum $ET {
            $($e),+
        }

        impl $E {
            pub fn type_of(self: $E) -> $ET {
                match self {
                    $( $E::$e(_) => $ET::$e, )+
                }
            }
        }
    }
}

macro_rules! reg_is_param {
    ($x:ident) => {
        use super::{Register, Param};

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
    };
}

macro_rules! reg_is_bools {
    ($reg:ident => { $( $bit:ident : $cte:expr ,)* }
     bits_errors { $( $cte_error:expr,)* }) => (
        use super::{Register, Param};

        pub fn from_params(params: &[Param]) -> Result<Register,()> {
            let mut reg = 0x00;
            for &param in params {
                match param {
                    $( Param::$bit(x) => reg = if x {
                        reg |  $cte
                    } else {
                        reg & !$cte
                    },)*
                    _ => return Err(()),
                }
            }
            Ok(Register::$reg(reg))
        }

        pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
            let mut params = vec![];
            match reg {
                Register::$reg(r) => {
                    $( if r & $cte_error == $cte_error {return Err(())} )*
                    $( params.push(Param::$bit(r & $cte == $cte)); )*
                }
                _ => return Err(()),
            }
            Ok(params)
        }
    );

    ($reg:ident => { $( $bit:ident : $cte:expr ,)* }) => (
        reg_is_bools!{
            $reg => {
                $( $bit : $cte,)*
            }
            bits_errors {
            }
        }
    );
}
