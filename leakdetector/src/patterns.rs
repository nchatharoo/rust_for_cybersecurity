use serde::Deserialize;
use std::fs;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub regex: String,
}

#[derive(Debug, Deserialize)]
pub struct Patterns {
    pub patterns: Vec<Pattern>,
}

impl Patterns {
    pub fn from_file(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).expect("unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    }
}

pub fn detect_secret(file_path: &str, content: &str, pattern: &Pattern) {
    let re = Regex::new(&pattern.regex).expect("Invalid regex");
    for mat in re.find_iter(content) {
        println!("Found {} in {}: {}", pattern.name, file_path, mat.as_str());
    }
}
