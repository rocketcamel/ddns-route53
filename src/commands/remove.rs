use clap::Args;

use crate::{config::Config, prompt};

#[derive(Debug, Args)]
pub struct RemoveCommand;

impl RemoveCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut config = Config::parse()?;
        let remove_records = prompt::remove_records(&mut config)?;
        config.records.retain(|s| !remove_records.contains(s));
        config.write()?;
        Ok(())
    }
}
