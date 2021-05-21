use crate::app::config::model::Config;
use crate::app::reddit::repository::Repository;
use std::error::Error;

pub struct DownloadService {
    repo: Repository,
    config: Config,
}

impl DownloadService {
    pub fn new(repo: Repository, config: Config) -> DownloadService {
        DownloadService { repo, config }
    }

    pub async fn start_download(&self) -> Result<(), Box<dyn Error>> {
        for subreddit in self.config.downloads.subreddits.iter() {
            let blocklist: Vec<String> = Vec::new();
            let downloads = self
                .repo
                .get_downloads(subreddit.as_str(), self.config.downloads.sort, &blocklist)
                .await?;

            let responses = self.repo.download_images(&downloads).await?;
            for response in responses.iter() {}
        }
        Ok(())
    }
}
