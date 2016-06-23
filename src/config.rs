#![allow(dead_code)]
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Enable,
    Disable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AndOrState {
    And,
    Or,
}

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

enum_with_type!{
    /// Parameters used to configure or given when reading a
    /// *LSM9DS1*.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Param,
    /// Parameters type used to get a *LSM9DS1* configuration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum_type ParamType {
        /// Gyroscope inactivity threshold.
        ///
        /// __TODO: Question: is accelerometer concerned?__
        /// 
        /// * Possible values in `0..2**7`
        /// * Default value is `0`
        variant ActThs => u8,
        /// Gyroscope operating mode during inactivity.
        ///
        /// * Default value is `Disable`
        variant SleepOn => State,
        /// Gyroscope inactivity duration.
        ///
        /// __TODO: Question: is accelerometer concerned?__
        /// 
        /// * Possible values in `u8`
        /// * Default value is `0`
        variant ActDur => u8,
        /// AND/OR combination of accelerometer's interrupt events.
        /// 
        /// * Default value: `Or`
        variant AoiXl => AndOrState,
        variant Detect6D => State,
        variant ZhieXl => State,
        variant ZlieXl => State,
        variant YhieXl => State,
        variant YlieXl => State,
        variant XhieXl => State,
        variant XlieXl => State,
    }
}


enum_with_type!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Register,
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum_type RegisterType {
        variant ActThs => u8,
        variant ActDur => u8,
    }
}

impl RegisterType {
    fn from_params(&self, params: &[Param]) -> Result<Register,()> {
        match *self {
            RegisterType::ActThs => act_ths::from_params(params),
            _ => Err(()),
        }
    }
}

pub mod act_ths {
    use super::{Register, Param, State};

    const ACT_THS_MASK:  u8 = 0b0111_1111;
    const SLEEP_ON_MASK: u8 = 0b1000_0000;

    /// From params return a register with defaut values or values
    /// given in the param or error.
    /// 
    /// If all parameters are not supplies, defaut values are:
    /// * `Param::ActThs(0)`
    /// * `Param::SleepOn(State::Disable)`
    pub fn from_params(params: &[Param]) -> Result<Register,()> {
        let mut reg: u8 = 0x00;     // Default is 0.
        for &param in params {
            match param {
                Param::ActThs(x) => {
                    // Check value correctness ("u7")
                    match x & !ACT_THS_MASK {
                        0 =>  reg |= x,
                        _ => return Err(()),
                    }
                }
                Param::SleepOn(x) => match x {
                    State::Enable =>  reg |=  SLEEP_ON_MASK,
                    State::Disable => reg &= !SLEEP_ON_MASK,
                },
                _ => return Err(()),
            }
        }
        Ok(Register::ActThs(reg))
    }

    pub fn from_register(reg: Register) -> Result<Vec<Param>,()> {
        match reg {
            Register::ActThs(r) => {
                let sleep_on = match r & SLEEP_ON_MASK {
                    SLEEP_ON_MASK => Param::SleepOn(State::Enable),
                    0 => Param::SleepOn(State::Enable),
                    _ => unreachable!(),
                };
                let act_ths = Param::ActThs(r & ACT_THS_MASK);
                Ok(vec![sleep_on, act_ths])
            }
            _ => Err(()),
        }
    }
}

mod act_dur {
    use super::{Register, Param};

    fn from_params(params: &[Param]) -> Result<Register,()> {
        unimplemented!();
    }
}


/// `ConfigBuilder` is use to create a partial or total new configuration of
/// a *LSM9DS1*.
/// 
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfigBuilder, Param, State};
/// 
/// let conf1 = ConfigBuilder::new()
///     .set(Param::ActThs(5))
///     .set(Param::SleepOn(State::Enable))
///     .build().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct ConfigBuilder {
    params: Vec<Param>,
}

impl ConfigBuilder {
    /// `ConfigBuilder::new()` create a new emtpy `ConfigBuilder`.
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            params: Vec::new(),
        }
    }

    /// Set a specific parameter.
    pub fn set<'a>(&'a mut self, param: Param) -> &'a mut ConfigBuilder {
        self.params.push(param);
        self
    }

    /// Set a list of parameters.
    pub fn set_all<'a>(&'a mut self, params: &[Param]) -> &'a mut ConfigBuilder {
        self.params.extend(params);
        self
    }

    /// Build a `Config` from a `ConfigBuilder`.
    ///
    /// If the same parameter is set multiple times, with different
    /// values, the last one is used.
    ///
    /// # Errors
    ///
    /// `build()` may failed if a parameter is not compatible with the *LSM9DS1*
    /// specification.
    ///
    /// # Example
    ///
    /// ```
    /// use lsm9ds1::config::{ConfigBuilder, Param, State};
    /// 
    /// let conf1 = ConfigBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(State::Enable))
    ///     .build().unwrap();
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut hash_reg = HashMap::new();
        let mut hash_param = HashMap::new();
        for &p in &self.params {
            hash_param.insert(p.type_of(), p);
            match p {
                Param::ActThs(_) => {
                    let params = hash_reg.entry(RegisterType::ActThs).or_insert(Vec::new());
                    params.push(p);
                }
                Param::SleepOn(_) => {
                    let params = hash_reg.entry(RegisterType::ActThs).or_insert(Vec::new());
                    params.push(p);
                }
                _ => unimplemented!(),
            }
        }

        let mut registers = Vec::new();
        for (key, params) in hash_reg.iter() {
            registers.push(try!(key.from_params(params)));
        }

        Ok(Config {
            params: hash_param,
            registers: registers,
        })
    }
}

/// `Register` is use to create a partial or total new configuration of
/// a *LSM9DS1* from a set of register values.
/// 
/// # Examples
///
/// ```
/// use lsm9ds1::config::{Registers, Register};
/// 
/// let conf = Registers::new().
///     set(Register::ActThs(0b1000_0000 | 5))
///     .build().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Registers {
    registers: Vec<Register>,
}

impl Registers {
    /// `ConfigBuilder::new()` create a new emtpy `ConfigBuilder`.
    pub fn new() -> Registers {
        Registers {
            registers: Vec::new(),
        }
    }

    /// Set a specific register.
    pub fn set<'a>(&'a mut self, register: Register) -> &'a mut Registers {
        self.registers.push(register);
        self
    }

    /// Set a list of registers.
    pub fn set_all<'a>(&'a mut self, registers: &[Register]) -> &'a mut Registers {
        self.registers.extend(registers);
        self
    }

    /// Build a `Config` from a `Registers`.
    ///
    /// # Errors
    ///
    /// `build()` may failed if:
    ///
    /// * 2 identic registers have differents values or;
    ///
    /// * if a register value is not compatible with the *LSM9DS1*
    /// specification.
    ///
    /// # Example
    ///
    /// ```
    /// use lsm9ds1::config::{Registers, Register, Param, ParamType, State};
    /// 
    /// let conf = Registers::new().
    ///     set(Register::ActThs(0b1000_0000 | 5))
    ///     .build().unwrap();
    /// match *conf.param(ParamType::ActThs).unwrap() {
    ///     Param::ActThs(ref act_ths) => assert_eq!(*act_ths, 5),
    ///     _ => panic!(),
    /// };
    /// match *conf.param(ParamType::SleepOn).unwrap() {
    ///     Param::SleepOn(ref sleep_on) => assert_eq!(*sleep_on, State::Enable),
    ///     _ => panic!(),
    /// };
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut params = HashMap::new();
        for &reg in &self.registers {
            match reg {
                Register::ActThs(_) => {
                    for p in try!(act_ths::from_register(reg)) {
                        params.insert(p.type_of(), p);
                    }
                }
                _ => unimplemented!(),
            }
        }
        Ok(Config {
            params: params,
            registers: self.registers.clone(),
        })
    }
}


/// `Config` represents a partial or total possible configuration of a
/// *LSM9DS1*.
/// 
/// It can be build from:
///
/// * a set of registers value, with `Register`: It's useful if you
/// want to decode the config read from a *LSM9DS1* device.
///
/// * or from a set of values, with `ConfigBuilder`: It's useful if
/// you want to encode a configuration to a *LSM9DS1* device.
///
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfigBuilder, Param, Registers, State};
/// 
/// let conf1 = ConfigBuilder::new()
///     .set(Param::ActThs(5))
///     .set(Param::SleepOn(State::Enable))
///     .build().unwrap();
/// let conf2 = Registers::new().
///     set_all(conf1.registers())
///     .build().unwrap();
/// 
/// assert_eq!(conf1, conf2);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    params: HashMap<ParamType, Param>,
    registers: Vec<Register>
}

impl Config {
    /// Get the parameter of a parameter type from `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfigBuilder, Param, ParamType, State};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(State::Enable))
    ///     .build().unwrap();
    /// let &p = c.param(ParamType::ActThs).unwrap();
    /// assert_eq!(p, Param::ActThs(5));
    /// ```
    pub fn param(&self, param: ParamType) -> Option<&Param> {
        self.params.get(&param)
    }

    /// List all the parameters set for the current `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfigBuilder, Param, State};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(State::Enable))
    ///     .build().unwrap();
    /// for &p in c.params() {
    ///     assert!(p == Param::ActThs(5) || p == Param::SleepOn(State::Enable));
    /// }
    /// ```
    pub fn params(&self) -> Vec<&Param> {
        let params: Vec<&Param> = self.params.values().collect();
        params
    }

    /// List all the registers set for the current `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfigBuilder, Param, Register, RegisterType, State};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::ActThs(5))
    ///     .set(Param::SleepOn(State::Enable))
    ///     .build().unwrap();
    /// for &r in c.registers() {
    ///     assert_eq!(r.type_of(), RegisterType::ActThs);
    /// }
    /// ```
    pub fn registers(&self) -> &[Register] {
        &self.registers
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigBuilder, Registers, Param, State};
    // use params::Param;
    
    #[test]
    fn it_works() {
        let conf1 = ConfigBuilder::new()
            .set(Param::ActThs(5))
            .set(Param::SleepOn(State::Enable))
            .build().unwrap();
        let conf2 = Registers::new().
            set_all(conf1.registers())
            .build().unwrap();

        assert_eq!(conf1, conf2);
        
            
    }
}
