
use std::time::Instant;
use image;
use eframe::{ egui::{Button, Color32}, * };
use sysinfo::System;

use crate::process_util;

struct ProcessManagerApp {
    sys: System,
    last_refresh: Instant,
    live_refresh: bool,
    processes: Vec<process_util::ProcessInfo>,
    selected_pid: Option<u32>,
    sort: String,
    filter: String,
}

impl eframe::App for ProcessManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mouse1_clicked = ctx.input(|i| { i.pointer.button_clicked(egui::PointerButton::Primary) });

        if self.last_refresh.elapsed().as_millis() > 500 && self.live_refresh {
            self.last_refresh = Instant::now();
            self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
        }

        // Actions
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
            let layout = if ui.available_width() >= 630.0 { 
                egui::Layout::left_to_right(egui::Align::Center) 
            } else {
                egui::Layout::top_down(egui::Align::Center) 
            };
            
            ui.with_layout(layout, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    let filter_text_edit = egui::TextEdit::singleline(&mut self.filter).show(ui);
                    if filter_text_edit.response.changed() {
                        self.last_refresh = Instant::now();
                        self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                    }
                });

                ui.horizontal(|ui| {
                    let mut button = Button::new("Sort CPU").fill(
                        if self.sort == "CPU" { Color32::LIGHT_BLUE } else { Color32::LIGHT_GRAY }
                    );
                    if ui.add(button).clicked() {
                        self.sort = "CPU".to_string();
                        self.last_refresh = Instant::now();
                        self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                    }
                    button = Button::new("Sort Memory").fill(
                        if self.sort == "RAM" { Color32::LIGHT_BLUE } else { Color32::LIGHT_GRAY }
                    );
                    if ui.add(button).clicked() {
                        self.sort = "RAM".to_string();
                        self.last_refresh = Instant::now();
                        self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                    }
                    button = Button::new("Live Refresh").fill(
                        if self.live_refresh { Color32::LIGHT_BLUE } else { Color32::LIGHT_GRAY }
                    );
                    if ui.add(button).clicked() {
                        self.live_refresh = if self.live_refresh {false} else {true}
                    }
                    if ui.button("Refresh").clicked() {
                        self.last_refresh = Instant::now();
                        self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                    }
                    if ui.button("Kill").clicked() {
                        if self.selected_pid != None {
                            process_util::kill_process(&mut self.sys, self.selected_pid.unwrap())
                        }
                    }
                });
            });
        });

        // Processes
        egui::CentralPanel::default()
            .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    for process_info in self.processes.iter() {
                        let border_color = if self.selected_pid != None && self.selected_pid.unwrap() == process_info.pid { 
                            Color32::LIGHT_BLUE 
                        } else {
                            Color32::LIGHT_GRAY 
                        };

                        let frame = egui::Frame::group(ui.style())
                            .stroke(egui::Stroke::new(2.0, border_color))
                            .inner_margin(egui::Margin::symmetric(10, 6));
                        
                        let group = frame.show(ui, |ui| {
                            ui.set_min_size(egui::vec2(ui.available_width(), 20.0));

                            ui.horizontal(|ui| {
                                ui.label(process_info.name.clone());

                                ui.add_space(ui.available_width()-200.0);
                                ui.label(format!("CPU: {}%", process_info.cpu_usage));

                                ui.add_space(ui.available_width()-100.0);
                                ui.label(format!("Memory: {}%", process_info.memory_usage));
                            });
                        });

                        if ui.rect_contains_pointer(group.response.rect) && mouse1_clicked {
                            self.selected_pid = if self.selected_pid != None && self.selected_pid.unwrap() == process_info.pid {None} else {Some(process_info.pid)}
                        };
                    }
                });
            
        });
    }
}

fn set_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("Inter".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/Inter_28pt-SemiBold.ttf")).into());
    fonts.families.entry(egui::FontFamily::Proportional).or_default()
        .insert(0, "Inter".to_owned());
    ctx.set_fonts(fonts);
}

fn load_icon() -> egui::IconData {
    let image = image::open(std::path::Path::new("./assets/icon-256x256.png"));
    if !image.is_ok() { return egui::IconData::default() }
    let rgba = image.unwrap().into_rgba8().into_raw();
    egui::IconData { rgba, width: 256, height: 256 }
}

pub fn run() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder:: default()
            .with_inner_size([500.0, 300.0])
            .with_min_inner_size([500.0, 300.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    run_native(
        "ProcMan-rs",
        native_options,
        Box::new(
            |cc| {
                set_font(&cc.egui_ctx);

                let mut sys = System::new_all();
                let sort = "CPU".to_string();
                let filter = "".to_string();
                sys.refresh_all();

                let processes = process_util::get_processes(&mut sys, sort.clone(), filter.clone());

                Ok(Box::new(ProcessManagerApp {
                    sys,
                    last_refresh: Instant::now(),
                    live_refresh: true,
                    processes,
                    selected_pid: None,
                    sort,
                    filter,
                }))
            }
        ),
    )
}
