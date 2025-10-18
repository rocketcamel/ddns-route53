use std::{fs::File, io::Read};

use anyhow::Context;
use aws_config::BehaviorVersion;
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType,
};
use clap::Args;
use log::info;

use crate::config::Config;

#[derive(Debug, Args)]
pub struct UpdateCommand;

impl UpdateCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        let ddns_config = Config::parse().context("Failed to parse config file")?;

        let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
        let client = aws_sdk_route53::Client::new(&config);

        let ip = reqwest::get("https://ifconfig.me/ip").await?.text().await?;
        info!("IP: {}", ip);

        let changes = ddns_config
            .records
            .iter()
            .map(|record| {
                let record_set = ResourceRecordSet::builder()
                    .name(format!("{}.", record))
                    .r#type(RrType::A)
                    .ttl(ddns_config.ttl)
                    .resource_records(ResourceRecord::builder().value(&ip).build()?)
                    .build()?;
                Change::builder()
                    .resource_record_set(record_set)
                    .action(ChangeAction::Upsert)
                    .build()
            })
            .collect::<anyhow::Result<Vec<_>, _>>()?;

        let changes = ChangeBatch::builder().set_changes(Some(changes)).build()?;

        let response = client
            .change_resource_record_sets()
            .hosted_zone_id(&ddns_config.zone_id)
            .change_batch(changes)
            .send()
            .await
            .context("Error occurred changing record sets")?;

        info!("Updated Records: {:?}", response.change_info().unwrap());

        Ok(())
    }
}
