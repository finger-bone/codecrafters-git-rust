use std::path::{Path, PathBuf};
use reqwest::Url;
pub fn clone(url: &Url, directory: Option<&Path>) {
    let repo_name = url
        .path_segments()
        .expect("bad path 1")
        .last()
        .expect("bad path 2");
    let repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
    let directory_default = PathBuf::from(repo_name);
    let directory = directory.unwrap_or(&directory_default);
    git2::build::RepoBuilder::new()
        .clone(&url.to_string(), directory)
        .expect("failed to clone");
}
