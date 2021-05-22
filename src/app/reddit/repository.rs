use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::sort::Sort;
use async_fs::File;
use futures_lite::AsyncWriteExt;
use reqwest::{self, Response};
use std::{error::Error, path::Path};
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
        let stg = FixedInterval::from_millis(200).take(2);
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
        println!("downloading listing: {}", listing_url);

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
        location: &str,
        metas: Vec<(Response, DownloadMeta)>,
    ) -> Vec<DownloadMeta> {
        let mut v: Vec<DownloadMeta> = Vec::new();
        for (response, meta) in metas.into_iter() {
            match self.inner_store_image(location, response, meta).await {
                Ok(m) => v.push(m),
                Err(err) => println!("error storing image: {}", err),
            }
        }
        v
    }

    async fn inner_store_image(
        &self,
        location: &str,
        mut response: Response,
        meta: DownloadMeta,
    ) -> Result<DownloadMeta, Box<dyn Error>> {
        let full_file_name = vec![meta.filename.clone(), meta.ext.clone()].join("");
        let full_loc = Path::new(location)
            .join(meta.subreddit_name.as_str())
            .join(full_file_name.as_str());
        let mut f = File::create(full_loc).await?;
        'looper: loop {
            let chunk = response.chunk().await?;
            if let Some(b) = chunk {
                if let Err(err) = f.write(&b[..]).await {
                    println!(
                        "failed to write to write to {}. cause: {}",
                        meta.filename, err
                    );
                    break 'looper;
                }
            } else {
                if let Err(err) = f.flush().await {
                    println!("failed to flush file {}. cause: {}", meta.filename, err)
                };
                break 'looper;
            }
        }
        Ok(meta)
    }
}
