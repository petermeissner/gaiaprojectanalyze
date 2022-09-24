use std::{env};



mod helper; 



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
      panic!("Missing map file argument. Pease specify path to map file as commandline arguments.")
    }
    let file_name = &args[1];
    let file_data = helper::read_text(&file_name);
    println!("{}", file_data);
    let gm = helper::Gmap::from_map_txt(&file_data); 
    gm.print();
}
