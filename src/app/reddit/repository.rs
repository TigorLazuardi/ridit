use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::sort::Sort;
use reqwest::{self, Response};
use std::{error::Error, path::Path};
use std::{fs::write, io::Write};
use std::{
    fs::File,
    path::{self, PathBuf},
};
use tokio::task;
use tokio_retry::strategy::FixedInterval;
use tokio_retry::{self, Retry};

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
        println!("fetching listing from {}", subreddit_name);
        let stg = FixedInterval::from_millis(200).take(3);
        Retry::spawn(stg, || {
            self.internal_get_downloads(subreddit_name, sort, blocklist)
        })
        .await
    }

    async fn internal_get_downloads(
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
        downloads: Vec<DownloadMeta>,
    ) -> Vec<(Response, DownloadMeta)> {
        let mut result = Vec::new();
        for download in downloads.into_iter() {
            println!("downloading {}", download.url);
            let stg = FixedInterval::from_millis(200).take(3);
            match Retry::spawn(stg, || self.client.get(download.url.as_str()).send()).await {
                Ok(res) => result.push((res, download)),
                Err(err) => println!(
                    "failed downloading image from {}. Cause: {}",
                    download.url, err
                ),
            }
        }
        result
    }

    pub async fn store_images(
        &self,
        location: String,
        metas: Vec<(Response, DownloadMeta)>,
    ) -> Vec<DownloadMeta> {
        let v: Vec<DownloadMeta> = Vec::new();
        for (response, meta) in metas.into_iter() {
            let loc = location.clone().as_str();
            task::spawn(async {
                let loc = Path::new(loc).join(meta.filename);
                match File::create(location.as_str()) {
                    Ok(mut f) => {
                        while let chunk = response.chunk().await {
                            match chunk {
                                Ok(b) => match b {
                                    Some(b) => {}
                                    None => break,
                                },
                                Err(err) => {
                                    println!("failed to write to file");
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) => println!("failed to create "),
                }
            })
            .await;
        }
        v
    }
}
