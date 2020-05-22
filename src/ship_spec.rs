use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path;

const SPECS_PATH: &str = "./resources/ship_specs";

#[derive(Deserialize)]
pub struct ShipSpec {
    pub fx: Fx,
    pub ship: Ship,
}

#[derive(Deserialize)]
pub struct Fx {
    pub image: String,
}

#[derive(Deserialize)]
pub struct Ship {
    pub shield_1: i16,
    pub shield_2: i16,
    pub shield_3: i16,
    pub shield_4: i16,
    pub shield_5: i16,
    pub shield_6: i16,
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

        let spec: ShipSpec = toml::from_str(&s).unwrap();
        spec
    }
}
