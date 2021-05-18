use home;
use std::fs::{self, File};
use std::path::PathBuf;
use std::{error::Error, io::Write};

static DEFAULT_CONFIG: &'static str = r##"
[run]
# Prevent closing the cli window on task done.
hold_on_job_done = false

[downloads]
# The subreddits to subscribe to
subreddits = ["wallpaper", "wallpapers"]

# sort. valid values: "hot", "new", "top", "controversial", "rising" (case insensitive). Default or Invalid values are treated as "hot"
sort = "hot"

# Download concurrency. How many valid images to download at once
concurrency = 4

# Download location. Default $HOME/ridit/downloads on unix system
# On windows it should default to C:\Users\<user>\ridit\Downloads
# Windows support forward slash (/), but if you want to use backslash (\), make sure to use double backslash.
# e.g. "C:\\Users\\<user>\\ridit\\downloads"
path = "~/ridit/downloads"

[aspect_ratio]
enable = true
height_aspect = 9
width_aspect = 16

# ratio value is get by dividing `width_aspect` with `height_aspect`.
# if width_aspect = 16 and height_aspect = 9, then the ratio value is around 1.77~
# ratio_range of 0.2 means images that have ratio value around 1.57 to 1.97 is considered valid
# higher value of ratio_range means more valid images, but if wallpaper is set to `stretched` on your desktop, some of those images may look bad
ratio_range = 0.2

[minimum_size]
enable = true
minimum_height = 1080
minimum_width = 1920

"##;

static FILENAME: &'static str = "ridit.toml";

pub fn get_xdg_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut p = home::home_dir().ok_or("failed to detect user directory")?;
    if std::env::consts::OS == "windows" {
        p.push("AppData");
        p.push("local");
        p.push("ridit");
    } else {
        p.push(".config");
        p.push("ridit");
    }
    Ok(p)
}

pub fn get_xdg_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut p = home::home_dir().ok_or("failed to detect user directory")?;
    if std::env::consts::OS == "windows" {
        p.push("AppData");
        p.push("local");
        p.push("ridit");
    } else {
        p.push(".config");
        p.push("ridit");
    }
    p.push(FILENAME);
    Ok(p)
}

pub fn get_relative_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    match std::env::current_dir() {
        Ok(p) => Ok(p),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn get_relative_config_path() -> Result<PathBuf, Box<dyn Error>> {
    match std::env::current_dir() {
        Ok(mut p) => {
            p.push(FILENAME);
            Ok(p)
        }
        Err(err) => Err(Box::new(err)),
    }
}

/// check config exsits. First bool in tuple is relative dir, second bool in tuple is xdg dir
pub fn check_config_exists() -> (bool, bool) {
    let mut res = (false, false);
    if let Ok(p) = get_relative_config_path() {
        res.0 = p.exists();
    }
    if let Ok(p) = get_xdg_config_path() {
        res.1 = p.exists();
    }
    res
}

pub fn print_config() -> Result<PathBuf, Box<dyn Error>> {
    let b = DEFAULT_CONFIG.trim().as_bytes();

    // Writing to application dir if it has permissions
    if let Ok(p) = get_relative_config_path() {
        let dir = get_relative_config_dir()?;
        fs::create_dir_all(dir).ok();
        if let Ok(mut file) = File::create(p.clone()) {
            file.write_all(b)?;
            return Ok(p);
        };
    }

    // Writing to ~/.config or ~/AppData
    let p = get_xdg_config_path()?;
    let dir = get_xdg_config_dir()?;
    fs::create_dir_all(dir)?;
    let mut file = File::create(p.clone())?;
    file.write_all(b)?;
    Ok(p)
}
