use std::collections::HashMap;
use cpulib::{VecRegName, GPRName, u256, u512};
use eframe::egui::{self, Vec2, Pos2, Ui};

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

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::U8(x) => Value::U8(*x),
            Value::U16(x) => Value::U16(*x),
            Value::U32(x) => Value::U32(*x),
            Value::U64(x) => Value::U64(*x),
            Value::U128(x) => Value::U128(*x),
            Value::U256(x) => Value::U256(x.clone()),
            Value::U512(x) => Value::U512(x.clone()),
            Value::F32(x) => Value::F32(*x),
            Value::F64(x) => Value::F64(*x),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

fn get_size_from_value(value: &Value) -> Vec2 {
    match value {
        Value::U8(_) => {Vec2::new(20.0, 25.0)}
        Value::U16(_) => {Vec2::new(40.0, 25.0)}
        Value::U32(_) => {Vec2::new(80.0, 25.0)}
        Value::U64(_) => {Vec2::new(160.0, 25.0)}
        Value::U128(_) => {Vec2::new(320.0, 25.0)}
        Value::U256(_) => {Vec2::new(640.0, 25.0)}
        Value::U512(_) => {Vec2::new(1280.0, 25.0)}
        Value::F32(_) => {Vec2::new(80.0, 25.0)}
        Value::F64(_) => {Vec2::new(160.0, 25.0)}
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

struct Element {
    // Data
    value: Value,
    // Animation
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
}

impl Element {
    fn show(&self, ui: &mut Ui) {
        ui.painter().rect_filled(
            egui::Rect::from_min_size(self.position, get_size_from_value(&self.value)),
            0.0,
            egui::Color32::RED,
        );
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
    vector_registers: HashMap<(VecRegName, usize), Vec<Value>>,
    gprs: HashMap<GPRName, Value>,
    // Visualization Data
    elements: HashMap<String, Vec<Element>>,
    // Animation
    velocity: f32,
}

impl Default for RegVisualizer {
    fn default() -> Self {
        Self {
            // Visualization Data
            vector_registers: HashMap::new(),
            gprs: HashMap::new(),
            // Visualization Data
            elements: HashMap::new(),
            // Animation
            velocity: 10f32,
        }
    }
}

impl RegVisualizer {
    pub fn insert_vector(&mut self, reg: VecRegName, reg_index: usize, values: Vec<Value>) {
        self.vector_registers.insert((reg, reg_index), values);
    }
    pub fn remove_vector(&mut self, reg: VecRegName, reg_index: usize) {
        // Remove elements
        let reg_name = get_vec_reg_name(&reg, &reg_index);
        self.elements.remove(&reg_name);
        // Remove vector
        self.vector_registers.remove(&(reg, reg_index));
    }
    pub fn insert_gpr(&mut self, reg: GPRName, value: Value) {
        self.gprs.insert(reg, value);
    }
    pub fn remove_gpr(&mut self, reg: GPRName) {
        // Remove elements
        let reg_name = get_gpr_name(&reg);
        self.elements.remove(&reg_name);
        // Remove gpr
        self.gprs.remove(&reg);
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
            for ((reg, reg_index), values) in &self.vector_registers {
                let reg_name = get_vec_reg_name(reg, reg_index);
                ui.vertical(|ui| {
                    ui.label(reg_name.clone());
                    ui.horizontal(|ui| {
                        let size = get_size_from_value(&values[0]);
                        let mut element_vec = vec![];
                        values.iter().for_each(|value| {
                            let (layout_rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                            element_vec.push(Element::default().with_value(value.clone()).with_position(layout_rect.min));
                            // FOR DEBUG
                            if ui.is_rect_visible(layout_rect) {
                                ui.painter().rect_filled(layout_rect, 0.0, egui::Color32::LIGHT_BLUE);
                                let galley = ui.painter().layout_no_wrap("9".into(), egui::FontId::new(20f32, egui::FontFamily::Monospace), egui::Color32::BLACK);
                                let text_pos = layout_rect.center() - galley.size() / 2.0;
                                ui.painter().galley(text_pos, galley);
                            }
                            // END
                        });
                        self.elements.insert(reg_name, element_vec);
                    });
                });
            }
            for (reg, value) in &self.gprs {
                let reg_name = get_gpr_name(reg);
                ui.vertical(|ui| {
                    ui.label(reg_name.clone());
                    ui.horizontal(|ui| {
                        let size = get_size_from_value(value);
                        let mut element_vec = vec![];
                        let (layout_rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                        element_vec.push(Element::default().with_value(value.clone()).with_position(layout_rect.min));
                        // FOR DEBUG
                        if ui.is_rect_visible(layout_rect) {
                            ui.painter().rect_filled(layout_rect, 0.0, egui::Color32::LIGHT_BLUE);
                            let galley = ui.painter().layout_no_wrap("9".into(), egui::FontId::new(20f32, egui::FontFamily::Monospace), egui::Color32::BLACK);
                            let text_pos = layout_rect.center() - galley.size() / 2.0;
                            ui.painter().galley(text_pos, galley);
                        }
                        // END
                        self.elements.insert(reg_name, element_vec);
                    });
                });
            }
        });
        // Show every elements
        self.elements.iter().for_each(|(_, vec)| {
            vec.iter().for_each(|element| {
                element.show(ui);
            })
        });
    }
}
