mod app;
use app::config::{self, default};
use std::error::Error;
use std::process::exit;

#[tokio::main]
pub async fn execute() {
    if let Err(err) = print_config() {
        println!("failed to create config: {}", err);
        exit(1);
    }
    let c = config::read_config().unwrap();
    println!("{:#?}", c);
    println!("{}", c.downloads.sort)
}

fn print_config() -> Result<(), Box<dyn Error>> {
    let (rel, xdg) = default::check_config_exists();
    if !rel && !xdg {
        let p = default::print_config()?;
        println!("config created on {}", p.display());
    }
    Ok(())
}
