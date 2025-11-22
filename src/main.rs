#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
pub mod module;
pub mod modules;
pub mod pipeline;

use eframe::egui;

rust_i18n::i18n!("locales");

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "YuriCypher",
        options,
        Box::new(|cc| Ok(Box::new(app::YuryCipherApp::new(cc)))),
    )
}
