// DO NOT REMOVE - hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpulib::{CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName};
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
mod mem_visualizer;

use reg_visualizer::{RegVisualizer, LayoutLocation, ElementAnimationData};
use visualizer_setting::{VisualizerSetting};
use utilities::*;
use reg_visualizer_data::RegVisualizerData;
use crate::animation_fsm::{AnimationFSM};
use instruction_actuator::*;
use mem_visualizer::{MemVisualizer};

struct APP {
    // Data
    cpu: Arc<Mutex<CPU>>,
    reg_visualizer_data: RegVisualizerData,
    // Windows
    register_visualizer: Arc<Mutex<RegVisualizer>>,
    visualizer_setting: VisualizerSetting,
    animation_fsm: AnimationFSM,
    memory_visualizer: MemVisualizer,
    // Code Editor
    code: String,
    highlight: usize,
    // Layout
    show_sidebar: bool,
    show_preference: bool,
    show_settings: bool,
    show_visualizer: bool,
    show_memory: bool,
    show_wip: bool,
    show_about: bool,
    show_help: bool,
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
            memory_visualizer: MemVisualizer::default(),
            // Code Editor
            code: "".into(),
            highlight: 0,
            // Layout
            show_sidebar: true,
            show_preference: false,
            show_settings: false,
            show_visualizer: false,
            show_memory: false,
            show_wip: false,
            show_about: false,
            show_help: true,
        }
    }
}

impl APP {
    fn step(&mut self, ctx: &Context, with_animation: bool) {
        if self.highlight < self.code.lines().count() {
            self.highlight += 1;
            if self.highlight > 0 {
                let instruction = self.code.lines().collect::<Vec<&str>>()[self.highlight - 1];
                let s: String = instruction.into();
                if s.ends_with(":") {
                    // Do nothing
                } else if s.starts_with("jne ") {
                    let cpu = self.cpu.lock().unwrap();
                    let flag = cpu.registers.get_flags_value(FLAGSName::RFLAGS);
                    drop(cpu);
                    let ctrl = (flag >> 6) & 0b1; // Get ZF
                    if ctrl == 0 {
                        let parts: Vec<&str> = s.split_whitespace().collect();
                        if parts.len() == 2 {
                            let label: String = (*parts.get(1).unwrap()).into();
                            self.code.lines().enumerate().for_each(|(index, tmp)| {
                                let mut tmp: String = tmp.into();
                                tmp.pop();
                                if label == tmp {
                                    self.highlight = index + 1;
                                }
                            });
                        }
                    }
                } else {
                    execute(self.register_visualizer.clone(), self.cpu.clone(), &mut self.animation_fsm, &self.reg_visualizer_data, ctx, instruction, with_animation);
                }
            }
        } else {
            self.highlight = 0;
        }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals::dark()); // Use dark mode
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Visualizer").triple_clicked() {
                    self.show_about = true;
                }
                ui.add_space(20.0);
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Open..").clicked() {
                        self.show_wip = true;
                    }
                    if ui.button("Save..").clicked() {
                        self.show_wip = true;
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
                        self.show_help = true;
                    }
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });
        if self.show_sidebar {
            SidePanel::left("side_panel")
                .default_width(150.0)
                .min_width(150.0)
                .max_width(200.0)
                .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Visualization Options:");
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                        let button_width = ui.available_width();

                        if ui.add_sized([button_width, 0.0], egui::SelectableLabel::new(self.show_settings, "Settings")).clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.add_sized([button_width, 0.0], egui::SelectableLabel::new(self.show_visualizer, "Visualizer")).clicked() {
                            self.show_visualizer = !self.show_visualizer;
                        }
                        if ui.add_sized([button_width, 0.0], egui::SelectableLabel::new(self.show_memory, "Memory")).clicked() {
                            self.show_memory = !self.show_memory;
                        }
                    });
                    ui.label("Debug Options:");
                    let button_width = ui.available_width();
                    if ui.add_sized([button_width, 0.0], egui::Button::new("Step with Animation")).clicked() {
                        self.step(ctx, true);
                    }
                    if ui.add_sized([button_width, 0.0], egui::Button::new("Step without Animation")).clicked() {
                        self.step(ctx, false);
                    }
                    ui.label("DEMO:");
                    if ui.add_sized([button_width, 0.0], egui::Button::new("Prefix Sum")).clicked() {
                        self.code = "valignd zmm1, zmm0, zmm2, 15
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 14
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 12
vpaddd zmm0, zmm0, zmm1
valignd zmm1, zmm0, zmm2, 8
vpaddd zmm0, zmm0, zmm1".into();
                        self.cpu = Arc::new(Mutex::new(CPU::default()));
                        let mut cpu = self.cpu.lock().unwrap();
                        cpu.registers.set_by_sections::<u32>(VecRegName::ZMM, 0, vec![
                            1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                            9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32
                        ]);
                        drop(cpu);
                        self.reg_visualizer_data.registers[0].clear();
                        (0..3).for_each(|i| {
                            self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::ZMM, i));
                        });
                        self.reg_visualizer_data.vector_regs_type.clear();
                        (0..3).for_each(|i| {
                            self.reg_visualizer_data.vector_regs_type.insert((VecRegName::ZMM, i), ValueType::U32);
                        });
                    }
                    if ui.add_sized([button_width, 0.0], egui::Button::new("Matrix Transpose")).clicked() {
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
                        self.cpu = Arc::new(Mutex::new(CPU::default()));
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
                        self.reg_visualizer_data.registers[0].clear();
                        (0..16).for_each(|i| {
                            self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::YMM, i));
                        });
                        self.reg_visualizer_data.vector_regs_type.clear();
                        (0..16).for_each(|i| {
                            self.reg_visualizer_data.vector_regs_type.insert((VecRegName::YMM, i), ValueType::U32);
                        });
                    }
                    if ui.add_sized([button_width, 0.0], egui::Button::new("Matrix Multiplication")).clicked() {
                        self.code = "loop:
vmovapd ymm15, [0x40000000 + rdi]
vextractf128 xmm14, ymm15, 0
vbroadcastsd ymm4, xmm14
shufpd xmm14, xmm14, 1
vbroadcastsd ymm5, xmm14
vextractf128 xmm14, ymm15, 1
vbroadcastsd ymm6, xmm14
shufpd xmm14, xmm14, 1
vbroadcastsd ymm7, xmm14
vmulpd ymm7, ymm3, ymm7
vfmadd213pd ymm6, ymm2, ymm7
vfmadd213pd ymm5, ymm1, ymm6
vfmadd213pd ymm4, ymm0, ymm5
vmovapd [0x40000080 + rdi], ymm4
add rdi, 32
cmp rdi, 128
jne loop".into();
                        self.cpu = Arc::new(Mutex::new(CPU::default()));
                        let mut cpu = self.cpu.lock().unwrap();
                        cpu.registers.set_by_sections::<u64>(VecRegName::YMM, 0, Utilities::f64vec_to_u64vec(vec![
                            16f64, 15f64, 14f64, 13f64,
                        ]));
                        cpu.registers.set_by_sections::<u64>(VecRegName::YMM, 1, Utilities::f64vec_to_u64vec(vec![
                            12f64, 11f64, 10f64, 9f64,
                        ]));
                        cpu.registers.set_by_sections::<u64>(VecRegName::YMM, 2, Utilities::f64vec_to_u64vec(vec![
                            8f64, 7f64, 6f64, 5f64,
                        ]));
                        cpu.registers.set_by_sections::<u64>(VecRegName::YMM, 3, Utilities::f64vec_to_u64vec(vec![
                            4f64, 3f64, 2f64, 1f64,
                        ]));
                        cpu.memory.write_vec::<u64>(0x40000000, Utilities::f64vec_to_u64vec(vec![
                            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
                        ]));
                        drop(cpu);
                        self.reg_visualizer_data.registers[0].clear();
                        (0..8).for_each(|i| {
                            self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::YMM, i));
                        });
                        self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::YMM, 15));
                        self.reg_visualizer_data.registers[0].push(Register::vector(VecRegName::XMM, 14));
                        self.reg_visualizer_data.registers[0].push(Register::gpr(GPRName::RDI));
                        self.reg_visualizer_data.vector_regs_type.clear();
                        (0..8).for_each(|i| {
                            self.reg_visualizer_data.vector_regs_type.insert((VecRegName::YMM, i), ValueType::F64);
                        });
                        self.reg_visualizer_data.vector_regs_type.insert((VecRegName::YMM, 15), ValueType::F64);
                        self.reg_visualizer_data.vector_regs_type.insert((VecRegName::XMM, 14), ValueType::F64);
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
                let cpu = self.cpu.lock().unwrap();
                self.memory_visualizer.show(ui, ctx, &cpu);
                drop(cpu);
            });
        Window::new("About")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_about)
            .show(ctx, |ui| {
                ui.heading("PixelAssemblySIMD");
                ui.label("PixelAssemblySIMD is a visualization tool for assembly language SIMD instructions, designed to help beginners and programmers understand SIMD operations. This project is part of Zhong's Master's thesis at Ueda Lab., the Department of Computer Science and Communications Engineering, the Graduate School of Fundamental Science and Engineering of Waseda University.");
                ui.hyperlink_to("GitHub", "https://github.com/Akiko97/simd_asm_code_visualization");
                ui.hyperlink_to("Demo", "https://www.gabzhong.dev/simd_asm_code_visualization/");
                ui.add_space(12.0);
                ui.vertical_centered(|ui| {
                    ui.label("2024 | Built with Rust & egui");
                });
            });
        Window::new("Help")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_help)
            .show(ctx, |ui| {
                ui.heading("README");
                ui.label("You can quickly load a demo using the button under 'DEMO' on the sidebar. The tool is still in development, and currently supports all SIMD instructions used in the algorithms showcased in 'DEMO'. In the settings on the sidebar, you can customize the animation settings, including animation speed, the order of register display, and more.");
                //ui.label("サイドバーの「DEMO」の下にあるボタンを使用して、デモをすばやく読み込むことができます。このツールはまだ開発中であり、現在「DEMO」で紹介されているアルゴリズムで使用されるすべてのSIMD命令をサポートしています。サイドバーの設定で、アニメーションの速度、レジスタ表示順序など、アニメーションの設定をカスタマイズすることができます。");
                //ui.label("您可以使用侧边栏中“DEMO”下的按钮直接快速加载Demo。该工具仍在开发中，目前支持“DEMO”中展示的算法中使用的所有SIMD指令。在侧边栏的设置中您可以进行动画的设置，包括动画速度、寄存器展示顺序等。");
            });
        Window::new("Features WIP")
            .default_pos(Pos2::new(ctx.available_rect().right() - 200.0, ctx.available_rect().top() + 20.0))
            .open(&mut self.show_wip)
            .show(ctx, |ui| {
                ui.label("This feature is under development");
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
