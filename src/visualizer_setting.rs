use eframe::egui::{Ui, Id, Sense, CursorIcon, LayerId, Order, InnerResponse, Vec2, Shape, Rect, epaint, Label, Slider, ComboBox};
use cpulib::{VecRegName, GPRName};
use super::*;

fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

    if !is_being_dragged {
        let response = ui.scope(body).response;

        // Check for drags:
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::Grab);
        }
    } else {
        ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Now we move the visuals of the body to where the mouse is.
        // Normally you need to decide a location for a widget first,
        // because otherwise that widget cannot interact with the mouse.
        // However, a dragged component cannot be interacted with anyway
        // (anything with `Order::Tooltip` always gets an empty [`Response`])
        // So this is fine!

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            // let delta = pointer_pos - response.rect.center();
            let delta = pointer_pos - response.rect.left_center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

    let margin = Vec2::splat(4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        fill = ui.visuals().gray_out(fill);
        stroke.color = ui.visuals().gray_out(stroke.color);
    }

    ui.painter().set(
        where_to_put_background,
        epaint::RectShape::new(rect, style.rounding, fill, stroke),
    );

    InnerResponse::new(ret, response)
}

pub struct VisualizerSetting {
    velocity: f32,
    reg_type: RegType,
    gpr_name: GPRName,
    vec_name: VecRegName,
    vec_index: usize,
    data_type: ValueType,
}

impl Default for VisualizerSetting {
    fn default() -> Self {
        Self {
            velocity: 0f32,
            reg_type: RegType::Vector,
            gpr_name: GPRName::RAX,
            vec_name: VecRegName::YMM,
            vec_index: 0,
            data_type: ValueType::U32,
        }
    }
}

macro_rules! create_gpr_selectable {
    ($ui:expr; $self:expr; $($reg:ident),*) => {
        $(
            $ui.selectable_value(&mut $self.gpr_name, GPRName::$reg, stringify!($reg));
        )*
    };
}

impl VisualizerSetting {
    pub fn show(&mut self, ui: &mut Ui, data: &mut RegVisualizerData) {
        // init
        self.velocity = data.velocity;
        // UI
        ui.label("Setting Velocity:");
        let slider_response = ui.add(
            Slider::new(&mut self.velocity, 10.0..=1000.0)
                .logarithmic(true)
                .text("Velocity"),
        );
        if slider_response.changed() {
            data.velocity = self.velocity;
        }
        ui.label("Setting Registers:");
        ui.horizontal(|ui| {
            ui.label("Type:");
            ui.selectable_value(&mut self.reg_type, RegType::Vector, "Vector");
            ui.selectable_value(&mut self.reg_type, RegType::GPR, "GPR");
        });
        match self.reg_type {
            RegType::GPR => {
                ComboBox::from_label("GPR")
                    .selected_text(format!("{}", self.gpr_name))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        create_gpr_selectable!(ui; self;
                            // 64-bit registers
                            RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
                            R8, R9, R10, R11, R12, R13, R14, R15,
                            // 32-bit registers
                            EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP,
                            R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D,
                            // 16-bit registers
                            AX, BX, CX, DX, SI, DI, BP, SP,
                            R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W,
                            // 8-bit registers
                            AH, BH, CH, DH, AL, BL, CL, DL, SIL, DIL, BPL, SPL,
                            R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B);
                    });
            }
            RegType::Vector => {
                ComboBox::from_label("Vector Register")
                    .selected_text(format!("{}", self.vec_name))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut self.vec_name, VecRegName::XMM, "XMM");
                        ui.selectable_value(&mut self.vec_name, VecRegName::YMM, "YMM");
                        ui.selectable_value(&mut self.vec_name, VecRegName::ZMM, "ZMM");
                    });
                ComboBox::from_label("Index")
                    .selected_text(format!("{}", self.vec_index))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for i in 0..16 {
                            ui.selectable_value(&mut self.vec_index, i, format!("{}", i));
                        }
                    });
                ui.horizontal(|ui| {
                    ui.label("Data Type:");
                    ui.radio_value(&mut self.data_type, ValueType::U8, "U8");
                    ui.radio_value(&mut self.data_type, ValueType::U16, "U16");
                    ui.radio_value(&mut self.data_type, ValueType::U32, "U32");
                    ui.radio_value(&mut self.data_type, ValueType::F32, "F32");
                    ui.radio_value(&mut self.data_type, ValueType::U64, "U64");
                    ui.radio_value(&mut self.data_type, ValueType::F64, "F64");
                    ui.radio_value(&mut self.data_type, ValueType::U128, "U128");
                    if self.vec_name == VecRegName::YMM || self.vec_name == VecRegName::ZMM {
                        ui.radio_value(&mut self.data_type, ValueType::U256, "U256");
                    }
                    if self.vec_name == VecRegName::ZMM {
                        ui.radio_value(&mut self.data_type, ValueType::U512, "U512");
                    }
                });
            }
            _ => {/*None: Do nothing, there is NO possible to run into here!*/}
        }
        ui.horizontal(|ui| {
            if ui.button("Add/Update").clicked() {
                match self.reg_type {
                    RegType::GPR => {
                        if !data.registers[0].iter().any(|r| *r == self.gpr_name) {
                            data.registers[0].push(Register::gpr(self.gpr_name));
                        }
                    }
                    RegType::Vector => {
                        if !data.registers[0].iter().any(|r| *r == (self.vec_name, self.vec_index)) {
                            data.registers[0].push(Register::vector(self.vec_name, self.vec_index));
                        }
                        data.vector_regs_type.insert((self.vec_name, self.vec_index), self.data_type);
                    }
                    _ => {/*None: Do nothing, there is NO possible to run into here!*/}
                }
            };
            if ui.button("Delete").clicked() {
                match self.reg_type {
                    RegType::GPR => {
                        data.registers[0].retain(|r| *r != self.gpr_name);
                    }
                    RegType::Vector => {
                        data.registers[0].retain(|r| *r != (self.vec_name, self.vec_index));
                        data.vector_regs_type.remove(&(self.vec_name, self.vec_index));
                    }
                    _ => {/*None: Do nothing, there is NO possible to run into here!*/}
                }
            };
        });
        // Order
        ui.label("Order:");
        let id_source = "visualizer_setting_registers";
        let mut source_col_row = None;
        let mut drop_col = None;
        let mut drop_row = None;
        ui.columns(data.registers.len(), |uis| {
            for (col_idx, column) in data.registers.clone().into_iter().enumerate() {
                let ui = &mut uis[col_idx];
                let can_accept_what_is_being_dragged = true;
                let response = drop_target(ui, can_accept_what_is_being_dragged, |ui| {
                    ui.set_min_size(Vec2::new(64.0, 100.0));
                    for (row_idx, item) in column.iter().enumerate() {
                        let item_id = Id::new(id_source).with(col_idx).with(row_idx);

                        let drop_response = drop_target(ui, can_accept_what_is_being_dragged, |ui| {
                            drag_source(ui, item_id, |ui| {
                                let response = ui.add(Label::new(format!("{}", item)).sense(Sense::click()));
                                response.context_menu(|ui| {
                                    if ui.button("Remove").clicked() {
                                        data.registers[col_idx].remove(row_idx);
                                        ui.close_menu();
                                    }
                                });
                            });
                        });

                        if drop_response.response.hovered() {
                            if ui.memory(|mem| mem.is_anything_being_dragged()) {
                                drop_col = Some(col_idx);
                                drop_row = Some(row_idx);
                            }
                        }

                        if ui.memory(|mem| mem.is_being_dragged(item_id)) {
                            source_col_row = Some((col_idx, row_idx));
                        }
                    }
                })
                    .response;

                let response = response.context_menu(|ui| {
                    if ui.button("New Item").clicked() {
                        data.registers[col_idx].push(Register::none());
                        ui.close_menu();
                    }
                });

                let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());
                if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
                    drop_col = Some(col_idx);
                }
            }
        });

        if let Some((source_col, source_row)) = source_col_row {
            if let Some(drop_col) = drop_col {
                if let Some(drop_row) = drop_row {
                    // with drop row
                    if ui.input(|i| i.pointer.any_released()) {
                        let item = data.registers[source_col].remove(source_row);
                        data.registers[drop_col].insert(drop_row, item);
                    }
                } else {
                    // without drop row
                    if ui.input(|i| i.pointer.any_released()) {
                        let item = data.registers[source_col].remove(source_row);
                        data.registers[drop_col].push(item);
                    }
                }
            }
        }
    }
}
