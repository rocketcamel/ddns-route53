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
        let ddns_config_path = xdg::BaseDirectories::with_prefix("ddns-route53")
            .find_config_file("config.toml")
            .context("Could not find config.toml")?;
        let mut ddns_buf = String::new();
        File::open(&ddns_config_path)?.read_to_string(&mut ddns_buf)?;
        let ddns_config: Config =
            toml::from_str(&ddns_buf).context("Failed to parse config.toml")?;

        let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
        let client = aws_sdk_route53::Client::new(&config);

        let ip = reqwest::get("https://ifconfig.me/ip").await?.text().await?;
        info!("IP: {}", ip);

        let record = ResourceRecordSet::builder()
            .name(format!("{}.", ddns_config.record))
            .r#type(RrType::A)
            .ttl(ddns_config.ttl)
            .resource_records(ResourceRecord::builder().value(&ip).build()?)
            .build()?;
        let change = Change::builder()
            .resource_record_set(record)
            .action(ChangeAction::Upsert)
            .build()?;
        let changes = ChangeBatch::builder().changes(change).build()?;

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
