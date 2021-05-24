use crate::app::config::model::Config;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

pub fn new_agent(c: &Config) -> Agent {
    AgentBuilder::new()
        .user_agent(c.advanced.user_agent.as_str())
        .timeout_connect(Duration::from_millis(c.downloads.timeout))
        .timeout(Duration::from_millis(c.downloads.download_timeout))
        .build()
}
