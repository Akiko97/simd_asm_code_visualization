use super::*;
use std::fmt::{Display, Formatter};
use cpulib::{VecRegName, GPRName, u256, u512};

#[derive(Clone, Copy)]
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

pub fn get_size_from_value(value: &Value) -> Vec2 {
    match value {
        Value::U8(_) => {Vec2::new(15.0, 25.0)}
        Value::U16(_) => {Vec2::new(30.0, 25.0)}
        Value::U32(_) => {Vec2::new(60.0, 25.0)}
        Value::U64(_) => {Vec2::new(120.0, 25.0)}
        Value::U128(_) => {Vec2::new(240.0, 25.0)}
        Value::U256(_) => {Vec2::new(480.0, 25.0)}
        Value::U512(_) => {Vec2::new(920.0, 25.0)}
        Value::F32(_) => {Vec2::new(60.0, 25.0)}
        Value::F64(_) => {Vec2::new(120.0, 25.0)}
    }
}

pub fn get_vec_reg_name(reg: &VecRegName, reg_index: &usize) -> String {
    match reg {
        VecRegName::XMM => {format!("XMM{}", reg_index)}
        VecRegName::YMM => {format!("YMM{}", reg_index)}
        VecRegName::ZMM => {format!("ZMM{}", reg_index)}
    }
}

pub fn get_gpr_name(reg: &GPRName) -> String {
    match reg {
        GPRName::RAX => String::from("RAX"),
        GPRName::RBX => String::from("RBX"),
        GPRName::RCX => String::from("RCX"),
        GPRName::RDX => String::from("RDX"),
        GPRName::RSI => String::from("RSI"),
        GPRName::RDI => String::from("RDI"),
        GPRName::RBP => String::from("RBP"),
        GPRName::RSP => String::from("RSP"),
        GPRName::R8 => String::from("R8"),
        GPRName::R9 => String::from("R9"),
        GPRName::R10 => String::from("R10"),
        GPRName::R11 => String::from("R11"),
        GPRName::R12 => String::from("R12"),
        GPRName::R13 => String::from("R13"),
        GPRName::R14 => String::from("R14"),
        GPRName::R15 => String::from("R15"),
        GPRName::EAX => String::from("EAX"),
        GPRName::EBX => String::from("EBX"),
        GPRName::ECX => String::from("ECX"),
        GPRName::EDX => String::from("EDX"),
        GPRName::ESI => String::from("ESI"),
        GPRName::EDI => String::from("EDI"),
        GPRName::EBP => String::from("EBP"),
        GPRName::ESP => String::from("ESP"),
        GPRName::R8D => String::from("R8D"),
        GPRName::R9D => String::from("R9D"),
        GPRName::R10D => String::from("R10D"),
        GPRName::R11D => String::from("R11D"),
        GPRName::R12D => String::from("R12D"),
        GPRName::R13D => String::from("R13D"),
        GPRName::R14D => String::from("R14D"),
        GPRName::R15D => String::from("R15D"),
        GPRName::AX => String::from("AX"),
        GPRName::BX => String::from("BX"),
        GPRName::CX => String::from("CX"),
        GPRName::DX => String::from("DX"),
        GPRName::SI => String::from("SI"),
        GPRName::DI => String::from("DI"),
        GPRName::BP => String::from("BP"),
        GPRName::SP => String::from("SP"),
        GPRName::R8W => String::from("R8W"),
        GPRName::R9W => String::from("R9W"),
        GPRName::R10W => String::from("R10W"),
        GPRName::R11W => String::from("R11W"),
        GPRName::R12W => String::from("R12W"),
        GPRName::R13W => String::from("R13W"),
        GPRName::R14W => String::from("R14W"),
        GPRName::R15W => String::from("R15W"),
        GPRName::AH => String::from("AH"),
        GPRName::BH => String::from("BH"),
        GPRName::CH => String::from("CH"),
        GPRName::DH => String::from("DH"),
        GPRName::AL => String::from("AL"),
        GPRName::BL => String::from("BL"),
        GPRName::CL => String::from("CL"),
        GPRName::DL => String::from("DL"),
        GPRName::SIL => String::from("SIL"),
        GPRName::DIL => String::from("DIL"),
        GPRName::BPL => String::from("BPL"),
        GPRName::SPL => String::from("SPL"),
        GPRName::R8B => String::from("R8B"),
        GPRName::R9B => String::from("R9B"),
        GPRName::R10B => String::from("R10B"),
        GPRName::R11B => String::from("R11B"),
        GPRName::R12B => String::from("R12B"),
        GPRName::R13B => String::from("R13B"),
        GPRName::R14B => String::from("R14B"),
        GPRName::R15B => String::from("R15B"),
    }
}
