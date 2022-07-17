mod utils;
use egui::Vec2;

use crate::utils::sheets::*;
use crate::utils::gui::*;
fn main() {
  tracing_subscriber::fmt::init();
    utils::files::file_checker();
    utils::files::file_size();
    utils::sheets::make_document();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native("F76ItemStuff", native_options, Box::new(|cc| Box::new(MainApp::new(cc))))
}


