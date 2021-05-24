use super::models::{listing::Listing, meta::DownloadMeta};
use crate::app::config::{model::Config, sort::Sort};
use async_fs::{self, File};
use futures_lite::AsyncWriteExt;
use path_absolutize::*;
use reqwest::{self, Response};
use std::sync::Arc;
use std::{error::Error, path::Path};
use tokio_retry::strategy::FixedInterval;
use tokio_retry::{self, Retry};

#[derive(Clone)]
pub struct Repository {
    client: reqwest::Client,
    config: Arc<Config>,
}

impl Repository {
    pub fn new(client: reqwest::Client, config: Arc<Config>) -> Repository {
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

    pub async fn download_image(&self, download: &DownloadMeta) -> Option<Response> {
        let loc = download.get_file_location(self.config.downloads.path.as_str());
        if loc.exists() && !self.config.downloads.proceed_download_on_file_exist {
            println!(
                "[{}] file already exists: {}. skipping download from {}",
                download.subreddit_name,
                loc.display(),
                download.url,
            );
            return None;
        };
        println!(
            "[{}] downloading: {}",
            download.subreddit_name, download.url
        );
        let stg = FixedInterval::from_millis(200).take(3);
        match Retry::spawn(stg, || self.client.get(download.url.as_str()).send()).await {
            Ok(res) => Some(res),
            Err(err) => {
                println!(
                    "[{}] failed downloading image from {}. Cause: {}",
                    download.subreddit_name, download.url, err
                );
                None
            }
        }
    }

    pub async fn store_image(&self, mut resp: Response, download: DownloadMeta) {
        let full_loc = download.get_file_location(self.config.downloads.path.as_str());
        let mut f = match File::create(full_loc.clone()).await {
            Ok(f) => f,
            Err(err) => {
                println!(
                    "[{}] failed creating file {}. cause: {}",
                    download.subreddit_name,
                    full_loc.display(),
                    err
                );
                return;
            }
        };

        'looper: loop {
            let chunk = match resp.chunk().await {
                Ok(c) => c,
                Err(err) => {
                    println!(
                        "[{}] response is closed from server when writing to file {}. cause: {}",
                        download.subreddit_name,
                        full_loc.display(),
                        err
                    );
                    return;
                }
            };
            if let Some(chunk) = chunk {
                if let Err(err) = f.write(&chunk[..]).await {
                    println!(
                        "failed to write to write to {}. cause: {}",
                        download.filename, err
                    );
                    break 'looper;
                }
            } else {
                match f.flush().await {
                    Ok(_) => {
                        println!(
                            "[{}] finished downloading {}. save location: {}",
                            download.subreddit_name,
                            download.url,
                            full_loc.display()
                        );
                        if let Err(err) = self.create_symlink(&download).await {
                            println!(
                                "[{}] failed to create symlink for file {}. cause: {}",
                                download.subreddit_name,
                                full_loc.display(),
                                err
                            );
                        };
                    }
                    Err(err) => println!(
                        "[{}] failed to flush file {}. cause: {}",
                        download.subreddit_name,
                        full_loc.display(),
                        err
                    ),
                }
                break 'looper;
            }
        }
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
        let full_loc = meta.get_file_location(location);
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
                    Ok(_) => {
                        println!(
                            "[{}] finished downloading {}. save location: {}",
                            meta.subreddit_name,
                            meta.url,
                            full_loc.display()
                        );
                        if let Err(err) = self.create_symlink(meta).await {
                            println!(
                                "[{}] failed to create symlink for file {}. cause: {}",
                                meta.subreddit_name,
                                full_loc.display(),
                                err
                            );
                        };
                    }
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

    async fn create_symlink(&self, meta: &DownloadMeta) -> Result<(), Box<dyn Error>> {
        if !self.config.symbolic_link.enable {
            return Ok(());
        }

        let download_path = Path::new(self.config.downloads.path.as_str())
            .absolutize()?
            .to_str()
            .unwrap()
            .to_string();

        if self.config.symbolic_link.use_custom_path {
            let custom_path = self.config.symbolic_link.custom_path.as_str();
            let target = Path::new(custom_path).join(meta.filename.as_str());
            async_fs::create_dir_all(custom_path).await?;
            symlink::symlink_file(meta.get_file_location(download_path.as_str()), target)?;
            return Ok(());
        }
        let joined_path = Path::new(download_path.as_str()).join("_joined");
        async_fs::create_dir_all(joined_path.clone()).await?;
        let target = joined_path.join(meta.filename.as_str());
        symlink::symlink_file(meta.get_file_location(download_path.as_str()), target)?;
        Ok(())
    }
}
