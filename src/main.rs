mod assets;
mod styles;

use anyhow::Context;
use clap::{CommandFactory, Parser, Subcommand};
use log::{LevelFilter, info};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::assets::Asset;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Setup DDNS with route53", long_about = None)]
#[command(styles = styles::get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Setup DDNS
    Setup {},

    /// Check and update DNS Records
    Update {},
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    env_logger::builder()
        .format_timestamp(None)
        .parse_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    match &cli.command {
        Some(Commands::Setup {}) => {
            println!("📦 setting up...");
            let service = Asset::get("ddns.service").context("Failed to find ddns.service")?;
            let timer = Asset::get("ddns.timer").context("Failed to find ddns.timer")?;

            let path = PathBuf::from("/etc/systemd/system");
            fs::create_dir_all(&path)?;

            let mut service_file = File::create(path.join("ddns.service"))?;
            let bytes = service_file
                .write(&service.data)
                .context("Failed to write ddns.service")?;
            info!("Wrote {} bytes to ddns.service", bytes);

            let mut timer_file = File::create(path.join("ddns.timer"))?;
            let bytes = timer_file.write(&timer.data)?;
            info!("Wrote {} bytes to ddns.timer", bytes);
            Ok(())
        }
        Some(Commands::Update {}) => Ok(()),
        None => Cli::command()
            .print_long_help()
            .context("Failed to print help message"),
    }
}
