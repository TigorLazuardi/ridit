use std::path::PathBuf;

use super::{default, sort::Sort};
use anyhow::{Context, Result};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub run: Run,
    pub downloads: Downloads,
    pub aspect_ratio: AspectRatio,
    pub minimum_size: MinimumSize,
    pub advanced: Advanced,
    pub symbolic_link: SymbolicLink,
}

impl Config {
    pub fn get_download_path(&self) -> PathBuf {
        let b = shellexpand::full(self.downloads.path.as_str()).unwrap();
        PathBuf::new()
            .join(b.as_ref())
            .absolutize()
            .unwrap()
            .to_path_buf()
    }

    pub fn create_dirs(&self) -> Result<()> {
        let p = self.get_download_path();
        for subreddit in self.downloads.subreddits.iter() {
            let p = p.join(subreddit.as_str());
            fs::create_dir_all(p.clone())
                .with_context(|| format!("failed to create directory on {}", p.display()))?;
        }
        Ok(())
    }

    pub fn print_config() -> Result<PathBuf> {
        default::print_config()
    }

    pub fn check_config_exists() -> (bool, bool) {
        default::check_config_exists()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Run {
    pub hold_on_job_done: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Downloads {
    pub subreddits: Vec<String>,
    pub sort: Sort,
    pub path: String,
    pub timeout: u64,
    pub download_timeout: u64,
    pub nsfw: bool,
    pub proceed_download_on_file_exist: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AspectRatio {
    pub enable: bool,
    pub height_aspect: usize,
    pub width_aspect: usize,
    pub ratio_range: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinimumSize {
    pub enable: bool,
    pub minimum_height: usize,
    pub minimum_width: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Advanced {
    pub user_agent: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolicLink {
    pub enable: bool,
    pub use_custom_path: bool,
    pub custom_path: String,
}
