mod app;
use app::{
    config::{self, default},
    reddit::{client::new_client, repository::Repository},
    service::download::DownloadService,
};
use std::error::Error;
use std::process::exit;
use std::sync::Arc;
use std::{fs::create_dir_all, io::Read};
use std::{io::Write, path::Path};

#[tokio::main]
pub async fn execute() {
    match print_config() {
        Ok(new) => {
            if new {
                println!("configure and rerun the executable");
                exit(0)
            }
        }
        Err(err) => {
            println!("failed to create config: {}", err);
            exit(1);
        }
    }
    let c = config::read_config().unwrap();
    let hold = c.run.hold_on_job_done;
    create_dirs(c.downloads.path.as_str(), &c.downloads.subreddits).unwrap();
    let client = new_client(&c).unwrap();

    let conf = Arc::new(c);
    let repo = Repository::new(client, conf.clone());
    let service = DownloadService::new(repo, conf.clone());

    service.start_download().await;

    pause(hold);
}

fn create_dirs(location: &str, subreddits: &Vec<String>) -> Result<(), Box<dyn Error>> {
    for subs in subreddits {
        let p = Path::new(location).join(subs.as_str());
        create_dir_all(p)?;
    }
    Ok(())
}

fn print_config() -> Result<bool, Box<dyn Error>> {
    let (rel, xdg) = default::check_config_exists();
    if !rel && !xdg {
        let p = default::print_config()?;
        println!("config created on {}", p.display());
        return Ok(true);
    }
    Ok(false)
}

fn pause(pause: bool) {
    if pause {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        write!(stdout, "\nPress any key to continue...").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
