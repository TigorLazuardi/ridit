use crate::app::config::model::Config;

use super::meta::DownloadMeta;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Listing {
    // pub kind: String,
    pub data: Data,
}

impl Listing {
    pub fn into_download_metas(self, config: &Config) -> Vec<DownloadMeta> {
        let mut result: Vec<DownloadMeta> = Vec::new();
        for children in self.data.children.into_iter() {
            let data = children.data;
            if data.is_video {
                continue;
            }
            if data.over_18 && !config.downloads.nsfw {
                continue;
            }
            let image_size: (u32, u32);
            if let Some(preview) = data.preview {
                if let Some(preview) = preview.get_image_size() {
                    image_size = preview;
                } else {
                    continue;
                }
            } else {
                continue;
            }

            if !Listing::passed_aspect_ratio(image_size, config) {
                continue;
            }

            if !Listing::passed_mininum_size(image_size, config) {
                continue;
            }

            let meta = DownloadMeta {
                subreddit_name: data.subreddit,
                post_link: format!("https://reddit.com{}", data.permalink),
                image_width: image_size.0,
                image_height: image_size.1,
                url: data.url.clone(),
                nsfw: data.over_18,
                title: data.title,
                author: data.author,
                filename: Listing::get_filename_from_url(data.url.as_str()),
            };
            result.push(meta);
        }
        result
    }

    fn passed_aspect_ratio(image_size: (u32, u32), config: &Config) -> bool {
        if !config.aspect_ratio.enable {
            return true;
        }
        let ar = config.aspect_ratio.width_aspect as f32 / config.aspect_ratio.height_aspect as f32;
        let min_ratio = ar - config.aspect_ratio.ratio_range;
        let max_ratio = ar + config.aspect_ratio.ratio_range;
        let image_ratio = image_size.0 as f32 / image_size.1 as f32;
        image_ratio >= min_ratio && image_ratio <= max_ratio
    }

    fn passed_mininum_size(image_size: (u32, u32), config: &Config) -> bool {
        if !config.minimum_size.enable {
            return true;
        }
        image_size.0 >= config.minimum_size.minimum_width as u32
            && image_size.1 >= config.minimum_size.minimum_height as u32
    }

    fn get_filename_from_url(url: &str) -> String {
        url.split("/").last().unwrap().to_string()
    }
}

// fn slice_from_end(s: &str, n: usize) -> Option<&str> {
//     s.char_indices().rev().nth(n).map(|(i, _)| &s[i..])
// }

#[derive(Deserialize)]
pub struct Data {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<Children>,
    pub after: String,
}

#[derive(Deserialize)]
pub struct Children {
    pub data: ChildrenData,
}

#[derive(Deserialize)]
pub struct ChildrenData {
    pub subreddit: String,
    pub title: String,
    pub post_hint: Option<String>,
    pub created: f64,
    pub over_18: bool,
    pub preview: Option<Preview>,
    pub id: String,
    pub author: String,
    pub permalink: String,
    pub stickied: bool,
    pub url: String,
    pub is_video: bool,
    pub is_gallery: Option<bool>,
}

#[derive(Deserialize)]
pub struct MediaEmbed {}

#[derive(Deserialize)]
pub struct SecureMediaEmbed {}

#[derive(Deserialize)]
pub struct Gildings {
    pub gid1: Option<i64>,
    pub gid2: Option<i64>,
}

#[derive(Deserialize)]
pub struct Preview {
    pub images: Vec<Image>,
    pub enabled: bool,
}

impl Preview {
    /// tuple looks like this `(width, height)`
    pub fn get_image_size(&self) -> Option<(u32, u32)> {
        if let Some(img) = self.images.get(0) {
            let source = &img.source;
            return Some((source.width, source.height));
        }
        None
    }
}

#[derive(Deserialize)]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub id: String,
}

#[derive(Deserialize)]
pub struct Source {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct Resolution {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
