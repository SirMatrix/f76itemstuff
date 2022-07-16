use umya_spreadsheet::*;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn make_document(){
    let fileexist = Path::new("items.xlsx").exists();
   
    if fileexist == true{
        println!("File Exist!");
    }else{
    let book = new_file();
    let path = Path::new("items.xlsx");
    let _ = writer::xlsx::write(&book, path);
    add_items();
    }
}

 fn add_items(){
    let bobbles = File::open("generated_files/bobbleheads.txt").expect("Couldn't open file!");
    let magz = File::open("generated_files/magazines.txt").expect("Couldn't open file!");
    let apparel = File::open("generated_files/apparel.txt").expect("Couldn't open file!");
    let m  = BufReader::new(magz);
    let b = BufReader::new(bobbles);
    let a = BufReader::new(apparel);
    let mut cnt = 0;
    let spreadsheet = Path::new("items.xlsx");
    let mut book = reader::xlsx::lazy_read(spreadsheet).unwrap();
    let _ = book.new_sheet("Bobble Heads");
    let _ = book.new_sheet("Magazines");
    let _ = book.new_sheet("Apparel");
    let _ = book.new_sheet("Plans");
    let _ = book.new_sheet("My Vendor");
    let _ = book.new_sheet("Trade");
    let _ = book.remove_sheet_by_name("Sheet1");
    for line in b.lines().filter_map(|result| result.ok()){
        let  add_bobbles= vec![line];
        let mut i = add_bobbles.iter();
        while let Some(v) = i.next() {
        book.get_sheet_by_name_mut("Bobble Heads").unwrap().get_cell_by_column_and_row_mut(&1, &cnt).set_value(v);
        book.get_sheet_by_name_mut("Bobble Heads").unwrap().get_cell_by_column_and_row_mut(&2, &cnt).set_value_from_i32(0); //We want to default the value to 0 since we don't know if we have any of the items yet
        cnt = cnt +1; 

        }
        
    }
    cnt = 0;
    for line in m.lines().filter_map(|result| result.ok()){
        let add_mags = vec![line];
        let mut i = add_mags.iter();
        while let Some(v) = i.next(){
            book.get_sheet_by_name_mut("Magazines").unwrap().get_cell_by_column_and_row_mut(&1, &cnt).set_value(v);
            book.get_sheet_by_name_mut("Magazines").unwrap().get_cell_by_column_and_row_mut(&2, &cnt).set_value_from_i32(0);
            cnt = cnt +1;
        }
    }
    cnt = 0;
    for line in a.lines().filter_map(|result| result.ok()){
        let add_app = vec![line];
        let mut i = add_app.iter();
        while let Some(v) = i.next(){
            book.get_sheet_by_name_mut("Apparel").unwrap().get_cell_by_column_and_row_mut(&1, &cnt).set_value(v);
            book.get_sheet_by_name_mut("Apparel").unwrap().get_cell_by_column_and_row_mut(&2, &cnt).set_value_from_i32(0);
            cnt = cnt +1;

        }
    }

    let _ = writer::xlsx::write(&book, spreadsheet);
}

pub fn writer(sheet: String, column: String, row: String, value: i32)
{

}
pub fn read_sheet_string(sheet: String, ini: bool) -> Vec<String>
{
    let spreadsheet = Path::new("items.xlsx");
    let mut cnt = 0;
    let mut init = ini;
    let mut book = reader::xlsx::lazy_read(spreadsheet).unwrap();
    let mut vecthing = Vec::new();
    while init == true {
        let mut b = book.get_sheet_by_name_mut(&sheet).unwrap().get_value_by_column_and_row(&1, &cnt);
        let a  = [b.as_mut_str()];
        let mut i = a.into_iter();
        while let Some(v) = i.next() {
            vecthing.push(v.to_string());
            
        }
        cnt = cnt + 1;
        if cnt > 98 {
           //hacky way to read all the cells
           //TODO: Make it read the cells that have values only so we dont get extra space when viewing in the program.
            break;
        }
        
    }
    init = false;
   vecthing
}
pub fn read_sheet_val(sheet: String, ini: bool) -> Vec<String>
{
    let spreadsheet = Path::new("items.xlsx");
    let mut cnt = 0;
    let mut init = ini;
    let mut book = reader::xlsx::lazy_read(spreadsheet).unwrap();
    let mut vecthing = Vec::new();
    while init == true{
        let mut b = book.get_sheet_by_name_mut(&sheet).unwrap().get_value_by_column_and_row(&2, &cnt);
        let a = [b.as_mut_str()];
        let mut i = a.into_iter();
        while let Some(v) = i.next(){
            vecthing.push(v.to_string())
        }
        cnt = cnt +1;
        if cnt > 98{
            break;
        }
        
    }
    init = false;
    vecthing

}
