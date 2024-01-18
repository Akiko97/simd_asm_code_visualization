use std::collections::{HashMap};
use lazy_static::lazy_static;
use std::convert::Into;
use std::ops::{Add, Mul};
use std::sync::{Arc, Mutex};
use cpulib::{CPU, VecRegName, GPRName, SectionCompatible, u256, u512, FLAGSName};
use cpulib::Utilities;
use eframe::egui::Context;
use crate::animation_fsm::{AnimationFSM, FSMCtrlMsg};
use crate::reg_visualizer::{LayoutLocation, RegVisualizer};
use crate::reg_visualizer_data::RegVisualizerData;
use crate::utilities::{create_value, create_values, Register, RegType, Value, ValueType};
use crate::{add_animation_data, vec_reg, ElementAnimationData, add_register_group_animation_data, gpr};
use regex::Regex;

fn split_instruction(instruction: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = instruction.splitn(2, ' ').collect();
    if parts.is_empty() {
        return (String::from(""), Vec::new());
    }
    let opcode = parts[0];
    let operands = if parts.len() > 1 {
        parts[1].split(',').map(|op| op.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    (opcode.into(), operands)
}

#[derive(Clone, Eq, PartialEq)]
enum Operand {
    Reg(Register),
    Mem(u64),
    Imm(u64),
}

fn get_gpr(str: String) -> Register {
    match str.as_str() {
        "RAX" => gpr!(RAX),
        "RBX" => gpr!(RBX),
        "RCX" => gpr!(RCX),
        "RDX" => gpr!(RDX),
        "RSI" => gpr!(RSI),
        "RDI" => gpr!(RDI),
        "RBP" => gpr!(RBP),
        "RSP" => gpr!(RSP),
        "R8" => gpr!(R8),
        "R9" => gpr!(R9),
        "R10" => gpr!(R10),
        "R11" => gpr!(R11),
        "R12" => gpr!(R12),
        "R13" => gpr!(R13),
        "R14" => gpr!(R14),
        "R15" => gpr!(R15),
        "EAX" => gpr!(EAX),
        "EBX" => gpr!(EBX),
        "ECX" => gpr!(ECX),
        "EDX" => gpr!(EDX),
        "ESI" => gpr!(ESI),
        "EDI" => gpr!(EDI),
        "EBP" => gpr!(EBP),
        "ESP" => gpr!(ESP),
        "R8D" => gpr!(R8D),
        "R9D" => gpr!(R9D),
        "R10D" => gpr!(R10D),
        "R11D" => gpr!(R11D),
        "R12D" => gpr!(R12D),
        "R13D" => gpr!(R13D),
        "R14D" => gpr!(R14D),
        "R15D" => gpr!(R15D),
        "AX" => gpr!(AX),
        "BX" => gpr!(BX),
        "CX" => gpr!(CX),
        "DX" => gpr!(DX),
        "SI" => gpr!(SI),
        "DI" => gpr!(DI),
        "BP" => gpr!(BP),
        "SP" => gpr!(SP),
        "R8W" => gpr!(R8W),
        "R9W" => gpr!(R9W),
        "R10W" => gpr!(R10W),
        "R11W" => gpr!(R11W),
        "R12W" => gpr!(R12W),
        "R13W" => gpr!(R13W),
        "R14W" => gpr!(R14W),
        "R15W" => gpr!(R15W),
        "AH" => gpr!(AH),
        "BH" => gpr!(BH),
        "CH" => gpr!(CH),
        "DH" => gpr!(DH),
        "AL" => gpr!(AL),
        "BL" => gpr!(BL),
        "CL" => gpr!(CL),
        "DL" => gpr!(DL),
        "SIL" => gpr!(SIL),
        "DIL" => gpr!(DIL),
        "BPL" => gpr!(BPL),
        "SPL" => gpr!(SPL),
        "R8B" => gpr!(R8B),
        "R9B" => gpr!(R9B),
        "R10B" => gpr!(R10B),
        "R11B" => gpr!(R11B),
        "R12B" => gpr!(R12B),
        "R13B" => gpr!(R13B),
        "R14B" => gpr!(R14B),
        "R15B" => gpr!(R15B),
        _ => panic!("Invalid GPR"),
    }
}

fn create_operands(operands: Vec<String>, cpu: Arc<Mutex<CPU>>) -> Vec<Operand> {
    let mut operand_vec = vec![];
    operands.iter().for_each(|operand| {
        let operand = operand.to_uppercase();
        if operand.starts_with('[') && operand.ends_with(']') {
            // Memory: reg + offset, TODO: support full memory format
            let address = &operand[1..operand.len() - 1];
            let mut address: String = address.into();
            address = address.replace(" ", "");
            address = address.to_uppercase();
            let re = Regex::new(r"(0X[0-9a-fA-F]+|\d+|[a-zA-Z]+|[+\-*/])").unwrap();
            let parts: Vec<&str> = re.find_iter(&*address).map(|m| m.as_str()).collect();
            if parts.len() != 3 {
                println!("Unsupported address format: {}", address);
            }
            let (number_str, operator, register_str) = if parts[0].chars().next().unwrap().is_digit(10) || &parts[0][0..2] == "0x" {
                (parts[0], parts[1], parts[2])
            } else {
                (parts[2], parts[1], parts[0])
            };
            let number = if number_str.starts_with("0X") {
                u64::from_str_radix(&number_str[2..], 16).expect("Invalid hex number")
            } else {
                number_str.parse::<u64>().expect("Invalid number")
            };
            let register = get_gpr(register_str.into());
            let mut cpu = cpu.lock().unwrap();
            let reg_value = cpu.registers.get_gpr_value(register.get_gpr());
            let result = match operator {
                "+" => number + reg_value,
                "-" => number - reg_value,
                "*" => number * reg_value,
                "/" => number / reg_value,
                _ => 0u64,
            };
            operand_vec.push(Operand::Mem(result));
        } else if operand.chars().all(|c| c.is_ascii_hexdigit() || c == 'X' || c == 'x') {
            // Immediate
            let imm = if operand.starts_with("0X") || operand.starts_with("0x") {
                u64::from_str_radix(&operand[2..], 16).expect("Invalid immediate")
            } else if operand.starts_with("0B") || operand.starts_with("0b") {
                u64::from_str_radix(&operand[2..], 2).expect("Invalid immediate")
            } else {
                u64::from_str_radix(operand.as_str(), 10).expect("Invalid immediate")
            };
            operand_vec.push(Operand::Imm(imm));
        } else if operand.starts_with("XMM") || operand.starts_with("YMM") || operand.starts_with("ZMM") {
            // Vector Register
            let reg_name = &operand[..3];
            let index = operand[3..].parse::<usize>().expect("Invalid Vector Register Index");
            let reg = match reg_name {
                "XMM" => vec_reg!(XMM, index),
                "YMM" => vec_reg!(YMM, index),
                "ZMM" => vec_reg!(ZMM, index),
                _ => panic!("Invalid Vector Register"),
            };
            operand_vec.push(Operand::Reg(reg));
        } else {
            // GPR
            let reg = get_gpr(operand);
            operand_vec.push(Operand::Reg(reg));
        }
    });
    operand_vec
}

trait FloatCalc {
    fn fadd(self, other: Self) -> Self;
    fn fmul(self, other: Self) -> Self;
}
impl FloatCalc for u32 {
    fn fadd(self, other: Self) -> Self {
        Utilities::f32_to_u32(Utilities::u32_to_f32(self) + Utilities::u32_to_f32(other))
    }
    fn fmul(self, other: Self) -> Self {
        Utilities::f32_to_u32(Utilities::u32_to_f32(self) * Utilities::u32_to_f32(other))
    }
}
impl FloatCalc for u64 {
    fn fadd(self, other: Self) -> Self {
        Utilities::f64_to_u64(Utilities::u64_to_f64(self) + Utilities::u64_to_f64(other))
    }
    fn fmul(self, other: Self) -> Self {
        Utilities::f64_to_u64(Utilities::u64_to_f64(self) * Utilities::u64_to_f64(other))
    }
}
macro_rules! fake_float_calc {
    ($ty:ident) => {
        impl FloatCalc for $ty {
            fn fadd(self, other: Self) -> Self {
                self + other
            }
            fn fmul(self, other: Self) -> Self {
                self * other
            }
        }
    };
}
fake_float_calc!(u8);
fake_float_calc!(u16);
fake_float_calc!(u128);
fake_float_calc!(u256);
fake_float_calc!(u512);

macro_rules! create_calc_common {
    ($func_name:ident, $calc:ident, $fcalc:ident) => {
        fn $func_name<T>(cpu: Arc<Mutex<CPU>>, _vrt: HashMap<(VecRegName, usize), ValueType>, target: Operand, source1: Operand, source2: Operand, is_float: bool)
            where Vec<T>: FromIterator<<T as Add>::Output>, Vec<T>: FromIterator<<T as Mul>::Output>, T: SectionCompatible + Add + FloatCalc + Mul
        {
            match target {
                Operand::Reg(dst) => {
                    if let (Operand::Reg(src1), Operand::Reg(src2)) = (source1.clone(), source2.clone()) {
                        if !(dst.get_type() == src1.get_type() && dst.get_type() == src2.get_type()) { return; }
                        if dst.get_type() == RegType::GPR {
                            let mut cpu = cpu.lock().unwrap();
                            let result = if is_float {
                                cpu.registers.get_gpr_value(src1.get_gpr()).$fcalc(cpu.registers.get_gpr_value(src2.get_gpr()))
                            } else {
                                cpu.registers.get_gpr_value(src1.get_gpr()).$calc(cpu.registers.get_gpr_value(src2.get_gpr()))
                            };
                            cpu.registers.set_gpr_value(dst.get_gpr(), result);
                        } else if dst.get_type() == RegType::Vector && dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
                            let mut cpu = cpu.lock().unwrap();
                            let mut a: Vec<T> = cpu.registers.get_by_sections::<T>(src1.get_vector().0, src1.get_vector().1).unwrap();
                            let mut b: Vec<T> = cpu.registers.get_by_sections::<T>(src2.get_vector().0, src2.get_vector().1).unwrap();
                            let result = if is_float {
                                a.iter().zip(b.iter()).map(|(x, y)| { (*x).$fcalc(*y) }).collect()
                            } else {
                                a.iter().zip(b.iter()).map(|(x, y)| { (*x).$calc(*y) }).collect()
                            };
                            cpu.registers.set_by_sections::<T>(dst.get_vector().0, dst.get_vector().1, result);
                        }
                    } else if let (Operand::Reg(src1), Operand::Imm(src2)) = (source1.clone(), source2.clone()) {
                        if dst.get_type() == src1.get_type() && dst.get_type() == RegType::GPR {
                            let mut cpu = cpu.lock().unwrap();
                            let result = if is_float {
                                cpu.registers.get_gpr_value(src1.get_gpr()).$fcalc(src2)
                            } else {
                                cpu.registers.get_gpr_value(src1.get_gpr()).$calc(src2)
                            };
                            cpu.registers.set_gpr_value(dst.get_gpr(), result);
                        }
                    } else if let (Operand::Reg(_src1), Operand::Mem(_src2)) = (source1.clone(), source2.clone()) {
                        todo!()
                    }
                }
                _ => panic!("Invalid Instruction"),
            }
        }
    };
}

create_calc_common!(add_common, add, fadd);
create_calc_common!(mul_common, mul, fmul);

fn vaddps(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    add_common::<u32>(cpu, vrt, operands[0].clone(), operands[1].clone(), operands[2].clone(), true);
}

fn vpaddd(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    add_common::<u32>(cpu, vrt, operands[0].clone(), operands[1].clone(), operands[2].clone(), false);
}

fn vmulpd(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    mul_common::<u64>(cpu, vrt, operands[0].clone(), operands[1].clone(), operands[2].clone(), true);
}

fn add(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    add_common::<u64>(cpu, vrt, operands[0].clone(), operands[1].clone(), operands[2].clone(), false);
}

fn valignd(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 4 { return; }
    let target = operands[0].clone();
    let source1 = operands[1].clone();
    let source2 = operands[2].clone();
    let imm8 = operands[3].clone();
    if let (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target, source1, source2, imm8) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector {
            let mut cpu = cpu.lock().unwrap();
            let mut vec1 = cpu.registers.get_by_sections::<u32>(src1.get_vector().0, src1.get_vector().1).unwrap();
            let mut vec2 = cpu.registers.get_by_sections::<u32>(src2.get_vector().0, src2.get_vector().1).unwrap();
            let mut veca = vec2.clone();
            veca.append(&mut vec1);
            let mut veca = Vec::from(&veca[imm8 as usize..]);
            if veca.len() > 16 {
                veca = Vec::from(&veca[..16]);
            } else {
                veca.resize(16, 0);
            }
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, veca);
        }
    }
}

fn unpack_common(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>, is_high: bool) {
    let target = operands[0].clone();
    let source1 = operands[1].clone();
    let source2 = operands[2].clone();
    if let (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2))
        = (target, source1, source2) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let mut cpu = cpu.lock().unwrap();
            let interleave_dwords = if !is_high {
                |src1: Vec<u32>, src2: Vec<u32>| -> Vec<u32> {
                    let mut dst: Vec<u32> = vec![];
                    dst.push(src1[0]);
                    dst.push(src2[0]);
                    dst.push(src1[1]);
                    dst.push(src2[1]);
                    dst
                }
            } else {
                |src1: Vec<u32>, src2: Vec<u32>| -> Vec<u32> {
                    let mut dst: Vec<u32> = vec![];
                    dst.push(src1[2]);
                    dst.push(src2[2]);
                    dst.push(src1[3]);
                    dst.push(src2[3]);
                    dst
                }
            };
            let v1 = cpu.registers.get_by_sections::<u32>(src1.get_vector().0, src1.get_vector().1).unwrap();
            let v2 = cpu.registers.get_by_sections::<u32>(src2.get_vector().0, src2.get_vector().1).unwrap();
            let vs = match dst.get_vector().0 {
                VecRegName::XMM => {
                    interleave_dwords(v1, v2)
                },
                VecRegName::YMM => {
                    let mut tmp = vec![];
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[0..4]), Vec::from(&v2[0..4])));
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[4..8]), Vec::from(&v2[4..8])));
                    tmp
                },
                VecRegName::ZMM => {
                    let mut tmp = vec![];
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[0..4]), Vec::from(&v2[0..4])));
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[4..8]), Vec::from(&v2[4..8])));
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[8..12]), Vec::from(&v2[8..12])));
                    tmp.append(&mut interleave_dwords(Vec::from(&v1[12..16]), Vec::from(&v2[12..16])));
                    tmp
                },
            };
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, vs);
        }
    }
}

fn vunpcklps(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    unpack_common(cpu, operands, vrt, false);
}

fn vunpckhps(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    unpack_common(cpu, operands, vrt, true);
}

fn vshufps(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 4 { return; }
    let target = operands[0].clone();
    let source1 = operands[1].clone();
    let source2 = operands[2].clone();
    let imm = operands[3].clone();
    if let (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target, source1, source2, imm) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let mut cpu = cpu.lock().unwrap();
            let mut imm8v = vec![];
            [0u8, 2, 4, 6].iter().for_each(|i| {
                let tmp = imm8 as u8 >> i;
                imm8v.push((tmp & 0b00000011) as usize);
            });
            let s1v = cpu.registers.get_by_sections::<u32>(src1.get_vector().0, src1.get_vector().1).unwrap();
            let s2v = cpu.registers.get_by_sections::<u32>(src2.get_vector().0, src2.get_vector().1).unwrap();
            let mut dv = vec![];
            match dst.get_vector().0 {
                VecRegName::XMM => {todo!()}
                VecRegName::YMM => {
                    dv.push(s1v[imm8v[0]]);
                    dv.push(s1v[imm8v[1]]);
                    dv.push(s2v[imm8v[2]]);
                    dv.push(s2v[imm8v[3]]);
                    dv.push(s1v[4 + imm8v[0]]);
                    dv.push(s1v[4 + imm8v[1]]);
                    dv.push(s2v[4 + imm8v[2]]);
                    dv.push(s2v[4 + imm8v[3]]);
                }
                VecRegName::ZMM => {todo!()}
            }
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, dv);
        }
    }
}

fn vperm2f128(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 4 { return; }
    let target = operands[0].clone();
    let source1 = operands[1].clone();
    let source2 = operands[2].clone();
    let imm = operands[3].clone();
    if let (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target, source1, source2, imm) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let mut cpu = cpu.lock().unwrap();
            let v1 = cpu.registers.get_by_sections::<u128>(src1.get_vector().0, src1.get_vector().1).unwrap();
            let v2 = cpu.registers.get_by_sections::<u128>(src2.get_vector().0, src2.get_vector().1).unwrap();
            let mut dv = vec![];
            let select4 = |src1: &Vec<u128>, src2: &Vec<u128>, control: u8| -> u128 {
                if (control >> 3) & 0b00000001 == 1 {
                    return 0u128;
                }
                match control & 0b00000011 {
                    0 => src1[0],
                    1 => src1[1],
                    2 => src2[0],
                    3 => src2[1],
                    _ => 0u128
                }
            };
            dv.push(select4(&v1, &v2, imm8 as u8 & 0b00001111));
            dv.push(select4(&v1, &v2, (imm8 as u8 >> 4) & 0b00001111));
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, dv);
        }
    }
}

fn vextractf128(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 3 { return; }
    let target = operands[0].clone();
    let source = operands[1].clone();
    let imm = operands[2].clone();
    if let (Operand::Reg(dst), Operand::Reg(src), Operand::Imm(imm8)) =
        (target, source, imm) {
        if dst.get_type() == RegType::Vector && dst.get_vector().0 == VecRegName::XMM &&
            src.get_type() == RegType::Vector && src.get_vector().0 == VecRegName::YMM {
            let mut cpu = cpu.lock().unwrap();
            let sv = cpu.registers.get_by_sections::<u128>(src.get_vector().0, src.get_vector().1).unwrap();
            let mut dv = vec![];
            if imm8 as u8 & 0b00000001 == 1 {
                dv.push(sv[1]);
            } else {
                dv.push(sv[0]);
            }
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, dv);
        }
    }
}

fn shufpd(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 4 { return; }
    let target = operands[0].clone();
    let source1 = operands[1].clone();
    let source2 = operands[2].clone();
    let imm = operands[3].clone();
    if let (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8)) =
        (target, source1, source2, imm) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == VecRegName::XMM && dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let mut cpu = cpu.lock().unwrap();
            let v1 = cpu.registers.get_by_sections::<u64>(src1.get_vector().0, src1.get_vector().1).unwrap();
            let v2 = cpu.registers.get_by_sections::<u64>(src2.get_vector().0, src2.get_vector().1).unwrap();
            let mut dv = vec![];
            if imm8 as u8 & 0b00000001 == 0 {
                dv.push(v1[0]);
            } else {
                dv.push(v1[1]);
            }
            if (imm8 as u8 >> 1) & 0b00000001 == 0 {
                dv.push(v2[0]);
            } else {
                dv.push(v2[1]);
            }
            cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, dv);
        }
    }
}

fn mov_common(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, _vrt: HashMap<(VecRegName, usize), ValueType>) {
    if operands.len() != 2 { return; }
    let target = operands[0].clone();
    let source = operands[1].clone();
    match target {
        Operand::Reg(dst) => {
            match source {
                Operand::Reg(src) => {
                    // reg -> reg
                    if dst.get_type() == src.get_type() {
                        if dst.get_type() == RegType::GPR {
                            // gpr -> gpr
                            let mut cpu = cpu.lock().unwrap();
                            let v = cpu.registers.get_gpr_value(src.get_gpr());
                            cpu.registers.set_gpr_value(dst.get_gpr(), v);
                        } else if dst.get_type() == RegType::Vector && dst.get_vector().0 == src.get_vector().0 {
                            // vector -> vector
                            let mut cpu = cpu.lock().unwrap();
                            match dst.get_vector().0 {
                                VecRegName::XMM => {
                                    let v = cpu.registers.get_by_sections::<u128>(src.get_vector().0, src.get_vector().1).unwrap();
                                    cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, v);
                                }
                                VecRegName::YMM => {
                                    let v = cpu.registers.get_by_sections::<u256>(src.get_vector().0, src.get_vector().1).unwrap();
                                    cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, v);
                                }
                                VecRegName::ZMM => {
                                    let v = cpu.registers.get_by_sections::<u512>(src.get_vector().0, src.get_vector().1).unwrap();
                                    cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, v);
                                }
                            }
                        }
                    }
                }
                Operand::Mem(src) => {
                    // mem -> reg
                    let mut cpu = cpu.lock().unwrap();
                    if dst.get_type() == RegType::GPR {
                        // mem -> gpr
                        let v = cpu.memory.read::<u64>(src as usize);
                        cpu.registers.set_gpr_value(dst.get_gpr(), v);
                    } else if dst.get_type() == RegType::Vector {
                        // mem -> vector
                        match dst.get_vector().0 {
                            VecRegName::XMM => {
                                let v = cpu.memory.read::<u128>(src as usize);
                                cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, vec![v]);
                            }
                            VecRegName::YMM => {
                                let v = cpu.memory.read::<u256>(src as usize);
                                cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, vec![v]);
                            }
                            VecRegName::ZMM => {
                                let v = cpu.memory.read::<u512>(src as usize);
                                cpu.registers.set_by_sections(dst.get_vector().0, dst.get_vector().1, vec![v]);
                            }
                        }
                    }
                }
                Operand::Imm(src) => {
                    // imm -> reg(gpr)
                    if dst.get_type() == RegType::GPR {
                        let mut cpu = cpu.lock().unwrap();
                        cpu.registers.set_gpr_value(dst.get_gpr(), src);
                    }
                }
            }
        }
        Operand::Mem(dst) => {
            if let Operand::Reg(src) = source {
                // reg -> mem
                if src.get_type() == RegType::GPR {
                    // gpr -> mem
                    let mut cpu = cpu.lock().unwrap();
                    let v = cpu.registers.get_gpr_value(src.get_gpr());
                    cpu.memory.write::<u64>(dst as usize, v);
                } else if src.get_type() == RegType::Vector {
                    // vector -> mem
                    let mut cpu = cpu.lock().unwrap();
                    match src.get_vector().0 {
                        VecRegName::XMM => {
                            let v = cpu.registers.get_by_sections::<u128>(src.get_vector().0, src.get_vector().1).unwrap();
                            cpu.memory.write::<u128>(dst as usize, v[0]);
                        }
                        VecRegName::YMM => {
                            let v = cpu.registers.get_by_sections::<u256>(src.get_vector().0, src.get_vector().1).unwrap();
                            cpu.memory.write::<u256>(dst as usize, v[0]);
                        }
                        VecRegName::ZMM => {
                            let v = cpu.registers.get_by_sections::<u512>(src.get_vector().0, src.get_vector().1).unwrap();
                            cpu.memory.write::<u512>(dst as usize, v[0]);
                        }
                    }
                }
            }
        }
        _ => {/*ERROR*/}
    }
}

fn vmovapd(cpu: Arc<Mutex<CPU>>, operands: Vec<Operand>, vrt: HashMap<(VecRegName, usize), ValueType>) {
    mov_common(cpu, operands, vrt);
}

fn get_values_from_register(reg: Register, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<Value> {
    match reg.get_type() {
        RegType::GPR => {
            let cpu = cpu.lock().unwrap();
            vec![create_value(cpu.registers.get_gpr_value(reg.get_gpr()))]
        }
        RegType::Vector => {
            let cpu = cpu.lock().unwrap();
            match vrt.get(&reg.get_vector()).unwrap() {
                ValueType::U8 => create_values(cpu.registers.get_by_sections::<u8>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U16 => create_values(cpu.registers.get_by_sections::<u16>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U32 => create_values(cpu.registers.get_by_sections::<u32>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U64 => create_values(cpu.registers.get_by_sections::<u64>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U128 => create_values(cpu.registers.get_by_sections::<u128>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U256 => create_values(cpu.registers.get_by_sections::<u256>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::U512 => create_values(cpu.registers.get_by_sections::<u512>(reg.get_vector().0, reg.get_vector().1).unwrap()),
                ValueType::F32 => create_values(Utilities::u32vec_to_f32vec(cpu.registers.get_by_sections::<u32>(reg.get_vector().0, reg.get_vector().1).unwrap())),
                ValueType::F64 => create_values(Utilities::u64vec_to_f64vec(cpu.registers.get_by_sections::<u64>(reg.get_vector().0, reg.get_vector().1).unwrap())),
            }
        }
        RegType::None => vec![],
    }
}

macro_rules! create_calc_common_animation {
    ($func_name:ident, $calc:literal) => {
        fn $func_name(cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>,
                                target: (Operand, LayoutLocation, (usize, usize)),
                                source1: (Operand, LayoutLocation, (usize, usize)),
                                source2: (Operand, LayoutLocation, (usize, usize))) -> Vec<(Vec<ElementAnimationData>, bool)>
        {
            return match target.0 {
                Operand::Reg(tgt) => {
                    if let (Operand::Reg(src1), Operand::Reg(src2)) = (source1.0.clone(), source2.0.clone()) {
                        if src1.get_type() == src2.get_type() {
                            let s1v = get_values_from_register(src1, cpu.clone(), vrt.clone());
                            let s2v = get_values_from_register(src2, cpu.clone(), vrt.clone());
                            let s1l = s1v.len();
                            let s2l = s2v.len();
                            let _l = s1l + s2l;
                            let mut v1 = vec![];
                            for i in 0..s1l {
                                add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, i,
                                    tgt, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                    |_| {});
                            }
                            let mut v2 = vec![];
                            for i in 0..s2l {
                                let s1v_c = s1v.clone();
                                let s2v_c = s2v.clone();
                                add_animation_data!(v2; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, i,
                                    tgt, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                    move |e| {e.set_string(format!("{} {} {}", s1v_c[i], $calc, s2v_c[i]))});
                            }
                            let mut v3 = vec![];
                            for i in 0..s2l {
                                add_animation_data!(v3;
                                    tgt, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                    tgt, LayoutLocation::None, 0, i, |_| {});
                            }
                            return vec![(v1, false), (v2, false), (v3, false)];
                        }
                    } else if let (Operand::Reg(src1), Operand::Imm(src2)) = (source1.0.clone(), source2.0.clone()) {
                        if src1.get_type() == RegType::GPR {
                            let mut v1 = vec![];
                            add_register_group_animation_data!(v1;
                                src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1},
                                tgt, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1},
                                move |e| {e.set_string(format!("{} {} {}", e.get_value(), $calc, src2))});
                            let mut v2 = vec![];
                            add_register_group_animation_data!(v2;
                                tgt, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1},
                                tgt, LayoutLocation::None, 0, |_| {});
                            return vec![(v1, false), (v2, false)];
                        }
                    } else if let (Operand::Reg(_src1), Operand::Mem(_src2)) = (source1.0.clone(), source2.0.clone()) {
                        todo!()
                    }
                    // Error
                    vec![(vec![], false)]
                }
                _ => vec![(vec![], false)],
            }
        }
    };
}

create_calc_common_animation!(add_common_animation, "+");
create_calc_common_animation!(mul_common_animation, "*");

fn vaddps_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    add_common_animation(cpu, vrt, odd[0].clone(), odd[1].clone(), odd[2].clone())
}

fn vpaddd_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    add_common_animation(cpu, vrt, odd[0].clone(), odd[1].clone(), odd[2].clone())
}

fn vmulpd_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    mul_common_animation(cpu, vrt, odd[0].clone(), odd[1].clone(), odd[2].clone())
}

fn add_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    add_common_animation(cpu, vrt, odd[0].clone(), odd[1].clone(), odd[2].clone())
}

fn valignd_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 4 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source1 = odd[1].clone();
    let source2 = odd[2].clone();
    let imm = odd[3].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target.0, source1.0, source2.0, imm.0) {
        if imm8 < 16 {
            let s1v = get_values_from_register(src1, cpu.clone(), vrt.clone());
            let s2v = get_values_from_register(src2, cpu.clone(), vrt.clone());
            if s1v.len() == 16 && s2v.len() == 16 {
                let _s1v = Vec::from(&s1v[..imm8 as usize]); // high in result
                let _s2v = Vec::from(&s2v[imm8 as usize..]); // low in result
                let mut v1 = vec![];
                let mut j = 0;
                for i in imm8 as usize..16 {
                    // s2v
                    add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, i,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, j,
                        |_| {});
                    j += 1;
                }
                for i in 0..imm8 as usize {
                    // s1v
                    add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, i,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, j,
                        |_| {});
                    j += 1;
                }
                let mut v2 = vec![];
                for i in 0..16 {
                    add_animation_data!(v2;
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                        dst, LayoutLocation::None, 0, i, |_| {});
                }
                return vec![(v1, false), (v2, false)];
            }
        }
    }
    vec![(vec![], false)]
}

fn unpack_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, _cpu: Arc<Mutex<CPU>>, _vrt: HashMap<(VecRegName, usize), ValueType>, is_high: bool) -> Vec<(Vec<ElementAnimationData>, bool)> {
    let target = odd[0].clone();
    let source1 = odd[1].clone();
    let source2 = odd[2].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2))
        = (target.0, source1.0, source2.0) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            match dst.get_vector().0 {
                VecRegName::XMM => {todo!()}
                VecRegName::YMM => {
                    let mut v1 = vec![];
                    let sv: Vec<usize> = if is_high {
                        vec![2, 2, 3, 3, 6, 6, 7, 7]
                    } else {
                        vec![0, 0, 1, 1, 4, 4, 5, 5]
                    };
                    (0usize..8).for_each(|i| {
                        if i % 2 == 0 {
                            add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, sv[i],
                                dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                |_| {});
                        } else {
                            add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, sv[i],
                                dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                |_| {});
                        }
                    });
                    let mut v2 = vec![];
                    for i in 0..8 {
                        add_animation_data!(v2;
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            dst, LayoutLocation::None, 0, i, |_| {});
                    }
                    return vec![(v1, false), (v2, false)];
                }
                VecRegName::ZMM => {todo!()}
            }
        }
    }
    vec![(vec![], false)]
}

fn vunpcklps_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    unpack_animation(odd, cpu, vrt, false)
}

fn vunpckhps_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    unpack_animation(odd, cpu, vrt, true)
}

fn vshufps_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, _cpu: Arc<Mutex<CPU>>, _vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 4 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source1 = odd[1].clone();
    let source2 = odd[2].clone();
    let imm = odd[3].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target.0, source1.0, source2.0, imm.0) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let mut imm8v = vec![];
            [0u8, 2, 4, 6].iter().for_each(|i| {
                let tmp = imm8 as u8 >> i;
                imm8v.push((tmp & 0b00000011) as usize);
            });
            match dst.get_vector().0 {
                VecRegName::XMM => {todo!()}
                VecRegName::YMM => {
                    let mut v1 = vec![];
                    add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, imm8v[0],
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 0,
                        |_| {});
                    add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, imm8v[1],
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 1,
                        |_| {});
                    add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, imm8v[2],
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 2,
                        |_| {});
                    add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, imm8v[3],
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 3,
                        |_| {});
                    add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, imm8v[0] + 4,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 4,
                        |_| {});
                    add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, imm8v[1] + 4,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 5,
                        |_| {});
                    add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, imm8v[2] + 4,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 6,
                        |_| {});
                    add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, imm8v[3] + 4,
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 7,
                        |_| {});
                    let mut v2 = vec![];
                    for i in 0..8 {
                        add_animation_data!(v2;
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            dst, LayoutLocation::None, 0, i, |_| {});
                    }
                    return vec![(v1, false), (v2, false)];
                }
                VecRegName::ZMM => {todo!()}
            }
        }
    }
    vec![(vec![], false)]
}

fn vperm2f128_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, _cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 4 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source1 = odd[1].clone();
    let source2 = odd[2].clone();
    let imm = odd[3].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target.0, source1.0, source2.0, imm.0) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            let size1 = vrt.get(&(src1.get_vector().0, src1.get_vector().1)).unwrap().size();
            let size2 = vrt.get(&(src2.get_vector().0, src2.get_vector().1)).unwrap().size();
            if size1 == size2 {
                let num = 128 / size1;
                match dst.get_vector().0 {
                    VecRegName::XMM => {todo!()}
                    VecRegName::YMM => {
                        let mut v1 = vec![];
                        let mut dst_i = 0;
                        [imm8 as u8 & 0b00001111, (imm8 as u8 >> 4) & 0b00001111].iter().for_each(|imm8| {
                            match imm8 {
                                0 => {
                                    (0..num).for_each(|src_i| {
                                        add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, src_i,
                                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, dst_i,
                                            |_| {});
                                        dst_i += 1;
                                    });
                                }
                                1 => {
                                    (num..num * 2).for_each(|src_i| {
                                        add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, src_i,
                                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, dst_i,
                                            |_| {});
                                        dst_i += 1;
                                    });
                                }
                                2 => {
                                    (0..num).for_each(|src_i| {
                                        add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, src_i,
                                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, dst_i,
                                            |_| {});
                                        dst_i += 1;
                                    });
                                }
                                3 => {
                                    (num..num * 2).for_each(|src_i| {
                                        add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, src_i,
                                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, dst_i,
                                            |_| {});
                                        dst_i += 1;
                                    });
                                }
                                _ if (imm8 >> 3) & 0b00000001 == 1 => {
                                    (0..num).for_each(|src_i| {
                                        add_animation_data!(v1; dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, src_i,
                                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, dst_i,
                                            |e| {e.set_string("0->".into())});
                                        dst_i += 1;
                                    });
                                }
                                _ => {}
                            }
                        });
                        let mut v2 = vec![];
                        for i in 0..8 {
                            add_animation_data!(v2;
                                dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                dst, LayoutLocation::None, 0, i, |_| {});
                        }
                        return vec![(v1, false), (v2, false)];
                    }
                    VecRegName::ZMM => {todo!()}
                }
            }
        }
    }
    vec![(vec![], false)]
}

fn vextractf128_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, _cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 3 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source = odd[1].clone();
    let imm = odd[2].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src), Operand::Imm(imm8))
        = (target.0, source.0, imm.0) {
        if dst.get_type() == RegType::Vector && src.get_type() == RegType::Vector &&
            dst.get_vector().0 == VecRegName::XMM && src.get_vector().0 == VecRegName::YMM {
            if vrt.get(&dst.get_vector()).unwrap() == vrt.get(&src.get_vector()).unwrap() {
                let t_size = vrt.get(&dst.get_vector()).unwrap().size();
                let xmm_num = 128 / t_size;
                let _ymm_num = 256 / t_size;
                let mut v1 = vec![];
                if imm8 as u8 & 0b00000001 == 1 {
                    (0..xmm_num).for_each(|i| {
                        add_animation_data!(v1; src, source.1, if source.1 == LayoutLocation::TOP {source.2.0} else {source.2.1}, i + xmm_num,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                } else {
                    (0..xmm_num).for_each(|i| {
                        add_animation_data!(v1; src, source.1, if source.1 == LayoutLocation::TOP {source.2.0} else {source.2.1}, i,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                }
                let mut v2 = vec![];
                for i in 0..xmm_num {
                    add_animation_data!(v2;
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                        dst, LayoutLocation::None, 0, i, |_| {});
                }
                return vec![(v1, false), (v2, false)];
            }
        }
    }
    vec![(vec![], false)]
}

fn shufpd_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, _cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 4 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source1 = odd[1].clone();
    let source2 = odd[2].clone();
    let imm = odd[3].clone();
    if let  (Operand::Reg(dst), Operand::Reg(src1), Operand::Reg(src2), Operand::Imm(imm8))
        = (target.0, source1.0, source2.0, imm.0) {
        if dst.get_type() == RegType::Vector && src1.get_type() == RegType::Vector && src2.get_type() == RegType::Vector &&
            dst.get_vector().0 == VecRegName::XMM && dst.get_vector().0 == src1.get_vector().0 && dst.get_vector().0 == src2.get_vector().0 {
            if vrt.get(&dst.get_vector()).unwrap() == vrt.get(&src1.get_vector()).unwrap() &&
                vrt.get(&dst.get_vector()).unwrap() == vrt.get(&src2.get_vector()).unwrap() {
                let size = vrt.get(&dst.get_vector()).unwrap().size();
                let num = 128 / size;
                let mut v1 = vec![];
                if imm8 as u8 & 0b00000001 == 0 {
                    (0..num / 2).for_each(|i| {
                        add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, i,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                } else {
                    (0..num / 2).for_each(|i| {
                        add_animation_data!(v1; src1, source1.1, if source1.1 == LayoutLocation::TOP {source1.2.0} else {source1.2.1}, i + num / 2,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                }
                if (imm8 as u8 >> 1) & 0b00000001 == 0 {
                    (num / 2..num).for_each(|i| {
                        add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, i - num / 2,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                } else {
                    (num / 2..num).for_each(|i| {
                        add_animation_data!(v1; src2, source2.1, if source2.1 == LayoutLocation::TOP {source2.2.0} else {source2.2.1}, i,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                            |_| {});
                    });
                }
                let mut v2 = vec![];
                for i in 0..num {
                    add_animation_data!(v2;
                        dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                        dst, LayoutLocation::None, 0, i, |_| {});
                }
                return vec![(v1, false), (v2, false)];
            }
        }
    }
    vec![(vec![], false)]
}

fn mov_common_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    if odd.len() != 2 { return vec![(vec![], false)]; }
    let target = odd[0].clone();
    let source = odd[1].clone();
    match target.0 {
        Operand::Reg(dst) => {
            match source.0 {
                Operand::Reg(src) => {
                    // reg -> reg
                    // TODO
                }
                Operand::Mem(src) => {
                    // mem -> reg
                    let cpu = cpu.lock().unwrap();
                    if dst.get_type() == RegType::GPR {
                        // mem -> gpr TODO
                        let v = cpu.memory.read::<u64>(src as usize);
                        let mut v1 = vec![];
                        add_animation_data!(v1; dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 0,
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 0,
                            move |e| {e.set_string(format!("{}->", v))});
                        let mut v2 = vec![];
                        add_animation_data!(v2;
                            dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, 0,
                            dst, LayoutLocation::None, 0, 0, |_| {});
                        return vec![(v1, false), (v2, false)];
                    } else if dst.get_type() == RegType::Vector {
                        // mem -> vector
                        let size = vrt.get(&dst.get_vector()).unwrap();
                        let num = match dst.get_vector().0 {
                            VecRegName::XMM => 128 / size.size(),
                            VecRegName::YMM => 256 / size.size(),
                            VecRegName::ZMM => 512 / size.size(),
                        };
                        let values = match size {
                            ValueType::U8 => create_values(cpu.memory.read_vec::<u8>(src as usize, num)),
                            ValueType::U16 => create_values(cpu.memory.read_vec::<u16>(src as usize, num)),
                            ValueType::U32 => create_values(cpu.memory.read_vec::<u32>(src as usize, num)),
                            ValueType::U64 => create_values(cpu.memory.read_vec::<u64>(src as usize, num)),
                            ValueType::U128 => create_values(cpu.memory.read_vec::<u128>(src as usize, num)),
                            ValueType::U256 => create_values(cpu.memory.read_vec::<u256>(src as usize, num)),
                            ValueType::U512 => create_values(cpu.memory.read_vec::<u512>(src as usize, num)),
                            ValueType::F32 => create_values(Utilities::u32vec_to_f32vec(cpu.memory.read_vec::<u32>(src as usize, num))),
                            ValueType::F64 => create_values(Utilities::u64vec_to_f64vec(cpu.memory.read_vec::<u64>(src as usize, num))),
                        };
                        let mut v1 = vec![];
                        values.iter().enumerate().for_each(|(i, v)| {
                            let v = v.clone();
                            add_animation_data!(v1; dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                move |e| {e.set_string(format!("{}->", v))});
                        });
                        let mut v2 = vec![];
                        (0..values.len()).for_each(|i| {
                            add_animation_data!(v2;
                                dst, target.1, if target.1 == LayoutLocation::TOP {target.2.0} else {target.2.1}, i,
                                dst, LayoutLocation::None, 0, i, |_| {});
                        });
                        return vec![(v1, false), (v2, false)];
                    }
                }
                Operand::Imm(src) => {
                    // imm -> reg(gpr)
                    // TODO
                }
            }
        }
        Operand::Mem(dst) => {
            // reg -> mem
            // if let Operand::Reg(src) = source.0 {
            //     if src.get_type() == RegType::GPR {
            //         // gpr -> mem
            //         // TODO
            //     } else if src.get_type() == RegType::Vector {
            //         // vector -> mem
            //         let values = get_values_from_register(src, cpu.clone(), vrt.clone());
            //         let mut v1 = vec![];
            //         (0..values.len()).for_each(|i| {
            //             add_animation_data!(v1; src, source.1, if source.1 == LayoutLocation::TOP {source.2.0} else {source.2.1}, i,
            //                 src, source.1, if source.1 == LayoutLocation::TOP {source.2.0} else {source.2.1}, i,
            //                 |e| {e.set_string(format!("<-{}", e.get_value()))});
            //         });
            //         let mut v2 = vec![];
            //         (0..values.len()).for_each(|i| {
            //             add_animation_data!(v1; src, source.1, if source.1 == LayoutLocation::TOP {source.2.0} else {source.2.1}, i,
            //                 src, LayoutLocation::None, 0, i, |_| {});
            //         });
            //         return vec![(v1, false), (v2, false)];
            //     }
            // }
        }
        _ => {/*ERROR*/}
    }
    vec![(vec![], false)]
}

fn vmovapd_animation(odd: Vec<(Operand, LayoutLocation, (usize, usize))>, cpu: Arc<Mutex<CPU>>, vrt: HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)> {
    mov_common_animation(odd, cpu, vrt)
}

type Func = fn(Arc<Mutex<CPU>>, Vec<Operand>, HashMap<(VecRegName, usize), ValueType>);
type AniFunc = fn(Vec<(Operand, LayoutLocation, (usize, usize))>, Arc<Mutex<CPU>>, HashMap<(VecRegName, usize), ValueType>) -> Vec<(Vec<ElementAnimationData>, bool)>;

macro_rules! new_instruction {
    ($map:expr; $inst:expr, $target_read:expr, $func:expr, $ani_func:expr) => {
        $map.insert(String::from($inst), ($target_read, $func as Func, $ani_func as AniFunc))
    };
}

fn create_instruction_list() -> HashMap<String, (bool, Func, AniFunc)>
{
    let mut map = HashMap::new();
    new_instruction!(map; "vaddps", false, vaddps, vaddps_animation);
    new_instruction!(map; "vpaddd", false, vpaddd, vpaddd_animation);
    new_instruction!(map; "valignd", false, valignd, valignd_animation);
    new_instruction!(map; "vunpcklps", false, vunpcklps, vunpcklps_animation);
    new_instruction!(map; "vunpckhps", false, vunpckhps, vunpckhps_animation);
    new_instruction!(map; "vshufps", false, vshufps, vshufps_animation);
    new_instruction!(map; "vperm2f128", false, vperm2f128, vperm2f128_animation);
    new_instruction!(map; "vextractf128", false, vextractf128, vextractf128_animation);
    new_instruction!(map; "shufpd", true, shufpd, shufpd_animation);
    new_instruction!(map; "vmulpd", false, vmulpd, vmulpd_animation);
    new_instruction!(map; "vmovapd", false, vmovapd, vmovapd_animation);
    new_instruction!(map; "add", true, add, add_animation);
    map
}

lazy_static! {
    static ref OPCODES: HashMap<String, (bool, Func, AniFunc)> = {
        create_instruction_list()
    };
}

pub fn execute(rv: Arc<Mutex<RegVisualizer>>, cpu: Arc<Mutex<CPU>>, fsm: &mut AnimationFSM, rvd: &RegVisualizerData, ctx: &Context, instruction: &str) {
    // Reset register highlight
    let mut rv_lock = rv.lock().unwrap();
    rv_lock.reset_highlight();
    drop(rv_lock);
    // Parse operands and opcode
    let (opcode, operands) = split_instruction(instruction);
    let mut operands = create_operands(operands, cpu.clone());
    // CMP Instruction
    if opcode == "cmp" {
        if let (Operand::Reg(r1), Operand::Reg(r2)) = (operands[0].clone(), operands[1].clone()) {
            let mut cpu = cpu.lock().unwrap();
            let v1 = cpu.registers.get_gpr_value(r1.get_gpr());
            let v2 = cpu.registers.get_gpr_value(r2.get_gpr());
            let mut f = cpu.registers.get_flags_value(FLAGSName::RFLAGS);
            if v1 == v2 {
                f |= 0b1000000;
                cpu.registers.set_flags_value(FLAGSName::RFLAGS, f);
            } else {
                f &= 0xFFFFFFBF;
                cpu.registers.set_flags_value(FLAGSName::RFLAGS, f);
            }
        } else if let (Operand::Reg(r), Operand::Imm(imm)) = (operands[0].clone(), operands[1].clone()) {
            let mut cpu = cpu.lock().unwrap();
            let v = cpu.registers.get_gpr_value(r.get_gpr());
            let mut f = cpu.registers.get_flags_value(FLAGSName::RFLAGS);
            if v == imm {
                f |= 0b1000000;
                cpu.registers.set_flags_value(FLAGSName::RFLAGS, f);
            } else {
                f &= 0xFFFFFFBF;
                cpu.registers.set_flags_value(FLAGSName::RFLAGS, f);
            }
        }
        return;
    }
    // Other Instructions
    if !OPCODES.contains_key(&opcode) {
        println!("Unsupport opcode: {}", opcode);
        return;
    }
    let (is_target_read, func, ani_func) = OPCODES.get(&opcode).unwrap();
    if *is_target_read {
        if let Some(target) = operands.first() {
            operands.insert(0, target.clone());
        }
    }
    // TODO
    // Animation FSM
    // Update CPU data - must run update date
    let cpu_clone = cpu.clone();
    let operands_clone = operands.clone();
    let vrt = rvd.vector_regs_type.clone();
    fsm.set_update_data(move |fsm| {
        func(cpu_clone, operands_clone, vrt);
        fsm.next();
    });
    // Check if need animation - if all register in the display list, show the animation
    let need_animation = operands.iter().all(|operand| {
        match operand {
            Operand::Reg(reg) => {
                rvd.registers[0].contains(reg)
            }
            _ => true,
        }
    }) && match operands[0] { Operand::Reg(_) => true, _ => false, };
    if !need_animation {
        // if animation is not needed, update cpu and highlight target reg
        fsm.set_create_layout(|fsm| {
            fsm.next();
        });
        fsm.set_run_animation(|fsm| {
            fsm.next();
        });
        let operands_clone = operands.clone();
        let rv_clone = rv.clone();
        let ctx_clone = ctx.clone();
        fsm.set_destroy_layout(move |fsm| {
            match operands_clone[0] {
                Operand::Reg(reg) => {
                    let mut rv = rv_clone.lock().unwrap();
                    rv.highlight(&reg);
                }
                _ => {}
            }
            ctx_clone.request_repaint();
            fsm.next();
        });
        fsm.start();
        return;
    }
    // Create location and repeat times for every operands
    let mut reg_operand_data: HashMap<Register, (usize, LayoutLocation, (usize, usize), bool)> = HashMap::new();
    operands.iter().for_each(|operand| {
        match operand {
            Operand::Reg(reg) => {
                if let Some(i) = rvd.registers[0].iter().position(|r| *r == *reg) {
                    if reg_operand_data.contains_key(reg) {
                        let mut data = reg_operand_data.get_mut(reg).unwrap();
                        data.2.1 += 1;
                    } else {
                        reg_operand_data.insert(reg.clone(), (i, LayoutLocation::None, (0, 1), false));
                    }
                }
            }
            _ => {}
        }
    });
    if let Operand::Reg(target) = operands[0] {
        let mut target_data = reg_operand_data.get(&target).unwrap().clone();
        operands.iter().for_each(|operand| {
            match operand {
                Operand::Reg(reg) => {
                    if *reg != target {
                        let data = reg_operand_data.get_mut(reg).unwrap();
                        if target_data.1 == LayoutLocation::None {
                            if target_data.0 < data.0 {
                                target_data.1 = LayoutLocation::BOTTOM;
                                data.1 = LayoutLocation::TOP;
                                (data.2.0, data.2.1) = (data.2.1, data.2.0);
                            } else {
                                target_data.1 = LayoutLocation::TOP;
                                data.1 = LayoutLocation::BOTTOM;
                                (target_data.2.0, target_data.2.1) = (target_data.2.1, target_data.2.0);
                            }
                        } else {
                            if target_data.0 < data.0 {
                                data.1 = LayoutLocation::TOP;
                                (data.2.0, data.2.1) = (data.2.1, data.2.0);
                            } else {
                                data.1 = LayoutLocation::BOTTOM;
                            }
                        }
                    }
                }
                _ => {}
            }
        });
        let target_data_mut = reg_operand_data.get_mut(&target).unwrap();
        (target_data_mut.0, target_data_mut.1, target_data_mut.2, target_data_mut.3) =
            (target_data.0, if target_data.1 == LayoutLocation::None {LayoutLocation::BOTTOM} else {target_data.1}, target_data.2, true);
    }
    // Create layout
    let rv_clone = rv.clone();
    let ctx_clone = ctx.clone();
    let reg_operand_data_clone = reg_operand_data.clone();
    fsm.set_create_layout(move |fsm| {
        let mut rv = rv_clone.lock().unwrap();
        reg_operand_data_clone.iter().for_each(|(reg, (_, loc, rn, _))| {
            rv.create_animation_layout_with_repeat_numbers(
                reg, *loc, *rn, &ctx_clone
            );
        });
        fsm.next();
    });
    // Determine operand position
    let mut tmp: HashMap<Register, (usize, usize)> = HashMap::new();
    let mut operand_display_data = vec![];
    operands.iter().for_each(|operand| {
        if let Operand::Reg(reg) = operand {
            let loc = reg_operand_data.get(reg).unwrap().1;
            if tmp.contains_key(reg) {
                if loc == LayoutLocation::TOP {
                    tmp.get_mut(reg).unwrap().0 += 1;
                } else {
                    tmp.get_mut(reg).unwrap().1 += 1;
                }
            } else {
                tmp.insert(*reg, (0usize, 0usize));
            }
            operand_display_data.push((operand.clone(), loc, tmp.get(reg).unwrap().clone()));
        } else {
            operand_display_data.push((operand.clone(), LayoutLocation::None, (0usize, 0usize)));
        }
    });
    // Destroy layout
    let rv_clone = rv.clone();
    let ctx_clone = ctx.clone();
    let reg_operand_data_clone = reg_operand_data.clone();
    fsm.set_destroy_layout(move |fsm| {
        let mut rv = rv_clone.lock().unwrap();
        rv.remove_animation_layout(&ctx_clone);
        reg_operand_data_clone.iter().for_each(|(reg, (_, loc, rn, is_target))| {
            if *is_target {
                rv.highlight(reg);
            }
        });
        fsm.next();
    });
    // Run animation
    let rv_clone = rv.clone();
    let reg_operand_data_clone = reg_operand_data.clone();
    let cpu_clone = cpu.clone();
    let vrt = rvd.vector_regs_type.clone();
    let odd = operand_display_data.clone();
    let ctx_clone = ctx.clone();
    fsm.set_run_animation(move |fsm| {
        let mut rv = rv_clone.lock().unwrap();
        let sequence = ani_func(odd, cpu_clone, vrt);
        rv.set_group_move_animation_sequence(
            Arc::new(Mutex::new(sequence))
        );
        let mut regs = vec![];
        reg_operand_data_clone.keys().for_each(|key| {
            regs.push(key.clone());
        });
        rv.start_move_animation_sequence_after_start_animation(&regs);
        let sender = fsm.sender.clone();
        rv.set_sequence_finished_callback(move || {
            ctx_clone.request_repaint();
            sender.send(FSMCtrlMsg::Next).unwrap();
        });
    });
    // Start FSM
    fsm.start();
}
