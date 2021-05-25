# Ridit

I have not found a reddit image downloader that supports my needs like configurable aspect ratio downloads with minimum size.

This tool is mainly intended to download wallpapers and put them in neat locations.

Reddit Image Downloader CLI. It downloads images from subreddits.

Still WIP Alpha.

# Features

1. Configurable aspect ratio download and the ratio range.
2. Select minimum size image to download.
3. Symbolic Link feature to aggregate the downloaded images into one single folder without copying the actual file. Useful when using Windows 10 slideshow feature.
4. No user credentials needed.

# Unsupported/Untested (Yet)

1. Downloading albums. For now stick to just single image posts.

# How to Run

**WARNING:** Unix users should run the application from terminal window to know what's going on in the background.

1. Download the binary image on the [release page](https://github.com/TigorLazuardi/ridit/releases) according to your OS.
2. run the executable once. It should generate a config.
3. Open and configure the generated config file `ridit.toml`.
4. Run the executable once more.

# Troubleshoot

1. Got permission denied error. Try to change download path destination to somewhere your user has permission to. Make sure it also not pointing to a file.
2. Download failed. Try increasing the download_timeout in `ridit.toml`

# TODO

1. Implement function cachers to increase performance.
2. Add Album Download support
3. Add structured logging
4. Add history sqlite db.
5. Set this app as a service to continuously download at interval set of time.
