use super::meta::DownloadMeta;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Listing {
    pub kind: String,
    pub data: Data,
    subreddit_name: String,
}

impl Listing {
    pub fn get_urls(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        for data in self.data.children.iter() {
            result.push(data.data.url.as_str());
        }
        result
    }

    pub fn into_download_metas(self, blocklist: &Vec<String>) -> Vec<DownloadMeta> {
        let mut result: Vec<DownloadMeta> = Vec::new();
        for children in self.data.children.into_iter() {
            let data = children.data;
            for v in blocklist.iter() {
                if data.url == *v {
                    continue;
                }
            }
            let image_size: (u32, u32);
            match data.preview {
                Some(preview) => {
                    image_size = preview.get_image_size();
                }
                None => continue,
            }
            let meta = DownloadMeta {
                subreddit_name: self.subreddit_name.clone(),
                post_link: format!("https://reddit.com{}", data.permalink),
                image_width: image_size.0,
                image_height: image_size.1,
                url: data.url,
                nsfw: data.over18,
            };
            result.push(meta);
        }
        result
    }

    pub fn get_subreddit_name(&self) -> &str {
        self.subreddit_name.as_str()
    }

    pub fn set_subreddit_name(&mut self, s: String) {
        self.subreddit_name = s;
    }
}

#[derive(Deserialize)]
pub struct Data {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<Children>,
    pub after: String,
}

#[derive(Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: ChildrenData,
}

#[derive(Deserialize)]
pub struct ChildrenData {
    pub subreddit: String,
    pub selftext: String,
    pub author_fullname: String,
    pub saved: bool,
    pub gilded: i64,
    pub clicked: bool,
    pub title: String,
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: i64,
    pub link_flair_css_class: Option<String>,
    pub downs: i64,
    pub thumbnail_height: Option<i64>,
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    pub link_flair_text_color: String,
    pub upvote_ratio: f64,
    pub subreddit_type: String,
    pub ups: i64,
    pub total_awards_received: i64,
    pub media_embed: MediaEmbed,
    pub thumbnail_width: Option<i64>,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_meta: bool,
    pub secure_media_embed: SecureMediaEmbed,
    pub link_flair_text: Option<String>,
    pub can_mod_post: bool,
    pub score: i64,
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: bool,
    pub gildings: Gildings,
    pub post_hint: Option<String>,
    pub is_self: bool,
    pub created: f64,
    pub link_flair_type: String,
    pub wls: i64,
    pub author_flair_type: String,
    pub domain: String,
    pub allow_live_comments: bool,
    pub url_overridden_by_dest: String,
    pub archived: bool,
    pub no_follow: bool,
    pub is_crosspostable: bool,
    pub pinned: bool,
    pub over18: bool,
    pub preview: Option<Preview>,
    pub all_awardings: Vec<AllAwarding>,
    pub media_only: bool,
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    pub visited: bool,
    pub subreddit_id: String,
    pub link_flair_background_color: String,
    pub id: String,
    pub is_robot_indexable: bool,
    pub author: String,
    pub num_comments: i64,
    pub send_replies: bool,
    pub whitelist_status: String,
    pub contest_mode: bool,
    pub author_patreon_flair: bool,
    pub permalink: String,
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub url: String,
    pub subreddit_subscribers: i64,
    pub created_utc: f64,
    pub num_crossposts: i64,
    pub is_video: bool,
    pub is_gallery: Option<bool>,
    pub link_flair_template_id: Option<String>,
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
    pub fn get_image_size(&self) -> (u32, u32) {
        let source = &self.images[0].source;
        (source.width, source.height)
    }
}

#[derive(Deserialize)]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub variants: Variants,
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

#[derive(Deserialize)]
pub struct Variants {}

#[derive(Deserialize)]
pub struct AllAwarding {
    pub giver_coin_reward: Option<i64>,

    pub is_new: bool,
    pub days_of_drip_extension: i64,
    pub coin_price: i64,
    pub id: String,
    pub penny_donate: Option<i64>,
    pub award_sub_type: String,
    pub coin_reward: i64,
    pub icon_url: String,
    pub days_of_premium: i64,

    pub resized_icons: Vec<ResizedIcon>,
    pub icon_width: i64,
    pub static_icon_width: i64,

    pub is_enabled: bool,

    pub description: String,

    pub subreddit_coin_reward: i64,
    pub count: i64,
    pub static_icon_height: i64,
    pub name: String,
    pub resized_static_icons: Vec<ResizedStaticIcon>,
    pub icon_format: Option<String>,
    pub icon_height: i64,
    pub penny_price: Option<i64>,
    pub award_type: String,
    pub static_icon_url: String,
}

#[derive(Deserialize)]
pub struct ResizedIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Deserialize)]
pub struct ResizedStaticIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
