#![allow(unused)]

use std::fs;

pub fn read_text(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
}

struct Planet {
    planet_type: PlanetType,
    x: i32,
    y: i32
}

struct Gaia;
struct Transdim;
struct Green;
struct Blue;
struct Brown;
struct Yellow;
struct Red;
struct Orange;
struct White;
struct Grey;

#[derive(Debug)]
enum PlanetType {
    Gaia,
    Transdim,
    Blue,
    Brown,
    Yellow,
    Red,
    Orange,
    White,
    Grey,
}

impl PlanetType {
  fn match_char_to_type(c: char) -> PlanetType {
    match c {
      '+' => PlanetType::Gaia,
      '*' => PlanetType::Transdim,
      'b' => PlanetType::Blue,
      'n' => PlanetType::Brown,
      'y' => PlanetType::Yellow,
      'r' => PlanetType::Red,
      'o' => PlanetType::Orange,
      'w' => PlanetType::White,
      'g' => PlanetType::Grey,
      _ => panic!()
    }
  }
}

pub struct Gmap {
    planets: Vec<Planet>,
}

impl Gmap {
    pub fn from_map_txt(map_txt: &String) -> Gmap {
        let mut this_map = Gmap {
            planets: Vec::new(),
        };
        let xi = 1;
        let yi = 1;
        let mut ptype: PlanetType;
        for m in map_txt.chars() {
            if "+*bnyrowg".contains(m) {
              ptype = PlanetType::match_char_to_type(m);
              this_map.planets.push(Planet { planet_type: ptype, x: xi, y: yi});
              
            }
        }
      return this_map;
    }
    
    pub fn print(self){
      for p in self.planets {
        println!("{},{}: {:?}", p.x, p.y, p.planet_type)
      }
    }
}
