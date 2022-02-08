use std::fs::read_to_string;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: String,
}

pub fn read_to_config(path: String) -> Result<Config> {
    let raw = read_to_string(path)?;

    let conf: Config = toml::from_str(&raw)?;

    Ok(conf)
}
