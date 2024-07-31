use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubFile {
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    pub file_type: String,
    pub path: String,
}