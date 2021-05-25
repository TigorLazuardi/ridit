mod app;
use anyhow::Result;
use app::{
    config::{self, model::Config, thread::configure_concurrency},
    reddit::{agent::new_agent, repository::Repository},
    service::download::DownloadService,
};
use std::io::Read;
use std::io::Write;
use std::process::exit;

pub fn execute() {
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
    c.create_dirs().unwrap();
    configure_concurrency(0).unwrap();
    let agent = new_agent(&c);

    let repo = Repository::new(agent, c.clone());
    let service = DownloadService::new(repo, c);

    service.start_download();

    pause(hold);
}

fn print_config() -> Result<bool> {
    let (rel, xdg) = Config::check_config_exists();
    if !rel && !xdg {
        let p = Config::print_config()?;
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
