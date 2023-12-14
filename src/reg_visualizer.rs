use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use eframe::egui::{self, Vec2, Pos2, Ui, Color32};
use super::*;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
    display: bool,
    order: ElementOrder,
    color: Color32,
    border_color: Color32,
    is_highlight: bool,
    layout_position: Pos2,
    position: Pos2,
    target_position: Pos2,
    animating: bool,
    // Callback
    animation_finished_callback: Option<Box<dyn FnOnce(&mut Self) + Send + 'static>>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            // Data
            value: Value::default(),
            string: None,
            // Animation
            display: true,
            order: ElementOrder::Normal,
            color: Color32::TRANSPARENT,
            border_color: Color32::TRANSPARENT,
            is_highlight: false,
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
    fn with_display(self, display: bool) -> Self {
        Self {
            display,
            ..self
        }
    }
}

impl Element {
    pub fn set_animation_finished_callback<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + Send + 'static,
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
            egui::Stroke::new(2.0, if self.is_highlight {Color32::RED} else if self.animating {Color32::KHAKI} else {self.border_color}),
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
    fn update(&mut self, delta_time: f32, factor: f32, min_speed: f32, max_speed: f32) {
        let direction = self.target_position - self.position;
        let distance = direction.length();
        if distance > 1.0 {
            self.animating = true;
            let base_speed = distance * factor;
            let speed = base_speed.min(max_speed).max(min_speed);
            let normalized_direction = direction.normalized();
            self.position += normalized_direction * speed * delta_time;
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
    pub fn get_value(&self) -> Value {
        self.value
    }
    pub fn set_value(&mut self, v: Value) {
        self.value = v;
    }
    pub fn set_order(&mut self, o: ElementOrder) {
        self.order = o;
    }
    pub fn set_color(&mut self, c: Color32) {
        self.color = c;
    }
    pub fn set_border_color(&mut self, c: Color32) {
        self.border_color = c;
    }
    pub fn reset_string(&mut self) {
        self.string = None;
    }
    pub fn highlight(&mut self) {
        self.is_highlight = true;
    }
    pub fn reset_highlight(&mut self) {
        self.is_highlight = false;
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
    // Animation Sequence
    sender: Sender<AnimationControlMsg>,
    receiver: Receiver<AnimationControlMsg>,
    sequence: Option<Vec<Arc<Mutex<Vec<(Vec<ElementAnimationData>, bool)>>>>>,
    finish_sender: Sender<ElementAnimationFinishMsg>,
    finish_receiver: Receiver<ElementAnimationFinishMsg>,
    sequence_finished_callback: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl Default for RegVisualizer {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let (finish_sender, finish_receiver) = mpsc::channel();
        Self {
            // Visualization Data
            layout_data: HashMap::new(),
            elements: HashMap::new(),
            // Animation Data
            animation_config: HashMap::new(),
            animation_layout_data: HashMap::new(),
            animation_elements: HashMap::new(),
            // Animation Sequence
            sender,
            receiver,
            sequence: None,
            finish_sender,
            finish_receiver,
            sequence_finished_callback: None,
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
        if $element.display {
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
        }
    };
}

impl RegVisualizer {
    pub fn update(&mut self, delta_time: f32, factor: f32, min_speed: f32, max_speed: f32) {
        self.elements.iter_mut().for_each(|(_, vec)| {
            vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.update(delta_time, factor, min_speed, max_speed);
                });
            });
        });
        self.animation_elements.iter_mut().for_each(|(_, vec)| {
            vec.iter_mut().for_each(|elements| {
                elements.iter_mut().for_each(|element| {
                    element.update(delta_time, factor, min_speed, max_speed);
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
            layout_data.insert(key.clone(), layout_vecs);
        });
    }

    fn create_elements<T: Hash + Clone + Eq + PartialEq>(values: &Vec<Value>, key: &T, reg: &Register, layout_data: &HashMap<T, Vec<Vec<(Pos2, Vec2)>>>, elements: &mut HashMap<T, Vec<Vec<Element>>>, display: bool) {
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
                                .with_display(display)
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
                        values = vec![create_value_with_gpr(v, &reg.get_gpr())];
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
                        RegVisualizer::create_elements(&values, &(reg.clone(), LayoutLocation::TOP), reg, &self.animation_layout_data, &mut self.animation_elements, false);
                    }
                    // Elements Layout
                    RegVisualizer::create_layout(ui, size, reg, values.len(), 1, &mut self.layout_data);
                    RegVisualizer::create_elements(&values, reg, reg, &self.layout_data, &mut self.elements, true);
                    // Animation Layout - BOTTOM
                    if location == LayoutLocation::BOTTOM || location == LayoutLocation::BOTH {
                        RegVisualizer::create_layout(ui, size, &(reg.clone(), LayoutLocation::BOTTOM), values.len(), repeat_number.1, &mut self.animation_layout_data);
                        RegVisualizer::create_elements(&values, &(reg.clone(), LayoutLocation::BOTTOM), reg, &self.animation_layout_data, &mut self.animation_elements, false);
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
        });
    }
    pub fn move_animation_sequence(&mut self, ctx: &Context) {
        match self.receiver.try_recv() {
            Ok(AnimationControlMsg::ExecuteAnimation(index)) => {
                if self.sequence.is_none() {
                    self.sender.send(AnimationControlMsg::Terminate).unwrap();
                    return;
                }
                let sequence = self.sequence.as_ref().unwrap()[0].clone();
                let mut groups = sequence.lock().unwrap();
                let length = groups.len();
                let group = std::mem::take(&mut groups[index]);
                if group.0.is_empty() {
                    if index + 1 < length {
                        self.sender.send(AnimationControlMsg::ExecuteAnimation(index + 1)).unwrap();
                    } else {
                        self.sender.send(AnimationControlMsg::Terminate).unwrap();
                    }
                } else {
                    let sender_clone = self.sender.clone();
                    self.group_move_animation(group.0, group.1, move || {
                        if index + 1 < length {
                            sender_clone.send(AnimationControlMsg::ExecuteAnimation(index + 1)).unwrap();
                        } else {
                            sender_clone.send(AnimationControlMsg::Terminate).unwrap();
                        }
                    });
                }
            }
            Ok(AnimationControlMsg::Terminate) => {
                if let Some(s) = self.sequence.as_deref_mut() {
                    let mut s = s.to_vec();
                    s.remove(0);
                    self.sequence = Some(s.clone());
                    if s.is_empty() {
                        self.sequence = None;
                        if let Some(callback) = self.sequence_finished_callback.take() {
                            callback();
                        }
                    } else {
                        self.sender.send(AnimationControlMsg::ExecuteAnimation(0)).unwrap();
                        ctx.request_repaint();
                    }
                } else {
                    if let Some(callback) = self.sequence_finished_callback.take() {
                        callback();
                    }
                }
            }
            Err(TryRecvError::Empty) => {
                /* Do nothing */
            }
            Err(_) => {
                /* Error */
                self.sequence = None;
            }
        }
    }
    pub fn move_animation_finish(&mut self, ctx: &Context) {
        loop {
            match self.finish_receiver.try_recv() {
                Ok(ElementAnimationFinishMsg::SetTarget(source, target)) => {
                    self.set_target_for_move_animation_finish(source, target);
                    ctx.request_repaint();
                }
                Err(TryRecvError::Empty) => {
                    break;
                    /* Do nothing */
                }
                Err(_) => {
                    /* Error */
                }
            }
        }
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
}

impl Default for RegAnimationConfig {
    fn default() -> Self {
        Self {
            location: LayoutLocation::None,
            repeat_numbers: (0, 0),
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
}

impl RegVisualizer {
    pub fn create_animation_layout(&mut self, reg: &Register, location: LayoutLocation, ctx: &Context) {
        self.animation_config.insert(reg.clone(), RegAnimationConfig::default().with_location(location));
        ctx.request_repaint();
    }
    pub fn create_animation_layout_with_repeat_numbers(&mut self, reg: &Register, location: LayoutLocation, repeat_numbers: (usize, usize), ctx: &Context) {
        self.animation_config.insert(reg.clone(), RegAnimationConfig::default().with_location(location).with_repeat_numbers(repeat_numbers));
        ctx.request_repaint();
    }
    pub fn remove_animation_layout(&mut self, reg: &Register, ctx: &Context) {
        self.animation_config.remove(reg);
        ctx.request_repaint();
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
        if let Some(elements_vec) = self.animation_elements.get_mut(&(*reg, LayoutLocation::TOP)) {
            elements_vec.iter_mut().for_each(|elements| elements.iter_mut().for_each(|element| element.display = true));
        }
        if let Some(elements_vec) = self.animation_elements.get_mut(&(*reg, LayoutLocation::BOTTOM)) {
            elements_vec.iter_mut().for_each(|elements| elements.iter_mut().for_each(|element| element.display = true));
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
    pub fn set_target_for_move_animation_finish(&mut self, source: (Register, LayoutLocation, usize, usize), target: (Register, LayoutLocation, usize, usize)) {
        let mut value;
        let mut string;
        let mut order;
        let mut color;
        let mut border_color;
        if let Some(source_vec) = self.animation_elements.get_mut(&(source.0, source.1)) {
            let e = &mut source_vec[source.2][source.3];
            value = e.value.clone();
            string = e.string.clone();
            order = e.order.clone();
            color = e.color.clone();
            border_color = e.border_color.clone();
            e.display = false;
        } else {
            return;
        }
        if let Some(target_vec) = self.animation_elements.get_mut(&(target.0, target.1)) {
            let e = &mut target_vec[target.2][target.3];
            // can not change the value, so change the string
            // e.set_value(value);
            if let Some(s) = string {
                e.set_string(s);
            } else {
                e.set_string(value.to_string());
            }
            e.set_order(order.get_lower());
            e.set_color(color);
            e.set_border_color(border_color);
        }
    }
    pub fn highlight(&mut self, reg: &Register) {
        if let Some(elements) = self.elements.get_mut(reg) {
            elements[0].iter_mut().for_each(|element| { element.highlight() });
        }
    }
    pub fn reset_highlight(&mut self) {
        self.elements.values_mut().for_each(|elements| elements[0].iter_mut().for_each(|element| element.reset_highlight()));
    }
}

pub struct ElementAnimationData {
    pub source: (Register, LayoutLocation, usize, usize),
    pub target: (Register, LayoutLocation, usize, usize),
    pub callback: Option<Box<dyn FnOnce(&mut Element) + Send + 'static>>,
}

impl ElementAnimationData {
    pub fn new(
        source: (Register, LayoutLocation, usize, usize),
        target: (Register, LayoutLocation, usize, usize),
        callback: impl FnOnce(&mut Element) + Send + 'static
    ) -> Self {
        ElementAnimationData {
            source,
            target,
            callback: Some(Box::new(callback)),
        }
    }
}

enum AnimationControlMsg {
    ExecuteAnimation(usize),
    Terminate,
}

enum ElementAnimationFinishMsg {
    SetTarget((Register, LayoutLocation, usize, usize), (Register, LayoutLocation, usize, usize)),
}

impl RegVisualizer {
    pub fn start_show_animation_elements_with_anime<F>(&mut self, reg: &Register, callback: F)
        where
            F: FnMut() + Send + 'static,
    {
        let complete_animation = Arc::new(Mutex::new(0usize));
        let callback = Arc::new(Mutex::new(Some(callback)));
        if let Some(config) = self.animation_config.get_mut(reg) {
            match config.location {
                LayoutLocation::TOP => {
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::TOP)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            let length = elements_vec.iter().fold(0, |acc, v| acc + v.len());
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                        let complete_animation_clone = complete_animation.clone();
                                        let callback_clone = callback.clone();
                                        element.set_animation_finished_callback(move |_| {
                                            let mut callback = callback_clone.lock().unwrap();
                                            let mut complete_animation = complete_animation_clone.lock().unwrap();
                                            *complete_animation += 1;
                                            if *complete_animation == length {
                                                if let Some(mut callback) = callback.take() {
                                                    callback();
                                                }
                                            }
                                        });
                                        element.display = true;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::BOTTOM => {
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::BOTTOM)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            let length = elements_vec.iter().fold(0, |acc, v| acc + v.len());
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                        let complete_animation_clone = complete_animation.clone();
                                        let callback_clone = callback.clone();
                                        element.set_animation_finished_callback(move |_| {
                                            let mut callback = callback_clone.lock().unwrap();
                                            let mut complete_animation = complete_animation_clone.lock().unwrap();
                                            *complete_animation += 1;
                                            if *complete_animation == length {
                                                if let Some(mut callback) = callback.take() {
                                                    callback();
                                                }
                                            }
                                        });
                                        element.display = true;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::BOTH => {
                    let length = if let Some(elements_vec_top) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::TOP)) {
                        elements_vec_top.iter().fold(0, |acc, v| acc + v.len()) +
                            if let Some(elements_vec_bottom) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::BOTTOM)) {
                                elements_vec_bottom.iter().fold(0, |acc, v| acc + v.len())
                            } else {
                                0
                            }
                    } else {
                        0
                    };
                    if let Some(elements_vec) = self.animation_elements.get_mut(&(reg.clone(), LayoutLocation::TOP)) {
                        if let Some(layout) = self.layout_data.get(reg) {
                            elements_vec.iter_mut().for_each(|elements| {
                                if elements.len() == layout[0].len() {
                                    elements.iter_mut().enumerate().for_each(|(index, element)| {
                                        element.position = layout[0][index].0;
                                        let complete_animation_clone = complete_animation.clone();
                                        let callback_clone = callback.clone();
                                        element.set_animation_finished_callback(move |_| {
                                            let mut callback = callback_clone.lock().unwrap();
                                            let mut complete_animation = complete_animation_clone.lock().unwrap();
                                            *complete_animation += 1;
                                            if *complete_animation == length {
                                                if let Some(mut callback) = callback.take() {
                                                    callback();
                                                }
                                            }
                                        });
                                        element.display = true;
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
                                        let complete_animation_clone = complete_animation.clone();
                                        let callback_clone = callback.clone();
                                        element.set_animation_finished_callback(move |_| {
                                            let mut callback = callback_clone.lock().unwrap();
                                            let mut complete_animation = complete_animation_clone.lock().unwrap();
                                            *complete_animation += 1;
                                            if *complete_animation == length {
                                                if let Some(mut callback) = callback.take() {
                                                    callback();
                                                }
                                            }
                                        });
                                        element.display = true;
                                    });
                                }
                            });
                        }
                    }
                }
                LayoutLocation::None => {}
            }
        }
    }
    pub fn move_animation<F>(&mut self, data: ElementAnimationData, is_layout: bool, callback: F)
        where
            F: FnOnce() + Send + 'static,
    {
        // Get Target Data (Position and Order)
        let mut error = false;
        let target_data = if data.target.1 == LayoutLocation::None && data.target.2 == 0 {
            if let Some(elements_vec) = self.elements.get(&data.target.0) {
                if data.target.3 < elements_vec[0].len() {
                    if is_layout {
                        (elements_vec[0][data.target.3].layout_position, elements_vec[0][data.target.3].order)
                    } else {
                        (elements_vec[0][data.target.3].position, elements_vec[0][data.target.3].order)
                    }
                } else {
                    error = true;
                    (Pos2::new(0f32, 0f32), ElementOrder::Normal)
                }
            } else {
                error = true;
                (Pos2::new(0f32, 0f32), ElementOrder::Normal)
            }
        } else {
            if let Some(elements_vec) = self.animation_elements.get(&(data.target.0, data.target.1)) {
                if data.target.2 < elements_vec.len() && data.target.3 < elements_vec[0].len() {
                    if is_layout {
                        (elements_vec[data.target.2][data.target.3].layout_position, elements_vec[data.target.2][data.target.3].order)
                    } else {
                        (elements_vec[data.target.2][data.target.3].position, elements_vec[data.target.2][data.target.3].order)
                    }
                } else {
                    error = true;
                    (Pos2::new(0f32, 0f32), ElementOrder::Normal)
                }
            } else {
                error = true;
                (Pos2::new(0f32, 0f32), ElementOrder::Normal)
            }
        };
        if error {
            return;
        }
        // Next: Move Animation
        if let Some(elements_vec) = self.animation_elements.get_mut(&(data.source.0, data.source.1)) {
            if data.source.2 < elements_vec.len() && data.source.3 < elements_vec[0].len() {
                elements_vec[data.source.2][data.source.3].target_position = target_data.0;
                if elements_vec[data.source.2][data.source.3].order <= target_data.1 {
                    elements_vec[data.source.2][data.source.3].order = target_data.1.get_higher();
                }
                let sender = self.finish_sender.clone();
                if let Some(callback_in_data) = data.callback {
                    elements_vec[data.source.2][data.source.3].set_animation_finished_callback(move |element| {
                        callback_in_data(element);
                        callback();
                        sender.send(ElementAnimationFinishMsg::SetTarget(data.source, data.target)).unwrap();
                    });
                } else {
                    elements_vec[data.source.2][data.source.3].set_animation_finished_callback(|_| {
                        callback();
                    });
                }
            }
        }
    }
    pub fn group_move_animation<F>(&mut self, data_vec: Vec<ElementAnimationData>, is_layout: bool, callback: F)
        where
            F: FnMut() + Send + 'static,
    {
        let total_animations = data_vec.len();
        let completed_animations = Arc::new(Mutex::new(0));
        let shared_callback = Arc::new(Mutex::new(Some(callback)));
        for data in data_vec.into_iter() {
            let completed_animations_clone = Arc::clone(&completed_animations);
            let shared_callback_clone = Arc::clone(&shared_callback);
            self.move_animation(data, is_layout, move || {
                let mut callback = shared_callback_clone.lock().unwrap();
                let mut completed_animations = completed_animations_clone.lock().unwrap();
                *completed_animations += 1;
                if *completed_animations == total_animations {
                    if let Some(mut callback) = callback.take() {
                        callback();
                    }
                }
            });
        }
    }
    pub fn set_group_move_animation_sequence(&mut self, sequence: Arc<Mutex<Vec<(Vec<ElementAnimationData>, bool)>>>) {
        self.sequence = Some(vec![sequence]);
    }
    pub fn add_group_move_animation_sequence(&mut self, sequence: Arc<Mutex<Vec<(Vec<ElementAnimationData>, bool)>>>) {
        if self.sequence.is_none() {
            self.set_group_move_animation_sequence(sequence);
        } else {
            if let Some(s) = self.sequence.as_deref_mut() {
                let mut s = s.to_vec();
                s.push(sequence);
                self.sequence = Some(s.clone());
            }
        }
    }
    pub fn start_move_animation_sequence(&self) {
        if !self.sequence.is_none() {
            self.sender.send(AnimationControlMsg::ExecuteAnimation(0)).unwrap();
        }
    }
    pub fn start_move_animation_sequence_after_start_animation(&mut self, regs: &Vec<Register>) {
        let length = regs.len();
        let count = Arc::new(Mutex::new(0usize));
        regs.iter().for_each(|reg| {
            let count_clone = count.clone();
            let sender_clone = self.sender.clone();
            self.start_show_animation_elements_with_anime(reg, move || {
                let mut count = count_clone.lock().unwrap();
                *count += 1;
                if *count == length {
                    sender_clone.send(AnimationControlMsg::ExecuteAnimation(0)).unwrap();
                }
            });
        });
    }
    pub fn set_sequence_finished_callback<F>(&mut self, callback: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.sequence_finished_callback = Some(Box::new(callback));
    }
}

#[macro_export]
macro_rules! create_animation_data {
    ($sr:expr, $sl:expr, $sli:expr, $sri:expr, $tr:expr, $tl:expr, $tli:expr, $tri:expr, $cb:expr) => {
        ElementAnimationData::new(
            ($sr, $sl, $sli, $sri),
            ($tr, $tl, $tli, $tri),
            $cb
        )
    };
}

#[macro_export]
macro_rules! add_animation_data {
    ($vec:expr; $sr:expr, $sl:expr, $sli:expr, $sri:expr, $tr:expr, $tl:expr, $tli:expr, $tri:expr, $cb:expr) => {
        $vec.push(ElementAnimationData::new(
            ($sr, $sl, $sli, $sri),
            ($tr, $tl, $tli, $tri),
            $cb
        ));
    };
}

#[macro_export]
macro_rules! create_group_animation_data {
    ($($sr:expr, $sl:expr, $sli:expr, $sri:expr, $tr:expr, $tl:expr, $tli:expr, $tri:expr, $cb:expr);*) => {
        vec![
            $(
                create_animation_data!($sr, $sl, $sli, $sri, $tr, $tl, $tli, $tri, $cb),
            )*
        ]
    };
}

#[macro_export]
macro_rules! add_group_animation_data {
    ($vec:expr; $($sr:expr, $sl:expr, $sli:expr, $sri:expr, $tr:expr, $tl:expr, $tli:expr, $tri:expr, $cb:expr);*) => {
        $(
            add_animation_data!($vec; $sr, $sl, $sli, $sri, $tr, $tl, $tli, $tri, $cb);
        )*
    };
}

#[macro_export]
macro_rules! add_register_group_animation_data {
    (@step $_idx:expr; $vec:expr; $sr:expr, $sl:expr, $sli:expr, $tr:expr, $tl:expr, $tli:expr,) => {};

    (@step $idx:expr; $vec:expr; $sr:expr, $sl:expr, $sli:expr, $tr:expr, $tl:expr, $tli:expr, $cb_head:expr, $($cb_tail:expr,)*) => {
        add_animation_data!($vec; $sr, $sl, $sli, $idx, $tr, $tl, $tli, $idx, $cb_head);
        add_register_group_animation_data!(@step $idx + 1usize; $vec; $sr, $sl, $sli, $tr, $tl, $tli, $($cb_tail,)*);
    };

    ($vec:expr; $sr:expr, $sl:expr, $sli:expr, $tr:expr, $tl:expr, $tli:expr, $($cb:expr),*) => {
        add_register_group_animation_data!(@step 0usize; $vec; $sr, $sl, $sli, $tr, $tl, $tli, $($cb,)*);
    };
}
