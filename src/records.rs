use std::{collections::HashMap, thread::current};

use anyhow::Context;
use aws_config::BehaviorVersion;
use aws_sdk_route53::{
    Client,
    operation::change_resource_record_sets::ChangeResourceRecordSetsOutput,
    types::{Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType},
};
use crossterm::style::Stylize;
use log::{info, warn};

use crate::config::Config;

pub struct Records {
    client: Client,
}

impl Records {
    pub async fn new() -> Self {
        let aws_config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
        let client = Client::new(&aws_config);
        Self { client }
    }

    pub async fn apply_all_records(
        &self,
        config: &Config,
    ) -> anyhow::Result<ChangeResourceRecordSetsOutput> {
        let ip = reqwest::get("https://ifconfig.me/ip").await?.text().await?;
        info!("IP: {}", ip);

        let changes = config
            .records
            .iter()
            .map(|record| {
                let record_set = ResourceRecordSet::builder()
                    .name(format!("{}.", record))
                    .r#type(RrType::A)
                    .ttl(config.ttl)
                    .resource_records(ResourceRecord::builder().value(&ip).build()?)
                    .build()?;
                Change::builder()
                    .resource_record_set(record_set)
                    .action(ChangeAction::Upsert)
                    .build()
            })
            .collect::<anyhow::Result<Vec<_>, _>>()?;

        let changes = ChangeBatch::builder().set_changes(Some(changes)).build()?;

        let response = self
            .client
            .change_resource_record_sets()
            .hosted_zone_id(&config.zone_id)
            .change_batch(changes)
            .send()
            .await
            .context("Error occurred applying record sets")?;

        Ok(response)
    }

    pub async fn remove_records(
        &self,
        config: &Config,
        records: &Vec<String>,
    ) -> anyhow::Result<()> {
        let current_record_sets = self
            .client
            .list_resource_record_sets()
            .hosted_zone_id(&config.zone_id)
            .send()
            .await?
            .resource_record_sets()
            .iter()
            .filter(|rrs| rrs.r#type() == &RrType::A)
            .map(|rrs| (rrs.name().to_string(), rrs.clone()))
            .collect::<HashMap<String, ResourceRecordSet>>();

        let changes = records
            .iter()
            .filter_map(|record| {
                let record_set = current_record_sets.get(&format!("{}.", record))?;
                Some(
                    Change::builder()
                        .resource_record_set(record_set.clone())
                        .action(ChangeAction::Delete)
                        .build(),
                )
            })
            .collect::<anyhow::Result<Vec<_>, _>>()?;

        if changes.len() == 0 {
            println!("{}", "No changes to be removed.".yellow());
            return Ok(());
        }

        let changes = ChangeBatch::builder().set_changes(Some(changes)).build()?;
        let response = self
            .client
            .change_resource_record_sets()
            .hosted_zone_id(&config.zone_id)
            .change_batch(changes)
            .send()
            .await
            .context("Error occured removing record sets")?;

        warn!("{:?}", response);

        Ok(())
    }
}
