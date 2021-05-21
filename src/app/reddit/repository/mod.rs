use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::sort::Sort;
use reqwest::{self, Response};
use std::error::Error;

#[derive(Clone)]
pub struct Repository {
    client: reqwest::Client,
}

impl Repository {
    pub fn new(client: reqwest::Client) -> Repository {
        Repository { client }
    }

    pub async fn get_downloads(
        &self,
        subreddit_name: &str,
        sort: Sort,
        blocklist: &Vec<String>,
    ) -> Result<Vec<DownloadMeta>, Box<dyn Error>> {
        let listing_url = format!("https://reddit.com/r/{}/{}.json", subreddit_name, sort);
        let listing: Listing = self
            .client
            .get(listing_url.as_str())
            .send()
            .await?
            .json()
            .await?;
        Ok(listing.into_download_metas(blocklist))
    }

    pub async fn download_images(
        &self,
        downloads: &Vec<DownloadMeta>,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let mut result = Vec::new();
        for download in downloads.iter() {
            let a = self.client.get(download.url.as_str()).send().await?;
            result.push(a);
        }
        Ok(result)
    }
}
