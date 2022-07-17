mod utils;
use crate::utils::sheets::*;
use crate::utils::gui::*;
fn main() {
    utils::files::file_checker();
    utils::files::file_size();
    utils::sheets::make_document();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("F76ItemStuff", native_options, Box::new(|cc| Box::new(MainApp::new(cc))))
}


