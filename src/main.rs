// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::HashMap;
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{self, Vec2, Pos2, Context,  CentralPanel, Window, SidePanel, TopBottomPanel};

mod reg_visualizer;
use reg_visualizer::{RegVisualizer, Value};

struct APP {
    // CPU Emulator
    cpu: CPU,
    // Register
    register_visualizer: RegVisualizer,
    // Code Editor
    code: String,
    highlight: usize,
    // Layout
    show_sidebar: bool,
    show_preference: bool,
    show_settings: bool,
    show_visualizer: bool,
    show_memory: bool,
}

impl Default for APP {
    fn default() -> Self {
        Self {
            // CPU Emulator
            cpu: CPU::default(),
            // Register
            register_visualizer: RegVisualizer::default(),
            // Code Editor
            code: "".into(),
            highlight: 0,
            // Layout
            show_sidebar: true,
            show_preference: false,
            show_settings: false,
            show_visualizer: false,
            show_memory: false,
        }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Visualizer").triple_clicked() {
                    todo!()
                }
                ui.add_space(20.0);
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Open..").clicked() {
                        todo!()
                    }
                    if ui.button("Save..").clicked() {
                        todo!()
                    }
                });
                egui::menu::menu_button(ui, "View", |ui| {
                    if ui.selectable_label(self.show_sidebar, "Sidebar").clicked() {
                        self.show_sidebar = !self.show_sidebar;
                    }
                    if ui.selectable_label(self.show_preference, "Preference").clicked() {
                        self.show_preference = !self.show_preference;
                    }
                });
                egui::menu::menu_button(ui, "Help", |ui| {
                    if ui.button("Help..").clicked() {
                        todo!()
                    }
                    if ui.button("About").clicked() {
                        todo!()
                    }
                });
            });
        });
        if self.show_sidebar {
            SidePanel::left("side_panel").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Visualization Options:");
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("Visualizer").clicked() {
                        self.show_visualizer = true;
                    }
                    if ui.button("Memory").clicked() {
                        self.show_memory = true;
                    }
                    ui.label("Debug Options:");
                    if ui.button("Settings").clicked() {
                        //
                    }
                    if ui.button("Run").clicked() {
                        //
                    }
                    if ui.button("Step").clicked() {
                        //
                    }
                    if ui.button("Undo").clicked() {
                        //
                    }
                });
            });
        }
        CentralPanel::default()
            .show(ctx, |ui| {
                // show a code editor on central panel
                CodeEditor::default()
                    .id_source("code_editor")
                    .with_rows(24)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GRUVBOX)
                    .with_syntax(Syntax::asm())
                    .with_numlines(true)
                    .show(ui, &mut self.code, &mut self.highlight);
            });
        Window::new("Preference")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_preference)
            .show(ctx, |ui| {
                ui.label("Preference");
            });
        Window::new("Settings")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.label("Settings");
            });
        Window::new("Visualizer")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_visualizer)
            .show(ctx, |ui| {
                if ui.button("add").clicked() {
                    self.register_visualizer.insert_vector(VecRegName::XMM, 0, vec![
                        Value::U32(0), Value::U32(0), Value::U32(0), Value::U32(0),
                    ]);
                    self.register_visualizer.insert_gpr(GPRName::RAX, Value::U64(0));
                    self.register_visualizer.insert_vector(VecRegName::ZMM, 0, vec![
                        Value::U512(u512::max_value())
                    ]);
                    self.register_visualizer.insert_gpr(GPRName::AH, Value::U8(255));
                }
                if ui.button("remove").clicked() {
                    self.register_visualizer.remove_vector(VecRegName::XMM, 0);
                    self.register_visualizer.remove_gpr(GPRName::RAX);
                    self.register_visualizer.remove_vector(VecRegName::ZMM, 0);
                    self.register_visualizer.remove_gpr(GPRName::AH);
                }
                let delta_time = ctx.input(|input|{
                    input.unstable_dt
                });
                self.register_visualizer.update(delta_time);
                self.register_visualizer.show(ui);
                if self.register_visualizer.is_animating() {
                    ctx.request_repaint();
                }
            });
        Window::new("Memory")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_memory)
            .show(ctx, |ui| {
                ui.label("Memory");
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200f32, 800f32]),
        ..Default::default()
    };
    eframe::run_native(
        "SIMD Assembly Code Visualization Tool",
        options,
        Box::new(|_cc| Box::new(APP::default())),
    )
}
