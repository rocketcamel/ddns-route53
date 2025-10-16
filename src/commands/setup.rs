use std::{
    fs::{self, File},
    io::{Write, stdout},
    path::PathBuf,
    process::Command,
    vec,
};

use anyhow::{Context, bail};
use clap::Args;
use crossterm::{
    QueueableCommand,
    style::{self, Print, ResetColor, SetForegroundColor},
};
use inquire::{Confirm, CustomType, ui::RenderConfig, validator::Validation};
use log::info;

use crate::{assets::Asset, config::Config};

#[derive(Debug, Args)]
pub struct SetupCommand;

impl SetupCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("📦 setting up...");
        let required_validator = |input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid("This is a required field.".into()))
            } else {
                Ok(Validation::Valid)
            }
        };

        let zone_id = inquire::Text::new("Hosted Zone ID")
            .with_validator(required_validator)
            .prompt()?;

        let record = inquire::Text::new("Domain or Subdomain")
            .with_placeholder("home.example.com")
            .with_validator(required_validator)
            .prompt()?;

        let ttl = CustomType {
            message: "TTL",
            starting_input: None,
            default: Some(300),
            validators: vec![],
            placeholder: None,
            help_message: None,
            formatter: &|i| format!("TTL: {}", i),
            default_value_formatter: &|i| format!("TTL: {}", i),
            error_message: "Please enter a valid number".to_owned(),
            parser: &|i| match i.parse::<i64>() {
                Ok(val) => Ok(val),
                Err(_) => Err(()),
            },
            render_config: RenderConfig::default_colored(),
        }
        .prompt()?;

        let config = Config {
            zone_id,
            record,
            ttl,
        };

        let bytes = config.write()?;
        info!("Wrote {} bytes to config.toml", bytes);

        let service = Asset::get("ddns.service").context("Failed to get ddns.service")?;
        let timer = Asset::get("ddns.timer").context("Failed to get ddns.timer")?;

        match fs::read_to_string("/proc/1/comm") {
            Ok(contents) => {
                if contents.trim() != "systemd" {
                    bail!(
                        "systemd is required for the setup command to continue, you must write your own timer if systemd is not present."
                    )
                }
            }
            Err(e) => bail!("Failed to read /proc/1/comm: {}", e),
        }

        let path = PathBuf::from("/etc/systemd/system");
        fs::create_dir_all(&path)?;

        let mut service_file = File::create(path.join("ddns.service")).context(
            "Could not create systemd service files. You must write your own timer if this does not work",
        )?;
        let bytes = service_file
            .write(&service.data)
            .context("Failed to write ddns.service")?;
        info!("Wrote {} bytes to ddns.service", bytes);

        let mut timer_file = File::create(path.join("ddns.timer"))?;
        let bytes = timer_file.write(&timer.data)?;
        info!("Wrote {} bytes to ddns.timer", bytes);
        info!(
            "✅ Setup complete. run systemctl enable --now ddns.timer to enable and start the service"
        );
        let enable_service =
            Confirm::new("Would you like to run these commands automatically?").prompt()?;

        if enable_service {
            let output = Command::new("systemctl")
                .args(["enable", "--now", "ddns.timer"])
                .output()
                .context("Failed to execute systemctl enable")?;

            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);

            let mut stdout = stdout();
            stdout.queue(SetForegroundColor(style::Color::Green))?;
            stdout.queue(Print(stdout_str))?;
            stdout.queue(ResetColor)?;

            if !stderr_str.trim().is_empty() {
                stdout.queue(SetForegroundColor(style::Color::Red))?;
                stdout.queue(Print(stderr_str))?;
                stdout.queue(ResetColor)?;
            };

            stdout.flush()?
        }

        Ok(())
    }
}
