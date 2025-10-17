use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub zone_id: String,
    pub record: String,
    pub ttl: i64,
}

const CONFIG_NAME: &'static str = "ddns-route53";
const CONFIG_FILENAME: &'static str = "config.toml";

impl TryFrom<&str> for Config {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let config: Config = toml::from_str(value)?;
        Ok(config)
    }
}

impl TryFrom<&Path> for Config {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let contents = std::fs::read_to_string(path)?;
        contents.as_str().try_into()
    }
}

impl TryFrom<&PathBuf> for Config {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        path.try_into()
    }
}

impl TryInto<Vec<u8>> for &Config {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(toml::to_string(&self)
            .context("Failed to convert Config to toml")?
            .into_bytes())
    }
}

impl Config {
    pub fn parse() -> anyhow::Result<Self> {
        let path = xdg::BaseDirectories::with_prefix(CONFIG_NAME)
            .get_config_file(CONFIG_FILENAME)
            .context(format!("Failed to get {}", CONFIG_FILENAME))?;

        let config = Config::try_from(&path)
            .context(format!("Failed to parse toml from {}", CONFIG_FILENAME))?;

        Ok(config)
    }

    pub fn write(&self) -> anyhow::Result<usize> {
        let path = xdg::BaseDirectories::with_prefix(CONFIG_NAME)
            .place_config_file(CONFIG_FILENAME)
            .context(format!("Failed to create {}", CONFIG_FILENAME))?;

        let bytes: Vec<u8> = self.try_into()?;

        Ok(File::create(&path)?
            .write(&bytes)
            .context(format!("Failed to write to {}", CONFIG_FILENAME))?)
    }
}
