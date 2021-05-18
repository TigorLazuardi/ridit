use super::domain::RedditRepository;
use super::models::meta::DownloadMeta;
use crate::app::config::model::Config;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct Repository {
    config: Config,
}

impl Repository {
    pub fn new(config: Config) -> Repository {
        Repository { config }
    }
}

impl RedditRepository for Repository {
    fn get_downloads(&self) -> Receiver<DownloadMeta> {
        let (tx, rx): (Sender<DownloadMeta>, Receiver<DownloadMeta>) = channel();
        let cfg = self.config.clone();
        thread::spawn(move || {
            let pool = ThreadPoolBuilder::new()
                .num_threads(cfg.downloads.concurrency)
                .build()
                .expect("failed to create threads");
        });
        rx
    }

    fn download_images(
        &self,
        lists: Receiver<DownloadMeta>,
        blocklist: HashMap<&str, &str>,
    ) -> Receiver<DownloadMeta> {
        todo!();
    }
}
