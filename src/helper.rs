#![allow(unused)]

use std::{fs, ops::{Add, Deref}, fmt, iter::repeat};

pub fn read_text(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
}

struct Planet {
    planet_type: PlanetType,
    x: i32,
    y: i32,
    i: i32
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

#[derive(Debug, Clone, Copy)]
pub enum PlanetType {
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
  
  fn match_type_to_char (pt: PlanetType) -> char {
    match pt {
      PlanetType::Gaia     => '+',
      PlanetType::Transdim => '*',
      PlanetType::Blue     => 'b',
      PlanetType::Brown    => 'n',
      PlanetType::Yellow   => 'y',
      PlanetType::Red      => 'r',
      PlanetType::Orange   => 'o',
      PlanetType::White    => 'w',
      PlanetType::Grey     => 'g',
      _ => panic!()
    }
  }
}

impl fmt::Display for PlanetType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
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
        let mut xi = 0;
        let mut yi = 1;
        let mut ptype: PlanetType;
        let mut pi = 0;
        for m in map_txt.chars() {
            // next x
            xi = xi + 1; 
            
            // next y
            if "\n".contains(m) {
              yi = yi + 1;
              xi = 1;
            }
            
            // planet?
            if "+*bnyrowg".contains(m) {
              // next planet id
              pi = pi + 1;
              // get correct type
              ptype = PlanetType::match_char_to_type(m);
              // add new planet
              this_map.planets.push(Planet { planet_type: ptype, x: xi, y: yi, i: pi});
            }
        }
      return this_map;
    }
    
    pub fn as_list(&self) -> String {
      let mut map_string = String::from("");
      
      for p in &self.planets {
        map_string += "[";
        map_string += &p.x.to_string();
        map_string += ",";
        map_string += &p.y.to_string();
        map_string += "] ";
        map_string += &p.planet_type.to_string();
        map_string += "\n";
      }
      
      return map_string;
    }
    
    pub fn x(&self) -> Vec<i32> {
      let mut x: Vec<i32> = Vec::new();
      for p in &self.planets {
        x.push(p.x);
      }
      return x;
    }
    
    pub fn y(&self) -> Vec<i32> {
      let mut y: Vec<i32> = Vec::new();
      for p in &self.planets {
        y.push(p.y);
      }
      return y;
    }
    
    pub fn t(&self) -> Vec<PlanetType> {
      let mut t: Vec<PlanetType> = Vec::new();
      for p in &self.planets {
        t.push(p.planet_type);
      }
      return t;
    }
    
    pub fn x_max (&self) -> i32 {
      let x_vec = self.x();
      return *x_vec.iter().max().unwrap();
    }
    
    pub fn y_max (&self) -> i32 {
      let y_vec = self.y();
      return *y_vec.iter().max().unwrap();
    }
    
    pub fn as_map(&self) -> String {
      let mut map_string = String::from("");

      let max_x = self.x_max();
      let max_y = self.y_max();
      
      for yi in 0..max_y {
        for xi in 0..max_x {
          let mut found = false;
          for p in &self.planets {
            if p.x == xi && p.y == yi {
              map_string += " "; 
              map_string += &PlanetType::match_type_to_char(p.planet_type).to_string();
              map_string += " ";
              found = true;
              continue;
            }
          }
          if found {
            // do nothing 
          } else {
            map_string += "   ";
          }
        }
        map_string += "\n"; 
      }
      
      return map_string;
    }
    
    pub fn adj_matrix () {
      
    }
}


struct Resources {
  might_gaia: i32,
  might_1: i32,
  might_2: i32,
  might_3: i32,
  ore: i32,
  lab: i32,
  qic: i32,
  coin: i32
}



struct TechTiles;
struct Research;

struct Player {
  points: i32,
  resources: Resources,
  research: Research,
  tech_tiles: TechTiles,
}


