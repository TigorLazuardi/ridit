use crate::app::{config::model::Config, reddit::repository::Repository};
use anyhow::Result;
use rayon::prelude::*;
use std::sync::{mpsc, Arc};

#[derive(Clone)]
pub struct DownloadService {
    repo: Arc<Repository>,
    config: Arc<Config>,
}

impl DownloadService {
    pub fn new(repo: Repository, config: Config) -> DownloadService {
        DownloadService {
            repo: Arc::new(repo),
            config: Arc::new(config),
        }
    }

    pub fn start_download(&self) {
        let mut a = self
            .config
            .downloads
            .subreddits
            .par_iter()
            .map(|x| {
                match self
                    .repo
                    .get_listing(x.as_str(), self.config.downloads.sort)
                {
                    Ok(v) => Some(v),
                    Err(err) => {
                        println!("{:?}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
            .into_iter();

        let (tx, rx) = mpsc::channel();
        while let Some(Some(downloads)) = a.next() {
            for download in downloads.into_iter() {
                let zelf = self.clone();
                let tx = tx.clone();
                rayon::spawn(move || {
                    let result = || -> Result<()> {
                        let response = zelf.repo.download_image(&download)?;
                        println!(
                            "[{}] downloading image from: {}",
                            download.subreddit_name, download.url
                        );
                        zelf.repo.store_image(response, &download)?;
                        println!(
                            "[{}] image downloaded from {} to {}",
                            download.subreddit_name,
                            download.url,
                            download
                                .get_file_location(zelf.config.downloads.path.as_str())
                                .display()
                        );
                        zelf.repo.create_symlink(&download)?;
                        Ok(())
                    }();
                    tx.send(result).unwrap();
                })
            }
        }
        drop(tx);
        loop {
            match rx.recv() {
                Ok(Err(err)) => println!("{:?}", err),
                Ok(_) => {}
                Err(_) => break,
            }
        }
    }
}
