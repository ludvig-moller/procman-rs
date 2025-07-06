
#![windows_subsystem = "windows"]

mod gui;
mod process_util;

fn main() -> eframe::Result<(), eframe::Error> {
    gui::run()
}
