mod styles;

use anyhow::Context;
use clap::{CommandFactory, Parser, Subcommand};
use log::info;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

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
    env_logger::init();

    match &cli.command {
        Some(Commands::Setup {}) => {
            println!("📦 setting up...");
            let mut service =
                File::open("ddns.service").context("Could not find file: ddns.service")?;
            let mut timer = File::open("ddns.timer").context("Could not find file: ddns.timer")?;
            let mut service_buf = Vec::new();
            let mut timer_buf = Vec::new();
            service.read_to_end(&mut service_buf)?;
            timer.read_to_end(&mut timer_buf)?;

            let path = PathBuf::from("/etc/systemd/system");
            fs::create_dir_all(&path)?;

            let mut service_file = File::create(path.join("ddns.service"))?;
            let bytes = service_file
                .write(&service_buf)
                .context("Failed to write ddns.service")?;
            info!("Wrote {} bytes to ddns.service", bytes);

            let mut timer_file = File::create(path.join("ddns.timer"))?;
            let bytes = timer_file.write(&timer_buf)?;
            info!("Wrote {} bytes to ddns.timer", bytes);
            Ok(())
        }
        Some(Commands::Update {}) => Ok(()),
        None => Cli::command()
            .print_long_help()
            .context("Failed to print help message"),
    }
}
