use std::collections::HashMap;
use std::hash::Hash;
use eframe::egui::{self, Vec2, Pos2, Ui, Color32};
use super::*;

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
        "XMM10" | "XMM11" | "XMM12" | "XMM13" | "XMM14" | "XMM15" => Color32::from_rgb(128, 128, 128),
        "YMM0" | "YMM1" | "YMM2" | "YMM3" | "YMM4" | "YMM5" | "YMM6" | "YMM7" | "YMM8" | "YMM9" |
        "YMM10" | "YMM11" | "YMM12" | "YMM13" | "YMM14" | "YMM15" => Color32::from_rgb(128, 128, 128),
        "ZMM0" | "ZMM1" | "ZMM2" | "ZMM3" | "ZMM4" | "ZMM5" | "ZMM6" | "ZMM7" | "ZMM8" | "ZMM9" |
        "ZMM10" | "ZMM11" | "ZMM12" | "ZMM13" | "ZMM14" | "ZMM15" => Color32::from_rgb(128, 128, 128),
        "RAX" | "RBX" | "RCX" | "RDX" | "RSI" | "RDI" | "RBP" | "RSP" | "R8" | "R9" | "R10" |
        "R11" |"R12" | "R13" | "R14" | "R15" => Color32::from_rgb(128, 128, 128),
        "EAX" | "EBX" | "ECX" | "EDX" | "ESI" | "EDI" | "EBP" | "ESP" | "R8D" | "R9D" | "R10D" |
        "R11D" | "R12D" | "R13D" | "R14D" | "R15D" => Color32::from_rgb(128, 128, 128),
        "AX" | "BX" | "CX" | "DX" | "SI" | "DI" | "BP" | "SP" | "R8W" | "R9W" | "R10W" | "R11W" |
        "R12W" | "R13W" | "R14W" | "R15W" => Color32::from_rgb(128, 128, 128),
        "AH" | "BH" | "CH" | "DH" | "AL" | "BL" | "CL" | "DL" | "SIL" | "DIL" | "BPL" | "SPL" |"R8B" |
        "R9B" | "R10B" | "R11B" | "R12B" | "R13B" | "R14B" | "R15B" => Color32::from_rgb(128, 128, 128),
        _ => Color32::TRANSPARENT,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ElementOrder {
    Normal,        // None
    Low,           // Middle
    Middle,        // Foreground
    High,          // Tooltip
    Top,           // Debug
}

impl ElementOrder {
    pub fn get_higher(&self) -> Self {
        match self {
            ElementOrder::Normal => ElementOrder::Low,
            ElementOrder::Low => ElementOrder::Middle,
            ElementOrder::Middle => ElementOrder::High,
            ElementOrder::High => ElementOrder::Top,
            ElementOrder::Top => ElementOrder::Top,
        }
    }
    pub fn get_lower(&self) -> Self {
        match self {
            ElementOrder::Normal => ElementOrder::Normal,
            ElementOrder::Low => ElementOrder::Normal,
            ElementOrder::Middle => ElementOrder::Low,
            ElementOrder::High => ElementOrder::Middle,
            ElementOrder::Top => ElementOrder::High,
        }
    }
}

pub struct Element {
    // Data
    value: Value,
    string: Option<String>,
    // Animation
    order: ElementOrder,
    color: Color32,
    border_color: Color32,
    layout_position: Pos2,
    position: Pos2,
    target_position: Pos2,
    animating: bool,
    // Callback
    animation_finished_callback: Option<Box<dyn FnOnce(&mut Self)>>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            // Data
            value: Value::default(),
            string: None,
            // Animation
            order: ElementOrder::Normal,
            color: Color32::TRANSPARENT,
            border_color: Color32::TRANSPARENT,
            layout_position: Pos2::new(0f32, 0f32),
            position: Pos2::new(0f32, 0f32),
            target_position: Pos2::new(0f32, 0f32),
            animating: false,
            // Callback
            animation_finished_callback: None,
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
            layout_position: position,
            position,
            target_position: position,
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
    pub fn set_animation_finished_callback<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + 'static,
    {
        self.animation_finished_callback = Some(Box::new(callback));
    }
}

impl Element {
    fn show(&self, ui: &mut Ui) {
        let rect_size = get_size_from_value(&self.value);
        // Display Rectangle
        ui.painter().rect_filled(
            Rect::from_min_size(self.position, rect_size),
            0.0,
            self.color,
        );
        // Display Border
        ui.painter().rect_stroke(
            Rect::from_min_size(self.position, rect_size),
            0.0,
            egui::Stroke::new(2.0, self.border_color),
        );
        // Adaptive Text Size
        let mut font_size = 20f32;
        let mut text_size;
        loop {
            let galley = ui.painter().layout_no_wrap(
                if let Some(text) = &self.string {
                    format!("{}", text)
                } else {
                    format!("{}", self.value)
                },
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
            if let Some(text) = &self.string {
                format!("{}", text)
            } else {
                format!("{}", self.value)
            },
            egui::FontId::new(font_size, egui::FontFamily::Monospace),
            Color32::BLACK,
        );
        ui.painter().galley(text_pos, galley);
    }
    fn update(&mut self, delta_time: f32, velocity: f32) {
        let direction = self.target_position - self.position;
        if direction.length() > 1.0 {
            self.animating = true;
            let normalized_direction = direction.normalized();
            self.position += normalized_direction * velocity * delta_time;
        } else {
            self.position = self.target_position;
            self.animating = false;
            // Run callback
            if let Some(callback) = self.animation_finished_callback.take() {
                callback(self);
            }
        }
    }
    pub fn set_target_position(&mut self, target: Pos2) {
        self.target_position = target;
    }
}

impl Element {
    pub fn set_string(&mut self, str: String) {
        self.string = Some(str);
    }
    pub fn reset_string(&mut self) {
        self.string = None;
    }
}

pub struct RegVisualizer {
    // Visualization Data
    layout_data: HashMap<Register, Vec<Vec<(Pos2, Vec2)>>>,
    elements: HashMap<Register, Vec<Vec<Element>>>,
    // Animation Data
    animation_config: HashMap<Register, RegAnimationConfig>,
    animation_layout_data: HashMap<(Register, LayoutLocation), Vec<Vec<(Pos2, Vec2)>>>,
    animation_elements: HashMap<(Register, LayoutLocation), Vec<Vec<Element>>>,
}

impl Default for RegVisualizer {
    fn default() -> Self {
        Self {
            // Visualization Data
            layout_data: HashMap::new(),
            elements: HashMap::new(),
            // Animation Data
            animation_config: HashMap::new(),
            animation_layout_data: HashMap::new(),
            animation_elements: HashMap::new(),
        }
    }
}

impl RegVisualizer {
    pub fn is_animating(&self) -> bool {
        self.elements.values().any(|vec| vec.iter().any(|els| els.iter().any(|el| el.animating))) ||
            self.animation_elements.values().any(|vec| vec.iter().any(|els| els.iter().any(|el| el.animating)))
    }
}

macro_rules! show_element {
    ($ui:expr, $element:expr, $low_layer_id:expr, $middle_layer_id:expr, $high_layer_id:expr, $top_layer_id:expr) => {
        match $element.order {
            ElementOrder::Normal => {
                $element.show($ui);
            }
            ElementOrder::Low => {
                $ui.with_layer_id($low_layer_id, |ui| {
                    $element.show(ui);
                });
            }
            ElementOrder::Middle => {
                $ui.with_layer_id($middle_layer_id, |ui| {
                    $element.show(ui);
                });
            }
            ElementOrder::High => {
                $ui.with_layer_id($high_layer_id, |ui| {
                    $element.show(ui);
                });
            }
            ElementOrder::Top => {
                $ui.with_layer_id($top_layer_id, |ui| {
                    $element.show(ui);
                });
            }
        }
    };
}

impl RegVisualizer {
    pub fn update(&mut self, delta_time: f32, velocity: f32) {
        self.elements.iter_mut().for_each(|(_, vec)| {
            vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.update(delta_time, velocity);
                });
            });
        });
        self.animation_elements.iter_mut().for_each(|(_, vec)| {
            vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.update(delta_time, velocity);
                });
            });
        });
    }

    fn create_layout<T: Hash + Clone + Eq + PartialEq>(ui: &mut Ui, size: Vec2, key: &T, data_size: usize, repeat_number: usize, layout_data: &mut HashMap<T, Vec<Vec<(Pos2, Vec2)>>>) {
        ui.vertical(|ui| {
            let mut layout_vecs = vec![];
            (0..repeat_number).for_each(|_| {
                ui.horizontal(|ui| {
                    let mut layout_vec = vec![];
                    (0..data_size).for_each(|_| {
                        let (layout_rect, _response) = ui.allocate_exact_size(size, Sense::hover());
                        layout_vec.push((layout_rect.min, size));
                    });
                    layout_vecs.push(layout_vec);

                });
            });
            layout_data.insert(key.clone(), (layout_vecs));
        });
    }

    fn create_elements<T: Hash + Clone + Eq + PartialEq>(values: &Vec<Value>, key: &T, reg: &Register, layout_data: &HashMap<T, Vec<Vec<(Pos2, Vec2)>>>, elements: &mut HashMap<T, Vec<Vec<Element>>>) {
        let values_changed = if let Some(layout_vec) = layout_data.get(key) {
            if let Some(elements_vec) = elements.get(key) {
                layout_vec[0].len() != elements_vec[0].len() ||
                    elements_vec[0].iter().enumerate().any(|(index, element)| element.value != values[index])
            } else {
                false
            }
        } else {
            false
        };
        if !elements.contains_key(key) || values_changed {
            if let Some(vecs) = layout_data.get(key) {
                let mut element_vecs = vec![];
                vecs.iter().for_each(|vec| {
                    if vec.len() == values.len() {
                        let mut element_vec = vec![];
                        vec.iter().enumerate().for_each(|(index, (position, size))| {
                            element_vec.push(Element::default()
                                .with_value(values[index].clone())
                                .with_position(position.clone())
                                .with_color(get_color(&get_reg_name(reg)))
                                .with_border_color(get_border_color(&get_reg_name(reg))));
                        });
                        element_vecs.push(element_vec);
                    }
                });
                elements.insert(key.clone(), element_vecs);
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui, data: &RegVisualizerData, cpu: &CPU) {
        // Layout & Update Elements
        ui.vertical(|ui| {
            data.registers[0].iter().for_each(|reg| {
                // Load values from CPU
                let mut values: Vec<Value> = vec![];
                match reg.get_type() {
                    RegType::GPR => {
                        let v = cpu.registers.get_gpr_value(reg.get_gpr());
                        values = vec![create_value_with_gpr(v, &reg.get_gpr(), data.gprs_type.get(&reg.get_gpr()).unwrap())];
                    }
                    RegType::Vector => {
                        let (reg_type, reg_index) = reg.get_vector();
                        let value_type = *data.vector_regs_type.get(&reg.get_vector()).unwrap();
                        values = match value_type {
                            ValueType::U8 => create_values(cpu.registers.get_by_sections::<u8>(reg_type, reg_index).unwrap()),
                            ValueType::U16 => create_values(cpu.registers.get_by_sections::<u16>(reg_type, reg_index).unwrap()),
                            ValueType::U32 => create_values(cpu.registers.get_by_sections::<u32>(reg_type, reg_index).unwrap()),
                            ValueType::U64 => create_values(cpu.registers.get_by_sections::<u64>(reg_type, reg_index).unwrap()),
                            ValueType::U128 => create_values(cpu.registers.get_by_sections::<u128>(reg_type, reg_index).unwrap()),
                            ValueType::U256 => create_values(cpu.registers.get_by_sections::<u256>(reg_type, reg_index).unwrap()),
                            ValueType::U512 => create_values(cpu.registers.get_by_sections::<u512>(reg_type, reg_index).unwrap()),
                            ValueType::F32 => create_values(Utilities::u32vec_to_f32vec(cpu.registers.get_by_sections::<u32>(reg_type, reg_index).unwrap())),
                            ValueType::F64 => create_values(Utilities::u64vec_to_f64vec(cpu.registers.get_by_sections::<u64>(reg_type, reg_index).unwrap())),
                        }
                    }
                    _ => {/*None: Do nothing, there is NO possible to run into here!*/}
                }
                // Show UI
                ui.vertical(|ui| {
                    ui.label(get_reg_name(reg).clone());
                    ui.spacing_mut().item_spacing.x = 0.0; // Set spaces between elements to 0
                    let location = if let Some(config) = self.animation_config.get(reg) {
                        config.location
                    } else {
                        LayoutLocation::None
                    };
                    let size = get_size_from_value(&values[0]);
                    let repeat_number = if let Some(config) = self.animation_config.get(reg) {
                        config.repeat_numbers
                    } else {
                        (0, 0)
                    };
                    // Animation Layout - TOP
                    if location == LayoutLocation::TOP || location == LayoutLocation::BOTH {
                        RegVisualizer::create_layout(ui, size, &(reg.clone(), LayoutLocation::TOP), values.len(), repeat_number.0, &mut self.animation_layout_data);
                        RegVisualizer::create_elements(&values, &(reg.clone(), LayoutLocation::TOP), reg, &self.animation_layout_data, &mut self.animation_elements);
                    }
                    // Elements Layout
                    RegVisualizer::create_layout(ui, size, reg, values.len(), 1, &mut self.layout_data);
                    RegVisualizer::create_elements(&values, reg, reg, &self.layout_data, &mut self.elements);
                    // Animation Layout - BOTTOM
                    if location == LayoutLocation::BOTTOM || location == LayoutLocation::BOTH {
                        RegVisualizer::create_layout(ui, size, &(reg.clone(), LayoutLocation::BOTTOM), values.len(), repeat_number.1, &mut self.animation_layout_data);
                        RegVisualizer::create_elements(&values, &(reg.clone(), LayoutLocation::BOTTOM), reg, &self.animation_layout_data, &mut self.animation_elements);
                    }
                });
            });
        });
        // Clean
        self.layout_data.retain(|reg_in_layout, _| data.registers[0].iter().any(|reg_in_data| *reg_in_layout == *reg_in_data));
        self.elements.retain(|reg_in_elements, _| data.registers[0].iter().any(|reg_in_data| *reg_in_elements == *reg_in_data));
        self.animation_layout_data.retain(|(reg_in_layout, _), _| self.animation_config.iter().any(|(reg_in_config, _)| *reg_in_layout == *reg_in_config));
        self.animation_elements.retain(|(reg_in_elements, _), _| self.animation_config.iter().any(|(reg_in_config, _)| *reg_in_elements == *reg_in_config));
        // Fix Elements Position
        self.elements.iter_mut().for_each(|(reg, elements)| {
            // if element in elements, then element must in layout
            let vec = self.layout_data.get(reg).unwrap();
            (0..elements.len()).for_each(|i| {
                elements[i].iter_mut().enumerate().for_each(|(j, element)| {
                    if element.layout_position != vec[i][j].0 {
                        // layout changing
                        let p = element.position - element.layout_position;
                        let tp = element.target_position - element.layout_position;
                        element.layout_position = vec[i][j].0;
                        element.position = element.layout_position + p;
                        element.target_position =  element.layout_position + tp;
                    }
                });
            });
        });
        self.animation_elements.iter_mut().for_each(|(key, elements)| {
            // if element in elements, then element must in layout
            let vec = self.animation_layout_data.get(key).unwrap();
            (0..elements.len()).for_each(|i| {
                elements[i].iter_mut().enumerate().for_each(|(j, element)| {
                    if element.layout_position != vec[i][j].0 {
                        // layout changing
                        let p = element.position - element.layout_position;
                        let tp = element.target_position - element.layout_position;
                        element.layout_position = vec[i][j].0;
                        element.position = element.layout_position + p;
                        element.target_position =  element.layout_position + tp;
                    }
                });
            });
        });
        // Show Elements
        let low_layer_id = LayerId::new(Order::Middle, Id::new("register_visualizer_animation_elements_low"));
        let middle_layer_id = LayerId::new(Order::Foreground, Id::new("register_visualizer_animation_elements_middle"));
        let high_layer_id = LayerId::new(Order::Tooltip, Id::new("register_visualizer_animation_elements_high"));
        let top_layer_id = LayerId::new(Order::Debug, Id::new("register_visualizer_animation_elements_top"));
        self.elements.iter().for_each(|(_, vec)| {
            vec.iter().for_each(|elements| {
                elements.iter().for_each(|element| {
                    show_element!(ui, element, low_layer_id, middle_layer_id, high_layer_id, top_layer_id);
                });
            });
        });
        // Show Animation Elements
        self.animation_elements.iter().for_each(|((reg, loc), vec)| {
            if let Some(config) = self.animation_config.get(reg) {
                if config.show_element {
                    match config.location {
                        LayoutLocation::TOP => {
                            if *loc == LayoutLocation::TOP {
                                vec.iter().for_each(|elements| {
                                    elements.iter().for_each(|element| {
                                        show_element!(ui, element, low_layer_id, middle_layer_id, high_layer_id, top_layer_id);
                                    });
                                });
                            }
                        }
                        LayoutLocation::BOTTOM => {
                            if *loc == LayoutLocation::BOTTOM {
                                vec.iter().for_each(|elements| {
                                    elements.iter().for_each(|element| {
                                        show_element!(ui, element, low_layer_id, middle_layer_id, high_layer_id, top_layer_id);
                                    });
                                });
                            }
                        }
                        LayoutLocation::BOTH => {
                            if *loc == LayoutLocation::TOP || *loc == LayoutLocation::BOTTOM {
                                vec.iter().for_each(|elements| {
                                    elements.iter().for_each(|element| {
                                        show_element!(ui, element, low_layer_id, middle_layer_id, high_layer_id, top_layer_id);
                                    });
                                });
                            }
                        }
                        LayoutLocation::None => {}
                    }
                }
            }
        });
    }
}

// Animation
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LayoutLocation {
    TOP, BOTTOM, BOTH, None
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct RegAnimationConfig {
    location: LayoutLocation,
    repeat_numbers: (usize, usize), // (TOP, BOTTOM)
    show_element: bool,
}

impl Default for RegAnimationConfig {
    fn default() -> Self {
        Self {
            location: LayoutLocation::None,
            repeat_numbers: (0, 0),
            show_element: false,
        }
    }
}

impl RegAnimationConfig {
    fn with_location(self, location: LayoutLocation) -> Self {
        Self {
            location,
            repeat_numbers: if self.repeat_numbers == (0, 0) {
                match location {
                    LayoutLocation::TOP => (1, 0),
                    LayoutLocation::BOTTOM => (0, 1),
                    LayoutLocation::BOTH => (1, 1),
                    LayoutLocation::None => (0, 0),
                }
            } else {
                self.repeat_numbers
            },
            ..self
        }
    }
    fn with_repeat_numbers(self, repeat_numbers: (usize, usize)) -> Self {
        Self {
            repeat_numbers,
            ..self
        }
    }
    fn with_show(self, show_element: bool) -> Self {
        Self {
            show_element,
            ..self
        }
    }
    fn get_show(&mut self) -> bool {
        self.show_element
    }
    fn set_show(&mut self, show_element: bool) {
        self.show_element = show_element;
    }
}

impl RegVisualizer {
    pub fn create_animation_layout(&mut self, reg: &Register, location: LayoutLocation) {
        self.animation_config.insert(reg.clone(), RegAnimationConfig::default().with_location(location));
    }
    pub fn create_animation_layout_with_repeat_numbers(&mut self, reg: &Register, location: LayoutLocation, repeat_numbers: (usize, usize)) {
        self.animation_config.insert(reg.clone(), RegAnimationConfig::default().with_location(location).with_repeat_numbers(repeat_numbers));
    }
    pub fn remove_animation_layout(&mut self, reg: &Register) {
        self.animation_config.remove(reg);
    }
    pub fn change_animation_elements_layer_with_number(&mut self, key: &(Register, LayoutLocation), number: usize, layer_order: ElementOrder) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() {
                elements_vec[number].iter_mut().for_each(|element| {
                    element.order = layer_order;
                });
            }
        }
    }
    pub fn reset_animation_elements_layer_with_number(&mut self, key: &(Register, LayoutLocation), number: usize) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() {
                elements_vec[number].iter_mut().for_each(|element| {
                    element.order = ElementOrder::Normal;
                });
            }
        }
    }
    pub fn change_animation_elements_layer(&mut self, key: &(Register, LayoutLocation), layer_order: ElementOrder) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            elements_vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.order = layer_order;
                });
            });
        }
    }
    pub fn reset_animation_elements_layer(&mut self, key: &(Register, LayoutLocation)) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            elements_vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.order = ElementOrder::Normal;
                });
            });
        }
    }
    pub fn change_animation_elements_layer_with_number_index(&mut self, key: &(Register, LayoutLocation), number: usize, index:usize, layer_order: ElementOrder) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() && index < elements_vec[0].len() {
                elements_vec[number][index].order = layer_order;
            }
        }
    }
    pub fn reset_animation_elements_layer_with_number_index(&mut self, key: &(Register, LayoutLocation), number: usize, index:usize) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() && index < elements_vec[0].len() {
                elements_vec[number][index].order = ElementOrder::Normal;
            }
        }
    }
    pub fn start_show_animation_elements(&mut self, reg: &Register) {
        if let Some(config) = self.animation_config.get_mut(reg) {
            config.show_element = true;
        }
    }
    pub fn start_show_animation_elements_with_anime(&mut self, reg: &Register) {
        if let Some(config) = self.animation_config.get_mut(reg) {
            match config.location {
                LayoutLocation::TOP => {
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::TOP)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::BOTTOM => {
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::BOTTOM)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::BOTH => {
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::TOP)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                    });
                                }
                            });
                        }
                    }
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::BOTTOM)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::None => {}
            }
            config.show_element = true;
        }
    }
    pub fn set_string_for_animation_element(&mut self, key: &(Register, LayoutLocation), number: usize, index: usize, str: String) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() && index < elements_vec[0].len() {
                elements_vec[number][index].string = Some(str);
            }
        }
    }
    pub fn remove_string_from_animation_element(&mut self, key: &(Register, LayoutLocation), number: usize, index: usize) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() && index < elements_vec[0].len() {
                elements_vec[number][index].string = None;
            }
        }
    }
    pub fn set_string_for_animation_elements(&mut self, key: &(Register, LayoutLocation), number: usize, str_vec: Vec<String>) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() && str_vec.len() == elements_vec[0].len() {
                (0..str_vec.len()).for_each(|index| {
                    elements_vec[number][index].string = Some(str_vec[index].clone());
                });
            }
        }
    }
    pub fn remove_string_from_animation_elements(&mut self, key: &(Register, LayoutLocation), number: usize) {
        if let Some(elements_vec) = self.animation_elements.get_mut(key) {
            if number < elements_vec.len() {
                elements_vec[number].iter_mut().for_each(|element| {
                    element.string = None;
                });
            }
        }
    }
    pub fn move_animation<F>(&mut self, source: (Register, LayoutLocation, usize, usize), target: (Register, LayoutLocation, usize, usize), is_layout: bool, callback: F)
        where
            F: FnOnce(&mut Element) + 'static,
    {
        let mut error = false;
        let target_pos = if let Some(elements_vec) = self.animation_elements.get(&(target.0, target.1)) {
            if target.2 < elements_vec.len() && target.3 < elements_vec[0].len() {
                if is_layout {
                    elements_vec[target.2][target.3].layout_position
                } else {
                    elements_vec[target.2][target.3].position
                }
            } else {
                error = true;
                Pos2::new(0f32, 0f32)
            }
        } else {
            error = true;
            Pos2::new(0f32, 0f32)
        };
        if error {
            return;
        }
        if let Some(elements_vec) = self.animation_elements.get_mut(&(source.0, source.1)) {
            if source.2 < elements_vec.len() && source.3 < elements_vec[0].len() {
                elements_vec[source.2][source.3].target_position = target_pos;
                // TODO: change layer order
                elements_vec[source.2][source.3].set_animation_finished_callback(callback);
            }
        }
    }
}
