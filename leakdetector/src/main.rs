mod github;
mod patterns;
mod scanner;

use patterns::Patterns;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let patterns = Patterns::from_file("patterns.json");
    let arg = env::args().nth(1).expect("Please provide a directory or URL");

    if arg.starts_with("https://github.com/") {
        github::scan_github_repo(&arg, &patterns).await;
    } else if std::path::Path::new(&arg).join(".git").exists() {
        scanner::scan_git_repo(&arg, &patterns);
    } else {
        scanner::scan_directory(&arg, &patterns);
    }
}
