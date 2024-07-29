use patterns::Patterns;
pub mod patterns;

fn main() {
    let patterns = Patterns::from_file("patterns.json");
    for pattern in patterns.patterns {
        println!(
            "Loaded pattern: {} with regex: {}",
            pattern.name, pattern.regex
        );
    }
}
