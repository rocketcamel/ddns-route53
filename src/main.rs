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
    Setup {
        /// User that runs the job
        #[arg(short, long)]
        user: String,
    },

    /// Check and update DNS Records
    Update {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup { user }) => {
            println!("📦 setting up...");
            let cron = "* * * * * echo foo > /home/luca/src/ddns-route53/foo.txt";

            users::get_user_by_name(&user).expect("User does not exist");

            let path = PathBuf::from(format!("/var/spool/crontabs/{}", user));
            let _ = fs::create_dir_all(&path).unwrap();

            let mut file = File::create(path.join("ddns-route53-cron")).unwrap();
            match file.write(cron.as_bytes()) {
                Ok(b) => println!("Wrote {} bytes to ddns-route53-cron", b),
                Err(e) => eprintln!("Failed to write cron: {}", e),
            }
        }
        Some(Commands::Update {}) => {}
        None => Cli::command().print_long_help().unwrap(),
    }
}
