use eframe::egui;
use egui::{TopBottomPanel, Layout, Label};


#[derive(Default)]

pub struct MainApp{}

impl MainApp {
   pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    fn menu_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        TopBottomPanel::top("top_panel").show(ctx, |ui|{
            ui.add_space(10.);
            egui::menu::bar(ui, |ui|{
                ui.with_layout(Layout::left_to_right(), |ui|{
                    ui.menu_button("File", |ui|{
                      if ui.button("Quit").clicked(){
                        ui.close_menu();
                      }
                    })
                });
            });
        });

    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Test");
            self.menu_bar(ctx, frame);
        });
    }

}

  /*let bobblesS = utils::sheets::read_sheet_string("Bobble Heads".to_string(), true);
    let mut b: String = bobblesS.into_iter().collect();
    let magz = utils::sheets::read_sheet_string("Magazines".to_string(), true);
    let mut m: String = magz.into_iter().collect();
    let app = utils::sheets::read_sheet_string("Apparel".to_string(), true);
    let mut a: String = app.into_iter().collect();
    let tr = utils::sheets::read_sheet_string("Trade".to_string(), true);
    let mut t: String = tr.into_iter().collect();
    let vend = utils::sheets::read_sheet_string("My Vendor".to_string(), true);
    let mut v: String = vend.into_iter().collect(); 
    // Todo Impliment these to generate text boxes.
    */