#![allow(dead_code)]
use std::collections::HashMap;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Param {
    P1,
    P2,
    P3(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParamType {
    P1,
    P2,
    P3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    R1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RegisterType {
    R1
}

fn reg1_from_params(params: &Vec<Param>) -> Result<Register,()> {
    Ok(Register::R1)
}

fn reg1_to_params(reg: Register) -> Result<Vec<Param>,()> {
    Ok(vec![Param::P1, Param::P2])
}

#[derive(Clone, Debug)]
pub struct ConfigBuilder {
    params: Vec<Param>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            params: Vec::new(),
        }
    }

    pub fn set<'a>(&'a mut self, param: Param) -> &'a mut ConfigBuilder {
        self.params.push(param);
        self
    }

    pub fn set_all<'a>(&'a mut self, params: &Vec<Param>) -> &'a mut ConfigBuilder {
        self.params.extend(params);
        self
    }

    pub fn build(&self) -> Result<Config,()> {
        let mut hash_reg = HashMap::new();
        for &p in &self.params {
            match p {
                Param::P1 | Param::P2 | Param::P3(_) => {
                    let params = hash_reg.entry(RegisterType::R1).or_insert(Vec::new());
                    params.push(p);
                }
            }
        }

        let mut registers = Vec::new();
        for (key, params) in hash_reg.iter() {
            match *key {
                RegisterType::R1 => registers.push(try!(reg1_from_params(params)))
            }
        }

        Ok(Config {
            params: self.params.clone(),
            registers: registers,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Registers {
    registers: Vec<Register>,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: Vec::new(),
        }
    }

    pub fn set<'a>(&'a mut self, register: Register) -> &'a mut Registers {
        self.registers.push(register);
        self
    }

    pub fn set_all<'a>(&'a mut self, registers: &Vec<Register>) -> &'a mut Registers {
        self.registers.extend(registers);
        self
    }

    pub fn build(&self) -> Result<Config,()> {
        let mut params = Vec::new();
        for &reg in &self.registers {
            match reg {
                Register::R1 => params.extend(try!(reg1_to_params(reg))),
            }
        }
        Ok(Config {
            params: params,
            registers: self.registers.clone(),
        })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    params: Vec<Param>,
    registers: Vec<Register>
}

impl Config {
    pub fn registers(&self) -> &Vec<Register> {
        &self.registers
    }

    pub fn params(&self) -> &Vec<Param> {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigBuilder, Param, Registers};
    
    #[test]
    fn it_works() {
        let conf1 = ConfigBuilder::new()
            .set(Param::P1)
            .set(Param::P2)
            .build().unwrap();
        let conf2 = Registers::new().
            set_all(conf1.registers())
            .build().unwrap();

        assert_eq!(conf1, conf2);
        
            
    }
}
