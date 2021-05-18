use super::sort::Sort;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub run: Run,
    pub downloads: Downloads,
    pub aspect_ratio: AspectRatio,
    pub minimum_size: MinimumSize,
}

#[derive(Deserialize, Debug)]
pub struct Run {
    pub hold_on_job_done: bool,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
    pub subreddits: Vec<String>,
    pub sort: Sort,
    pub concurrency: u32,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct AspectRatio {
    pub enable: bool,
    pub height_aspect: u32,
    pub width_aspect: u32,
}

#[derive(Deserialize, Debug)]
pub struct MinimumSize {
    pub enable: bool,
    pub minimum_height: u32,
    pub minimum_width: u32,
}
