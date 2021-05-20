use super::domain::RedditRepository;
use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::sort::Sort;
use std::error::Error;
use std::io::Read;
use ureq::Agent;

pub struct Repository {
    agent: Agent,
}

impl Repository {
    pub fn new(agent: Agent) -> Repository {
        Repository { agent }
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

    fn download_image(&self, download: &DownloadMeta) -> Result<Box<dyn Read>, Box<dyn Error>> {
        let a = self.agent.get(download.url.as_str()).call()?.into_reader();
        Ok(Box::new(a))
    }
}
