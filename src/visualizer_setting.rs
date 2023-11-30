use eframe::egui::{Ui, Id, Sense, CursorIcon, LayerId, Order, InnerResponse, Vec2, Shape, Rect, epaint, Label, Slider, ComboBox};
use cpulib::{VecRegName, GPRName};
use super::*;

#[derive(PartialEq)]
enum RegType {
    GPR, Vector
}

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
            let delta = pointer_pos - response.rect.center();
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
    registers: Vec<Vec<String>>,
    velocity: f32,
    reg_type: RegType,
    gpr_name: GPRName,
    vec_name: VecRegName,
    vec_index: usize,
    data_size: usize,
}

impl Default for VisualizerSetting {
    fn default() -> Self {
        Self {
            registers: vec![
                vec![],
            ],
            velocity: 10f32,
            reg_type: RegType::Vector,
            gpr_name: GPRName::RAX,
            vec_name: VecRegName::YMM,
            vec_index: 0,
            data_size: 32,
        }
    }
}

impl VisualizerSetting {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.label("Setting Velocity:");
        let slider_response = ui.add(
            Slider::new(&mut self.velocity, 10.0..=100.0)
                .logarithmic(true)
                .text("Velocity"),
        );
        if slider_response.changed() {
            //
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
                        ui.selectable_value(&mut self.gpr_name, GPRName::RAX, "RAX");
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
                    ui.label("Data Size:");
                    ui.radio_value(&mut self.data_size, 8, "8");
                    ui.radio_value(&mut self.data_size, 16, "16");
                    ui.radio_value(&mut self.data_size, 32, "32");
                    ui.radio_value(&mut self.data_size, 64, "64");
                    ui.radio_value(&mut self.data_size, 64, "128");
                    if self.vec_name == VecRegName::YMM || self.vec_name == VecRegName::ZMM {
                        ui.radio_value(&mut self.data_size, 64, "256");
                    }
                    if self.vec_name == VecRegName::ZMM {
                        ui.radio_value(&mut self.data_size, 64, "512");
                    }
                    ui.label("Bits");
                });
            }
        }
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                //
            };
            if ui.button("Add/Edit").clicked() {
                //
            };
            if ui.button("Delete").clicked() {
                //
            };
        });
        // Order
        let id_source = "visualizer_setting_registers";
        let mut source_col_row = None;
        let mut drop_col = None;
        let mut drop_row = None;
        ui.columns(self.registers.len(), |uis| {
            for (col_idx, column) in self.registers.clone().into_iter().enumerate() {
                let ui = &mut uis[col_idx];
                let can_accept_what_is_being_dragged = true;
                let response = drop_target(ui, can_accept_what_is_being_dragged, |ui| {
                    ui.set_min_size(Vec2::new(64.0, 100.0));
                    for (row_idx, item) in column.iter().enumerate() {
                        let item_id = Id::new(id_source).with(col_idx).with(row_idx);

                        let drop_response = drop_target(ui, can_accept_what_is_being_dragged, |ui| {
                            drag_source(ui, item_id, |ui| {
                                let response = ui.add(Label::new(item).sense(Sense::click()));
                                response.context_menu(|ui| {
                                    if ui.button("Remove").clicked() {
                                        self.registers[col_idx].remove(row_idx);
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
                        self.registers[col_idx].push("New Item".to_owned());
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
                        let item = self.registers[source_col].remove(source_row);
                        let insert_at = if source_col != drop_col {
                            drop_row
                        } else {
                            if source_row >= drop_row {
                                drop_row
                            } else {
                                drop_row - 1
                            }
                        };
                        self.registers[drop_col].insert(insert_at, item);
                    }
                } else {
                    // without drop row
                    if ui.input(|i| i.pointer.any_released()) {
                        let item = self.registers[source_col].remove(source_row);
                        self.registers[drop_col].push(item);
                    }
                }
            }
        }
    }
}
