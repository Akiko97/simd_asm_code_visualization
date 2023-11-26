use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use cpulib::{VecRegName, GPRName, u256, u512};
use eframe::egui::{self, Vec2, Pos2, Ui, Color32};

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

fn get_size_from_value(value: &Value) -> Vec2 {
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

fn get_vec_reg_name(reg: &VecRegName, reg_index: &usize) -> String {
    match reg {
        VecRegName::XMM => {format!("XMM{}", reg_index)}
        VecRegName::YMM => {format!("YMM{}", reg_index)}
        VecRegName::ZMM => {format!("ZMM{}", reg_index)}
    }
}

fn get_gpr_name(reg: &GPRName) -> String {
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

fn get_color(reg: &String) -> Color32 {
    match reg.as_str() {
        "XMM0" | "YMM0" | "ZMM0" | "RAX" | "EAX" | "AX" | "AH" | "AL" => Color32::from_rgb(255, 255, 255),
        "XMM1" | "YMM1" | "ZMM1" | "RBX" | "EBX" | "BX" | "BH" | "BL" => Color32::from_rgb(255, 255, 255),
        "XMM2" | "YMM2" | "ZMM2" | "RCX" | "ECX" | "CX" | "CH" | "CL" => Color32::from_rgb(255, 255, 255),
        "XMM3" | "YMM3" | "ZMM3" | "RDX" | "EDX" | "DX" | "DH" | "DL" => Color32::from_rgb(255, 255, 255),
        "XMM4" | "YMM4" | "ZMM4" | "RSI" | "ESI" | "SI" | "SIL" => Color32::from_rgb(255, 255, 255),
        "XMM5" | "YMM5" | "ZMM5" | "RDI" | "EDI" | "DI" | "DIL" => Color32::from_rgb(255, 255, 255),
        "XMM6" | "YMM6" | "ZMM6" | "RBP" | "EBP" | "BP" | "BPL" => Color32::from_rgb(255, 255, 255),
        "XMM7" | "YMM7" | "ZMM7" | "RSP" | "ESP" | "SP" | "SPL" => Color32::from_rgb(255, 255, 255),
        "XMM8" | "YMM8" | "ZMM8" | "R8" | "R8D" | "R8W" | "R8B" => Color32::from_rgb(255, 255, 255),
        "XMM9" | "YMM9" | "ZMM9" | "R9" | "R9D" | "R9W" | "R9B" => Color32::from_rgb(255, 255, 255),
        "XMM10" | "YMM10" | "ZMM10" | "R10" | "R10D" | "R10W" | "R10B" => Color32::from_rgb(255, 255, 255),
        "XMM11" | "YMM11" | "ZMM11" | "R11" | "R11D" | "R11W" | "R11B" => Color32::from_rgb(255, 255, 255),
        "XMM12" | "YMM12" | "ZMM12" | "R12" | "R12D" | "R12W" | "R12B" => Color32::from_rgb(255, 255, 255),
        "XMM13" | "YMM13" | "ZMM13" | "R13" | "R13D" | "R13W" | "R13B" => Color32::from_rgb(255, 255, 255),
        "XMM14" | "YMM14" | "ZMM14" | "R14" | "R14D" | "R14W" | "R14B" => Color32::from_rgb(255, 255, 255),
        "XMM15" | "YMM15" | "ZMM15" | "R15" | "R15D" | "R15W" | "R15B" => Color32::from_rgb(255, 255, 255),
        _ => Color32::TRANSPARENT,
    }
}

fn get_border_color(reg: &String) -> Color32 {
    match reg.as_str() {
        "XMM0" | "XMM1" | "XMM2" | "XMM3" | "XMM4" | "XMM5" | "XMM6" |"XMM7" | "XMM8" | "XMM9" |
        "XMM10" | "XMM11" | "XMM12" | "XMM13" | "XMM14" | "XMM15" => Color32::from_rgb(0, 0, 0),
        "YMM0" | "YMM1" | "YMM2" | "YMM3" | "YMM4" | "YMM5" | "YMM6" | "YMM7" | "YMM8" | "YMM9" |
        "YMM10" | "YMM11" | "YMM12" | "YMM13" | "YMM14" | "YMM15" => Color32::from_rgb(0, 0, 0),
        "ZMM0" | "ZMM1" | "ZMM2" | "ZMM3" | "ZMM4" | "ZMM5" | "ZMM6" | "ZMM7" | "ZMM8" | "ZMM9" |
        "ZMM10" | "ZMM11" | "ZMM12" | "ZMM13" | "ZMM14" | "ZMM15" => Color32::from_rgb(0, 0, 0),
        "RAX" | "RBX" | "RCX" | "RDX" | "RSI" | "RDI" | "RBP" | "RSP" | "R8" | "R9" | "R10" |
        "R11" |"R12" | "R13" | "R14" | "R15" => Color32::from_rgb(0, 0, 0),
        "EAX" | "EBX" | "ECX" | "EDX" | "ESI" | "EDI" | "EBP" | "ESP" | "R8D" | "R9D" | "R10D" |
        "R11D" | "R12D" | "R13D" | "R14D" | "R15D" => Color32::from_rgb(0, 0, 0),
        "AX" | "BX" | "CX" | "DX" | "SI" | "DI" | "BP" | "SP" | "R8W" | "R9W" | "R10W" | "R11W" |
        "R12W" | "R13W" | "R14W" | "R15W" => Color32::from_rgb(0, 0, 0),
        "AH" | "BH" | "CH" | "DH" | "AL" | "BL" | "CL" | "DL" | "SIL" | "DIL" | "BPL" | "SPL" |"R8B" |
        "R9B" | "R10B" | "R11B" | "R12B" | "R13B" | "R14B" | "R15B" => Color32::from_rgb(0, 0, 0),
        _ => Color32::TRANSPARENT,
    }
}

struct Element {
    // Data
    value: Value,
    // Animation
    color: Color32,
    border_color: Color32,
    position: Pos2,
    target_position: Pos2,
    animating: bool,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            // Data
            value: Value::U64(0u64),
            // Animation
            color: Color32::TRANSPARENT,
            border_color: Color32::TRANSPARENT,
            position: Pos2::new(0f32, 0f32),
            target_position: Pos2::new(0f32, 0f32),
            animating: false,
        }
    }
}

impl Element {
    fn with_value(self, value: Value) -> Self {
        Self {
            value,
            ..self
        }
    }
    fn with_position(self, position: Pos2) -> Self {
        Self {
            position,
            ..self
        }
    }
    fn with_color(self, color: Color32) -> Self {
        Self {
            color,
            ..self
        }
    }
    fn with_border_color(self, border_color: Color32) -> Self {
        Self {
            border_color,
            ..self
        }
    }
}

impl Element {
    fn show(&self, ui: &mut Ui) {
        let rect_size = get_size_from_value(&self.value);
        // Display Rectangle
        ui.painter().rect_filled(
            egui::Rect::from_min_size(self.position, rect_size),
            0.0,
            self.color,
        );
        // Display Border
        ui.painter().rect_stroke(
            egui::Rect::from_min_size(self.position, rect_size),
            0.0,
            egui::Stroke::new(2.0, self.border_color),
        );
        // Adaptive Text Size
        let mut font_size = 20f32;
        let mut text_size;
        loop {
            let galley = ui.painter().layout_no_wrap(
                format!("{}", self.value),
                egui::FontId::new(font_size, egui::FontFamily::Monospace),
                Color32::BLACK,
            );
            text_size = galley.size().x;
            if text_size < rect_size.x - 4.0 { // Subtract a small margin
                break;
            }
            font_size -= 1.0;
            if font_size <= 1.0 { // Minimum font size
                break;
            }
        }
        // Display Text
        let text_pos = self.position + rect_size / 2.0 - Vec2::new(text_size / 2.0, font_size / 2.0);
        let galley = ui.painter().layout_no_wrap(
            format!("{}", self.value),
            egui::FontId::new(font_size, egui::FontFamily::Monospace),
            Color32::BLACK,
        );
        ui.painter().galley(text_pos, galley);
    }
    fn update(&mut self, delta_time: f32, velocity: Vec2) {
        let direction = self.target_position - self.position;
        if direction.length() > 1.0 {
            self.animating = true;
            let normalized_direction = direction.normalized();
            self.position += normalized_direction * velocity * delta_time;
        } else {
            self.position = self.target_position;
            self.animating = false;
        }
    }
    pub fn set_target_position(&mut self, target: Pos2) {
        self.target_position = target;
    }
}

pub struct RegVisualizer {
    // Visualization Data
    registers: HashMap<String, Vec<Value>>,
    order: Vec<String>,
    // Visualization Data
    elements: HashMap<String, Vec<Element>>,
    // Animation
    velocity: f32,
}

impl Default for RegVisualizer {
    fn default() -> Self {
        Self {
            // Layout Data
            registers: HashMap::new(),
            order: vec![],
            // Visualization Data
            elements: HashMap::new(),
            // Animation
            velocity: 10f32,
        }
    }
}

impl RegVisualizer {
    pub fn insert_vector(&mut self, reg: VecRegName, reg_index: usize, values: Vec<Value>) {
        let reg_name = get_vec_reg_name(&reg, &reg_index);
        self.registers.insert(reg_name.clone(), values);
        self.order.push(reg_name);
    }
    pub fn remove_vector(&mut self, reg: VecRegName, reg_index: usize) {
        let reg_name = get_vec_reg_name(&reg, &reg_index);
        // Remove registers
        self.registers.remove(&reg_name);
        // Remove elements
        self.elements.remove(&reg_name);
        // Remove Order
        self.order.retain(|item| *item != reg_name);
    }
    pub fn insert_gpr(&mut self, reg: GPRName, value: Value) {
        let reg_name = get_gpr_name(&reg);
        self.registers.insert(reg_name.clone(), vec![value]);
        self.order.push(reg_name);
    }
    pub fn remove_gpr(&mut self, reg: GPRName) {
        let reg_name = get_gpr_name(&reg);
        // Remove registers
        self.registers.remove(&reg_name);
        // Remove elements
        self.elements.remove(&reg_name);
        // Remove Order
        self.order.retain(|item| *item != reg_name);
    }
}

impl RegVisualizer {
    pub fn get_velocity(&self) -> f32 {
        self.velocity
    }
    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }
    pub fn is_animating(&self) -> bool {
        self.elements.values().any(|vec| vec.iter().any(|el| el.animating))
    }
}

impl RegVisualizer {
    pub fn update(&mut self, delta_time: f32) {
        self.elements.iter_mut().for_each(|(_, vec)| {
            vec.iter_mut().for_each(|element| {
                element.update(delta_time, Vec2::new(self.velocity, self.velocity));
            });
        });
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.order.iter().for_each(|reg_name| {
                if let Some(values) = self.registers.get(reg_name) {
                    ui.vertical(|ui| {
                        ui.label(reg_name.clone());
                        ui.horizontal(|ui| {
                            let size = get_size_from_value(&values[0]);
                            let mut element_vec = vec![];
                            values.iter().for_each(|value| {
                                let (layout_rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                                element_vec.push(Element::default()
                                    .with_value(value.clone())
                                    .with_position(layout_rect.min)
                                    .with_color(get_color(&reg_name))
                                    .with_border_color(get_border_color(&reg_name)));
                            });
                            self.elements.insert(reg_name.clone(), element_vec);
                        });
                    });
                }
            });
        });
        // Show every elements
        self.elements.iter().for_each(|(_, vec)| {
            vec.iter().for_each(|element| {
                element.show(ui);
            })
        });
    }
}
