use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path;

const SPECS_PATH: &str = "./resources/ship_specs";

#[derive(Deserialize)]
pub struct ShipSpec {
    pub fx: Fx,
    pub defenses: Defenses,
    pub power: Power,
    pub ship: Ship,
}

#[derive(Deserialize)]
pub struct Fx {
    pub image: String,
}

#[derive(Deserialize)]
pub struct Defenses {
    pub shield1: u8,
    pub shield2: u8,
    pub shield3: u8,
    pub shield4: u8,
    pub shield5: u8,
    pub shield6: u8,
    pub armor: u8,
}

#[derive(Deserialize)]
pub struct Power {
    pub left_warp: u8,
    pub center_warp: u8,
    pub right_warp: u8,
    pub impulse: u8,
    pub battery: u8,
}

#[derive(Deserialize)]
pub struct Ship {
    pub forward_hull: u8,
    pub aft_hull: u8,
}

impl ShipSpec {
    pub fn new(spec_file: &str) -> ShipSpec {
        let mut path = path::PathBuf::from(SPECS_PATH);
        path.push(spec_file);

        let mut file = match File::open(&path) {
            Err(err) => panic!("Failed to open {}: {}", path.display(), err),
            Ok(file) => file,
        };
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        match toml::from_str(&s) {
            Err(err) => panic!("Invalid ship spec '{}': {}", spec_file, err),
            Ok(s) => s,
        }
    }
}
