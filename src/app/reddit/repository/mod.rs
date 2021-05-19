use super::domain::RedditRepository;
use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::{model::Config, sort::Sort};
use std::{error::Error, time::Duration};
use ureq::Agent;

pub struct Repository {
    config: Config,
    agent: Agent,
}

impl Repository {
    pub fn new(config: Config, agent: Agent) -> Repository {
        Repository { config, agent }
    }
}

impl RedditRepository for Repository {
    fn get_downloads(
        &self,
        subreddit_name: &str,
        sort: Sort,
        blocklist: &Vec<String>,
    ) -> Result<Vec<DownloadMeta>, Box<dyn Error>> {
        let listing_url = format!("https://reddit.com/r/{}/{}.json", subreddit_name, sort);
        let listing: Listing = self.agent.get(listing_url.as_str()).call()?.into_json()?;
        Ok(listing.into_download_metas(blocklist))
    }

    fn download_images(&self, download: DownloadMeta) -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
