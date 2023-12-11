// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{CPU, Utilities, u256, u512, VecRegName, GPRName};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{self, Vec2, Pos2, Context,  CentralPanel, Window, SidePanel, TopBottomPanel, Ui, Id, Sense, CursorIcon, LayerId, Order, InnerResponse, Shape, Rect, epaint, Label, Slider, ComboBox, Color32};
use std::sync::{Arc, Mutex};

mod reg_visualizer;
mod visualizer_setting;
mod utilities;
mod reg_visualizer_data;
mod animation_fsm;
mod instruction_actuator;

use reg_visualizer::{RegVisualizer, LayoutLocation, ElementAnimationData};
use visualizer_setting::{VisualizerSetting};
use utilities::*;
use reg_visualizer_data::RegVisualizerData;
use crate::animation_fsm::{AnimationFSM, FSMCtrlMsg};
use instruction_actuator::*;

struct APP {
    // Data
    cpu: Arc<Mutex<CPU>>,
    reg_visualizer_data: RegVisualizerData,
    // Windows
    register_visualizer: Arc<Mutex<RegVisualizer>>,
    visualizer_setting: VisualizerSetting,
    animation_fsm: AnimationFSM,
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
            cpu: Arc::new(Mutex::new(CPU::default())),
            reg_visualizer_data: RegVisualizerData::default(),
            // Windows
            register_visualizer: Arc::new(Mutex::new(RegVisualizer::default())),
            visualizer_setting: VisualizerSetting::default(),
            animation_fsm: AnimationFSM::default(),
            // Code Editor
            code: "vaddps ymm0, ymm1, ymm0
vaddps ymm0, ymm2, ymm0
vaddps ymm0, ymm3, ymm0
vhaddps ymm0, ymm0, ymm0
vhaddps ymm0, ymm0, ymm0
vperm2f128 ymm1, ymm0, ymm0, 1
vaddps ymm0, ymm1, ymm0".into(),
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
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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
                        if self.highlight < self.code.lines().count() {
                            self.highlight += 1;
                            if self.highlight > 0 {
                                execute(self.register_visualizer.clone(), self.cpu.clone(), &mut self.animation_fsm, &self.reg_visualizer_data, ctx,
                                        self.code.lines().collect::<Vec<&str>>()[self.highlight - 1]);
                            }
                        } else {
                            self.highlight = 0;
                        }
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
                if ui.button("test").clicked() {
                    let mut cpu = self.cpu.lock().unwrap();
                    cpu.registers.set_by_sections::<u32>(VecRegName::YMM, 0, Utilities::f32vec_to_u32vec(vec![
                        1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32
                    ]));
                    cpu.registers.set_by_sections::<u32>(VecRegName::YMM, 1, Utilities::f32vec_to_u32vec(vec![
                        8f32, 7f32, 6f32, 5f32, 4f32, 3f32, 2f32, 1f32
                    ]));
                }
                // Show the register visualizer
                let delta_time = ctx.input(|input|{
                    input.unstable_dt
                });
                let mut register_visualizer = self.register_visualizer.lock().unwrap();
                register_visualizer.update(delta_time, self.reg_visualizer_data.velocity);
                let cpu = self.cpu.lock().unwrap();
                register_visualizer.show(ui, &self.reg_visualizer_data, &cpu);
                drop(cpu);
                register_visualizer.move_animation_sequence(ctx);
                register_visualizer.move_animation_finish(ctx);
                if register_visualizer.is_animating() {
                    ctx.request_repaint();
                }
                drop(register_visualizer);
                // Run Animation FSM
                self.animation_fsm.run();
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
