use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;

fn new_files(fname: String)
{
    let _file = File::create(format!("generated_files/{}",fname));

}
pub fn file_checker()
{
    let docs = vec!["bobbleheads.txt", "magazines.txt", "plans.txt", "apparel.txt"];
    let mut i  = docs.iter();
    if Path::new("generated_files").exists() == true{
        println!("Folder already exist!");
    }
    else
    {
        gen_dir();
        
    }
    while let Some(v) = i.next(){
       
        if Path::new(v).exists() == true{
            println!("Files exist!");
            break;
        }
        else
        {
            gen_files();
        }
    }



}
pub fn file_size()
{
    let docs = vec!["bobbleheads.txt", "magazines.txt", "plans.txt", "apparel.txt"];
    let mut i  = docs.iter();
    while let Some(v)= i.next(){
        let  f = File::open(format!("generated_files/{}", v)).expect("Couldn't open files!");
        let meta = f.metadata().expect("No Data Found!");
        if meta.len() == 0 {
          write_data();
        }
        else
        {
            println!("file:{},Len:{}", v, meta.len());
        }
    }
}

fn gen_dir()
{
      fs::create_dir("generated_files").expect("Failed to make the directroy");

}


fn gen_files()
{
    let docs = vec!["bobbleheads.txt", "magazines.txt", "plans.txt", "apparel.txt"];
    let mut i  = docs.iter();
    while let Some(v) = i.next(){
        new_files(v.to_string());

    }
    

}
fn write_data()
{
    let bobbles = ["\n","Bobblehead: Agility\n","Bobblehead: Big Guns\n","Bobblehead: Caps\n", "Bobblehead: Charisma\n", "Bobblehead: Endurance\n", "Bobblehead: Energy Weapons\n", "Bobblehead: Explosive\n", "Bobblehead: Intelligence\n","Bobblehead: Leader\n","Bobblehead: Lock Picking\n","Bobblehead: Luck\n","Bobblehead: Medicine\n","Bobblehead: Melee\n","Bobblehead: Perception\n","Bobblehead: Repaie\n","Bobblehead: Science\n","Bobblehead: Small Guns\n","Bobblehead: Sneak\n","Bobblehead: Strength\n","Bobblehead: Unarmed\n",];
    let magz = ["\n","Astoundingly Awesome Tales 1\n","Astoundingly Awesome Tales 2\n","Astoundingly Awesome Tales 3\n","Astoundingly Awesome Tales 4\n","Astoundingly Awesome Tales 5\n","Astoundingly Awesome Tales 6\n","Astoundingly Awesome Tales 7\n","Astoundingly Awesome Tales 8\n","Astoundingly Awesome Tales 9\n","Astoundingly Awesome Tales 10\n","Astoundingly Awesome Tales 11\n","Astoundingly Awesome Tales 12\n","Astoundingly Awesome Tales 13\n","Astoundingly Awesome Tales 14\n","Backwoodsman 1\n","Backwoodsman 2\n","Backwoodsman 3\n","Backwoodsman 4\n","Backwoodsman 5\n","Backwoodsman 6\n","Backwoodsman 7\n","Backwoodsman 8\n","Backwoodsman 9\n","Backwoodsman 10\n","Grognak the Barbarian 1\n","Grognak the Barbarian 2\n","Grognak the Barbarian 3\n","Grognak the Barbarian 4\n","Grognak the Barbarian 5\n","Grognak the Barbarian 6\n","Grognak the Barbarian 7\n","Grognak the Barbarian 8\n","Grognak the Barbarian 9\n","Grognak the Barbarian 10\n","Guns and Bullets 1\n","Guns and Bullets 2\n","Guns and Bullets 3\n","Guns and Bullets 4\n","Guns and Bullets 5\n","Guns and Bullets 6\n","Guns and Bullets 7\n","Guns and Bullets 8\n","Guns and Bullets 9\n","Guns and Bullets 10\n","Live & Love 1\n","Live & Love 2\n","Live & Love 3\n","Live & Love 4\n","Live & Love 5\n","Live & Love 6\n","Live & Love 7\n","Live & Love 8\n","Live & Love 9\n","Scouts' Life 1\n","Scouts' Life 2\n","Scouts' Life 3\n","Scouts' Life 4\n","Scouts' Life 5\n","Scouts' Life 6\n","Scouts' Life 7\n","Scouts' Life 8\n","Scouts' Life 9\n","Scouts' Life 10\n","Astonishing Tales 1\n","Astonishing Tales 2\n","Astonishing Tales 3\n","Astonishing Tales 4\n","Astonishing Tales 5\n","Tesla Science Magazine 1\n" ,"Tesla Science Magazine 2\n","Tesla Science Magazine 3\n","Tesla Science Magazine 4\n","Tesla Science Magazine 5\n","Tesla Science Magazine 6\n","Tesla Science Magazine 7\n","Tesla Science Magazine 8\n","Tesla Science Magazine 9\n","Tumblers Today 1\n","Tumblers Today 2\n","Tumblers Today 3\n","Tumblers Today 4\n","Tumblers Today 5\n","U.S. Covert Operations Manual 1\n","U.S. Covert Operations Manual 2\n","U.S. Covert Operations Manual 3\n","U.S. Covert Operations Manual 4\n","U.S. Covert Operations Manual 5\n","U.S. Covert Operations Manual 6\n","U.S. Covert Operations Manual 7\n","U.S. Covert Operations Manual 8\n","U.S. Covert Operations Manual 9\n","U.S. Covert Operations Manual 10\n","Unstoppables 1\n","Unstoppables 2\n","Unstoppables 3\n","Unstoppables 4\n","Unstoppables 5\n"];
    let apparel = ["\n","Asylum Worker Uniform Blue\n","Asylum Worker Uniform Green\n","Asylum Worker Uniform Pink\n","Asylum Worker Uniform Yellow\n","Chally the Moo-Moo Mask\n","Chally the Moo-Moo Outfit\n","Clean Spacesuit\n","Emmett Mountain Hazmat Suit\n","Fasnacht Baffoon Mask\n","Fasnacht Brahmin Mask\n","Fasnacht Crazy Guy Mask\n","Fasnacht Deathclaw Mask\n","Fasnacht Demon Mask\n","Fasnacht Fiend Mask\n","Fasnacht Hag Mask\n","Fasnacht Loon Mask\n","Fasnacht Raven Mask\n","Fisherman's Overalls\n","Grey Fisherman's Overalls\n","Hunter's Long Coat\n","Jack O'Lantern Pant Suit\n","Jack O'Lantern Short Suit\n","Longshoreman Outfit\n","Pirate Costume\n","Pirate Costume Hat\n","Prototype Hazmat Suit\n","Ranger Outfit Clean\n","Skiing Red And Green Outfit\n","Soiled Mr. Fuzzy Mascot Head\n","Soiled Mr. Fuzzy Mascot Suit\n","Straight Jacket Clean\n","Swamp Camo Hazmat Suit\n","Whitespring Jumpsuit\n","Winter Jacket and Jeans\n","Asylum Worker Uniform Forest\n","Asylum Worker Uniform Red\n","BOS Jumpsuit\n","Forest Camo Jumpsuit\n","Forest Scout Armor Mask\n","Leather Coat\n","Responder Fireman Helmet\n","Responder Fireman Uniform\n","Tattered Field Jacket\n","Traveling Leather Coat\n","Urban Scout Armor Mask\n","White Powder Jumpsuit\n"];
    let mut i = bobbles.into_iter();
    let mut m = magz.into_iter();
    let mut a = apparel.into_iter();
    if Path::new("generated_files/bobbleheads.txt").exists(){
       let mut f = File::create("generated_files/bobbleheads.txt").expect("Couldnt open file!");
        while let Some(v) = i.next(){

         f.write_all(v.as_bytes()).expect("Couldnt write to file :(");
        }

    }
    if Path::new("generated_files/magazines.txt").exists(){
     let mut f = File::create("generated_files/magazines.txt").expect("Couldn't open file!");
      while let Some(v) = m.next(){
        f.write_all(v.as_bytes()).expect("Couldn't write to file :(");
      }
    }
    if Path::new("generated_files/apparel.txt").exists(){
        let mut f  = File::create("generated_files/apparel.txt").expect("Couldn't open file");
        while let Some(v) = a.next(){
            f.write_all(v.as_bytes()).expect("Couldn't wrie to file :(");
        }
    }

}