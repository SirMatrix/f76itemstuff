use eframe::egui;
use egui::{TopBottomPanel, Layout, Label, Visuals, Checkbox, Button, Window, Color32, CentralPanel, ScrollArea};
use serde::{Serialize, Deserialize};
use tracing::Subscriber;
use super::sheets::*;


pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig{
     dark_mode: bool,

}

impl Default for AppConfig{
    fn default() -> Self {
        Self{dark_mode: Default::default()}
    }
}


#[derive(Default, Debug)]

pub struct MainApp{
    itemnames: String
}
#[derive(Default, Debug)]
struct StoredData{
  

}

impl MainApp {
   pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    fn fetch_data(&mut self ,ui: &mut eframe::egui::Ui, sheet: String){
        let gatherstrings = read_sheet_string(sheet.to_string(), true);
        let strings = gatherstrings.into_iter().collect();
        let x = MainApp{ itemnames: strings };
        for a in self.itemnames.chars(){
            ui.add_space(PADDING);
            let items = format!("{:?}", a);
            ui.colored_label(WHITE, items);

        }
       
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
                    });
                    ui.menu_button("Load Data",|ui|{
                        ui.menu_button("Collectibles", |ui|{
                            if ui.button("Bobble Heads").clicked(){
                             ScrollArea::both().show(ui, |ui|{
                                println!("Clicked");
                             });

                            }
                            if ui.button("Magazines").clicked(){
                                ui.close_menu();
                            }
                            if ui.button("Apparel").clicked(){
                                ui.close_menu();
                            }
                        });
                        ui.menu_button("Personal", |ui|{
                            if ui.button("For Trade").clicked(){
                                ui.close_menu();
                            }
                            if ui.button("Vendor").clicked(){
                                ui.close_menu();
                            }
                        })

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
            ctx.set_visuals(Visuals::dark());
          
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