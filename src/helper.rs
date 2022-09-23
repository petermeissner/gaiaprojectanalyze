
use std::{fs};


pub fn read_text(path: &str) -> String {
  let data = fs::read_to_string(path).expect("Unable to read file");
  return data;
}
