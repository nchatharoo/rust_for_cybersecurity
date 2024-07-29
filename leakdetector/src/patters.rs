use std::fs;
use serde_json;

impl Patterns {
    pub fn from_file(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).expect("unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON)")
    }
}