use std::error::Error;

use inquire::{CustomType, ui::RenderConfig, validator::Validation};

use crate::config::Config;

const REQUIRED_VALIDATOR: fn(&str) -> Result<Validation, Box<dyn Error + Send + Sync>> =
    |input: &str| {
        if input.is_empty() {
            Ok(Validation::Invalid("This is a required field.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

pub fn record_set() -> anyhow::Result<Vec<String>> {
    let records: Vec<String> = inquire::Text::new("Domain or Subdomain")
        .with_placeholder("home.example.com")
        .with_validator(REQUIRED_VALIDATOR)
        .prompt()?
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    Ok(records)
}

pub fn remove_records(config: &mut Config) -> anyhow::Result<Vec<String>> {
    let records: Vec<String> =
        inquire::MultiSelect::new("Domain or Subdomain(s) to remove:", config.records.clone())
            .prompt()?;
    Ok(records)
}

pub fn hosted_zone_id() -> anyhow::Result<String> {
    Ok(inquire::Text::new("Hosted Zone ID")
        .with_validator(REQUIRED_VALIDATOR)
        .prompt()?)
}

pub fn ttl() -> anyhow::Result<i64> {
    Ok(CustomType {
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
    .prompt()?)
}
