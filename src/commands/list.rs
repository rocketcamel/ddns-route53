use clap::Args;
use crossterm::style::Stylize;

use crate::config::Config;

#[derive(Debug, Args)]
pub struct ListCommand;

impl ListCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let config = Config::parse()?;

        if config.records.is_empty() {
            println!("{}", "No records configured.".yellow());
            return Ok(());
        }

        println!("{}", "┌─ DDNS Route53 Records ─┐\n".blue().bold());

        println!(
            "{:<15} {}",
            "Zone ID:".cyan(),
            config.zone_id.clone().white()
        );
        println!("{:<15} {}", "TTL:".cyan(), config.ttl.to_string().white());
        println!();

        println!("{}", "Records:".green().bold());
        for (index, record) in config.records.iter().enumerate() {
            println!(
                "  {:<2} {}",
                format!("{}.", index + 1).dark_grey(),
                record.clone().white()
            );
        }

        println!();
        println!(
            "{}",
            format!(
                "Total: {} record{}",
                config.records.len(),
                if config.records.len() == 1 { "" } else { "s" }
            )
            .dark_grey()
        );

        Ok(())
    }
}
