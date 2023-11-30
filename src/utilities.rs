use super::*;
use std::fmt::{Display, Formatter};
use cpulib::{VecRegName, GPRName, u256, u512};

#[derive(Copy, Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(u256),
    U512(u512),
    F32(f32),
    F64(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::U8(x) => write!(f, "{}", x),
            Value::U16(x) => write!(f, "{}", x),
            Value::U32(x) => write!(f, "{}", x),
            Value::U64(x) => write!(f, "{}", x),
            Value::U128(x) => write!(f, "{}", x),
            Value::U256(x) => write!(f, "{}", x),
            Value::U512(x) => write!(f, "{}", x),
            Value::F32(x) => write!(f, "{}", x),
            Value::F64(x) => write!(f, "{}", x),
        }
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::U8(v) => match other {
                Value::U8(ov) => v == ov,
                _ => false,
            },
            Value::U16(v) => match other {
                Value::U16(ov) => v == ov,
                _ => false,
            },
            Value::U32(v) => match other {
                Value::U32(ov) => v == ov,
                _ => false,
            },
            Value::U64(v) => match other {
                Value::U64(ov) => v == ov,
                _ => false,
            },
            Value::U128(v) => match other {
                Value::U128(ov) => v == ov,
                _ => false,
            },
            Value::U256(v) => match other {
                Value::U256(ov) => v == ov,
                _ => false,
            },
            Value::U512(v) => match other {
                Value::U512(ov) => v == ov,
                _ => false,
            },
            Value::F32(v) => match other {
                Value::F32(ov) => v == ov,
                _ => false,
            },
            Value::F64(v) => match other {
                Value::F64(ov) => v == ov,
                _ => false,
            },
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

macro_rules! value_compare {
    ($t:ty, $tn:ident) => {
        impl PartialEq<$t> for Value {
            fn eq(&self, other: &$t) -> bool {
                match self {
                    Value::$tn(v) => v == other,
                    _ => false,
                }
            }
            fn ne(&self, other: &$t) -> bool {
                !self.eq(other)
            }
        }
    };
}

value_compare!(u8, U8);
value_compare!(u16, U16);
value_compare!(u32, U32);
value_compare!(u64, U64);
value_compare!(u128, U128);
value_compare!(u256, U256);
value_compare!(u512, U512);
value_compare!(f32, F32);
value_compare!(f64, F64);

impl Eq for Value {}

pub fn get_vec_reg_name(reg: &VecRegName, reg_index: &usize) -> String {
    format!("{}{}", reg, reg_index)
}

pub fn get_gpr_name(reg: &GPRName) -> String {
    format!("{}", reg)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum RegType {
    GPR, Vector, None
}

#[derive(Copy, Clone)]
pub struct Register {
    reg_type: RegType,
    gpr: GPRName,
    vector: (VecRegName, usize),
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.reg_type {
            RegType::GPR => format!("{}", self.gpr),
            RegType::Vector => format!("{}{}", self.vector.0, self.vector.1),
            RegType::None => "None".into(),
        })
    }
}

impl PartialEq<Self> for Register {
    fn eq(&self, other: &Self) -> bool {
        self.reg_type == other.reg_type && match self.reg_type {
            RegType::GPR => self.gpr == other.gpr,
            RegType::Vector => self.vector == other.vector,
            RegType::None => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<GPRName> for Register {
    fn eq(&self, other: &GPRName) -> bool {
        self.reg_type == RegType::GPR && self.gpr == *other
    }
    fn ne(&self, other: &GPRName) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<(VecRegName, usize)> for Register {
    fn eq(&self, other: &(VecRegName, usize)) -> bool {
        self.reg_type == RegType::Vector && self.vector == *other
    }
    fn ne(&self, other: &(VecRegName, usize)) -> bool {
        !self.eq(other)
    }
}

impl Eq for Register {}

impl Register {
    pub fn none() -> Self {
        Self {
            reg_type: RegType::None,
            gpr: GPRName::RAX,
            vector: (VecRegName::XMM, 0),
        }
    }
    pub fn vector(name: VecRegName, index: usize) -> Self {
        Self {
            reg_type: RegType::Vector,
            gpr: GPRName::RAX,
            vector: (name, index),
        }
    }
    pub fn gpr(name: GPRName) -> Self {
        Self {
            reg_type: RegType::GPR,
            gpr: name,
            vector: (VecRegName::XMM, 0),
        }
    }
}
