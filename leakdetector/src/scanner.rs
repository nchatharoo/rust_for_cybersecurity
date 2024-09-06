use git2::Repository;
use walkdir::WalkDir;
use std::fs;
use crate::patterns::{Patterns, detect_secret};

pub fn scan_directory(dir: &str, patterns: &Patterns) {
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

pub fn scan_git_repo(repo_path: &str, patterns: &Patterns) {
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
