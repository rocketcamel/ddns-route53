use clap::Args;

use crate::{config::Config, prompt};

#[derive(Debug, Args)]
pub struct AddCommand;

impl AddCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut records = prompt::record_set()?;
        let mut config = Config::parse()?;
        records.retain(|r| !config.records.contains(r));

        config.records.append(&mut records);
        config.write()?;
        Ok(())
    }
}
