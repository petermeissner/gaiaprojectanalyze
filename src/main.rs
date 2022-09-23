use std::{env, fs};



mod helper; 




fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_data = helper::read_text(&file_name);
    println!("{}", file_data);
}
