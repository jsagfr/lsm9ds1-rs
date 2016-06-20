#![allow(dead_code)]
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Enable,
    Disable,
}

macro_rules! enum_with_type {
    ( $E:ident, $ET:ident {
        $($e:ident => $et:ty ),+
            $(,)*
    }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $E {
            $($e($et)),+
        }
        
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $ET {
            $($e),+
        }
    }
}

enum_with_type!{
    Param, ParamType {
        ActThs => u8,
        SleepOn => State,
        ActDur => u8,
        AoiXl => State,
        Detect6d => State,
        ZhieXl => State,
        ZlieXl => State,
        YhieXl => State,
        YlieXl => State,
        XhieXl => State,
        XlieXl => State,
    }
}

pub fn type_of_param(param: Param) -> ParamType {
    match param {
        Param::ActThs(_) => ParamType::ActThs,
        Param::SleepOn(_) => ParamType::SleepOn,
        Param::ActDur(_) => ParamType::ActDur,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    ActThs(u8),
    ActDur(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RegisterType {
    ActThs,
    ActDur,
}

mod act_ths {
    use super::{Register, Param};
    
    fn from_params(params: &Vec<Param>) -> Result<Register,()> {
        unimplemented!();
    }
}

mod act_dur {
    use super::{Register, Param};

    fn from_params(params: &Vec<Param>) -> Result<Register,()> {
        unimplemented!();
    }
}


fn reg1_to_params(reg: Register) -> Result<Vec<Param>,()> {
    unimplemented!();
}

/// `ConfigBuilder` is use to create a partial or total new configuration of
/// a *LSM9DS1*.
/// 
/// # Examples
///
/// ```
/// use lsm9ds1::config::{ConfigBuilder, Param};
/// 
/// let conf1 = ConfigBuilder::new()
///     .set(Param::P1)
///     .set(Param::P2)
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
    pub fn set_all<'a>(&'a mut self, params: &Vec<Param>) -> &'a mut ConfigBuilder {
        self.params.extend(params);
        self
    }

    /// Build a `Config` from a `ConfigBuilder`.
    ///
    /// # Errors
    ///
    /// `build()` may failed if:
    ///
    /// * 2 parameters have the same type but differents values or;
    ///
    /// * if a parameter is not compatible with the *LSM9DS1*
    /// specification.
    ///
    /// # Example
    ///
    /// ```
    /// assert!(false);
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut hash_reg = HashMap::new();
        let mut hash_param = HashMap::new();
        for &p in &self.params {
            let old = hash_param.insert(type_of_param(p), p);
            match old {
                None => {}
                Some(old_p) => if old_p != p {return Err(())}
            }
            match p {
                Param::ActThs(_) => {
                    let params = hash_reg.entry(RegisterType::ActThs).or_insert(Vec::new());
                    params.push(p);
                }
                _ => unimplemented!(),
                // Param::P1 | Param::P2 | Param::P3(_) => {
                //     let params = hash_reg.entry(RegisterType::R1).or_insert(Vec::new());
                //     params.push(p);
                // }
            }
        }

        let mut registers = Vec::new();
        for (key, params) in hash_reg.iter() {
            unimplemented!();
            // match *key {
            //     RegisterType::R1 => registers.push(try!(reg1_from_params(params)))
            // }
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
/// let conf1 = Registers::new()
///     .set(Register::R1)
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
    pub fn set_all<'a>(&'a mut self, registers: &Vec<Register>) -> &'a mut Registers {
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
    /// assert!(false);
    /// ```
    pub fn build(&self) -> Result<Config,()> {
        let mut params = HashMap::new();
        for &reg in &self.registers {
            match reg {
                Register::ActThs(_) => {
                    for p in try!(reg1_to_params(reg)) {
                        params.insert(type_of_param(p), p);
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
/// use lsm9ds1::config::{ConfigBuilder, Param, Registers};
/// 
/// let conf1 = ConfigBuilder::new()
///     .set(Param::P1)
///     .set(Param::P2)
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
    /// use lsm9ds1::config::{ConfigBuilder, Param, ParamType};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::P1).set(Param::P2)
    ///     .build().unwrap();
    /// let &p = c.param(ParamType::P2).unwrap();
    /// assert_eq!(p, Param::P2);
    /// ```
    pub fn param(&self, param: ParamType) -> Option<&Param> {
        self.params.get(&param)
    }

    /// List all the parameters set for the current `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsm9ds1::config::{ConfigBuilder, Param};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::P1)
    ///     .set(Param::P2)
    ///     .build().unwrap();
    /// for &p in c.params() {
    ///     assert!(p == Param::P1 || p == Param::P2);
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
    /// use lsm9ds1::config::{ConfigBuilder, Param, Register};
    ///
    /// let c = ConfigBuilder::new()
    ///     .set(Param::P1)
    ///     .set(Param::P2)
    ///     .build().unwrap();
    /// for &r in c.registers() {
    ///     assert_eq!(r, Register::R1);
    /// }
    /// ```
    pub fn registers(&self) -> &Vec<Register> {
        &self.registers
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigBuilder, Registers, Param};
    // use params::Param;
    
    #[test]
    fn it_works() {
        let conf1 = ConfigBuilder::new()
            // .set(Param::P1)
            // .set(Param::P2)
            .build().unwrap();
        let conf2 = Registers::new().
            set_all(conf1.registers())
            .build().unwrap();

        assert_eq!(conf1, conf2);
        
            
    }
}
