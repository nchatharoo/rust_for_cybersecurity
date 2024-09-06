use reqwest::Client;
use serde::Deserialize;
use crate::patterns::{Patterns, detect_secret};

#[derive(Debug, Deserialize)]
pub struct GithubFile {
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    pub file_type: String,
    pub path: String,
}

pub async fn scan_github_repo(repo_url: &String, patterns: &Patterns) {
    let repo_url = repo_url.trim_end_matches('/');
    let api_url = format!("{}/contents", repo_url.replace("https://github.com/", "https://api.github.com/repos/"));
    let http_client = Client::new();

    let response_text = http_client
        .get(&api_url)
        .header("User-Agent", "request")
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to read response text");

    let response: Vec<GithubFile> = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    for file in response {
        if file.file_type == "file" {
            if let Some(download_url) = file.download_url {
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
