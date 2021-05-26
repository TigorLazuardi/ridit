use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;

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
    pub fn get_file_location<P: AsRef<Path>>(&self, base_location: P) -> PathBuf {
        let b = shellexpand::full(base_location.as_ref().to_str().unwrap())
            .unwrap()
            .to_string();
        Path::new(b.as_str())
            .join(&self.subreddit_name)
            .join(&self.filename)
            .absolutize()
            .unwrap()
            .to_path_buf()
    }
}
