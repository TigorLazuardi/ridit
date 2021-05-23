use std::path::{Path, PathBuf};

pub struct DownloadMeta {
    pub url: String,
    pub subreddit_name: String,
    pub image_height: u32,
    pub image_width: u32,
    pub post_link: String,
    pub nsfw: bool,
    pub filename: String,
    pub title: String,
    pub author: String,
}

impl DownloadMeta {
    pub fn get_file_location(&self, base_location: &str) -> PathBuf {
        Path::new(base_location)
            .join(self.subreddit_name.as_str())
            .join(self.filename.as_str())
    }
}
