// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{ CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName };
use egui_code_editor::{ CodeEditor, ColorTheme, Syntax };
use std::collections::HashMap;
use eframe::{ App, Frame, NativeOptions };
use eframe::egui::{ self, Vec2, Pos2, Context,  CentralPanel, Window, SidePanel, TopBottomPanel };

enum ValueType {
    U32, U64, U128, U256, U512, F32, F64
}

struct Element {
    position: (f32, f32),
    target_position: (f32, f32),
    animating: bool,
    value_type: ValueType,
    value_u32: u32,
    value_u64: u64,
    value_u128: u128,
    value_u256: u256,
    value_u512: u512,
    value_f32: f32,
    value_f64: f64,
}

struct APP {
    // CPU Emulator
    cpu: CPU,
    using_vector_registers: Vec<VecRegName>,
    using_gpr: Vec<GPRName>,
    // Animation
    elements: HashMap<String, Element>,
    animation_speed: f32,
    // Code Editor
    code: String,
    highlight: usize,
    // Layout
    show_sidebar: bool,
    show_preference: bool,
    show_settings: bool,
    show_visualizer: bool,
}

impl Default for APP {
    fn default() -> Self {
        Self {
            // CPU Emulator
            cpu: CPU::default(),
            using_vector_registers: vec![],
            using_gpr: vec![],
            // Animation
            elements: HashMap::new(),
            animation_speed: 0.1f32,
            // Code Editor
            code: "".into(),
            highlight: 0,
            // Layout
            show_sidebar: true,
            show_preference: false,
            show_settings: false,
            show_visualizer: false,
        }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                ui.label("Visualizer");
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut options = NativeOptions::default();
    eframe::run_native(
        "SIMD Assembly Code Visualization",
        options,
        Box::new(|_cc| Box::new(APP::default())),
    )
}
