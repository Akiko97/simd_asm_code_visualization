// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{ CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName };
use egui_code_editor::{ CodeEditor, ColorTheme, Syntax };
use std::collections::HashMap;
use eframe::{ App, Frame, NativeOptions };
use eframe::egui::{ self, Vec2, Pos2, Context,  CentralPanel, Window };

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
        }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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
        Window::new("Window 1")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top()))
            .show(ctx, |ui| {
            ui.label("Hello");
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
