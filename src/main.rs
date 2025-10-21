mod assets;
mod commands;
mod config;
mod prompt;
mod records;
mod service;
mod styles;

use anyhow::Context;
use clap::{CommandFactory, Parser};
use log::LevelFilter;

use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Setup DDNS with route53", long_about = None)]
#[command(styles = styles::get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    env_logger::builder()
        .format_timestamp(None)
        .parse_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    match &cli.command {
        Some(Commands::Setup(cmd)) => cmd.run()?,
        Some(Commands::Update(cmd)) => cmd.run().await?,
        Some(Commands::Add(cmd)) => cmd.run()?,
        Some(Commands::Remove(cmd)) => cmd.run().await?,
        Some(Commands::List(cmd)) => cmd.run()?,
        None => Cli::command()
            .print_long_help()
            .context("Failed to print help message")?,
    }

    Ok(())
}
