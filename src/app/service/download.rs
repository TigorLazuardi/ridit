use crate::app::config::model::Config;
use crate::app::reddit::repository::Repository;

pub struct DownloadService {
    repo: Repository,
    config: Config,
}

impl DownloadService {
    pub fn new(repo: Repository, config: Config) -> DownloadService {
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
                Err(err) => println!("Failed to get listing from {}. Cause: {}", subreddit, err),
                Ok(downloads) => {
                    let responses = self.repo.download_images(downloads).await;
                    let d = self
                        .repo
                        .store_images(self.config.downloads.path.as_str(), responses)
                        .await;

                    for ddd in d.into_iter() {
                        println!(
                            "success download image [{}] {}",
                            ddd.subreddit_name, ddd.url
                        );
                    }
                }
            }
        }
    }
}
