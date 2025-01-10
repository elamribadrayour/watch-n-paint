use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dataset: config::Dataset,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}
