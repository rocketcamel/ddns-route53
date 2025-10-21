use clap::Args;
use crossterm::style::Stylize;
use inquire::Confirm;

use crate::{config::Config, prompt, records::Records};

#[derive(Debug, Args)]
pub struct RemoveCommand;

impl RemoveCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut config = Config::parse()?;
        if config.records.len() == 0 {
            println!("{}", "No records configured.".yellow());
            return Ok(());
        };

        let remove_records = prompt::remove_records(&mut config)?;
        config.records.retain(|s| !remove_records.contains(s));
        config.write()?;

        let delete_from_route53 =
            Confirm::new("Would you like to delete these records from the hosted zone?")
                .prompt()?;

        if !delete_from_route53 {
            return Ok(());
        }

        let records = Records::new().await;
        records.remove_records(&config, &remove_records).await?;

        Ok(())
    }
}
