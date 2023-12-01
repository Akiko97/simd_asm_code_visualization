// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::HashMap;
use std::thread::sleep;
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{self, Vec2, Pos2, Context,  CentralPanel, Window, SidePanel, TopBottomPanel, Ui, Id, Sense, CursorIcon, LayerId, Order, InnerResponse, Shape, Rect, epaint, Label, Slider, ComboBox, Color32};
use std::sync::{Arc, RwLock};
mod reg_visualizer;
mod visualizer_setting;
mod utilities;

use reg_visualizer::{RegVisualizer, LayoutLocation};
use visualizer_setting::{VisualizerSetting};
use utilities::*;

struct RegVisualizerData {
    // Registers Data
    registers: Vec<Vec<Register>>,
    vector_regs_type: HashMap<(VecRegName, usize), ValueType>,
    gprs_type: HashMap<GPRName, UIntFloat>,
    // Animation Data
    velocity: f32,
}

impl Default for RegVisualizerData {
    fn default() -> Self {
        Self {
            // Registers Data
            registers: vec![vec![]],
            vector_regs_type: HashMap::new(),
            gprs_type: HashMap::new(),
            // Animation Data
            velocity: 10f32,
        }
    }
}

struct APP {
    // Data
    cpu: CPU,
    reg_visualizer_data: RegVisualizerData,
    // Windows
    register_visualizer: RegVisualizer,
    visualizer_setting: VisualizerSetting,
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
            // Data
            cpu: CPU::default(),
            reg_visualizer_data: RegVisualizerData::default(),
            // Windows
            register_visualizer: RegVisualizer::default(),
            visualizer_setting: VisualizerSetting::default(),
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
                self.visualizer_setting.show(ui, &mut self.reg_visualizer_data);
            });
        Window::new("Visualizer")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_visualizer)
            .show(ctx, |ui| {
                // Debug
                if ui.button("Add").clicked() {
                    self.register_visualizer.create_animation_layout(Register::vector(VecRegName::YMM, 0), LayoutLocation::BOTH);
                }
                if ui.button("Remove").clicked() {
                    self.register_visualizer.remove_animation_layout(Register::vector(VecRegName::YMM, 0));
                }
                // Show the register visualizer
                let delta_time = ctx.input(|input|{
                    input.unstable_dt
                });
                self.register_visualizer.update(delta_time, self.reg_visualizer_data.velocity);
                self.register_visualizer.show(ui, &self.reg_visualizer_data, &self.cpu);
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
