use std::{
    io::{Write, stdout},
    process::Command,
    vec,
};

use anyhow::Context;
use clap::Args;
use crossterm::{
    QueueableCommand,
    style::{self, Print, ResetColor, SetForegroundColor},
};
use inquire::Confirm;
use log::{info, warn};

use crate::{
    assets::Asset,
    config::Config,
    prompt,
    service::{self, ServiceError},
};

#[derive(Debug, Args)]
pub struct SetupCommand;

impl SetupCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("📦 setting up...");

        let zone_id = prompt::hosted_zone_id()?;
        let records = prompt::record_set()?;
        let ttl = prompt::ttl()?;

        let config = Config {
            zone_id,
            records: records,
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
