use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error,
};

use crate::app::config::model::Config;

pub fn new_client(c: &Config) -> Result<Client, Error> {
    Client::builder()
        .connect_timeout(Duration::from_millis(c.downloads.timeout))
        .timeout(Duration::from_millis(c.downloads.download_timeout))
        .default_headers(default_headers(&c))
        .build()
}

fn default_headers(c: &Config) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(
        "USER-AGENT",
        HeaderValue::from_str(c.advanced.user_agent.as_str()).unwrap(),
    );
    h
}
