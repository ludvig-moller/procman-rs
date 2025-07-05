
use std::time::Instant;
use eframe::{egui::{Button, Color32, RichText}, *};
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

        if self.last_refresh.elapsed().as_millis() > 500 && self.live_refresh {
            self.last_refresh = Instant::now();
            self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Filter:");
                let filter_text_edit = egui::TextEdit::singleline(&mut self.filter).show(ui);
                if filter_text_edit.response.changed() {
                    self.last_refresh = Instant::now();
                    self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                }
                
                let mut button = Button::new("Sort CPU").fill(
                    if self.sort == "CPU" {Color32::BLUE} else {Color32::GRAY}
                );
                if ui.add(button).clicked() {
                    self.sort = "CPU".to_string();
                    self.last_refresh = Instant::now();
                    self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                }
                button = Button::new("Sort Memory").fill(
                    if self.sort == "RAM" {Color32::BLUE} else {Color32::GRAY}
                );
                if ui.add(button).clicked() {
                    self.sort = "RAM".to_string();
                    self.last_refresh = Instant::now();
                    self.processes = process_util::get_processes(&mut self.sys, self.sort.clone(), self.filter.clone());
                }
                button = Button::new("Live Refresh").fill(
                    if self.live_refresh {Color32::BLUE} else {Color32::GRAY}
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

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    for process_info in self.processes.iter() {
                        let text = if self.selected_pid != None && self.selected_pid.unwrap() == process_info.pid {
                            RichText::new(format!("{} CPU: {}% Memory: {}%", 
                            process_info.name.clone(), process_info.cpu_usage.to_string(), process_info.memory_usage.to_string()))
                                .color(Color32::BLUE)
                        } else {
                            RichText::new(format!("{} CPU: {}% Memory: {}%", 
                            process_info.name.clone(), process_info.cpu_usage.to_string(), process_info.memory_usage.to_string()))
                        };

                        if ui.label(text).clicked() {
                            self.selected_pid = if self.selected_pid != None && self.selected_pid.unwrap() == process_info.pid {None} else {Some(process_info.pid)}
                        };
                    }
                });
            
        });
    }
}

pub fn run() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();

    run_native(
        "Process Manager",
        native_options,
        Box::new(
            |_cc: &CreationContext<'_>| {
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
