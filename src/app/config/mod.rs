mod default;
pub mod model;
pub mod sort;
pub mod thread;

use anyhow::{Context, Result};
use model::Config;
use std::fs;
use toml;

pub fn read_config() -> Result<Config> {
    let c: Config;
    if let Ok(p) = default::get_relative_config_path() {
        let content = fs::read(&p)
            .with_context(|| format!("failed to read riddit.toml from {}", p.display()))?;
        c = toml::from_slice(content.as_ref())?;
    } else {
        let p = default::get_xdg_config_path()?;
        let content = fs::read(&p)
            .with_context(|| format!("failed to read riddit.toml on {}", p.display()))?;
        c = toml::from_slice(content.as_ref())?;
    }
    Ok(c)
}
