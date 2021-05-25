use std::{fs, fs::File, path::Path};

use anyhow::{Context, Error, Result};
use path_absolutize::Absolutize;
use retry::delay::Fixed;
use retry::retry;
use ureq::{Agent, Response};

use crate::app::config::{model::Config, sort::Sort};

use super::models::listing::Listing;
use super::models::meta::DownloadMeta;
use std::io;

#[derive(Clone)]
pub struct Repository {
    agent: Agent,
    config: Config,
}

// There should be no concurrency in repository level. concurrency should be done in service level.
impl Repository {
    pub fn new(agent: Agent, config: Config) -> Repository {
        Repository { agent, config }
    }

    pub fn get_listing(&self, subreddit: &str, sort: Sort) -> Result<Vec<DownloadMeta>> {
        let listing_url = format!("https://reddit.com/r/{}/{}.json?limit=100", subreddit, sort);
        println!("[{}] fetching listing from {}", subreddit, listing_url);
        let listing = retry(Fixed::from_millis(200).take(3), || {
            self.agent.get(listing_url.as_str()).call()
        })
        .with_context(|| format!("[{}] failed to get listing from {}", subreddit, listing_url))?
        .into_json::<Listing>()
        .with_context(|| {
            format!(
                "[{}] failed to parse response body into json from {}",
                subreddit, listing_url
            )
        })?;
        Ok(listing.into_download_metas(&self.config))
    }

    pub fn download_image(&self, download: &DownloadMeta) -> Result<Response> {
        let loc = download.get_file_location(self.config.downloads.path.as_str());
        if loc.exists() && !self.config.downloads.proceed_download_on_file_exist {
            return Err(Error::msg(format!(
                "[{}] file already exists: {}. skipping download from {}",
                download.subreddit_name,
                loc.display(),
                download.url,
            )));
        };
        let response = retry(Fixed::from_millis(200).take(3), || {
            self.agent.get(download.url.as_str()).call()
        })
        .with_context(|| {
            format!(
                "[{}] failed to open connection to {}",
                download.subreddit_name, download.url
            )
        })?;
        Ok(response)
    }

    pub fn store_image(&self, response: Response, download: &DownloadMeta) -> Result<()> {
        let full_loc = download.get_file_location(self.config.downloads.path.as_str());
        let mut f = File::create(full_loc.clone()).with_context(|| {
            format!(
                "[{}] failed creating file on {}",
                download.subreddit_name,
                full_loc.display()
            )
        })?;

        let mut buf = response.into_reader();
        io::copy(&mut buf, &mut f).with_context(|| {
            format!(
                "[{}] error when downloading image from {}",
                download.subreddit_name, download.url
            )
        })?;
        Ok(())
    }

    pub fn create_symlink(&self, download: &DownloadMeta) -> Result<()> {
        if !self.config.symbolic_link.enable {
            return Ok(());
        }

        let download_path = self.config.get_download_path().display().to_string();

        let file_path = download.get_file_location(download_path.as_str());

        if self.config.symbolic_link.use_custom_path {
            let custom_path = Path::new(self.config.symbolic_link.custom_path.as_str())
                .absolutize()?
                .to_path_buf();
            fs::create_dir_all(custom_path.as_path())
                .with_context(|| format!("failed to create folder on {}", custom_path.display()))?;
            let target = custom_path.join(download.filename.as_str());
            let src = download.get_file_location(download_path.as_str());
            symlink::symlink_file(src.clone(), target.clone()).with_context(|| {
                format!(
                    "[{}] failed to create symlink from {} to {}",
                    download.subreddit_name,
                    src.display(),
                    target.display()
                )
            })?;
            return Ok(());
        }

        let joined_path = Path::new(download_path.as_str()).join("_joined");
        fs::create_dir_all(joined_path.as_path())?;
        let target = joined_path.join(download.filename.as_str());
        symlink::symlink_file(file_path.clone(), target.clone()).with_context(|| {
            format!(
                "[{}] failed to create symlink from {} to {}",
                download.subreddit_name,
                file_path.display(),
                target.display()
            )
        })?;
        Ok(())
    }
}
