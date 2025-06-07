use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    vec,
};

use anyhow::Context;
use clap::Args;
use inquire::{CustomType, ui::RenderConfig};
use log::info;

use crate::{assets::Asset, config::Config};

#[derive(Debug, Args)]
pub struct SetupCommand;

impl SetupCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("📦 setting up...");
        let zone_id = inquire::Text::new("Hosted Zone ID").prompt()?;
        let record = inquire::Text::new("Subdomain")
            .with_placeholder("home.example.com")
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

        info!("Creating config.toml");
        let config_path = xdg::BaseDirectories::with_prefix("ddns-route53")
            .place_config_file("config.toml")
            .context("Cannot create config directory")?;

        File::create(&config_path)
            .context("Could not create config.toml")?
            .write(toml::to_string(&config)?.as_bytes())
            .context("Could not write config.toml")?;

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
}
