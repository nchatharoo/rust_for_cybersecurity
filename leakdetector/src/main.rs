pub mod githubfile;
pub mod patterns;
use patterns::Patterns;
use patterns::Pattern;
use regex::Regex;
use walkdir::WalkDir;
use std::fs;
use std::env;
use git2::Repository;
use reqwest::Client;
use githubfile::GithubFile;
use tokio;

#[tokio::main]
async fn main() {
    let patterns = Patterns::from_file("patterns.json");
    let arg = env::args().nth(1).expect("Please provide a directory or URL");
    
    if arg.starts_with("https://github.com/") {
        scan_github_repo(&arg, &patterns).await;
    } else {
        if std::path::Path::new(&arg).join(".git").exists() {
            scan_git_repo(&arg, &patterns);
        } else {
            scan_directory(&arg, &patterns);
        }    
    }
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

fn scan_git_repo(repo_path: &str, patterns: &Patterns) {
    let repo = Repository::open(repo_path).expect("Failed to open Git repository");
    let mut revwalk = repo.revwalk().expect("Failed to get revwalk");
    revwalk.push_head().expect("Failed to push head");

    for oid in revwalk {
        let oid = oid.expect("Failed to get oid");
        let commit = repo.find_commit(oid).expect("Failed to find commit");
        let tree = commit.tree().expect("Failed to get tree");

        tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
            if let Some(blob) = entry.to_object(&repo).ok().and_then(|obj| obj.as_blob().cloned()) {
                let content = std::str::from_utf8(blob.content()).unwrap_or("");
                for pattern in &patterns.patterns {
                    detect_secret(entry.name().unwrap_or("unknown"), content, pattern);
                }
            }
            git2::TreeWalkResult::Ok
        }).expect("Tree walk failed");
    }
}

async fn scan_github_repo(repo_url: &String, patterns: &Patterns) {
    let repo_url = repo_url.trim_end_matches('/');
    let api_url = format!("{}/contents", repo_url.replace("https://github.com/", "https://api.github.com/repos/"));
    let http_client = Client::new();

    // For debug
    let response_text = http_client
        .get(&api_url)
        .header("User-Agent", "request")
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to read response text");

        
    // println!("Raw response: {}", response_text);

    let response: Vec<GithubFile> = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    for file in response {
        if file.file_type == "file" {
            if let Some(download_url) = file.download_url {
                
                // Skip images
                if download_url.ends_with(".png") || download_url.ends_with(".jpg") || download_url.ends_with(".jpeg") || download_url.ends_with(".gif") {
                    println!("Skipping binary file: {}", download_url);
                    continue;
                }

                println!("GET file URL: {:?}", download_url);                

                let content = http_client
                .get(&download_url)
                .header("User-Agent", "request")
                .send()
                .await
                .expect("Failed to send request")
                .text()
                .await
                .expect("Failed to parse JSON");

                for pattern in &patterns.patterns {
                    detect_secret(&file.path, &content, pattern);
                }
            }
        }
    }
}