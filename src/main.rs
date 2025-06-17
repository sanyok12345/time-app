#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;

use app::ClockApp;

fn main() {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "Clock App",
        native_options,
        Box::new(|_cc| Box::new(ClockApp::new())),
    )
    .unwrap();
}
