use home;
use std::fs::{self, File};
use std::path::PathBuf;
use std::{error::Error, io::Write};

static DEFAULT_CONFIG: &'static str = r##"
[run]
# Prevent closing the cli window on task done.
# Set to off if you run this application via scripting.
hold_on_job_done = true

[downloads]
# The subreddits to subscribe to
subreddits = ["wallpaper", "wallpapers"]

# Allow/Disallow nsfw
nsfw = true

# sort. valid values: "hot", "new", "top", "controversial", "rising" (case insensitive). Default or Invalid values are treated as "hot"
sort = "hot"

# Download location. Defaults to download folder relative to where the app is running from. (it uses its cwd value for relative paths)
# Windows user haves to use double backslash to write the path value or else the app will throw an error.
# If say download location is `C:\wallpapers\ridit`, Then it must be written like this: `C:\\wallpapers\\ridit` 
path = "downloads"

# Connection Timeout. Skips download if failed to establish connection for downloading in the allocated time. Value is in milliseconds.
# This is not timeout for downloading image. Your downloads won't fail if the download duration takes longer than the timeout.
timeout = 5000

# The app checks if file already exist. If it does, it will skip the download of the image. Set to true to force redownloading
proceed_download_on_file_exist = false

[aspect_ratio]
enable = true
height_aspect = 9
width_aspect = 16

# ratio value is get by dividing `width_aspect` with `height_aspect`.
# if width_aspect = 16 and height_aspect = 9, then the ratio value is around 1.77~
# ratio_range of 0.5 means images that have ratio value around 1.27 to 2.27 is considered valid
# 
# An image with resolution of 4096x2576 has a ratio value of 1.59~ (value gained by dividing 4096 with 2576).
# And it's valid because the ratio value is between 1.27 - 2.27
#
# higher value of ratio_range means more valid images, but if wallpaper is set to `stretched` on your desktop, some of those images may look bad
ratio_range = 0.5

[minimum_size]
enable = true
minimum_height = 1080
minimum_width = 1920

# Common users should have no need to change these values.
[advanced]
# User Agent is a way for reddit to know who is calling their services.
user_agent = "ridit"

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
