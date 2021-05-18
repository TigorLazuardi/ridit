use super::models::meta::DownloadMeta;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub trait RedditRepository {
    fn get_downloads(&self) -> Receiver<DownloadMeta>;
    fn download_images(
        &self,
        lists: Receiver<DownloadMeta>,
        blocklist: HashMap<&str, &str>,
    ) -> Receiver<DownloadMeta>;
}
