use clap::{CommandFactory, Parser, Subcommand};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Setup DDNS with route53", long_about = None)]
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup {}) => {
            println!("📦 setting up...");
            let mut service =
                File::open("ddns.service").expect("Could not find file: ddns.service");
            let mut timer = File::open("ddns.timer").expect("Could not find file: ddns.timer");
            let mut service_buf = Vec::new();
            let mut timer_buf = Vec::new();
            let _ = service.read_to_end(&mut service_buf);
            let _ = timer.read_to_end(&mut timer_buf);

            let path = PathBuf::from("/etc/systemd/system");
            let _ = fs::create_dir_all(&path).unwrap();

            let mut service_file = File::create(path.join("ddns.service")).unwrap();
            match service_file.write(&service_buf) {
                Ok(b) => println!("Wrote {} bytes to ddns.service", b),
                Err(e) => eprintln!("Failed to write ddns.service: {}", e),
            }
            let mut timer_file = File::create(path.join("ddns.timer")).unwrap();
            match timer_file.write(&timer_buf) {
                Ok(b) => println!("Wrote {} bytes to ddns.timer", b),
                Err(e) => eprintln!("Failed to write ddns.timer: {}", e),
            }
        }
        Some(Commands::Update {}) => {}
        None => Cli::command().print_long_help().unwrap(),
    }
}
