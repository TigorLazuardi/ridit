use super::models::meta::DownloadMeta;
use crate::app::config::sort::Sort;
use std::error::Error;

/// Trait for RedditRepository. Must never touch with concurrency in this level
pub trait RedditRepository {
    /// Get download list from listing subreddit.
    fn get_downloads(
        &self,
        subreddit_name: &str,
        sort: Sort,
        blocklist: &Vec<String>,
    ) -> Result<Vec<DownloadMeta>, Box<dyn Error>>;
    /// Actually download the image
    fn download_images(&self, download: DownloadMeta) -> Result<(), Box<dyn Error>>;
}
