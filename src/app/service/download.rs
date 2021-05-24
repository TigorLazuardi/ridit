use crate::app::config::model::Config;
use crate::app::reddit::repository::Repository;
use std::sync::{mpsc, Arc};
use tokio::runtime::Handle;
use tokio::sync::oneshot;

pub struct DownloadService {
    repo: Arc<Repository>,
    config: Arc<Config>,
}

impl DownloadService {
    pub fn new(repo: Arc<Repository>, config: Arc<Config>) -> DownloadService {
        DownloadService { repo, config }
    }

    pub async fn start_download(&self) {
        for subreddit in self.config.downloads.subreddits.iter() {
            let blocklist: Vec<String> = Vec::new();
            match self
                .repo
                .get_downloads(subreddit.as_str(), self.config.downloads.sort, &blocklist)
                .await
            {
                Err(err) => println!("[{}] Failed to get listing. Cause: {}", subreddit, err),
                Ok(downloads) => {
                    let responses = self
                        .repo
                        .download_images(self.config.downloads.path.as_str(), downloads)
                        .await;
                    self.repo
                        .store_images(self.config.downloads.path.as_str(), responses)
                        .await;
                }
            }
        }
    }

    pub async fn start_concurrent_download(&self) {
        let mut receivers = Vec::new();
        for subreddit in self.config.downloads.subreddits.iter() {
            let (tx, rx) = oneshot::channel();
            let handle = Handle::current();
            let repo = self.repo.clone();
            let config = self.config.clone();
            let subreddit = subreddit.clone();
            rayon::spawn(move || {
                handle.spawn(async move {
                    let blocklist: Vec<String> = Vec::new();
                    let downloads = match repo
                        .get_downloads(subreddit.as_str(), config.downloads.sort, &blocklist)
                        .await
                    {
                        Err(err) => {
                            println!("[{}] Failed to get listing. Cause: {}", subreddit, err);
                            tx.send(()).unwrap();
                            return;
                        }
                        Ok(downloads) => downloads,
                    };

                    let mut receivers = Vec::new();
                    for download in downloads.into_iter() {
                        let (tx, rx) = oneshot::channel();
                        let handle = Handle::current();
                        let repo = repo.clone();

                        rayon::spawn(move || {
                            handle.spawn(async move {
                                if let Some(resp) = repo.download_image(&download).await {
                                    repo.store_image(resp, download).await;
                                };
                                tx.send(()).unwrap();
                            });
                        });
                        receivers.push(rx);
                    }
                    for rx in receivers.into_iter() {
                        rx.await.expect("a thread inside panicked");
                    }
                    tx.send(()).unwrap();
                });
            });
            receivers.push(rx);
        }
        for rx in receivers.into_iter() {
            rx.await.expect("a thread inside panicked");
        }
    }
}
