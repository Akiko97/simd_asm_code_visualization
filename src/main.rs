// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{CPU, Utilities, u256, u512, VecRegName, GPRName};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use eframe::{App, Frame};
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
                    ui.label("For TEST:");
                    if ui.button("Prefix Sum").clicked() {
                        self.code = "valignd zmm1, zmm0, zmm2, 15
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 14
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 12
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 8
vpaddd zmm0, zmm0, zmm1".into();
                        let mut cpu = self.cpu.lock().unwrap();
                        cpu.registers.set_by_sections::<u32>(VecRegName::ZMM, 0, vec![
                            1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                            9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32
                        ]);
                        drop(cpu);
                        (0..3).for_each(|i| {
                            self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::ZMM, i));
                        });
                        (0..3).for_each(|i| {
                            self.reg_visualizer_data.vector_regs_type.insert((VecRegName::ZMM, i), ValueType::U32);
                        });
                    }
                    if ui.button("Matrix Transpose").clicked() {
                        self.code = "vunpcklps ymm8, ymm0, ymm1
vunpcklps ymm9, ymm2, ymm3
vunpcklps ymm10, ymm4, ymm5
vunpcklps ymm11, ymm6, ymm7
vunpckhps ymm12, ymm0, ymm1
vunpckhps ymm13, ymm2, ymm3
vunpckhps ymm14, ymm4, ymm5
vunpckhps ymm15, ymm6, ymm7
vshufps ymm0, ymm8, ymm9, 0b01000100
vshufps ymm1, ymm8, ymm9, 0b11101110
vshufps ymm2, ymm10, ymm11, 0b01000100
vshufps ymm3, ymm10, ymm11, 0b11101110
vshufps ymm4, ymm12, ymm13, 0b01000100
vshufps ymm5, ymm12, ymm13, 0b11101110
vshufps ymm6, ymm14, ymm15, 0b01000100
vshufps ymm7, ymm14, ymm15, 0b11101110
vperm2f128 ymm8, ymm0, ymm2, 0x20
vperm2f128 ymm9, ymm1, ymm3, 0x20
vperm2f128 ymm10, ymm0, ymm2, 0x31
vperm2f128 ymm11, ymm1, ymm3, 0x31
vperm2f128 ymm12, ymm4, ymm6, 0x20
vperm2f128 ymm13, ymm5, ymm7, 0x20
vperm2f128 ymm14, ymm4, ymm6, 0x31
vperm2f128 ymm15, ymm5, ymm7, 0x31".into();
                        let mut cpu = self.cpu.lock().unwrap();
                        (0u32..16u32).for_each(|i| {
                            let vec = if i < 8 {
                                ((i * 8 + 1)..(i * 8 + 9)).map(|x| x).collect()
                            } else {
                                vec![0; 8]
                            };
                            cpu.registers.set_by_sections::<u32>(VecRegName::YMM, i as usize, vec);
                        });
                        drop(cpu);
                        (0..16).for_each(|i| {
                            self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::YMM, i));
                        });
                        (0..16).for_each(|i| {
                            self.reg_visualizer_data.vector_regs_type.insert((VecRegName::YMM, i), ValueType::U32);
                        });
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
                // Show the register visualizer
                let delta_time = ctx.input(|input|{
                    input.unstable_dt
                });
                let mut register_visualizer = self.register_visualizer.lock().unwrap();
                register_visualizer.update(delta_time, self.reg_visualizer_data.factor, self.reg_visualizer_data.min_speed, self.reg_visualizer_data.max_speed);
                let cpu = self.cpu.lock().unwrap();
                register_visualizer.show(ui, ctx, &self.reg_visualizer_data, &cpu);
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

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200f32, 800f32]),
        ..Default::default()
    };
    eframe::run_native(
        "SIMD Assembly Code Visualization Tool",
        options,
        Box::new(|_cc| Box::new(APP::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(APP::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
