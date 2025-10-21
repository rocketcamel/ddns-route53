use clap::Args;
use log::info;

use crate::{config::Config, records::Records};

#[derive(Debug, Args)]
pub struct UpdateCommand;

impl UpdateCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        let records = Records::new().await;
        let config = Config::parse()?;
        let response = records.apply_all_records(&config).await?;

        info!("Updated Records: {:?}", response.change_info().unwrap());

        Ok(())
    }
}
