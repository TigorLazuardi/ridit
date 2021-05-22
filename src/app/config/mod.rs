pub mod default;
pub mod model;
pub mod sort;

use model::Config;
use std::error::Error;
use std::fs;
use toml;

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let c: Config;
    if let Ok(p) = default::get_relative_config_path() {
        let content =
            fs::read(p).or_else(|err| Err(format!("failed to read riddit.toml: {}", err)))?;
        c = toml::from_slice(content.as_ref())?;
    } else {
        let p = default::get_xdg_config_path()?;
        let content = fs::read(p).or_else(|err| {
            Err(format!(
                "failed to read riddit.toml from global config path: {}",
                err
            ))
        })?;
        c = toml::from_slice(content.as_ref())?;
    }
    Ok(c)
}