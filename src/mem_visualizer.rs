use super::*;

pub struct MemVisualizer {
    addr: usize,
    data_type: ValueType,
}

impl Default for MemVisualizer {
    fn default() -> Self {
        Self {
            addr: 0x40000000,
            data_type: ValueType::U8,
        }
    }
}

impl MemVisualizer {
    pub fn show(&mut self, ui: &mut Ui, _ctx: &Context, cpu: &CPU) {
        ui.horizontal(|ui| {
            ui.label("Data Type:");
            ui.radio_value(&mut self.data_type, ValueType::U8, "U8");
            ui.radio_value(&mut self.data_type, ValueType::U16, "U16");
            ui.radio_value(&mut self.data_type, ValueType::U32, "U32");
            ui.radio_value(&mut self.data_type, ValueType::F32, "F32");
            ui.radio_value(&mut self.data_type, ValueType::U64, "U64");
            ui.radio_value(&mut self.data_type, ValueType::F64, "F64");
        });
        let size = match self.data_type {
            ValueType::U8 => {Vec2::new(20.0, 20.0)}
            ValueType::U16 => {Vec2::new(40.0, 20.0)}
            ValueType::U32 => {Vec2::new(80.0, 20.0)}
            ValueType::U64 => {Vec2::new(160.0, 20.0)}
            ValueType::F32 => {Vec2::new(80.0, 20.0)}
            ValueType::F64 => {Vec2::new(160.0, 20.0)}
            _ => {Vec2::new(0.0, 0.0)}
        };
        let mut max_width = 0.0;
        for row in 0..16 {
            let addr = self.addr + row * 8;
            let text = format!("{:X}", addr);
            let text_width = ui.painter().layout_no_wrap(
                text.clone(),
                egui::FontId::new(15.0, egui::FontFamily::Monospace),
                Color32::BLACK,
            ).size().x;
            if text_width > max_width {
                max_width = text_width;
            }
        }
        ui.vertical(|ui| {
            (0usize..16).for_each(|row| {
                ui.horizontal(|ui| {
                    let addr = self.addr + row * 8;
                    // ui.label(format!("{:X}", addr));
                    let text = format!("{:X}", addr);
                    let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(max_width, 20.0), Sense::hover());
                    ui.painter().text(rect.min, egui::Align2::LEFT_TOP, text, egui::FontId::new(15.0, egui::FontFamily::Monospace), Color32::GRAY);
                    let values = match self.data_type {
                        ValueType::U8 => {create_values(cpu.memory.read_vec::<u8>(addr, 64 / 8))}
                        ValueType::U16 => {create_values(cpu.memory.read_vec::<u16>(addr, 64 / 16))}
                        ValueType::U32 => {create_values(cpu.memory.read_vec::<u32>(addr, 64 / 32))}
                        ValueType::U64 => {create_values(cpu.memory.read_vec::<u64>(addr, 64 / 64))}
                        ValueType::F32 => {create_values(Utilities::u32vec_to_f32vec(cpu.memory.read_vec::<u32>(addr, 64 / 32)))}
                        ValueType::F64 => {create_values(Utilities::u64vec_to_f64vec(cpu.memory.read_vec::<u64>(addr, 64 / 64)))}
                        _ => {create_values(vec![0u8; 1])}
                    };
                    values.iter().for_each(|value| {
                        let text = format!("{}", value);
                        let (layout_rect, _response) = ui.allocate_exact_size(size, Sense::hover());
                        if ui.is_rect_visible(layout_rect) {
                            ui.painter().rect_filled(layout_rect, 0.0, Color32::GRAY);
                            let mut font_size = 20f32;
                            let mut text_size;
                            loop {
                                let galley = ui.painter().layout_no_wrap(
                                    text.clone(),
                                    egui::FontId::new(font_size, egui::FontFamily::Monospace),
                                    Color32::BLACK,
                                );
                                text_size = galley.size().x;
                                if text_size < size.x - 4.0 {
                                    break;
                                }
                                font_size -= 1.0;
                                if font_size <= 1.0 {
                                    break;
                                }
                            }
                            let text_pos = layout_rect.min + size / 2.0 - Vec2::new(text_size / 2.0, font_size / 2.0);
                            let galley = ui.painter().layout_no_wrap(
                                text.clone(),
                                egui::FontId::new(font_size, egui::FontFamily::Monospace),
                                Color32::BLACK,
                            );
                            ui.painter().galley(text_pos, galley, Color32::TRANSPARENT);
                        }
                    });
                });
            });
        });
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                self.addr -= 16 * 8;
            }
            if ui.button("Next").clicked() {
                self.addr += 16 * 8;
            }
        });
    }
}
