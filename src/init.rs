use std::{path::PathBuf, fs};


#[derive(Deserialize, Debug)]
pub struct SimpleData {
    pub symbol: char,
    pub color: Vec<u8>,
    pub bg_color: Vec<u8>,
}
#[derive(Deserialize, Debug)]
pub struct ComplexData {
    pub symbol: char,
    pub priority: i8,
    pub endings: Vec<String>,
    pub color: Vec<u8>,
    pub bg_color: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dir: SimpleData,
    pub other: SimpleData,
    pub files: Vec<ComplexData>,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let file = fs::read_to_string(path).unwrap();
        serde_json::from_str(&file).unwrap()
    }
}