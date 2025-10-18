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
use log::{info, warn};

use crate::{
    assets::Asset,
    config::Config,
    service::{self, ServiceError},
};

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

        let bytes = config
            .write()
            .context("Failed to write config to filesystem")?;
        info!("Wrote {} bytes to config.toml", bytes);

        let service_file = Asset::get("ddns.service").context("Failed to get ddns.service")?;
        let timer_file = Asset::get("ddns.timer").context("Failed to get ddns.timer")?;

        let results = vec![
            service::write(&service_file.data, "ddns.service"),
            service::write(&timer_file.data, "ddns.timer"),
        ];

        let should_enable_service = !results.iter().all(|r| {
            if let Err(e) = r {
                e.chain().any(|err| {
                    err.downcast_ref::<ServiceError>()
                        .map(|se| {
                            matches!(
                                se,
                                ServiceError::SystemdNotFound | ServiceError::FileWrite(_)
                            )
                        })
                        .unwrap_or(false)
                })
            } else {
                false
            }
        });

        for result in results.iter() {
            if let Some(err) = result.as_ref().err() {
                warn!("{:#}", err)
            }
        }

        if !should_enable_service {
            info!(
                "systemd is required for the setup command to continue. you must create your own timers that run the \"update\" command"
            );
            return Ok(());
        }

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
