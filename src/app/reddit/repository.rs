use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::{model::Config, sort::Sort};
use async_fs::File;
use futures_lite::AsyncWriteExt;
use reqwest::{self, Response};
use std::{error::Error, path::Path, rc::Rc};
use tokio_retry::strategy::FixedInterval;
use tokio_retry::{self, Retry};

#[derive(Clone)]
pub struct Repository {
    client: reqwest::Client,
    config: Rc<Config>,
}

impl Repository {
    pub fn new(client: reqwest::Client, config: Rc<Config>) -> Repository {
        Repository { client, config }
    }

    pub async fn get_downloads(
        &self,
        subreddit_name: &str,
        sort: Sort,
        blocklist: &Vec<String>,
    ) -> Result<Vec<DownloadMeta>, Box<dyn Error>> {
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
        let listing_url = format!(
            "https://reddit.com/r/{}/{}.json?limit=100",
            subreddit_name, sort
        );
        println!("[{}] try fetching listing: {}", subreddit_name, listing_url);

        let listing: Listing = self
            .client
            .get(listing_url.as_str())
            .send()
            .await?
            .json()
            .await?;

        Ok(listing.into_download_metas(blocklist, self.config.clone()))
    }

    pub async fn download_images(
        &self,
        location: &str,
        downloads: Vec<DownloadMeta>,
    ) -> Vec<(Response, DownloadMeta)> {
        let mut result = Vec::new();
        for download in downloads.into_iter() {
            let loc = download.get_file_location(location);
            if loc.exists() && !self.config.downloads.proceed_download_on_file_exist {
                println!(
                    "[{}] file already exists: {}. skipping download from {}",
                    download.subreddit_name,
                    loc.display(),
                    download.url,
                );
                continue;
            }
            println!(
                "[{}] marking for download: {}",
                download.subreddit_name, download.url
            );
            let stg = FixedInterval::from_millis(200).take(3);
            match Retry::spawn(stg, || self.client.get(download.url.as_str()).send()).await {
                Ok(res) => result.push((res, download)),
                Err(err) => println!(
                    "[{}] failed downloading image from {}. Cause: {}",
                    download.subreddit_name, download.url, err
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
            match self.inner_store_image(location, response, &meta).await {
                Ok(_) => v.push(meta),
                Err(err) => println!(
                    "[{subreddit}] error storing image {filename}. cause: {error}",
                    subreddit = meta.subreddit_name,
                    filename = meta.filename,
                    error = err
                ),
            }
        }
        v
    }

    async fn inner_store_image(
        &self,
        location: &str,
        mut response: Response,
        meta: &DownloadMeta,
    ) -> Result<(), Box<dyn Error>> {
        let full_loc = Path::new(location)
            .join(meta.subreddit_name.as_str())
            .join(meta.filename.as_str());
        let mut f = File::create(full_loc.clone()).await?;
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
                match f.flush().await {
                    Ok(_) => println!(
                        "[{}] finished downloading {}. save location: {}",
                        meta.subreddit_name,
                        meta.url,
                        full_loc.display()
                    ),
                    Err(err) => println!(
                        "[{}] failed to flush file {}. cause: {}",
                        meta.subreddit_name,
                        full_loc.display(),
                        err
                    ),
                }
                break 'looper;
            }
        }
        Ok(())
    }
}
