use super::sort::Sort;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub run: Run,
    pub downloads: Downloads,
    pub aspect_ratio: AspectRatio,
    pub minimum_size: MinimumSize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Run {
    pub hold_on_job_done: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Downloads {
    pub subreddits: Vec<String>,
    pub sort: Sort,
    pub concurrency: usize,
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AspectRatio {
    pub enable: bool,
    pub height_aspect: usize,
    pub width_aspect: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinimumSize {
    pub enable: bool,
    pub minimum_height: usize,
    pub minimum_width: usize,
}
