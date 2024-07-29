use patterns::Patterns;
use patterns::Pattern;
pub mod patterns;
use regex::Regex;
use walkdir::WalkDir;
use std::fs;

fn main() {
    let patterns = Patterns::from_file("patterns.json");
    let dir = std::env::args().nth(1).expect("Please provide a directory");
    scan_directory(&dir, &patterns);
}

fn scan_directory(dir: &str, patterns: &Patterns) {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path().display().to_string();
            let content = fs::read_to_string(&file_path).unwrap_or_default();
            for pattern in &patterns.patterns {
                detect_secret(&file_path, &content, pattern);
            }
        }
    }
}

fn detect_secret(file_path: &str, content: &str, pattern: &Pattern) {
    let re = Regex::new(&pattern.regex).expect("Invalid regex");
    for mat in re.find_iter(content) {
        println!("Found {} in {}: {}", pattern.name, file_path, mat.as_str());
    }
}
